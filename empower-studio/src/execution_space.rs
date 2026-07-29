use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
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
            output.instructions_executed.iter().for_each(|i| nodes.push( *meta.trace.get(i).unwrap() ));

            nodes
        }
    };

    let highlight_nodes = studio_context.get_executor_settings().artificial_delay.is_none();

    if highlight_nodes
    {
        return;
    }

    let cache = studio_context.get_cache_mut();

    cache.debug_highlighted_nodes = nodes_executed;
    cache.outputs.extend(output.outputs);
}
