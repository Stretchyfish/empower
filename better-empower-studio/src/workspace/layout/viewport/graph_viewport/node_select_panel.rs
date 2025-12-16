use crate::graph_editor::GraphEditor;

#[derive(Default)]
pub struct NodeSelectionPanel
{
    search_text: String,
    mouse_position_when_node_select_menu_was_activated: egui::Pos2,
    node_to_add: Option<&'static str>,
}

impl NodeSelectionPanel 
{
    // @TODO, find a better way to approach position  
    pub fn new(mouse_position_when_node_select_menu_was_activated: egui::Pos2) -> Self 
    {
        Self
        {
            search_text: String::new(),
            mouse_position_when_node_select_menu_was_activated,
            node_to_add: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, mouse_position_in_scene: &egui::Pos2) -> bool
    {
        if self.node_to_add.is_some()
        {
            graph_editor.add_node(self.node_to_add.unwrap(), *mouse_position_in_scene);
            return true;
        }

        let window_position = self.mouse_position_when_node_select_menu_was_activated;
        let node_selection_window = egui::Window::new("")
                                                .current_pos(egui::Pos2 {x: window_position.x - 100.0, y: window_position.y - 15.0})
                                                .collapsible(false)
                                                .max_size(egui::Vec2 {x: 200.0, y: 200.0})
                                                .title_bar(false);

        node_selection_window.show(ui.ctx(), |ui|
        {
            ui.text_edit_singleline(&mut self.search_text);
            egui::ScrollArea::vertical()
            .max_height(200.0)
            .max_width(200.0)
            .show(ui, |ui| 
            {
                ui.group(|ui|
                {
                    if ui.add(egui::Button::new("Number").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "number" );
                    };

                    if ui.add(egui::Button::new("Addition").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "addition" );
                    };

                    if ui.add(egui::Button::new("Multiply").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "multiply" );
                    };

                    if ui.add(egui::Button::new("Vector").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "vector" );
                    };

                    if ui.add(egui::Button::new("Print").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "print" );
                    };

                    if ui.add(egui::Button::new("file path").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "file path" );
                    };

                    if ui.add(egui::Button::new("show image").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "show image" );
                    };

                    if ui.add(egui::Button::new("math graph").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                    {
                        self.node_to_add = Some( "math graph" );
                    };
                });
            });
        });

        false
    }
}
