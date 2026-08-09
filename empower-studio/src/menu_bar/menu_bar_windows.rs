use crate::{docking_space::{Viewport, viewport::{ContentBrowserViewport, EmptyViewport, GraphViewport, TerminalViewport}}, studio_context::StudioContext};

pub fn show(ui: &mut egui::Ui, studio_context: &mut StudioContext)
{
    ui.menu_button("Windows", |ui|
    {
        if ui.button("default layout").clicked()
        {
            studio_context.request_default_layout();
        }
        ui.menu_button("Add window", |ui|
        {
            if ui.button("Graph Viewport").clicked()
            {
                studio_context.request_new_viewport( Viewport::Graph { graph_viewport: GraphViewport::new( 3 ) } ); // @TODO, find a better way to handle the graph asset id
            }
            if ui.button("Terminal Viewport").clicked()
            {
                studio_context.request_new_viewport( Viewport::Terminal { terminal_viewport: TerminalViewport::new() } );
            }
            if ui.button("Content Browser Viewport").clicked()
            {
                studio_context.request_new_viewport( Viewport::ContentBrowser { content_browser_viewport: ContentBrowserViewport::new() } );
            }
            if ui.button("Empty Viewport").clicked()
            {
                studio_context.request_new_viewport( Viewport::Empty { empty_viewport: EmptyViewport::new() });
            }
            // if ui.button("Variable Editor Viewport").clicked()
            // {
            //     studio_context.request_new_viewport("variable editor viewport");
            // }
            // if ui.button("Timeline Viewport").clicked()
            // {
            //     studio_context.request_new_viewport("timeline viewport");
            // }
        });

        if ui.button("save layout").clicked()
        {
            studio_context.request_save_studio();
        }

        if ui.button("load layout").clicked()
        {
            studio_context.request_load_studio();
        }
    });
}
