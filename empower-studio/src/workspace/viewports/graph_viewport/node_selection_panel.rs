use egui_dock::node;
use empower_node_graph::NodeType;
use crate::graph_editor::{self, GraphEditor};
use super::user_input::UserInputs;

#[derive(Default)]
pub struct NodeSelectionPanel
{
    pub visible: bool,
    pub search_text: String,
    pub mouse_position_when_node_select_menu_was_activated: Option<egui::Pos2>,
    
}

impl NodeSelectionPanel // @TODO, consider a better name for this?
{
    pub fn new() -> Self
    {
        Self
        {
            visible: false,
            search_text: String::new(),
            mouse_position_when_node_select_menu_was_activated: Option::None, // @TODO, make this non-optional, in practice has no effect
        }
    }
}

// @TODO, simplify this function, especially the user_input and mouse postion
pub fn show(ui: &mut egui::Ui, node_selection_panel: &mut NodeSelectionPanel, graph_editor: &mut GraphEditor, user_inputs: &UserInputs, mouse_position_in_scene: &egui::Pos2)
{    
    if user_inputs.right_clicked // @TODO, consider moving this into the show function?
    {
        node_selection_panel.visible = !node_selection_panel.visible;

        if node_selection_panel.visible // @TODO, find a more elegant way of writting this
        {
            node_selection_panel.mouse_position_when_node_select_menu_was_activated = Some(user_inputs.mouse_position);
        }
    }

    if node_selection_panel.visible == false
    {
       return; 
    }
    
    let window_position = node_selection_panel.mouse_position_when_node_select_menu_was_activated.unwrap_or_else(|| user_inputs.mouse_position );
    
    let node_selection_window = egui::Window::new("").current_pos(egui::Pos2 {x: window_position.x - 100.0, y: window_position.y - 15.0}).collapsible(false).max_size(egui::Vec2 {x: 200.0, y: 200.0}).title_bar(false);

    node_selection_window.show(ui.ctx(), |ui|
    {                    
        // @TODO, make it focus on writting text when oppened

        ui.text_edit_singleline(&mut node_selection_panel.search_text);
        egui::ScrollArea::vertical()
        .max_height(200.0)
        .max_width(200.0)
        .show(ui, |ui| 
        {
            ui.group(|ui|
            {
                if ui.add(egui::Button::new("int variable").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                {                                        
                    let new_node_type = NodeType::IntegerVariable;
                    graph_editor.add_node(new_node_type, *mouse_position_in_scene );

                    node_selection_panel.visible = false;
                }
    
                if ui.add(egui::Button::new("int add value").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                {
                    println!("Clicked another button");
                }
    
                ui.add(egui::Button::new("test (no function)").min_size(egui::Vec2 {x: 190.0, y: 20.0}));
                ui.add(egui::Button::new("test (no function)").min_size(egui::Vec2 {x: 190.0, y: 20.0}));
            });
        });
    });   

}
