use empower_engine::NodeGraphKey;

// use crate::{graph_editor::{display_port::DisplayPortValueRepresentation, GraphEditor}, workspace::viewports::graph_viewport::{node_widget::NodeWidgetResponseType, port_searcher::{PortKind, PortSearcher}}};

use crate::{graph_editor::GraphEditor, workspace::viewports::graph_viewport::{node_widget::NodeWidgetResponseType, port_searcher::PortKind}};

mod node_widget;
mod connection_widget;
mod user_input; // @TODO, find a better structure for this
mod port_searcher;
use port_searcher::PortSearcher;
mod node_selection_panel;
use node_selection_panel::NodeSelectionPanel;

pub struct GraphViewport
{
    pub title: String,
    node_selection_panel: NodeSelectionPanel,
    node_selection_rect: Option<egui::Rect>,
    mouse_scene_position_last_frame: egui::Pos2, // @TODO, only temporary public for debug purpose
    mouse_delta_last_frame: egui::Vec2, // @TODO, this is only temporary for debug purpose
    port_searcher: Option<PortSearcher>, 
    quick_menu: Option<(NodeGraphKey, egui::Pos2)>,
    scene_rect: egui::Rect,
}

impl GraphViewport
{
   pub fn new(title: String) -> Self
   {
        Self 
        { 
            title, 
            node_selection_panel: NodeSelectionPanel::new(),
            node_selection_rect: None,
            mouse_scene_position_last_frame: egui::Pos2 { x: 0.0, y: 0.0 },
            mouse_delta_last_frame: egui::Vec2 { x: 0.0, y: 0.0 },
            port_searcher: None,
            quick_menu: None,
            // scene_rect: egui::Rect { min: egui::Pos2 { x: -1000.0, y: -1000.0 }, max: egui::Pos2 { x: 1000.0, y: 1000.0 }},
             scene_rect: egui::Rect { min: egui::Pos2 { x: -650.0, y: -650.0 }, max: egui::Pos2 { x: 650.0, y: 650.0 }},
       }
   } 
}

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport)
{
    let mut scene_rect = graph_viewport.scene_rect.clone(); // This is needed to avoid borrow issues

    let mut mouse_position_in_scene = graph_viewport.mouse_scene_position_last_frame; // Set to last frame, in case there is no new position in the scene
    let mut mouse_scene_delta = egui::Vec2::ZERO; // @TODO, take another look at this placement
    let mut node_widgets_responses = Vec::new();

    // println!("Contains pointer: {}", ui.rect_contains_pointer(ui.min_rect()));

    let mouse_pointer_inside_viewport = ui.rect_contains_pointer(ui.min_rect());

    let user_inputs = user_input::detect_user_inputs(ui);

    let mut drag_pan_button = egui::DragPanButtons::PRIMARY;
    if user_inputs.left_shift_is_down
    {
        drag_pan_button = egui::DragPanButtons::empty();
    }

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
                // println!("{},{}", mouse_position_in_scene.x, mouse_position_in_scene.y);
            }
        }
        mouse_scene_delta = mouse_position_in_scene - graph_viewport.mouse_scene_position_last_frame; 
        graph_viewport.mouse_delta_last_frame = mouse_scene_delta;

        // @TODO, find a more computationally effecient way of doing this
        // @TODO, conder going the other way around this, looking at connection_in instead?
        let connection_keys = graph_editor.node_graph.get_all_connections();
        for connection in connection_keys
        {
            connection_widget::show(scene_ui, graph_editor, graph_viewport, connection);
        }

        let node_keys: Vec<NodeGraphKey> = graph_editor.display_nodes.keys().cloned().collect(); // @TODO, find a more elegant way of writting this
        for node_key in node_keys
        {
            // @TODO, consider if the naming should be changed so, show functions include changes to the values, and view functions is purely rendering?
            let node_widget_response = node_widget::show(scene_ui, graph_editor, graph_viewport, &node_key);

            if node_widget_response.is_some() // @TODO, find a better way of writting this
            {
                node_widgets_responses.push(node_widget_response.unwrap());
            }
        }

        connection_widget::show_connection_search(scene_ui, graph_editor, graph_viewport.port_searcher, &mouse_position_in_scene);

        if graph_viewport.node_selection_rect.is_some()
        {
            scene_ui.painter().rect_filled(graph_viewport.node_selection_rect.unwrap(), 0.5, egui::Color32::from_rgba_unmultiplied(255, 140, 0, 70));
        }

        if graph_viewport.quick_menu.is_some()
        {
            let quick_menu = graph_viewport.quick_menu.unwrap();

            // @TODO, change from tuple to struct
            // let clicked_quick_menu_button = show_quick_menu(scene_ui, graph_editor, quick_menu.0, quick_menu.1);

            // if clicked_quick_menu_button
            // {
            //     graph_viewport.quick_menu = None;
            // }
        }

        graph_viewport.mouse_scene_position_last_frame = mouse_position_in_scene;
    });

    graph_viewport.scene_rect = scene_rect;

    let mut interaction_happened_this_loop = false; // @TODO, find a better way of doing this
    let mut nodes_inside_selection_rect = Vec::new();
    let mut port_was_clicked = false; // @TODO, find a better way to approach this
    for node_widget_response in node_widgets_responses
    {
        interaction_happened_this_loop = true;
        let node_with_response_key = node_widget_response.key;
        match node_widget_response.kind
        {
        NodeWidgetResponseType::ClickedTitle =>
        {
            add_selected_node(graph_editor, &node_widget_response.key);
        }

        NodeWidgetResponseType::ClickedInputPort(port_key) =>
        {
            input_port_interaction(graph_editor, graph_viewport, &port_key);
            port_was_clicked = true;
            println!("Clicked input port");
        }

        NodeWidgetResponseType::ClickedOutputPort(port_key) =>
        {
            output_port_interaction(graph_editor, graph_viewport, &port_key);
            port_was_clicked = true;
            println!("Clicked output port");
       }

        // @TODO, prepare for multiple kinds of input ports
        NodeWidgetResponseType::ChangedInputPortDisplayValue(port_key, new_display_value) =>
        {
            graph_editor.set_input_port_value(&port_key, new_display_value);
        }

        // NodeWidgetResponseType::ChangedState( new_state ) =>
        // {
        //     graph_editor.change_node_state(&node_with_response_key, &new_state);
        // }

        NodeWidgetResponseType::ClickedQuickMenuButton( quick_menu_button_position ) =>
        {
            if graph_viewport.quick_menu.is_some()
            {
                // @TODO, change from a tuple to a struct
                if graph_viewport.quick_menu.unwrap().0 == node_with_response_key
                {
                    graph_viewport.quick_menu = None;
                    break;
                }
            }

            graph_viewport.quick_menu = Some( (node_with_response_key, quick_menu_button_position) );
        }

        NodeWidgetResponseType::InsideSelectionRect =>
        {
            nodes_inside_selection_rect.push(node_with_response_key);
        }
      }  
    }

    if user_inputs.left_is_down && user_inputs.left_shift_is_down && graph_viewport.node_selection_rect.is_none()
    {
        graph_viewport.node_selection_rect = Some( egui::Rect::from_min_max(mouse_position_in_scene, mouse_position_in_scene) );
    }

    if graph_viewport.node_selection_rect.is_some()
    {
        graph_viewport.node_selection_rect = Some( egui::Rect::from_two_pos(graph_viewport.node_selection_rect.unwrap().min, mouse_position_in_scene) );
    }

    if graph_viewport.node_selection_rect.is_some() && (!user_inputs.left_is_down || !user_inputs.left_shift_is_down)
    {
        graph_editor.selected_nodes = nodes_inside_selection_rect;
        graph_viewport.node_selection_rect = None;
    }

    for selected_node_key in graph_editor.selected_nodes.iter()
    {
        // @TODO, make this more safe
        let display_node = graph_editor.display_nodes.get_mut(selected_node_key).unwrap();
        display_node.position += mouse_scene_delta;
    }

    if user_inputs.left_clicked && graph_viewport.port_searcher.is_some() && port_was_clicked == false && graph_viewport.node_selection_panel.visible == false
    {
        graph_viewport.port_searcher = None;
    }

    // @TODO, look into combining these two if statements, and consider if node_selection_panel check is needed in the first one
    if !interaction_happened_this_loop && user_inputs.left_clicked && !graph_editor.selected_nodes.is_empty() && graph_viewport.node_selection_panel.visible == false 
    {
        graph_editor.selected_nodes = Vec::new();
    }

    if user_inputs.right_clicked && graph_viewport.node_selection_panel.visible == false
    {
        graph_editor.selected_nodes = Vec::new();
    }


    // This needs to be this low to avoid problems with the if statement above, consider a better approach for this?
    node_selection_panel::show(ui, &mut graph_viewport.node_selection_panel, graph_editor, &user_inputs, &mouse_position_in_scene);

}

// @TODO, consider where this function should be 
fn add_selected_node(graph_editor: &mut GraphEditor, node_key: &NodeGraphKey)
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

fn input_port_interaction(graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, port_key: &NodeGraphKey) // @TODO, find a better name and location
{
    if graph_editor.node_graph.input_port_has_connection(port_key) // @TODO, consider changing this to be part of the graph editor itself
    {
        let connect_output_port_key = graph_editor.node_graph.get_input_port_connection_key(port_key).expect("Tried to access ouptut port in connection-in, not available").clone();
        graph_editor.node_graph.remove_connection(port_key, &connect_output_port_key);

        if graph_viewport.port_searcher.is_some()
        {
            graph_editor.node_graph.add_connection(graph_viewport.port_searcher.unwrap().port_key, *port_key);
            graph_viewport.port_searcher = None;
            return; 
        }

        graph_viewport.port_searcher = Some( PortSearcher::output_port_searching(connect_output_port_key) );
        return; 
    }
    
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher::input_port_searching( port_key.clone() ) );
        return;
    }

    let port_searcher = graph_viewport.port_searcher.unwrap();

    match  port_searcher.port_kind 
    {
        PortKind::InputPort =>
        {
            if port_searcher.port_key == *port_key
            {
                graph_viewport.port_searcher = None; // @TODO, expand this functionality to be more complex
                return;
            } 
        },

        PortKind::OutputPort =>
        {    
            // println!("Reached here 4");

            // if graph_editor.node_graph.output_port_has_connection(port_key)
            // {

            // }
            // let port_connection = connections.iter().map(|(key, vec)|
            // {
            //     if vec.contains(&empower_input_port.key)
            //     {
            //         Some(key)
            //     }
            //     else // @TODO, find a better way to write
            //     {
            //         None
            //     }
            // });

            // let connection = connections.get_mut(&port_searcher.port_key).unwrap(); 
            let add_connection_result = graph_editor.node_graph.add_connection(port_searcher.port_key, *port_key);

            match add_connection_result
            {
                Ok(()) => println!("Added connection: {}, {}", port_searcher.port_key, *port_key),
                Err( text ) => println!("Failed to add connection because: {}", text),
            }

            // @TODO, simplify this
            // let empower_port = graph_editor.empower_node_graph.input_ports.get(port_key).unwrap();

            // let value_representation = graph_editor.get_input_port_value_representation(port_key);
            // let display_port = graph_editor.display_input_ports.get_mut(port_key).unwrap();

            // display_port.value_representation = value_representation;

            // display_port.value = empower_port.get_value_as_string();
            // display_port.value_text_valid = true;


            // display_port.value_representation_valid = true;

            graph_viewport.port_searcher = None;
            return;
        }
    }
}

fn output_port_interaction(graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, port_key: &NodeGraphKey) // @TODO, find a better name and location
{
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher::output_port_searching(*port_key) );
        return;
    }

    let port_searcher = graph_viewport.port_searcher.unwrap(); // @TODO, switch to an unwrap

    match  port_searcher.port_kind // @Look again at this match stement, the returns currently seems redundant
    {
        PortKind::InputPort =>
        {
            // @TODO, might add another check here
            graph_editor.node_graph.add_connection(*port_key, port_searcher.port_key); // @TODO, consider changing this API
            graph_viewport.port_searcher = None;
            return;
        },

        PortKind::OutputPort =>
        {
            if port_searcher.port_key == *port_key
            {
                graph_viewport.port_searcher = None;
            }
        }
    }

    println!("output port id from show function: {}", port_key);
}

// fn input_port_representation_interaction(graph_editor: &mut GraphEditor, port_key: &EmpowerKey, new_value_representation: DisplayPortValueRepresentation)
// {
//     // let display_port_value_representation = graph_editor.display_input_ports.get(port_key).unwrap().value_representation.clone();

//     let succesfully_set_value = graph_editor.set_input_port_value_from_representation(port_key, new_value_representation.clone());

//    // @TODO, this can be written better
//     let display_port = graph_editor.display_input_ports.get_mut(port_key).unwrap();
//     display_port.value_representation = new_value_representation;

//     display_port.value_representation_valid = succesfully_set_value; // @TODO, this should work no problem, but keep an eye on it
//     // if succesfully_set_value
//     // {
//     //     display_port.value_representation_valid = true;
//     // }
// }

// fn show_quick_menu(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, node_key: EmpowerKey, menu_position: egui::Pos2) -> bool
// {
//     // let node_selection_window = egui::Window::new("").current_pos(egui::Pos2 {x: window_position.x - 100.0, y: window_position.y - 15.0}).collapsible(false).max_size(egui::Vec2 {x: 200.0, y: 200.0}).title_bar(false);

//     let quick_menu_rect = egui::Rect::from_min_size(menu_position, egui::Vec2::splat(500.0));

//     let mut button_clicked = false;
//     ui.allocate_ui_at_rect(quick_menu_rect, |ui|
//     {
//         egui::Frame::popup(ui.style()).show(ui, |ui| 
//         {

//             if ui.add(egui::Button::new( egui::RichText::new("Compile").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
//             {
//                 button_clicked = true;
//             }

//             if ui.add(egui::Button::new( egui::RichText::new("Delete").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
//             {
//                 graph_editor.remove_node(&node_key);
//                 button_clicked = true;

//             }

//         });
//     });

//     button_clicked
// }
