// use empower_node_graph::node::node_state::number_node_state::ConvertionApproach;
// use empower_node_graph::node::node_type;
// use empower_node_graph::node::NodeState;
// use empower_node_graph::port::PortType;
// use empower_node_graph::EmpowerData;
// use empower_node_graph::EmpowerKey;
// use empower_node_graph::EmpowerNodeGraph;
// use empower_node_graph::Node;
// use empower_node_graph::NodeType;

use empower_engine::node_graph::port::port_value;
use empower_engine::NodeGraph;
use empower_engine::NodeGraphKey;
use empower_engine::NodeKind;

use std::collections::HashMap;

pub mod display_node;
use display_node::DisplayNode;

pub mod display_port;
use display_port::DisplayPort;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use crate::graph_editor::display_port::display_port_value::DisplayPortValueType;

// use crate::graph_editor::display_port::DisplayPortValueRepresentation;
// use display_port::DisplayPortValueRepresentation;

#[derive(Default, Clone)]
pub struct GraphEditor
{
    pub(crate) display_nodes: HashMap<NodeGraphKey, DisplayNode>, 
    pub(crate) display_input_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub(crate) display_output_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub(crate) node_graph: NodeGraph, 
    pub(crate) selected_nodes: Vec<NodeGraphKey>,
}

impl GraphEditor
{
    pub fn new() -> Self
    {
        let mut graph_editor = Self
        {
            display_nodes: HashMap::new(),
            display_input_ports: HashMap::new(),
            display_output_ports: HashMap::new(),
            node_graph: NodeGraph::new(),
            selected_nodes: Vec::new(),
        };

        let new_node_type = NodeKind::Start;

        // @TODO, find a better approach to centering the start node
        let start_node_left_offset = egui::Pos2 { x: -1700.0, y: -165.0 / 2.0 }; // Half the center nodes height and oriented left

        graph_editor.add_node(new_node_type, start_node_left_offset);

        graph_editor
    }

    pub fn add_node(&mut self, node_kind: NodeKind, position: egui::Pos2) -> bool
    {
        // @TODO, find a better way of assigning keys
        let node_handle= self.node_graph.add_node(node_kind); 

        if self.display_nodes.contains_key(&node_handle.node_key) // @TODO, simplify these calls
        {
            println!("ERROR, attempted to add engine node key already in node graph (denied)"); // This should never happen, only if something wrongly implemented
            return false;
        }

        let node = match self.node_graph.get_node(&node_handle.node_key)
        {
            Some( node ) => node,
            None => 
            {
                println!("Add node tried to fetch node {} that doesn't exist", node_handle.node_key);
                return false;
            }
        };

        let display_node_title = node.kind.to_string();

        let input_port_values = self.node_graph.get_node_input_port_values(&node_handle.node_key);
        let output_port_values = self.node_graph.get_output_port_values(&node_handle.node_key);

        let display_node_size = display_node::display_node_kind::get_display_node_size(&node_kind);
        let display_node_value_offset = display_node::display_node_kind::get_display_node_value_offset(&node_kind);
        let input_port_display_values = display_node::display_node_kind::get_display_input_ports(&node_kind, input_port_values);
        let output_port_display_values = display_node::display_node_kind::get_display_output_ports(&node_kind, output_port_values);

        // let extra_input_port_offset; // @TODO, consider if this is the correct approach
        // match new_node_type 
        // {
        //     NodeType::Start => 
        //     {
        //         display_node_size = egui::Vec2 { x: 250.0, y: 165.0 };
        //         extra_input_port_offset = 0.0;
        //         input_port_names = Vec::new();
        //         output_port_names = vec![ "".to_string() ]
        //     },
        //     NodeType::IntegerVariable => 
        //     {
        //         display_node_size = egui::Vec2 { x: 450.0, y: 165.0 };
        //         extra_input_port_offset = 0.0;
        //         input_port_names = vec![ "int".to_string() ];
        //         output_port_names = vec![ "".to_string()];
        //     },
        //     NodeType::Addition => 
        //     {
        //         display_node_size = egui::Vec2 { x: 450.0, y: 220.0 };
        //         extra_input_port_offset = 0.0;
        //         input_port_names = vec![ "A".to_string(), "B".to_string() ];
        //         output_port_names = vec![ "".to_string() ];
        //     },
        //     NodeType::Multiply =>
        //     {
        //         display_node_size = egui::Vec2 { x: 450.0, y: 220.0 };
        //         extra_input_port_offset = 0.0;
        //         input_port_names = vec![ "A".to_string(), "B".to_string() ];
        //         output_port_names = vec![ "".to_string() ];
        //     },
        //     NodeType::Print => 
        //     {
        //         display_node_size = egui::Vec2 { x: 300.0, y: 220.0 };
        //         extra_input_port_offset = 0.0;
        //         input_port_names = vec![ "".to_string(), "print".to_string() ];
        //         output_port_names = Vec::new();
        //     },
        //     NodeType::Number => 
        //     {
        //         display_node_size = egui::Vec2 { x: 450.0, y: 205.0 };
        //         extra_input_port_offset = 45.0;
        //         input_port_names = vec![ "value".to_string() ];
        //         output_port_names = vec![ "out".to_string() ];
        //     },
        //     NodeType::Text =>
        //     {
        //         display_node_size = egui::Vec2 { x: 450.0, y: 165.0 };
        //         extra_input_port_offset = 0.0;
        //         input_port_names = vec![ "text".to_string() ];
        //         output_port_names = vec![ "out".to_string() ];
        //     },
        //     NodeType::Bool =>
        //     {
        //         display_node_size = egui::Vec2 { x: 450.0, y: 165.0 };
        //         extra_input_port_offset = 0.0;
        //         input_port_names = vec![ "".to_string() ];
        //         output_port_names = vec![ "out".to_string() ];
        //     }
        // }

        if input_port_display_values.len() != node.input_port_keys.len()
        {
            println!("ERROR, the size of node in graph editor (add_node) input port and display input ports does not match");
            return false;
        }
        if output_port_display_values.len() != node.output_port_keys.len()
        {
            println!("ERROR, the size of node in graph editor (add_node) output and display output ports does not match");
            return false;
        }
              
        let port_gap = 60.0;
        let mut input_port_offset = 120.0 + display_node_value_offset;
        let mut output_port_offset = input_port_offset.clone();

        for (input_port_key_index, input_port_key) in node.input_port_keys.iter().enumerate()
        {
            // let display_port_value_representation = self.get_input_port_value_representation(input_port_key);
            let new_display_port = DisplayPort { 
                                                        node_key: node.key.clone(), 
                                                        relative_position: egui::Vec2::new(0.0, input_port_offset), 
                                                        display_value: input_port_display_values[input_port_key_index].clone(),
                                                        // text: input_port_names[input_port_key_index].clone(),
                                                        // value_representation: DisplayPortValueRepresentation::None, 
                                                        // value_representation_valid: true 
                                                        }; 
            self.display_input_ports.insert(input_port_key.clone(), new_display_port);

            input_port_offset += port_gap;
        }

        for (output_port_key_index, output_port_key) in node.output_port_keys.iter().enumerate()
        {
            // @TODO, figure out if setting display port values is needed for output
            let new_display_port = DisplayPort { 
                                                    node_key: node.key.clone(), 
                                                    relative_position: egui::Vec2::new(display_node_size.x, output_port_offset), 
                                                    display_value: output_port_display_values[output_port_key_index].clone(),
                                                    // value_representation: DisplayPortValueRepresentation::Text( String::new() ), 
                                                    // value_representation: DisplayPortValueRepresentation::None, 
                                                    // value_representation_valid: false // @TODO, decide if this should be true?
                                                    }; 
            self.display_output_ports.insert(output_port_key.clone(), new_display_port);

            output_port_offset += port_gap;
        }

        let new_display_node = DisplayNode::new( display_node_title, position, display_node_size );
        self.display_nodes.insert(node.key.clone(), new_display_node );

        true
    }

    // pub fn remove_node(&mut self, node_key: &EmpowerKey)
    // {
    //     let node = self.empower_node_graph.nodes.get(node_key).unwrap().clone();
    //     let input_port_keys = node.input_port_keys;
    //     let output_port_keys = node.output_port_keys;

    //     self.empower_node_graph.remove_node(*node_key); // @TODO, remove connections

    //     for input_port_key in input_port_keys
    //     {
    //         self.display_input_ports.remove(&input_port_key);
    //     }

    //     for output_port_key in output_port_keys
    //     {
    //         self.display_output_ports.remove(&output_port_key);
    //     }

    //     self.display_nodes.remove(node_key);
    // }

    pub fn set_input_port_value(&mut self, input_port_key: &NodeGraphKey, display_port_value: DisplayPortValue)
    {
        let input_port = match self.node_graph.get_mut_input_port(input_port_key)
        {
            Some( port ) => port,
            None => panic!("ERROR, tried to set display input port in set input port value, but input port doesn't exist."),
        };

        let display_input_port = match self.display_input_ports.get_mut(input_port_key)
        {
            Some( port ) => port,
            None => panic!("ERROR, tried to get display input port in set input port value, but input port doesn't exist."),
        };

        display_input_port.display_value = display_port_value.clone();

        // @TODO, consider if this should be a result instead of an optional
        let new_input_port_value = match display_port_value.to(&input_port.compatability)
        {
            Some( input_port_value) => 
            {
                display_input_port.display_value.display_value_valid = true;
                input_port_value
            }
            None => 
            {
                display_input_port.display_value.display_value_valid = false;
                return
            }
        };

        input_port.value = new_input_port_value;
    }

    pub fn refresh_all_node_display(&mut self)
    {
        // @TODO, try to find a better approach for this
        let node_keys = self.node_graph.get_all_node_keys();
        for node_key in node_keys
        {
            self.refresh_node_display(&node_key);
        }
     }

    pub fn refresh_node_display(&mut self, node_key: &NodeGraphKey)
    {
        let node_handle = self.node_graph.get_node_handle(node_key);

        for input_port_key in node_handle.input_port_keys
        {
            let input_port= self.node_graph.get_input_port(&input_port_key).expect("Failed to fetch input port");
            let display_input_port = self.display_input_ports.get_mut(&input_port_key).expect("Failed to fetch display input port");

            // display_input_port.display_value.value_type = DisplayPortValueType::new(&input_port.value);
            display_input_port.display_value.update(&input_port.value);
        }

        for output_port_key in node_handle.output_port_keys
        {
            let output_port= self.node_graph.get_output_port(&output_port_key).expect("Failed to fetch output port");
            let display_output_port= self.display_output_ports.get_mut(&output_port_key).expect("Failed to fetch display output port");

            // display_output_port.display_value.value_type = DisplayPortValueType::new(&output_port.value);
            display_output_port.display_value.update(&output_port.value);
        }
    }


    // pub fn set_input_port_value_from_representation(&mut self, input_port_key: &EmpowerKey, value_representation: DisplayPortValueRepresentation) -> bool
    // {
    //     let input_port = self.empower_node_graph.input_ports.get_mut(input_port_key).unwrap();

    //     match value_representation
    //     {
    //         DisplayPortValueRepresentation::Text( value_text ) =>
    //         {
    //             match input_port.value
    //             {
    //                 EmpowerData::Undefined(_) =>
    //                 {
    //                     input_port.value = EmpowerData::Undefined( value_text );
    //                     return true;
    //                 },
    //                 EmpowerData::Integer(_) => 
    //                 {
    //                     let parsed_integer = value_text.parse::<i32>();

    //                     if parsed_integer.is_ok()
    //                     {
    //                         println!("was successfull");
    //                         input_port.value = EmpowerData::Integer( parsed_integer.unwrap() );
    //                         return true;
    //                     }
    //                 },
    //                 EmpowerData::Float(_) =>
    //                 {
    //                     let parsed_float = value_text.parse::<f32>();

    //                     if parsed_float.is_ok()
    //                     {
    //                         println!("was successfull 2");
    //                         input_port.value = EmpowerData::Float( parsed_float.unwrap() );
    //                         return true;
    //                     }
    //                 },
    //                 EmpowerData::Text(_) =>
    //                 {
    //                     input_port.value = EmpowerData::Text( value_text );
    //                     return true;
    //                 },
    //                 _ =>
    //                 {
    //                     println!("ERROR, trying to add value representation text to invalid in graph editor");
    //                 },
    //             }
    //         },

    //         DisplayPortValueRepresentation::Checkbox( value_bool ) =>
    //         {
    //             match input_port.value
    //             {
    //                 EmpowerData::Bool(_) =>
    //                 {
    //                     input_port.value = EmpowerData::Bool( value_bool );
    //                     return true;
    //                 },
    //                 _ =>
    //                 {

    //                 }
    //             }
    //         },

    //         DisplayPortValueRepresentation::None =>
    //         {

    //         },
    //     }

    //     false
    // }

    // pub fn get_input_port_value_representation(&self, input_port_key: &EmpowerKey) -> DisplayPortValueRepresentation
    // {
    //     let input_port_value = self.empower_node_graph.input_ports.get(input_port_key).unwrap().value.clone();

    //     match input_port_value
    //     {
    //         EmpowerData::Integer(integer) =>
    //         {
    //             DisplayPortValueRepresentation::Text( integer.to_string() )
    //         },

    //         EmpowerData::Float(float) =>
    //         {
    //             DisplayPortValueRepresentation::Text( float.to_string() )
    //         },

    //         EmpowerData::Trigger =>
    //         {
    //             DisplayPortValueRepresentation::None
    //         },

    //         EmpowerData::Undefined(ref text) =>
    //         {
    //             DisplayPortValueRepresentation::Text( text.clone() )
    //         }

    //         EmpowerData::Text(ref text) =>
    //         {
    //             DisplayPortValueRepresentation::Text( text.clone() )
    //         }

    //         EmpowerData::Bool(boolean) =>
    //         {
    //             DisplayPortValueRepresentation::Checkbox( boolean )
    //         }

    //         EmpowerData::Unknown =>
    //         {
    //             DisplayPortValueRepresentation::None
    //         },
    //     }
    // }

    // pub fn refresh_display_port_values(&mut self) // @TODO, create one that only refreshed a single node?
    // {
    //     for input_port_key in self.empower_node_graph.input_ports.keys()
    //     {
    //         let display_port_representation = self.get_input_port_value_representation(input_port_key);

    //         let display_port = self.display_input_ports.get_mut(input_port_key).unwrap();
    //         display_port.value_representation = display_port_representation;
    //     }
    // }

    // pub fn input_port_has_connection(&self, port_key: &EmpowerKey) -> bool
    // {
    //     self.empower_node_graph.connections_in.contains_key(port_key)
    // }

    // pub fn get_input_port_connection_key(&self, input_port_key: &EmpowerKey) -> EmpowerKey
    // {
    //     // @TODO, make safe
    //     let connected_output_port_key = self.empower_node_graph.connections_in.get(input_port_key).unwrap();
    //     *connected_output_port_key
    // }

    // pub fn compile_individual_node(&self, node_key: &EmpowerKey)
    // {

    // }

    // pub fn change_node_state(&mut self, node_key: &EmpowerKey, new_node_state: &NodeState)
    // {
    //     let successfully_set_state = self.empower_node_graph.set_node_state(node_key, new_node_state);

    //     if !successfully_set_state
    //     {
    //         return;
    //     }

    //     self.refresh_display_port_values();
    // }
}
