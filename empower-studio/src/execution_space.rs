use empower_engine::node_graph::NodeAddress;

use crate::studio_context::{Log, StudioContext};

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    let mut logs = Vec::new();
    
    let output = {
        let executor = studio_context.get_executor_mut();

        if executor.is_none()
        {
            return;
        }

        let executor = executor.as_mut().unwrap();

        if !executor.is_running()
        {
            studio_context.request_stop_execute();
            return;
        }

        executor.run(Some( ui ))
    };

    let nodes_executed = {
        let mut nodes = Vec::new(); // @TODO, this is possible to optimize by preallocating sizes of the vector

        if studio_context.get_compile_result().is_none()
        {
            nodes
        }
        else
        {
            let meta = studio_context.get_compile_result().as_ref().unwrap().meta.as_ref().unwrap();
            output.execute_units.iter().for_each(|u|
            {
                let node_key = meta.trace.get(&u.graph_id).unwrap().get(&u.instruction_address);

                if node_key.is_none()
                {
                    logs.push( Log::warning(format!("Execution space tried and failed to trace instruction {} to a node key", u.instruction_address).as_str()) );
                    return;
                }
                
                 nodes.push( NodeAddress { graph_id: u.graph_id, node_key: *node_key.unwrap() } );               
            });

            nodes
        }
    };

    let highlight_nodes = studio_context.get_executor_settings().artificial_delay.is_some();
    let cache = studio_context.get_cache_mut();

    if highlight_nodes
    {
        cache.session.debug_highlighted_nodes = nodes_executed;
    }

    cache.session.outputs.extend(output.outputs);
}
