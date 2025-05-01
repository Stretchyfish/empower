use eframe;

pub mod node_graph;
pub use node_graph::NodeGraph;

pub mod interactions;
pub mod panels;
pub mod viewports;

fn main() -> Result<(), eframe::Error>
{
    let mut native_options = eframe::NativeOptions::default();
    native_options.vsync = false;

    eframe::run_native(
        "empower engine",
        native_options,
        Box::new(|_| Ok(Box::new(EmpowerEditorApplication::new()))),
    )
}

pub struct EmpowerEditorApplication
{
    pub state: EmpowerEditorState,
    pub node_graph: NodeGraph,
    pub workspace: panels::Workspace,
    pub docking_state: egui_dock::DockState<String>,
}

impl EmpowerEditorApplication
{
    pub fn new() -> Self
    {
        let mut new_docking_state = egui_dock::DockState::new(Vec::new());
        let mut new_workspace = panels::Workspace::new();

        let new_viewport_type = viewports::ViewportTypes::GraphViewport;

        let new_tab_name = new_workspace.create_viewport(new_viewport_type);
        new_docking_state.push_to_focused_leaf(new_tab_name);

        let mut new_node_graph = NodeGraph::new();
        new_node_graph.add_node( egui::Pos2::new(0.0, 0.0) );

        Self {
            state: EmpowerEditorState::new(),
            node_graph: new_node_graph,
            workspace: new_workspace,
            docking_state: new_docking_state,
        }
    }
}

impl eframe::App for EmpowerEditorApplication
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            panels::top_bar::view_menu_bar(
                ui,
                &mut self.state,
                &mut self.workspace,
                &mut self.docking_state,
                &mut self.node_graph,
            );
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
            // .frame(egui::Frame::none())
            // .show_inside(ctx, |ui| {
            .show(ctx, |ui| {
                egui_dock::DockArea::new(&mut self.docking_state)
                    .style({
                        let mut style = egui_dock::Style::from_egui(ctx.style().as_ref());
                        style.tab_bar.fill_tab_bar = true;
                        style
                    })
                    .show_close_buttons(true) // @TODO, add behavior here?
                    .show_add_popup(true)
                    .show_inside(
                        ui,
                        &mut panels::TabsViewer {
                            node_graph: &mut self.node_graph,
                            workspace: &mut self.workspace,
                        },
                    );
            });

        if self.state.debug_panel_active {
            panels::debug::show_debug_panel(ctx, &mut self.state, &mut self.workspace, &mut self.node_graph);
        }
    }
}

pub struct EmpowerEditorState
{
    pub debug_panel_active: bool,
}

impl EmpowerEditorState
{
    fn new() -> Self
    {
        Self
        {
            debug_panel_active: false,
        }
    }
}
