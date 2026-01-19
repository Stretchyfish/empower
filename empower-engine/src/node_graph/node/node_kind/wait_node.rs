use std::time::Duration;
use std::time::Instant;

use chrono::DateTime;
use chrono::Local;

use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone)]
pub struct WaitNode
{
    start_time: Instant,
    wait_time_seconds: u64,
}

impl NodeKind for WaitNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self { start_time: Instant::now(), wait_time_seconds: 5 } )
    }

    fn name(&self) -> &'static str {
        "wait"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ) ]
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ) ]
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn setup(&mut self, _: Vec<&PortValue>) -> NodeSetupResponse {

        self.start_time = Instant::now();
        
        NodeSetupResponse::Began
    }

    fn update(&mut self) -> NodeUpdateResponse {

        if self.start_time.elapsed() >= Duration::from_secs(self.wait_time_seconds)
        {
            return NodeUpdateResponse::Finished( vec![ PortValue::Trigger(true) ]);
        }

        NodeUpdateResponse::Running
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
