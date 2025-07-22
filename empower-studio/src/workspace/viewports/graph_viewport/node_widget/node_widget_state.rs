use egui_dock::node;
use empower_node_graph::{EmpowerData, EmpowerKey};
use crate::graph_editor::display_node::DisplayNode; // @TODO, simplify this include
use empower_node_graph::Node;
use empower_node_graph::node::{NodeState, NumberNodeState};
use empower_node_graph::node::node_state::number_node_state::ConvertionApproach;
use super::{NodeWidgetResponse, NodeWidgetResponseType};

pub fn show_node_widget_state(ui: &mut egui::Ui, empower_node: &Node, display_node: &DisplayNode, graph_viewport_title: &String, node_widget_response: &mut Option<NodeWidgetResponse>)
{
    let node_key = empower_node.key.clone();

    match &empower_node.state
    {
        NodeState::NumberState( state ) =>
        {
            show_number_node_widget_state(ui, node_key, state, display_node, graph_viewport_title, node_widget_response);
        }

        NodeState::None =>
        {

        }
    }
}

fn show_number_node_widget_state(ui: &mut egui::Ui, node_key: EmpowerKey, state: &NumberNodeState, display_node: &DisplayNode, graph_viewport_title: &String, node_widget_response: &mut Option<NodeWidgetResponse>)
{
    // @TODO, consider a better way to calculate the position
    let convertion_rect = egui::Rect::from_center_size(display_node.position + display_node.size / 2.0 + egui::Vec2 { x: 0.0, y: 90.0 }, egui::Vec2 { x: 200.0, y: 200.0 });

    let mut current_convertion = state.current_convertion_approach.clone();

    ui.allocate_ui_at_rect(convertion_rect, |ui|
    {
        egui::ComboBox::new(egui::Id::from( format!("node_{}_state_{}", node_key, graph_viewport_title ) ), String::new())
        .width(200.0)
        .selected_text( egui::RichText::new(current_convertion.to_string()).font( egui::FontId::proportional( 36.0 ) ))
        // .selected_text( current_convertion.to_string() )
       .show_ui(ui, |ui|
        {
            for option in [ ConvertionApproach::Automatic, ConvertionApproach::Int, ConvertionApproach::Float]
            {
                ui.selectable_value(
                    &mut current_convertion,
                    option.clone(), 
                    option.to_string() );

                // let button = ui.add_sized(egui::vec2(10.0, 20.0), egui::SelectableLabel::new(
                //     current_convertion == option.clone(), option.to_string() ));

                // if button.clicked()
                // {
                //     current_convertion = option.clone();
                // }
            }
        });
    });

    if current_convertion != state.current_convertion_approach
    {
        let new_state = NodeState::NumberState( NumberNodeState { current_convertion_approach: current_convertion });
        *node_widget_response = Some( 
                                        NodeWidgetResponse { 
                                            key: node_key, 
                                            kind: NodeWidgetResponseType::ChangedState( new_state )} );
    }
}