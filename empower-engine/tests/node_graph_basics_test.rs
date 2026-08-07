use empower_engine::{self, node_graph::{node::NodeKind2}};

#[test]
fn projet_creation()
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
    let print_node_handle = entry_node_graph.add_node(NodeKind2::Print, None);

    let _ = entry_node_graph.add_connection(&start_node_handle.output_port_keys[0], &print_node_handle.input_port_keys[0] );
}
