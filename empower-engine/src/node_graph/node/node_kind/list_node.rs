use serde::{Deserialize, Serialize};

use super::NodeKind;
use crate::{assets::Assets, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::{node::SettingsKind, port::PortDefinition}, project::ProjectState, utility::alphabet_counter::AlphabetCounter, value::Value};

#[derive(Serialize, Deserialize, Clone)]
pub struct ListNode
{
    pub value_type: String,
    pub size: usize,
}

impl ListNode
{
    pub fn new() -> Self
    {
        Self
        {
            value_type: "integer".to_string(),
            size: 2,
        }
    }

}

#[typetag::serde]
impl NodeKind for ListNode
{
    fn name(&self) -> &str { "list" }
    fn size(&self) -> egui::Vec2
    {
        let extra_vertical_gap = 60.0 * self.size as f32;
        egui::vec2(300.0, 220.0 + extra_vertical_gap)
    }

    fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        vec![
            SettingsKind::Integer { label: "size", value: &mut self.size },
            SettingsKind::Choice {
                label: "value",
                value: &mut self.value_type,
                options: vec!["integer", "float", "point2d"],
            },
        ]
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        let mut inputs = Vec::with_capacity(self.size);

        let base_value = Value::from_type_string(&self.value_type).unwrap_or(Value::Integer(0));

        let mut alphabet_counter = AlphabetCounter::new();

        for _ in 0..self.size
        {
            let letter = alphabet_counter.next_letter().to_string();
            inputs.push( PortDefinition::new_input_data_port(letter, vec![ base_value.clone() ]) );
        }
        
        inputs
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_output_data_port("".to_string(), vec![ Value::List( Vec::new() ) ])
        ]
    }
    fn compile(&self, input: &Vec<RegisterAddress>, output: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        if Value::from_type_string(&self.value_type).is_none() // This should never happen
        {
            return Err(format!("Unknown list value type: {}", self.value_type));
        }
        Ok(CompilePlan { steps: vec![
                                    CompilePlanStep::Emit(Instruction::CreateList(input.clone(), output[0]))] // @TODO, this clone is potentially expensive, as all ports before should remain constant, this can likely be optimized
        })
    }
}

