use eframe::glow::DISPLAY_LIST;
use egui;

use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::NodeType;

mod display_node_graph;
use display_node_graph::DisplayNodeGraph;

pub struct StudioContext
{
    pub empower_node_graph: EmpowerNodeGraph, // @TODO, should make these private
    pub display_node_graph: DisplayNodeGraph,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            empower_node_graph: EmpowerNodeGraph::new(),
            display_node_graph: DisplayNodeGraph::new(),
        }
    }

    pub fn add_node(&mut self, node_type: NodeType, position: egui::Pos2) -> EmpowerKey
    {
        let new_node_type = NodeType::IntegerVariable;
        let empower_node_key: EmpowerKey = self.empower_node_graph.add_node(new_node_type); 

        if self.display_node_graph.display_nodes.contains_key(&empower_node_key) // @TODO, simplify these calls
        {
            println!("ERROR, attempted to add engine node key already in node graph (denied)"); // This should never happen, only if something wrongly implemented
            return empower_node_key;
        }

        let empower_node = self.empower_node_graph.nodes.get(&empower_node_key).unwrap(); // This should never fail @TODO, consider simplifying this call

        self.display_node_graph.add_display_node(empower_node, position);

        empower_node_key
    }

    pub fn add_connection(&mut self, input_port_key: EmpowerKey, output_port_key: EmpowerKey)
    {
        self.empower_node_graph.add_connection(input_port_key, output_port_key);
    }

    pub fn show_node(&mut self, ui: &mut egui::Ui, display_node_key: &EmpowerKey)
    {
        // let mut graph_node_reponse = Option::None;

        // Draw connections first to avoid overlaps
        let output_port_keys = self.empower_node_graph.nodes.get(display_node_key).unwrap().output_port_keys.clone();
        for output_port_key in output_port_keys.iter()
        {
            if !self.empower_node_graph.connections.contains_key(output_port_key) // @TODO, simplify this call
            // if !node_graph.engine.connections.contains_key(output_port_key)
            {
                continue;
            }

            let display_output_port = self.display_node_graph.display_output_ports.get(output_port_key).unwrap();
            let display_node_position = self.display_node_graph.display_nodes.get(output_port_key).unwrap().position.clone();

            let connected_port_keys = self.empower_node_graph.connections.get(output_port_key).unwrap(); // @TODO, simplify this call
            for connected_port_key in connected_port_keys
            {
                if !self.display_node_graph.display_input_ports.contains_key(connected_port_key) // @TODO, simplify this call
                {
                    println!("ERROR, while trying to draw connection, key not in hashtable");
                    continue; 
                }

                // @TODO, add this back!
                let connected_display_port = self.display_node_graph.display_input_ports.get(connected_port_key).unwrap(); // @TODO simplify these calls
                let connected_display_node_position = self.display_node_graph.display_nodes.get(&connected_display_port.node_key).unwrap().position.clone();

                let connected_port_position = connected_display_node_position + connected_display_port.relative_position;


                let output_port_position = display_node_position + display_output_port.relative_position;

                ui.painter().line_segment([ output_port_position, connected_port_position], egui::Stroke::new(10.0, egui::Color32::YELLOW));
            }
        }

        let display_node = self.display_node_graph.display_nodes.get_mut(&display_node_key).unwrap(); // @TODO, simplify this call
        let empower_node = self.empower_node_graph.nodes.get_mut(&display_node_key).unwrap(); // @TODO, simplify this call


        display_node.show(ui, empower_node);

        let node_position = display_node.position.clone();
        
        for input_port_key in empower_node.input_port_keys.iter() // @TODO, find a better way to approach this?
        {
            let display_input_port = self.display_node_graph.display_input_ports.get_mut(input_port_key).unwrap();// @TODO, change this back to being borrowed once the value is read from engine instead of display port
            let empower_input_port = self.empower_node_graph.input_ports.get_mut(input_port_key).unwrap();
            // @TODO, change the method of this call

            display_input_port.show_input_port(ui, display_node, empower_input_port);
        }

        for output_port_key in empower_node.output_port_keys.iter()
        {
            let display_output_port = self.display_node_graph.display_output_ports.get_mut(output_port_key).unwrap();
            let empower_output_port = self.empower_node_graph.output_ports.get_mut(output_port_key).unwrap();

            display_output_port.show_output_port(ui, display_node, empower_output_port);


        }

        
    }

}
