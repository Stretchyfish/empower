use std::sync::{Arc, Mutex};

use empower_engine::{PortValue, node_graph::{Variable, Variables}};

use crate::{studio_context::StudioContext, user_inputs::UserInputs};

use super::Viewport;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableEditorViewport
{
    mode: VariableEditorViewportMode,
    new_variable: Variable,
}

#[typetag::serde]
impl Viewport for VariableEditorViewport
{
    fn new() -> Box<dyn Viewport> where Self:Sized {
        Box::new(
            Self
            {
                mode: VariableEditorViewportMode::CreateVariable,
                new_variable: Variable::new(),
            }
        )
    }

    fn clone_box(&self) -> Box<dyn Viewport>  {
        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "variable editor viewport"
    }

    fn show(&mut self,ui: &mut egui::Ui,studio_context: &mut StudioContext, _: &String, _: &UserInputs) {

        let variables = studio_context.get_project_mut().graph_editor.node_graph.get_variables_mut();

        ui.horizontal(|ui|
        {
            ui.selectable_value(&mut self.mode, VariableEditorViewportMode::CreateVariable, "add variable");
            ui.selectable_value(&mut self.mode, VariableEditorViewportMode::ShowVariables, "show variables");
            ui.selectable_value(&mut self.mode, VariableEditorViewportMode::ShowVariable, "edit variable");
        });

        ui.separator();

        match self.mode
        {
            VariableEditorViewportMode::ShowVariable => self.show_edit_variable_panel(ui, variables),
            VariableEditorViewportMode::CreateVariable => self.show_create_variable_panel(ui, variables),
            VariableEditorViewportMode::ShowVariables => self.show_all_variables_panel(ui, variables),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
enum VariableEditorViewportMode // @TODO, make sure these names match the show functions names
{
    CreateVariable,
    ShowVariable,
    ShowVariables,
}

impl VariableEditorViewport
{
    fn show_create_variable_panel(&mut self, ui: &mut egui::Ui, variables: &mut Variables)
    {
        ui.horizontal(|ui|
        {
            ui.label("Variable name: ");
            ui.text_edit_singleline(&mut self.new_variable.name);

            if ui.button("reset").clicked()
            {
                self.new_variable = Variable::new();
            }
        });

        ui.separator();

        ui.horizontal(|ui|
        {
            ui.label("values");

            if ui.button("add value").clicked()
            {
                self.new_variable.values.insert(String::new(), PortValue::None);
            }
        });

        for value_name in self.new_variable.values.clone().keys()
        {
            ui.horizontal(|ui|
            {
                let mut new_value_name = value_name.clone();
                ui.text_edit_singleline(&mut new_value_name);

                if new_value_name != *value_name
                {
                    let value_clone = self.new_variable.values.get(value_name).unwrap().clone(); // This unwrap should be safe
                    self.new_variable.values.remove(value_name);
                    self.new_variable.values.insert(new_value_name.clone(), value_clone);
                }

                let value_mutable = self.new_variable.values.get_mut(&new_value_name).unwrap();

                ui.menu_button( value_mutable.type_name(), |ui|
                {
                    if ui.button("integer").clicked()
                    {
                        *value_mutable = PortValue::Integer(0);
                    }
                    if ui.button("float").clicked()
                    {
                        *value_mutable = PortValue::Float(0.0);
                    }
                    if ui.button("bool").clicked()
                    {
                        *value_mutable = PortValue::Bool(false);
                    }
                    if ui.button("text").clicked()
                    {
                        *value_mutable = PortValue::Text(String::new());
                    }
                    if ui.button("vector").clicked()
                    {
                        *value_mutable = PortValue::Vector(Vec::new());
                    }
                });
            });
        }

        ui.separator();
        
        if ui.button("Add variable").clicked()
        {
            variables.insert(self.new_variable.name.clone(), Arc::new( Mutex::new( self.new_variable.clone() )));
            self.new_variable = Variable::new();
        }
    }

    fn show_edit_variable_panel(&mut self, _: &mut egui::Ui, _: &mut Variables)
    {
        
    }

    fn show_all_variables_panel(&mut self, ui: &mut egui::Ui, variables: &mut Variables)
    {
        egui::ScrollArea::vertical().show(ui, |ui|
        {
            egui::Grid::new("show_variables")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui|
            {
                for (name, variable) in variables
                {
                    ui.add_sized([150.0, 20.0], egui::Label::new(name));

                    // @TODO, this is very unsafe
                    egui::CollapsingHeader::new(format!("values {}", variable.lock().unwrap().values.len()))
                    .id_salt(name)
                    .show(ui, |ui|
                    {
                        for (name, value) in &variable.lock().unwrap().values
                        {
                            ui.label(format!("{}: {}", name, value));
                        }
                    });

                    ui.end_row();
                }
            });
        });
    }
}
