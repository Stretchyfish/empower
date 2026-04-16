use crate::{studio_context::project::GraphEditor, user_inputs::{self, UserInputs}};

const NODES_NAMES_AVAILABLE: &'static [&str] = &[
    "number",
    "condition",    
    "loop",
    "wait",
    "restart loop",
    "stop loop",
    "boolean",
    "text",
    "addition",
    "multiply",
    "vector",
    "print",
    "variable",
    "file path",
    "show image",
    "math graph"
];

// @TODO, looks like the serde serialization and deserialization could be removed here with a bit of rewritting
#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeSelectionPanel
{
    search_text: String,
    mouse_position_when_node_select_menu_was_activated: egui::Pos2,

    // @TODO, these should not be serialized    
    highlighted_node_index: Option<usize>,
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
            highlighted_node_index: None,
        }
    }

    // pub fn show(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, mouse_position_in_scene: &egui::Pos2, user_inputs: &UserInputs) -> bool
    pub fn show(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, mouse_position_in_scene: &egui::Pos2) -> bool
    {
        let mut node_to_add = None;

        let window_position = self.mouse_position_when_node_select_menu_was_activated;
        let node_selection_window = egui::Window::new("")
                                                .current_pos(egui::Pos2 {x: window_position.x - 100.0, y: window_position.y - 15.0})
                                                .collapsible(false)
                                                .max_size(egui::Vec2 {x: 200.0, y: 200.0})
                                                .title_bar(false);

        let mut node_showed_number = 0; // Used later in the big scope for the total number of nodes and in the smaller scope for indexing

        node_selection_window.show(ui.ctx(), |ui|
        {
            let singleline_edit = ui.text_edit_singleline(&mut self.search_text);
            singleline_edit.request_focus();

            if singleline_edit.changed()
            {
                self.highlighted_node_index = Some( 0 );
            }

            if self.search_text.is_empty()
            {
                self.highlighted_node_index = None;
            }

            egui::ScrollArea::vertical()
            .max_height(200.0)
            .max_width(200.0)
            .show(ui, |ui| 
            {
                ui.group(|ui|
                {
                    
                    for node_name in NODES_NAMES_AVAILABLE
                    {
                        if self.search_text.len() > 0
                        {
                            if !(*node_name).contains( &self.search_text.to_lowercase() )
                            {
                                continue;
                            }
                        }

                        let mut current_node_is_selected = false;
                        if self.highlighted_node_index.is_some()
                        {
                            if node_showed_number == self.highlighted_node_index.unwrap() // Cast to i32 to avoid subtrackt with overflow error
                            {
                                current_node_is_selected = true;
                            }
                        }

                        let button = egui::Button::new( *node_name )
                        .min_size( egui::Vec2 {x: 190.0, y: 20.0} )
                        .fill(

                            if current_node_is_selected
                            {
                                egui::Color32::YELLOW
                            }
                            else
                            {
                                ui.visuals().widgets.inactive.bg_fill
                            }
                        );
                        
                        if ui.add(button).clicked() 
                        {
                            node_to_add = Some( *node_name );
                        };

                        node_showed_number += 1;

                        let user_clicked_enter = ui.ctx().input(|i| { i.key_pressed(egui::Key::Enter) });
                        if user_clicked_enter && current_node_is_selected
                        {
                            node_to_add = Some( *node_name );
                        }
                    }
                });
            });
        });

        if self.highlighted_node_index.is_some() && node_showed_number != 0 // important to check for 0 due to -1 later
        {
            let highlighted_index = self.highlighted_node_index.as_mut().unwrap();

            let clicked_up_arrow =  ui.ctx().input(|i| { i.key_pressed(egui::Key::ArrowUp) });
            let clicked_down_arrow =  ui.ctx().input(|i| { i.key_pressed(egui::Key::ArrowDown) });

            if clicked_up_arrow && *highlighted_index != 0
            {
                *highlighted_index -= 1;
            }

            if clicked_down_arrow && *highlighted_index < node_showed_number - 1 // node_showed_number in this case is the max number of nodes currently shown
            {
                *highlighted_index += 1;
            }
        }

        if node_to_add.is_some()
        {
            graph_editor.add_node(node_to_add.unwrap(), *mouse_position_in_scene);
            // action_queue.push( Action::CreateNode { name: node_to_add.unwrap(), position: *mouse_position_in_scene });
            return true;
        }

        false
    }
}
