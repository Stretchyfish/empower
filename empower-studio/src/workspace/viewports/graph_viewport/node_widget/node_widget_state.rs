use egui_dock::node;
use empower_node_graph::{EmpowerData, EmpowerKey};
use crate::graph_editor::display_node::DisplayNode; // @TODO, simplify this include
use empower_node_graph::Node;
use empower_node_graph::node::{NodeState, NumberNodeState};
use super::{NodeWidgetResponse, NodeWidgetResponseType};

pub fn show_node_widget_state(ui: &mut egui::Ui, empower_node: &Node, display_node: &DisplayNode, node_widget_response: &mut Option<NodeWidgetResponse>)
{
    let node_key = empower_node.key.clone();

    match &empower_node.state
    {
        NodeState::NumberState( state ) =>
        {
            show_number_node_widget_state(ui, node_key, state, display_node, node_widget_response);
        }

        NodeState::None =>
        {

        }
    }
}

fn show_number_node_widget_state(ui: &mut egui::Ui, node_key: EmpowerKey, state: &NumberNodeState, display_node: &DisplayNode, node_widget_response: &mut Option<NodeWidgetResponse>)
{
    // @TODO, consider a better way to calculate the position
    let convertion_rect = egui::Rect::from_center_size(display_node.position + display_node.size / 2.0 + egui::Vec2 { x: 0.0, y: 90.0 }, egui::Vec2 { x: 200.0, y: 200.0 });

    let mut current_convertion = state.data_conversion.clone();

    ui.allocate_ui_at_rect(convertion_rect, |ui|
    {
        // @TODO, make id unique
        egui::ComboBox::new(egui::Id::from("test"), String::new())
        // .selected_text(current_convertion.get_type())
         .selected_text( egui::RichText::new(current_convertion.get_type()).font( egui::FontId::proportional( 36.0 ) ))
       .show_ui(ui, |ui|
        {
            for option in [EmpowerData::Undefined(String::new()), EmpowerData::Integer(0), EmpowerData::Float(0.0)]
            {
                ui.selectable_value(
                    &mut current_convertion ,
                    option.clone(), 
                    option.get_type() );
                //  egui::RichText::new(option.get_type()).font( egui::FontId::proportional( 36.0 ) ) );
            }
        });
    });

    if current_convertion != state.data_conversion
    {
        let new_state = NodeState::NumberState( NumberNodeState { data_conversion: current_convertion });
        *node_widget_response = Some( 
                                        NodeWidgetResponse { 
                                            key: node_key, 
                                            kind: NodeWidgetResponseType::ChangedState( new_state )} );
    }

}