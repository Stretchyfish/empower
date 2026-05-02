use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

use super::NodeResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LoopNode
{
    pub loop_type: LoopType,
    range: Option<(i32, i32, i32)>,
    next_iteration_send: i32,
}

#[typetag::serde]
impl NodeKind for LoopNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new(
            Self
            {
                loop_type: LoopType::Forever,
                range: None,
                next_iteration_send: 0,
            }
        )
    }

    fn name(&self) -> &'static str {
        "loop"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        match self.loop_type
        {
            LoopType::Forever => vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ) ],
            LoopType::Range => vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ), PortCompatability::Exatch( PortValue::Range( 0, 1, 1) )  ],
        }
        
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        match self.loop_type
        {
            LoopType::Forever => vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ) ],
            LoopType::Range => vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ), PortCompatability::Exatch( PortValue::Integer( 0 ) ) ],
        }
        
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn setup(&mut self, input_port_value: Vec<&PortValue>) -> NodeResponse {

        if input_port_value.len() == 2
        {
            self.range = match input_port_value[1]
            {
                PortValue::Range( from, interval, to) => Some( (*from, *interval, *to) ),
                _ => panic!("Tried to match incompatible port value in loop node"),
            };

            self.next_iteration_send = 0;
        }


        match self.loop_type
        {
            LoopType::Forever => { NodeResponse::CreateLoop( vec![ PortValue::Trigger(true) ] ) },
            LoopType::Range => { NodeResponse::CreateLoop( vec![ PortValue::Trigger(true), PortValue::Integer(0) ] ) },
        }
    }

    fn update(&mut self) -> NodeResponse {

        let mut next_value_to_send = None;
        if self.range.is_some()
        {
            next_value_to_send = Some( self.range.unwrap().0 + self.next_iteration_send * self.range.unwrap().1 );
            self.next_iteration_send += 1;

            if next_value_to_send.unwrap() >= self.range.unwrap().2
            {
                return NodeResponse::Finished( vec![ PortValue::Trigger(true), PortValue::Integer( self.next_iteration_send ) ]  );
            }
        }

        match self.loop_type
        {
            LoopType::Forever => { NodeResponse::ContinueLoop( vec![ PortValue::Trigger(true) ] ) },
            LoopType::Range => { NodeResponse::ContinueLoop( vec![ PortValue::Trigger(true), PortValue::Integer( next_value_to_send.unwrap() ) ] ) },
        }
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub enum LoopType
{
    Forever,
    Range,
}

// impl fmt::Display for LoopType
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

