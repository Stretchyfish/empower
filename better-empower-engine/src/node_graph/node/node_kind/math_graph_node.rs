use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

#[derive(Clone)]
pub struct MathGraphNode
{
    graph: Option<Vec<[f64; 2]>>
}

impl NodeKind for MathGraphNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self { graph: None } )
    }

    fn name(&self) -> &'static str {
        "math graph"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Window
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
                [ 
                    PortCompatability::Exatch( PortValue::Trigger ),
                    PortCompatability::Exatch( PortValue::Vector( Vec::new() )),
                    PortCompatability::Exatch( PortValue::Vector( Vec::new() )),
                ]
            )
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn state(&mut self, _: &mut egui::Ui) {
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> {
        let x_values = match inputs[1]
        {
            PortValue::Vector(port_values) => port_values,
            _ => panic!("ERROR, invalid vector value"),
        };

        let y_values = match inputs[2]
        {
            PortValue::Vector(port_values) => port_values,
            _ => panic!("ERROR, invalid vector value"),
        };

        if x_values.len() != y_values.len()
        {
            println!("ERROR, math graph values are not same size");
            // return Some( Vec::new() );
            return None;
        }

        let mut x = Vec::new();
        let mut y = Vec::new();
        for i in 0..x_values.len()
        {
            let x_value = match x_values[i]
            {
                PortValue::Float( value ) => value,
                PortValue::Integer( value ) => value as f32,
                _ => panic!("ERROR, invalid value type"),
            };

            x.push(x_value);

            let y_value = match y_values[i]
            {
                PortValue::Float( value ) => value,
                PortValue::Integer( value ) => value as f32,
                _ => panic!("ERROR, invalid value type"),
            };

            y.push(y_value);
        }

        let graph = x.iter().zip(y.iter()).map(|(&x1, &y1)| [x1 as f64, y1 as f64]).collect();

        self.graph = Some( graph );

        None
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        None
        
    }

    fn execute(&mut self, ui: &mut egui::Ui) {

        if self.graph.is_none()
        {
            return;
        }

        let graph_clone = self.graph.clone().unwrap();

        Plot::new("My Plot")
        .legend(Legend::default())
        .show(ui, |plot_ui| 
        {
            plot_ui.line(Line::new(
                "3rd Curve",
                PlotPoints::from(graph_clone),
            ));
        });
    }
}
