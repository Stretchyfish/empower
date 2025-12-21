use egui;

pub mod display_port;
pub use display_port::DisplayPort;
pub use display_port::DisplayValue;

mod display_node_kind;
pub use display_node_kind::DisplayNodeKind;
use display_node_kind::DISPLAY_NODE_KIND_REGISTRY;

#[derive(Clone)]
pub struct DisplayNode
{
    pub title: &'static str,
    pub position: egui::Pos2,
    pub display_kind: Box<dyn DisplayNodeKind>,
}

impl DisplayNode
{
    pub fn new(title: &'static str, position: egui::Pos2) -> Self
    {
        let display_node_kind_constructor = DISPLAY_NODE_KIND_REGISTRY.get(title);

        let display_kind = match display_node_kind_constructor
        {
            Some( constructor ) => constructor(),
            None => 
            {
                DISPLAY_NODE_KIND_REGISTRY.get("default").unwrap()()
            },
        };

        Self 
        { 
            title, 
            position,
            display_kind,
        }
    }
}
