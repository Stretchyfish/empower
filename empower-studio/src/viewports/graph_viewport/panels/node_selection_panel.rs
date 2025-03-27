use egui;
use crate::viewports::graph_viewport;
use crate::NodeGraph;
use crate::interactions;

#[derive(Default)]
pub struct NodeSelectionPanelState
{
    pub show: bool,
    pub search_text: String,
    pub mouse_position_when_node_select_menu_was_activated: Option<egui::Pos2>,
    
}

impl NodeSelectionPanelState
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            search_text: String::new(),
            mouse_position_when_node_select_menu_was_activated: Option::None,
        }
    }
}

pub fn view_node_selector(node_selection_panel_state: &mut NodeSelectionPanelState, node_graph: &mut NodeGraph, ui: &mut egui::Ui, user_input: &interactions::user::UserInputs, pan_zoom: &mut graph_viewport::PanZoom)
{
    let window_position = node_selection_panel_state.mouse_position_when_node_select_menu_was_activated.unwrap_or_else(|| user_input.mouse_position );
    
    let node_selection_window = egui::Window::new("").current_pos(egui::Pos2 {x: window_position.x - 100.0, y: window_position.y - 15.0}).collapsible(false).max_size(egui::Vec2 {x: 200.0, y: 200.0}).title_bar(false);

    node_selection_window.show(ui.ctx(), |ui|
    {                    
        // @TODO, make it focus on writting text when oppened

        ui.text_edit_singleline(&mut node_selection_panel_state.search_text);
        egui::ScrollArea::vertical()
        .max_height(200.0)
        .max_width(200.0)
        .show(ui, |ui| 
        {
            ui.group(|ui|
            {
                if ui.add(egui::Button::new("int variable").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                {                                        
                    node_graph.add_node();
                    // let new_node_type = empower_engine::nodes::NodeType::IntVariable;
                    // graph_state.add_node_at_position_in_canvas(new_node_type, &canvas_position);
                    //graph_state.add_node_at_position(new_node_type, &canvas_position);
                    //graph_state.add_node_at_mouse_position(new_node_type, &user_input.mouse_position);    
                    
                    node_selection_panel_state.show = false;
                }
    
                if ui.add(egui::Button::new("int add value").min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked() // @TODO, get rid of these hard coded values (if possible?)
                {
                    let new_node_world_position = pan_zoom.screen_to_world(&user_input.mouse_position); // Write this better

                    println!("Newly placed node screen pos: {}, {}", user_input.mouse_position.x, user_input.mouse_position.y);
                    println!("Newly placed node world pos: {}, {}", new_node_world_position.x, new_node_world_position.y);
                    
                    node_graph.add_node_at_position(new_node_world_position);
                    // let new_node_type = empower_engine::nodes::NodeType::IntAddValue;
                    //graph_state.add_node_at_mouse_position(new_node_type, &user_input.mouse_position);

                    // graph_state.add_node_at_position_in_canvas(new_node_type, &canvas_position);

                    //graph_state.add_node_centered_at_mouse_position(new_node_type, &user_input.mouse_position);

                    node_selection_panel_state.show = false;
                }
    
                ui.add(egui::Button::new("test (no function)").min_size(egui::Vec2 {x: 190.0, y: 20.0}));
                ui.add(egui::Button::new("test (no function)").min_size(egui::Vec2 {x: 190.0, y: 20.0}));
            });
        });
    });   
}
