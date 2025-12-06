use better_empower_engine::NodeGraphKey;
use egui;

use crate::{GraphEditor, workspace::layout::viewport::graph_viewport::node_widget::NodeWidgetResponse};

use super::Viewport;

mod user_inputs;
mod node_widget;

pub struct GraphViewport
{
    mouse_scene_position_last_frame: egui::Pos2, // @TODO, only temporary public for debug purpose
    mouse_scene_delta: egui::Vec2, // @TODO, consider a better approach for storing this, istead of at struck level?
    scene_rect: egui::Rect,
}

impl Viewport for GraphViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {
        
        Box::new( 
            Self 
            {
                mouse_scene_position_last_frame: egui::Pos2::ZERO,
                mouse_scene_delta: egui::Vec2::ZERO,
                scene_rect: egui::Rect { min: egui::Pos2 { x: -650.0, y: -650.0 }, max: egui::Pos2 { x: 650.0, y: 650.0 }},
            } 
        )
    }

    fn name(&self) -> &'static str {
        "graph viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor) {
 
        let widget_responses = self.view_canvas(ui, graph_editor);
        self.process_widget_responses(&widget_responses, graph_editor);
        self.process_user_actions(graph_editor);
    }
}

impl GraphViewport
{
    fn view_canvas(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor) -> Vec<NodeWidgetResponse>
    {
        let mut scene_rect = self.scene_rect.clone(); // This is needed to avoid borrow issues

        let mut mouse_position_in_scene = self.mouse_scene_position_last_frame; // Set to last frame, in case there is no new position in the scene
        let mut mouse_scene_delta = egui::Vec2::ZERO; // @TODO, take another look at this placement
        // let mut node_widgets_responses = Vec::new();

        let mouse_pointer_inside_viewport = ui.rect_contains_pointer(ui.min_rect());

        // @TODO, this is not a great way to approach user inputs, so fix in the future!
        let user_inputs = user_inputs::get_graph_viewport_user_inputs(ui);

        let mut drag_pan_button = egui::DragPanButtons::PRIMARY;
        if user_inputs.left_shift_is_down
        {
            drag_pan_button = egui::DragPanButtons::empty();
        }

        let mut interaction_happened_this_loop = false; // @TODO, find a better way of doing this

        let mut widget_responses = Vec::new();

        egui::Scene::new()
        .zoom_range(0.01..=2.0)
        .max_inner_size(egui::Vec2 { x: 200.0, y: 200.0 })
        .drag_pan_buttons(drag_pan_button)
        .show(ui, &mut scene_rect, |scene_ui|
        {
            if mouse_pointer_inside_viewport // Is needed to avoid applying double delta position to selected nodes
            {
                let scene_transform = scene_ui.ctx().layer_transform_from_global(scene_ui.painter().layer_id());
                let scene_latest_pos = scene_ui.input(|i| i.pointer.latest_pos());
                
                if scene_transform.is_some() && scene_latest_pos.is_some()
                {
                    mouse_position_in_scene = scene_transform.unwrap() * scene_latest_pos.unwrap();
                }
            }

            mouse_scene_delta = mouse_position_in_scene - self.mouse_scene_position_last_frame; 
            self.mouse_scene_delta = mouse_scene_delta;

            let node_keys: Vec<NodeGraphKey> = graph_editor.display_nodes.keys().cloned().collect(); // @TODO, find a more elegant way of writting this
            for node_key in node_keys
            {
                let node_widget_response = node_widget::show(scene_ui, graph_editor, &node_key, self.name());

                if node_widget_response.is_some()
                {
                    widget_responses.push(node_widget_response.unwrap());
                }
            }

            self.mouse_scene_position_last_frame = mouse_position_in_scene;
        });

        widget_responses
    }


    fn process_widget_responses(&mut self, widget_responses: &Vec<NodeWidgetResponse>, graph_editor: &mut GraphEditor)
    {
        for response in widget_responses
        {
            match response.kind
            {
                node_widget::NodeWidgetResponseType::ClickedTitle => self.add_node_to_selected_nodes(&response.key, graph_editor),
                node_widget::NodeWidgetResponseType::ClickedInputPort(_) => println!("Clicked input port"),
                node_widget::NodeWidgetResponseType::ClickedOutputPort(_) => println!("Clicked output port"),
                node_widget::NodeWidgetResponseType::InsideSelectionRect => println!("Is inside selection rect"),
            }
        }
    }

    fn process_user_actions(&mut self, graph_editor: &mut GraphEditor)
    {
        // @TODO This action now can potentially be applied double!
        for selected_node_key in graph_editor.selected_nodes.clone()
        {
            let display_node = graph_editor.display_nodes.get_mut(&selected_node_key).unwrap();
            display_node.position += self.mouse_scene_delta; 

            graph_editor.refresh_display_node(selected_node_key);
        }
    }

    // @TODO, consider where this function should be (maybe it should be in graph editor?)
    fn add_node_to_selected_nodes(&self, node_key: &NodeGraphKey, graph_editor: &mut GraphEditor)
    {
        if graph_editor.selected_nodes.contains(node_key) // @TODO, figure out if this is the most performance apporaach.
        {
            graph_editor.selected_nodes.retain(|x| x != node_key );
        }
        else // @TODO, rewrite this
        {
            graph_editor.selected_nodes.push(*node_key);
        }
    }
}