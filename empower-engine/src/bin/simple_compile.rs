use empower_engine::EmpowerEngine;

fn main()
{
    let mut engine = EmpowerEngine::new();

    let new_node_type = empower_engine::node::NodeType::Integer;
    engine.add_node(new_node_type);
    
    engine.compile();
}
