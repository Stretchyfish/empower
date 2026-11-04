use empower_engine::{self, assets::AssetKind, node_graph::node::node_kind::{BranchNode, ImageNode, ListNode, LoopNode, MathGraphNode, PrintNode, ReadFileNode, ShowImageNode, SubGraphNode, WaitNode}, value::Value};

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
    let print_node_handle = entry_node_graph.add_node(PrintNode, None);

    let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
}

#[test]
fn refreshing_image_node_updates_its_output_port()
{
    let mut project = empower_engine::project::Project::new();
    let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
    let image = graph.add_node(ImageNode::new(), None);
    let show_image = graph.add_node(ShowImageNode, None);

    graph.nodes.get_mut(&image.node_key).unwrap().kind = Box::new(ImageNode::from(0));
    graph.refresh_node(&image.node_key);

    let output_key = graph.nodes.get(&image.node_key).unwrap().output_port_keys[0];
    assert_eq!(graph.ports.get(&output_key).unwrap().value, Some(Value::Image(Some(0))));
    graph.add_connection(&output_key, &show_image.input_port_keys[1]).unwrap();

    graph.nodes.get_mut(&image.node_key).unwrap().kind = Box::new(ImageNode::from(1));
    graph.refresh_node(&image.node_key);

    assert_eq!(graph.nodes.get(&image.node_key).unwrap().output_port_keys, vec![output_key]);
    assert_eq!(graph.ports.get(&output_key).unwrap().value, Some(Value::Image(Some(1))));
    assert_eq!(graph.connections_in.get(&show_image.input_port_keys[1]), Some(&output_key));

    graph.nodes.get_mut(&image.node_key).unwrap().kind = Box::new(ImageNode::new());
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
    let list = graph.add_node(ListNode::new(), None);
    let removed_input = list.input_port_keys[1];

    let mut list_node = ListNode::new();
    list_node.size = 1;
    graph.nodes.get_mut(&list.node_key).unwrap().kind = Box::new(list_node);
    graph.refresh_node(&list.node_key);

    assert_eq!(graph.nodes.get(&list.node_key).unwrap().input_port_keys, vec![list.input_port_keys[0]]);
    assert!(!graph.ports.contains_key(&removed_input));

    graph.nodes.get_mut(&list.node_key).unwrap().kind = Box::new(ListNode::new());
    graph.refresh_node(&list.node_key);

    assert_eq!(graph.nodes.get(&list.node_key).unwrap().input_port_keys.len(), 2);

    let mut list_node = ListNode::new();
    list_node.value_type = "float".to_string();
    graph.nodes.get_mut(&list.node_key).unwrap().kind = Box::new(list_node);
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
    let loop_node = graph.add_node(LoopNode::forever(), None);

    graph.nodes.get_mut(&loop_node.node_key).unwrap().kind = Box::new(LoopNode::ranged());
    graph.refresh_node(&loop_node.node_key);

    let node = graph.nodes.get(&loop_node.node_key).unwrap();
    assert_eq!(node.input_port_keys.len(), 4);
    assert_eq!(node.output_port_keys.len(), 3);

    graph.nodes.get_mut(&loop_node.node_key).unwrap().kind = Box::new(LoopNode::forever());
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
    let subgraph_node = SubGraphNode::from(graph_id, project.assets.get_node_graph(&graph_id).unwrap());

    let graph = project.assets.get_node_graph_mut(&project.entry_graph).unwrap();
    let subgraph = graph.add_node(SubGraphNode::new(), None);

    graph.nodes.get_mut(&subgraph.node_key).unwrap().kind = Box::new(subgraph_node);
    graph.refresh_node(&subgraph.node_key);
    assert_eq!(graph.nodes.get(&subgraph.node_key).unwrap().input_port_keys.len(), 1);

    graph.nodes.get_mut(&subgraph.node_key).unwrap().kind = Box::new(SubGraphNode::new());
    graph.refresh_node(&subgraph.node_key);
    assert!(graph.nodes.get(&subgraph.node_key).unwrap().input_port_keys.is_empty());
}

#[test]
fn node_kinds_can_be_saved_and_copied()
{
    let mut graph = empower_engine::node_graph::NodeGraph::new("test");
    graph.add_node(PrintNode, None);
    graph.add_node(BranchNode, None);
    graph.add_node(LoopNode::ranged(), None);
    graph.add_node(WaitNode, None);
    let list = graph.add_node(ListNode::new(), None);
    graph.add_node(ImageNode::new(), None);
    graph.add_node(ShowImageNode, None);
    graph.add_node(MathGraphNode, None);
    graph.add_node(SubGraphNode::new(), None);
    graph.add_node(ReadFileNode::asset(None), None);

    let loaded = empower_engine::node_graph::NodeGraph::from_json(&graph.to_json()).unwrap();
    for (key, node) in &graph.nodes
    {
        assert_eq!(loaded.nodes[key].kind.name(), node.kind.name());
    }

    let mut copied = graph.nodes[&list.node_key].clone();
    copied.kind.as_mut().as_any_mut().downcast_mut::<ListNode>().unwrap().size = 5;
    let original = graph.nodes[&list.node_key].kind.as_ref().as_any().downcast_ref::<ListNode>().unwrap();
    assert_eq!(original.size, 2);
}
