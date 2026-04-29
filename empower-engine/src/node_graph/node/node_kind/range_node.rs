use std::any::Any;

use crate::{PortValue, node_graph::node::{node_kind::{NodeSetupResponse, NodeUpdateResponse}, port::PortCompatability}};

use super::NodeKind;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeNode
{
    
}

#[typetag::serde]
impl NodeKind for RangeNode
{
    fn new() -> Box<dyn NodeKind>where Self:Sized {

        Box::new( Self
        {
            
        })
    }

    fn name(&self) ->  &'static str {
        "range"
    }

    fn clone_box(&self) -> Box<dyn NodeKind>  {

        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability>  {

        vec![
            PortCompatability::Exatch( PortValue::Integer(0) ), 
            PortCompatability::Exatch( PortValue::Integer(0) ), 
            PortCompatability::Exatch( PortValue::Integer(0) ) 
        ]
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability>  {
        vec![
            PortCompatability::Exatch( crate::PortValue::Range(0, 0, 0) ), 
        ]
    }

    fn as_any_mut(&mut self) ->  &mut dyn Any {
        self
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }

    fn setup(&mut self,inputs: Vec<&PortValue>) -> NodeSetupResponse {

        let from_value = match inputs[0]
        {
            PortValue::Integer( from ) => *from,
            _ => todo!(),
        };

        let interval_value = match inputs[1]
        {
            PortValue::Integer( interval ) => *interval,
            _ => todo!(),
        };

        let to_value = match inputs[2]
        {
            PortValue::Integer( to ) => *to,
            _ => todo!(),
        };

        let combined_range = PortValue::Range( from_value, interval_value, to_value);

        NodeSetupResponse::Finished( vec![ combined_range ] )
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
