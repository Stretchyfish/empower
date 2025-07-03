use egui::accesskit::Node;
use egui_dock::node;
use empower_node_graph::EmpowerKey;
use crate::{graph_editor::{self, display_port, GraphEditor}, workspace::viewports::graph_viewport::{self, node_widget::NodeWidgetResponseType, port_searcher::{PortKind, PortSearcher}}};

mod node_widget;
mod connection_widget;
mod user_input; // @TODO, find a better structure for this
mod port_searcher;
mod node_selection_panel;
use node_selection_panel::NodeSelectionPanel;

pub struct GraphViewport
{
    pub title: String,
    node_selection_panel: NodeSelectionPanel,
    mouse_scene_position_last_frame: egui::Pos2,
    port_searcher: Option<PortSearcher>, 
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
            mouse_scene_position_last_frame: egui::Pos2 { x: 0.0, y: 0.0 },
            port_searcher: None,
            scene_rect: egui::Rect { min: egui::Pos2 { x: -500.0, y: -500.0 }, max: egui::Pos2 { x: 500.0, y: 500.0 }},
        }
   } 
}

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport)
{
    let mut scene_rect = graph_viewport.scene_rect.clone(); // This is needed to avoid borrow issues

    let mut mouse_position_in_scene = graph_viewport.mouse_scene_position_last_frame; // Set to last frame, in case there is no new position in the scene
    let mut mouse_scene_delta = egui::Vec2::ZERO; // @TODO, take another look at this placement
    let mut node_widgets_responses = Vec::new();

    egui::Scene::new()
    .zoom_range(0.01..=2.0)
    .max_inner_size(egui::Vec2 { x: 8.0, y: 8.0 })
    .show(ui, &mut scene_rect, |scene_ui|
    {
        let scene_transform = scene_ui.ctx().layer_transform_from_global(scene_ui.painter().layer_id());
        let scene_latest_pos = scene_ui.input(|i| i.pointer.latest_pos());

        if scene_transform.is_some() && scene_latest_pos.is_some()
        {
            mouse_position_in_scene = scene_transform.unwrap() * scene_latest_pos.unwrap();
        }

        mouse_scene_delta = mouse_position_in_scene - graph_viewport.mouse_scene_position_last_frame; 

        // @TODO, find a more computationally effecient way of doing this
        // @TODO, conder going the other way around this, looking at connection_in instead?
        let connection_keys = graph_editor.empower_node_graph.connections_out.clone();
        for connection_key in connection_keys.keys()
        {
            connection_widget::show(scene_ui, graph_editor, graph_viewport, &connection_key);
        }

        let node_keys: Vec<EmpowerKey> = graph_editor.display_nodes.keys().cloned().collect(); // @TODO, find a more elegant way of writting this
        for node_key in node_keys
        {
            // @TODO, consider if the naming should be changed so, show functions include changes to the values, and view functions is purely rendering?
            let node_widget_response = node_widget::show(scene_ui, graph_editor, graph_viewport, &node_key);

            if node_widget_response.is_some() // @TODO, find a better way of writting this
            {
                node_widgets_responses.push(node_widget_response.unwrap());
            }
        }
        // for node_key in node_keys
        // {
        //     let node_widget_response = node_widget::show(scene_ui, graph_editor, mouse_scene_delta, graph_viewport, &node_key);

        //     if node_widget_response.is_some() // @TODO, find a better way of writting this
        //     {
        //         node_widgets_responses.push(node_widget_response.unwrap());
        //     }
        // }

        connection_widget::show_connection_search(scene_ui, graph_editor, graph_viewport.port_searcher, &mouse_position_in_scene);

        graph_viewport.mouse_scene_position_last_frame = mouse_position_in_scene;
    });

    graph_viewport.scene_rect = scene_rect;

    let user_inputs = user_input::detect_user_inputs(ui);
    node_selection_panel::show(ui, &mut graph_viewport.node_selection_panel, graph_editor, &user_inputs, &mouse_position_in_scene);

    for node_widget_response in node_widgets_responses
    {
      match node_widget_response.kind 
      {
        NodeWidgetResponseType::ClickedTitle =>
        {
            add_selected_node(graph_editor, &node_widget_response.key);
        }

        NodeWidgetResponseType::ClickedInputPort(port_key) =>
        {
            input_port_interaction(graph_editor, graph_viewport, &port_key);
        }

        NodeWidgetResponseType::ClickedOutputPort(port_key) =>
        {
            output_port_interaction(graph_editor, graph_viewport, &port_key);
        }

        // @TODO, prepare for multiple kinds of input ports
        NodeWidgetResponseType::ChangedInputPortValueText(port_key, new_value_text) =>
        {
            input_port_text_interaction(graph_editor, &port_key, &new_value_text);
        }
      
        _ =>
        {

        },
      }  
    }

    for selected_node_key in graph_editor.selected_nodes.iter()
    {
        // @TODO, make this more safe
        let display_node = graph_editor.display_nodes.get_mut(selected_node_key).unwrap();
        display_node.position += mouse_scene_delta;
    }


    // @TODO, find a permanent fix for this
    //  if user_inputs.right_clicked && graph_viewport.port_searcher.is_some() 
    // if user_inputs.left_clicked && graph_viewport.port_searcher.is_some() && node_widgets_response.is_none()
    // {
    //     graph_viewport.port_searcher = None;
    // }
}

// @TODO, consider where this function should be 
fn add_selected_node(graph_editor: &mut GraphEditor, node_key: &EmpowerKey)
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

fn input_port_interaction(graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, port_key: &EmpowerKey) // @TODO, find a better name and location
{
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher { port_key: *port_key, port_kind: PortKind::InputPort });
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
            graph_editor.empower_node_graph.add_connection(*port_key, port_searcher.port_key);

            // @TODO, simplify this
            let empower_port = graph_editor.empower_node_graph.input_ports.get(port_key).unwrap();
            let display_port = graph_editor.display_input_ports.get_mut(port_key).unwrap();

            display_port.value = empower_port.get_value_as_string();
            display_port.value_text_valid = true;

            graph_viewport.port_searcher = None;
            return;
        }
    }

    println!("input port id from show function: {}", *port_key);
}

fn output_port_interaction(graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, port_key: &EmpowerKey) // @TODO, find a better name and location
{
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher { port_key: *port_key, port_kind: PortKind::OutputPort });
        return;
    }

    let port_searcher = graph_viewport.port_searcher.unwrap();

    match  port_searcher.port_kind 
    {
        PortKind::InputPort =>
        {
            graph_editor.empower_node_graph.add_connection(port_searcher.port_key, *port_key); // @TODO, consider changing this API
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

fn input_port_text_interaction(graph_editor: &mut GraphEditor, port_key: &EmpowerKey, new_value_text: &String)
{
    // @TODO, improve this interface
    let empower_port = graph_editor.empower_node_graph.input_ports.get_mut(port_key).unwrap();
    let display_port = graph_editor.display_input_ports.get_mut(port_key).unwrap();

    display_port.value = new_value_text.to_string(); // @TODO, figure out why this to_string is needed

    let succesfully_set_value = empower_port.set_value_with_text(new_value_text);

    display_port.value_text_valid = false; // @TODO, this should work no problem, but keep an eye on it
    if succesfully_set_value
    {
        display_port.value_text_valid = true;
    }
}