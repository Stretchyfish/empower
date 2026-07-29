use std::collections::{HashMap, HashSet};

pub mod node;
pub use node::Node;
pub use node::node_kind::NodeEdit;

pub mod port;
pub use port::Port;
use serde::{Deserialize, Serialize};

use crate::node_graph::{node::node_kind::NODE_KIND_REGISTRY, port::PortDefinition};

pub type NodeGraphKey = i32;

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeGraph {
    pub name: String,
    pub start_node_key: NodeGraphKey, // @TODO, add a const for start node key?
    pub end_node_key: Option<NodeGraphKey>,

    pub nodes: HashMap<NodeGraphKey, Node>,
    pub ports: HashMap<NodeGraphKey, Port>,

    pub connections_in: HashMap<NodeGraphKey, NodeGraphKey>, // inputs -> outputs
    pub connections_out: HashMap<NodeGraphKey, HashSet<NodeGraphKey>>, // outputs -> inputs
}

impl NodeGraph {
    pub fn new(name: &'static str) -> Self {
        let mut node_graph = Self {
            name: name.to_string(),
            start_node_key: 1, // @TODO, make this some kind of const?
            end_node_key: None,

            nodes: HashMap::new(),
            ports: HashMap::new(),

            connections_out: HashMap::new(),
            connections_in: HashMap::new(),
        };

        let _ = node_graph.add_node("start", None);

        node_graph
    }

    pub fn from_json(node_graph_json: &String) -> Result<NodeGraph, serde_json::Error> {
        serde_json::from_str(node_graph_json)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn new_entry_graph() -> Self // @TODO, this function could probably be removed
    {
        NodeGraph::new("entry_graph")
    }

    pub fn add_node(
        &mut self,
        node_name: &'static str,
        position: Option<egui::Pos2>,
    ) -> NodeGraphKey {
        let node_kind_constructor = match NODE_KIND_REGISTRY.get(node_name) {
            Some(constructor) => constructor,
            None => panic!("Requested a non-existing node name"),
        };

        let node_kind = node_kind_constructor();

        let input_port_definitions = node_kind.input_port_definitions();
        let output_port_definitions = node_kind.output_port_definitions();

        let node_key = self.get_available_node_key();

        let input_port_keys = self.add_ports(&node_key, input_port_definitions);
        let output_port_keys = self.add_ports(&node_key, output_port_definitions);

        let new_node = Node::new(
            position.unwrap_or(egui::Pos2::default()),
            input_port_keys,
            output_port_keys,
            node_kind,
        );

        self.nodes.insert(node_key, new_node);

        node_key
    }

    pub fn create_node_copy(&mut self, node_key: &NodeGraphKey) -> NodeGraphKey
    {
        // @TODO, this whole function needs a second look

        let mut cloned_node = self.nodes.get(node_key).expect("ERROR in create_node_copy, asked to get node with incorrect key").clone();

        let new_node_key = self.get_available_node_key();

        // cloned_node.key = new_node_key;

        // @TODO, consider finding a way of utilizing the create input and output port function here
        let mut new_input_port_keys = Vec::with_capacity(cloned_node.input_port_keys.len());
        for input_port_key in &cloned_node.input_port_keys
        {
            let mut input_port_copy = self.ports.get(input_port_key).expect("ERROR in create_node_copy, requested to fetch input port with a key not in the input port list")
                                                                                .clone();
            let new_copied_input_port_key = self.get_available_port_key();
            input_port_copy.key = new_copied_input_port_key;
            input_port_copy.node_key = new_node_key;

            new_input_port_keys.push(input_port_copy.key);
            self.ports.insert(input_port_copy.key, input_port_copy);

            if self.connections_in.contains_key(input_port_key)
            {
                let connection = self.connections_in.get(input_port_key).unwrap().clone();
                let add_connection_result = self.add_connection(&connection, &new_copied_input_port_key);

                if add_connection_result.is_err()
                {
                    println!("{:?}", "failed to transfer connection to new port");
                }
            }
        }

        let mut new_output_port_keys = Vec::with_capacity(cloned_node.output_port_keys.len());
        for output_port_key in &cloned_node.output_port_keys
        {
            let mut output_port_copy = self.ports.get(output_port_key).expect("ERROR in create_node_copy, requested to fetch output port with a key not in the output port list")
                                                                                    .clone();
            output_port_copy.key = self.get_available_port_key();
            output_port_copy.node_key = new_node_key;

            new_output_port_keys.push(output_port_copy.key);
            self.ports.insert(output_port_copy.key, output_port_copy);
        }

        cloned_node.input_port_keys = new_input_port_keys.clone();
        cloned_node.output_port_keys = new_output_port_keys.clone();

        self.nodes.insert(new_node_key, cloned_node);

        new_node_key
    }

    pub fn refresh_node(&mut self, node_key: &NodeGraphKey) {
        // @TODO, this function is still far from finished
        // - output ports changing
        // - check if a connection is still valid for all ports
        // - removal of the first ports withuot overwritting all other ports (maybe?)

        let mut ports_to_replace = Vec::new();
        let mut input_ports_to_add = Vec::new();
        let mut output_ports_to_add = Vec::new();
        let mut input_ports_to_remove = Vec::new();
        let mut output_ports_to_remove = Vec::new();

        {
            let (node, input_ports, output_ports) = self.get_node_input_output(*node_key).unwrap();

            let new_input_port_definitions = node.kind.input_port_definitions();
            let new_output_port_definitions = node.kind.output_port_definitions();

            for (index, new_input_port_definition) in new_input_port_definitions.iter().enumerate()
            {
                let port = input_ports.get(index);

                if port.is_none() {
                    input_ports_to_add.push(new_input_port_definition.clone());
                    continue;
                }

                let port = port.as_ref().unwrap();

                if port.compatability == new_input_port_definition.compatability {
                    continue;
                }

                ports_to_replace.push((port.key, new_input_port_definition.clone()));
            }

            for (index, new_output_port_definitions) in new_output_port_definitions.iter().enumerate()
            {
                let port = output_ports.get(index);

                if port.is_none()
                {
                    output_ports_to_add.push(new_output_port_definitions.clone());
                    continue;
                }

                let port = port.as_ref().unwrap();

                if port.compatability == new_output_port_definitions.compatability
                {
                    continue;
                }

                ports_to_replace.push((port.key, new_output_port_definitions.clone()));
            }

            if new_input_port_definitions.len() < node.input_port_keys.len() {
                input_ports_to_remove = node.input_port_keys[new_input_port_definitions.len()..].to_vec();
            }

            if new_output_port_definitions.len() < node.output_port_keys.len() {
                output_ports_to_remove = node.output_port_keys[new_output_port_definitions.len()..].to_vec();
            }
        }

        for (port_key, port_definition) in ports_to_replace {
            self.ports
                .insert(port_key, Port::new(port_key, *node_key, port_definition));
        }

        let mut new_input_port_keys = Vec::new();
        for port_definition in input_ports_to_add {
            let new_port_key = self.get_available_port_key();
            self.ports.insert(
                new_port_key,
                Port::new(new_port_key, *node_key, port_definition),
            );

            new_input_port_keys.push(new_port_key);
        }

        let mut new_output_port_keys = Vec::new();
        for port_definition in output_ports_to_add {
            let new_port_key = self.get_available_port_key();
            self.ports.insert(
                new_port_key,
                Port::new(new_port_key, *node_key, port_definition),
            );

            new_output_port_keys.push(new_port_key);
        }

        let node = self.nodes.get_mut(node_key).unwrap();
        node.input_port_keys.extend(new_input_port_keys);
        node.output_port_keys.extend(new_output_port_keys);

        for port_key in &input_ports_to_remove {
            node.input_port_keys.retain(|e| e != port_key);
            self.ports.remove(&port_key);
        }

        for port_key in &output_ports_to_remove {
            node.input_port_keys.retain(|e| e != port_key);
            self.ports.remove(&port_key);
        }

        for port_key in input_ports_to_remove {
            self.remove_connection(&port_key);
        }

        // @TODO, still need to add removal of output port connections
    }

    pub fn remove_node(&mut self, node_key: &NodeGraphKey) -> bool
    {
        let (input_port_keys, output_port_keys) =
        {
            let node = self.nodes.get(node_key).unwrap();
            ( node.input_port_keys.clone() , node.output_port_keys.clone() )
        };

        self.nodes.remove(node_key);

        for input_port_key in input_port_keys
        {
            self.ports.remove(&input_port_key);
            self.remove_connection(&input_port_key);
        }

        for output_port_key in output_port_keys
        {
            self.ports.remove(&output_port_key);

            if self.connections_out.contains_key(&output_port_key) // @TODO, this approach is not great, but works for now
            {
                for input_port_key in self.connections_out.get(&output_port_key).unwrap().clone()
                {
                    self.remove_connection(&input_port_key);
                }
            }
        }

        true
    }

    fn add_ports(
        &mut self,
        node_key: &NodeGraphKey,
        port_definitions: Vec<PortDefinition>,
    ) -> Vec<NodeGraphKey> {
        let mut new_port_keys = Vec::with_capacity(port_definitions.len());
        for port_definition in port_definitions {
            let new_port_key = self.get_available_port_key();
            let new_port = Port::new(new_port_key, *node_key, port_definition);
            self.ports.insert(new_port_key, new_port);

            new_port_keys.push(new_port_key);
        }

        new_port_keys
    }

    pub fn add_connection(
        &mut self,
        from_port_key: &NodeGraphKey,
        to_port_key: &NodeGraphKey,
    ) -> Result<(), &'static str> {
        if from_port_key == to_port_key {
            return Err("cannot connect to self");
        }

        let from_port = self.ports.get(from_port_key).unwrap(); // This is slightly dangerous, but with the current editor implementation it should be safe (maybe revise in the future though)
        let to_port = self.ports.get(to_port_key).unwrap();

        if !from_port.compatible_with(to_port) {
            return Err("ports not compatible");
        }

        self.connections_in.insert(*to_port_key, *from_port_key); // The ports are switched upon insert as output ports has an 1:N relation and inputs have a 1:1 relation to other ports, the order is then reversed during compilation.
        self.connections_out
            .entry(*from_port_key)
            .or_default()
            .insert(*to_port_key);

        Ok(())
    }

    pub fn remove_connection(&mut self, to_port_key: &NodeGraphKey) -> Option<NodeGraphKey> {
        if !self.connections_in.contains_key(to_port_key) {
            return None;
        }

        let from_port_key = self.connections_in.remove(to_port_key).unwrap();

        let remove_connection_out = {
            let to_port_keys = self.connections_out.get_mut(&from_port_key).unwrap();
            to_port_keys.remove(to_port_key);

            to_port_keys.is_empty()
        };

        if remove_connection_out {
            self.connections_out.remove(&from_port_key);
        }

        Some(from_port_key)
    }

    pub fn contains_connection(&mut self, to_port_key: &NodeGraphKey) -> bool {
        self.connections_in.contains_key(to_port_key) // We don't need to check both connections, as they should be macthing
    }

    fn get_available_node_key(&self) -> NodeGraphKey {
        self.nodes.keys().max().unwrap_or(&0) + 1
    }

    fn get_available_port_key(&self) -> NodeGraphKey {
        self.ports.keys().max().unwrap_or(&0) + 1
    }

    pub fn get_node_input_output(
        &self,
        node_key: NodeGraphKey,
    ) -> Option<(&Node, Vec<&Port>, Vec<&Port>)> {
        let node = self.nodes.get(&node_key).unwrap();
        let mut inputs = Vec::new();
        let mut ouptuts = Vec::new();

        for port_key in &node.input_port_keys {
            let port = self.ports.get(port_key).unwrap();
            inputs.push(port);
        }

        for port_key in &node.output_port_keys {
            let port = self.ports.get(port_key).unwrap();
            ouptuts.push(port);
        }

        Some((node, inputs, ouptuts))
    }

    pub fn get_node_input_ports(&self, node_key: NodeGraphKey) -> Vec<Port>
    {
        let node = self.nodes.get(&node_key).unwrap();
        let mut inputs = Vec::new();

        for port_key in &node.input_port_keys
        {
            let port = self.ports.get(port_key).unwrap().clone();
            inputs.push(port);
        }

        inputs
    }

    pub fn get_node_output_ports(&self, node_key: NodeGraphKey) -> Vec<Port>
    {
        let node = self.nodes.get(&node_key).unwrap();
        let mut outputs = Vec::new();

        for port_key in &node.output_port_keys
        {
            let port = self.ports.get(port_key).unwrap().clone();
            outputs.push(port);
        }

        outputs
    }

    pub fn get_connected_exec_nodes(&self, node_key: NodeGraphKey) -> Vec<NodeGraphKey> {
        let node = self.nodes.get(&node_key).unwrap();

        if node.output_port_keys.is_empty() {
            return Vec::new();
        }

        let exec_port = self.ports.get(&node.output_port_keys[0]).unwrap();

        match exec_port.kind {
            port::PortKind::Execution => {}
            port::PortKind::Data => {
                return Vec::new();
            }
        }

        Vec::new()
    }
}
