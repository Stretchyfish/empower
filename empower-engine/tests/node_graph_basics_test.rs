use empower_engine::{self, assets::AssetKind, node_graph::node::{NodeKind, node_kind::{ImageState, ListState, LoopMode, SubGraphState}}, value::Value};

#[test]
fn project_creation() // @TODO, move this into a different file
{
    let _ = empower_engine::project::Project::new();
}

#[test]
fn entry_graph_exist()
{
    let mut project = empower_engine::project::Project::new();
    let _ = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
}

#[test]
fn add_connection_works()
{
    let mut project = empower_engine::project::Project::new();
    let entry_node_graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();

    let start_node_handle = entry_node_graph.get_node_handle(entry_node_graph.start_node_key);
    let print_node_handle = entry_node_graph.add_node(NodeKind::Print, None);

    let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
}

#[test]
fn refreshing_image_node_updates_its_output_port()
{
    let mut project = empower_engine::project::Project::new();
    let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
    let image = graph.add_node(NodeKind::Image(ImageState::new()), None);
    let show_image = graph.add_node(NodeKind::ShowImage, None);

    graph.nodes.get_mut(&image.node_key).unwrap().kind = NodeKind::Image(ImageState::from(0));
    graph.refresh_node(&image.node_key);

    let output_key = graph.nodes.get(&image.node_key).unwrap().output_port_keys[0];
    assert_eq!(graph.ports.get(&output_key).unwrap().value, Some(Value::Image(Some(0))));
    graph.add_connection(&output_key, &show_image.input_port_keys[1]).unwrap();

    graph.nodes.get_mut(&image.node_key).unwrap().kind = NodeKind::Image(ImageState::from(1));
    graph.refresh_node(&image.node_key);

    assert_eq!(graph.nodes.get(&image.node_key).unwrap().output_port_keys, vec![output_key]);
    assert_eq!(graph.ports.get(&output_key).unwrap().value, Some(Value::Image(Some(1))));
    assert_eq!(graph.connections_in.get(&show_image.input_port_keys[1]), Some(&output_key));

    graph.nodes.get_mut(&image.node_key).unwrap().kind = NodeKind::Image(ImageState::new());
    graph.refresh_node(&image.node_key);

    assert!(graph.nodes.get(&image.node_key).unwrap().output_port_keys.is_empty());
    assert!(!graph.ports.contains_key(&output_key));
    assert!(!graph.connections_in.contains_key(&show_image.input_port_keys[1]));
    assert!(!graph.connections_out.contains_key(&output_key));
}

#[test]
fn refreshing_list_node_updates_its_input_ports()
{
    let mut project = empower_engine::project::Project::new();
    let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
    let list = graph.add_node(NodeKind::List(ListState::new()), None);
    let removed_input = list.input_port_keys[1];

    let mut state = ListState::new();
    state.size = 1;
    graph.nodes.get_mut(&list.node_key).unwrap().kind = NodeKind::List(state);
    graph.refresh_node(&list.node_key);

    assert_eq!(graph.nodes.get(&list.node_key).unwrap().input_port_keys, vec![list.input_port_keys[0]]);
    assert!(!graph.ports.contains_key(&removed_input));

    graph.nodes.get_mut(&list.node_key).unwrap().kind = NodeKind::List(ListState::new());
    graph.refresh_node(&list.node_key);

    assert_eq!(graph.nodes.get(&list.node_key).unwrap().input_port_keys.len(), 2);

    let mut state = ListState::new();
    state.value_type = "float".to_string();
    graph.nodes.get_mut(&list.node_key).unwrap().kind = NodeKind::List(state);
    graph.refresh_node(&list.node_key);

    for port_key in &graph.nodes.get(&list.node_key).unwrap().input_port_keys
    {
        assert_eq!(graph.ports.get(port_key).unwrap().compatability, vec![Value::Float(0.0)]);
    }
}

#[test]
fn refreshing_loop_node_updates_its_ports()
{
    let mut project = empower_engine::project::Project::new();
    let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
    let loop_node = graph.add_node(NodeKind::Loop(LoopMode::forever()), None);

    graph.nodes.get_mut(&loop_node.node_key).unwrap().kind = NodeKind::Loop(LoopMode::ranged());
    graph.refresh_node(&loop_node.node_key);

    let node = graph.nodes.get(&loop_node.node_key).unwrap();
    assert_eq!(node.input_port_keys.len(), 4);
    assert_eq!(node.output_port_keys.len(), 3);

    graph.nodes.get_mut(&loop_node.node_key).unwrap().kind = NodeKind::Loop(LoopMode::forever());
    graph.refresh_node(&loop_node.node_key);

    let node = graph.nodes.get(&loop_node.node_key).unwrap();
    assert_eq!(node.input_port_keys.len(), 1);
    assert_eq!(node.output_port_keys.len(), 1);
}

#[test]
fn refreshing_subgraph_node_updates_its_ports()
{
    let mut project = empower_engine::project::Project::new();
    let graph_id = project.assets.create_asset(None, AssetKind::NodeGraph, "function").unwrap();
    let subgraph_state = SubGraphState::from(graph_id, project.assets.get_node_graph(&graph_id).unwrap());

    let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
    let subgraph = graph.add_node(NodeKind::SubGraph(SubGraphState::new()), None);

    graph.nodes.get_mut(&subgraph.node_key).unwrap().kind = NodeKind::SubGraph(subgraph_state);
    graph.refresh_node(&subgraph.node_key);
    assert_eq!(graph.nodes.get(&subgraph.node_key).unwrap().input_port_keys.len(), 1);

    graph.nodes.get_mut(&subgraph.node_key).unwrap().kind = NodeKind::SubGraph(SubGraphState::new());
    graph.refresh_node(&subgraph.node_key);
    assert!(graph.nodes.get(&subgraph.node_key).unwrap().input_port_keys.is_empty());
}
