use std::fmt;
use std::time::Duration;
use std::time::Instant;

use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct WaitNode
{
    pub time_interval_type: WaitTimeIntervals,

    #[serde(skip)]
    start_time: Option<Instant>, // Option is only used here to satisfy deserialization
    pub wait_time: u64,
}

#[typetag::serde]
impl NodeKind for WaitNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self { time_interval_type: WaitTimeIntervals::Seconds, start_time: Some( Instant::now() ), wait_time: 5 } )
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

        self.start_time = Some( Instant::now() );
        
        NodeSetupResponse::Began
    }

    fn update(&mut self) -> NodeUpdateResponse {

        match self.time_interval_type
        {
            WaitTimeIntervals::Miliseconds =>
            {
                if self.start_time.unwrap().elapsed() >= Duration::from_millis(self.wait_time)
                {
                    return NodeUpdateResponse::Finished( vec![ PortValue::Trigger(true) ]);
                }
            },
            WaitTimeIntervals::Seconds =>
            {
                if self.start_time.unwrap().elapsed() >= Duration::from_secs(self.wait_time)
                {
                    return NodeUpdateResponse::Finished( vec![ PortValue::Trigger(true) ]);
                }
            },
            WaitTimeIntervals::Minutes =>
            {
                if self.start_time.unwrap().elapsed() >= Duration::from_mins(self.wait_time)
                {
                    return NodeUpdateResponse::Finished( vec![ PortValue::Trigger(true) ]);
                }
            },
            WaitTimeIntervals::Hours =>
            {
                if self.start_time.unwrap().elapsed() >= Duration::from_hours(self.wait_time)
                {
                    return NodeUpdateResponse::Finished( vec![ PortValue::Trigger(true) ]);
                }
            },
        }

        NodeUpdateResponse::Running
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum WaitTimeIntervals
{
    Miliseconds,
    #[default] Seconds,
    Minutes,
    Hours,
}

impl fmt::Display for WaitTimeIntervals
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}
