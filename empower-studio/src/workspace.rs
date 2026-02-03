use egui;

use crate::actions::Action;

use super::StudioContext;

mod tab_viewer;
use tab_viewer::TabViewer;

pub mod layout;
pub use layout::Layout;

mod menu_bar;
mod debug_window;
mod history_window;
mod project_settings_window;
mod project_name_window;

pub fn show(ctx: &egui::Context, studio_context: &mut StudioContext, action_queue: &mut Vec<Action>)
{
    // @TODO, combine windows, maybe into a global space?
    debug_window::show(ctx, studio_context);
    history_window::show(ctx, studio_context);
    project_settings_window::show(ctx, studio_context);
    project_name_window::show(ctx, studio_context);

    // @TODO, find a better way to handle keyboard actions
    let save_requested = ctx.input(|i| { i.key_pressed(egui::Key::S) && i.modifiers.ctrl});
    if save_requested
    {
        action_queue.push( Action::SaveProject );
    }

    egui::TopBottomPanel::top("menu bar").show(ctx, |ui| 
    {
        menu_bar::show(ui, studio_context, action_queue);
    });

    egui::CentralPanel::default()
    .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
    .show(ctx, |ui| 
    {
        egui_dock::DockArea::new(&mut studio_context.layout.docking_state)
            .style({
                let mut style = egui_dock::Style::from_egui(ctx.style().as_ref());
                style.tab_bar.fill_tab_bar = true;
                style
            })
            .show_close_buttons(true) // @TODO, add behavior here?
            .show_add_popup(true)
            .show_leaf_close_all_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_inside(
                ui,
                &mut TabViewer {
                    project: &mut studio_context.project,
                    viewports: &mut studio_context.layout.viewports,
                    action_queue,
                },
            );

        // @TODO, this needs to be moved elsewhere!
        if studio_context.project.graph_editor.executor.is_some()
        {
            let executor = studio_context.project.graph_editor.executor.as_mut().unwrap();

            if !executor.is_running()
            {
                action_queue.push(Action::StopNodeGraphExecution);
                return; // @TODO, figure out a better way than returning here
            }

            executor.execute_node_graph(Some( ui ));
        }
    });
}
