use std::any::Any;

use super::SettingsKind;
use crate::{assets::Assets, compiler::{CompilePlan, RegisterAddress}, node_graph::port::PortDefinition, project::ProjectState};

mod start_node;
pub use start_node::StartNode;
mod print_node;
pub use print_node::PrintNode;
mod branch_node;
pub use branch_node::BranchNode;
mod loop_node;
pub use loop_node::LoopNode;
mod wait_node;
pub use wait_node::WaitNode;
mod list_node;
pub use list_node::ListNode;
mod image_node;
pub use image_node::ImageNode;
mod show_image_node;
pub use show_image_node::ShowImageNode;
mod math_graph_node;
pub use math_graph_node::MathGraphNode;
mod sub_graph_node;
pub use sub_graph_node::SubGraphNode;
mod read_file_node;
pub use read_file_node::ReadFileNode;

pub trait NodeKindClone
{
    fn clone_box(&self) -> Box<dyn NodeKind>;
}

impl<T: NodeKind + Clone + 'static> NodeKindClone for T
{
    fn clone_box(&self) -> Box<dyn NodeKind>
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn NodeKind>
{
    fn clone(&self) -> Self
    {
        self.as_ref().clone_box()
    }
}

pub trait NodeKindAny
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: NodeKind + Any> NodeKindAny for T
{
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[typetag::serde]
pub trait NodeKind: NodeKindClone + NodeKindAny
{
    fn name(&self) -> &str;
    fn size(&self) -> egui::Vec2;
    fn settings(&mut self) -> Vec<SettingsKind<'_>> { Vec::new() }
    fn input_port_definitions(&self) -> Vec<PortDefinition>;
    fn output_port_definitions(&self) -> Vec<PortDefinition>;
    fn compile(&self, input_register_addresses: &Vec<RegisterAddress>, output_register_addresses: &Vec<RegisterAddress>, assets: &Assets, project_state: &ProjectState) -> Result<CompilePlan, String>;
}
