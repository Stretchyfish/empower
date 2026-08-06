use std::collections::VecDeque;

use empower_engine::node_graph::node::{NodeKind2, node_kind::{ImageState, ListState, SubGraphState}};

use super::GraphViewportAction;

struct NodeType // @TODO, find a better name
{
    name: &'static str,
    constructor: fn() -> NodeKind2,
}

static NODE_TYPES: &[NodeType] = &[
    // NodeType // We don't need start
    // {
    //     name: "start",
    //     constructor: || NodeKind2::Start,
    // },
    NodeType
    {
        name: "print",
        constructor: || NodeKind2::Print,
    },
    NodeType
    {
        name: "branch",
        constructor: || NodeKind2::Branch,
    },
    NodeType
    {
        name: "loop",
        constructor: || NodeKind2::Loop,
    },
    NodeType
    {
        name: "wait",
        constructor: || NodeKind2::Wait,
    },
    NodeType
    {
        name: "list",
        constructor: || NodeKind2::List( ListState::new() ),
    },
    NodeType
    {
        name: "image",
        constructor: || NodeKind2::Image( ImageState::new() ),
    },
    NodeType
    {
        name: "show image",
        constructor: || NodeKind2::ShowImage,
    },
    NodeType
    {
        name: "math graph",
        constructor: || NodeKind2::MathGraph,
    },
    NodeType
    {
        name: "sub graph",
        constructor: || NodeKind2::SubGraph( SubGraphState::new() ),
    },
];

const NODES_NAMES_AVAILABLE: &'static [&str] = &[
    "number",
    "print",
    "branch",
    "wait",
    "list",
    "loop",
    "sub graph",
    "image",
    "show image",
    "show math graph",
];

#[derive(Clone, Default, Debug)]
pub struct NodePicker
{
    pub show: bool,
    search_text: String,
    mouse_position_when_node_select_menu_was_activated: egui::Pos2,

    highlighted_node_index: Option<usize>,
}

impl NodePicker
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            search_text: String::new(),
            mouse_position_when_node_select_menu_was_activated: egui::Pos2::default(),
            highlighted_node_index: None,
        }
    }

    pub fn toggle_show(&mut self, mouse_position: &egui::Pos2)
    {
        self.show = !self.show;

        self.mouse_position_when_node_select_menu_was_activated = *mouse_position;
        self.search_text = String::new();
    }

    pub fn show(&mut self, ui: &mut egui::Ui, graph_viewport_actions: &mut VecDeque<GraphViewportAction>, mouse_position: &egui::Pos2)
    {
        if !self.show
        {
            return;
        }
        
        let mut node_to_add = None;

        let window_position = self.mouse_position_when_node_select_menu_was_activated;
        let node_selection_window = egui::Window::new("")
                                                .current_pos(egui::Pos2 {x: window_position.x - 100.0, y: window_position.y - 15.0})
                                                .collapsible(false)
                                                .max_size(egui::Vec2 {x: 200.0, y: 200.0})
                                                .title_bar(false);

        let mut node_showed_number = 0; // Used later in the big scope for the total number of nodes and in the smaller scope for indexing
        let user_clicked_enter = ui.ctx().input(|i| { i.key_pressed(egui::Key::Enter) }); // @TODO, add user_inputs instead of registering enter click again

        node_selection_window.show(ui.ctx(), |ui|
        {
            // if self.highlighted_node_index.is_some()
            // {
            //     println!("highlighted node: {}", self.highlighted_node_index.unwrap());
            // }
            // else
            // {
            //     println!("highlighted node none");
            // }

            let singleline_edit = ui.text_edit_singleline(&mut self.search_text);
            singleline_edit.request_focus();

            if singleline_edit.changed()
            {
                self.highlighted_node_index = Some( 0 );
            }

            if self.search_text.is_empty()
            {
                self.highlighted_node_index = None;
            }

            egui::ScrollArea::vertical()
            .max_height(200.0)
            .max_width(200.0)
            .show(ui, |ui| 
            {
                ui.group(|ui|
                {
                    // @TODO, in the future this should get changed to only appear like this when searching, otherwise they should be sorted into categories
                    for node_type in NODE_TYPES
                    {
                        if self.search_text.len() > 0
                        {
                            if !(*node_type).name.contains( &self.search_text.to_lowercase() )
                            {
                                continue;
                            }
                        }

                        let mut current_node_is_selected = false;
                        if self.highlighted_node_index.is_some()
                        {
                            if node_showed_number == self.highlighted_node_index.unwrap() // Cast to i32 to avoid subtrackt with overflow error
                            {
                                current_node_is_selected = true;
                            }
                        }

                        let button = egui::Button::new( node_type.name )
                        .min_size( egui::Vec2 {x: 190.0, y: 20.0} )
                        .fill(

                            if current_node_is_selected
                            {
                                egui::Color32::YELLOW
                            }
                            else
                            {
                                ui.visuals().widgets.inactive.bg_fill
                            }
                        );
                        
                        if ui.add(button).clicked() 
                        {
                            node_to_add = Some( (node_type.constructor)() );
                        };

                        node_showed_number += 1;

                        if user_clicked_enter && current_node_is_selected
                        {
                            node_to_add = Some( (node_type.constructor)() );
                        }
                    }
                });
            });
        });

        if self.highlighted_node_index.is_some() && node_showed_number != 0 // important to check for 0 due to -1 later
        {
            let highlighted_index = self.highlighted_node_index.as_mut().unwrap();

            let clicked_up_arrow =  ui.ctx().input(|i| { i.key_pressed(egui::Key::ArrowUp) });
            let clicked_down_arrow =  ui.ctx().input(|i| { i.key_pressed(egui::Key::ArrowDown) });

            if clicked_up_arrow && *highlighted_index != 0
            {
                *highlighted_index -= 1;
            }

            if clicked_down_arrow && *highlighted_index < node_showed_number - 1 // node_showed_number in this case is the max number of nodes currently shown
            {
                *highlighted_index += 1;
            }
        }

        if node_to_add.is_some()
        {
            graph_viewport_actions.push_back( GraphViewportAction::AddNodeToGraph { node_kind: node_to_add.unwrap(), position: Some(*mouse_position) });
            self.show = false;
        }
    }
}
