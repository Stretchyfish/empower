use std::collections::btree_map::Range;

use crate::node_graph::port::{PortCompatability, PortValue};
use pgfplots::{axis::plot::{Plot2D}, Engine, Picture};
// use egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};


#[derive(Default, Clone, Debug)]
pub struct MathGraphState
{
    graph: Option<Vec<[f64; 2]>>
}

impl PartialEq for MathGraphState
{
    fn eq(&self, other: &Self) -> bool {
        self.graph == other.graph
    }
}

impl MathGraphState
{
    pub fn new() -> Self
    {
        Self 
        { 
            graph: None,
        }
    }
}

pub fn get_name() -> &'static str
{
    "math graph"
}

pub fn get_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
            [ 
                PortCompatability::Exatch( PortValue::Trigger ),
                PortCompatability::Exatch( PortValue::Vector( Vec::new() )),
                PortCompatability::Exatch( PortValue::Vector( Vec::new() )),
            ]
        )
}

pub fn get_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::new()
}

pub fn setup(state: &mut MathGraphState, inputs: Vec<&PortValue>)
{
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
        return;
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

    state.graph = Some( graph );
} 

pub fn execute(state: &MathGraphState, ctx: &egui::Context) -> Option<Vec<PortValue>>
{
    let state_clone = state.clone();

    let mut open = true;

    egui::Window::new("Math Graph")
    .collapsible(true)
    .title_bar(true)
    .open(&mut open)
    .show(ctx, |window_ui|
    {
        Plot::new("My Plot")
        .legend(Legend::default())
        .show(window_ui, |plot_ui| 
        {
            plot_ui.line(Line::new(
                "3rd Curve",
                PlotPoints::from(state_clone.graph.unwrap()),
            ));
        });
    });
 
    if open == false
    {
        return Some( Vec::new() );
    }

    None

    // let mut plot = Plot2D::new();

    // plot.coordinates = ( 0..y_values.len() )
    // .into_iter()
    // .map(|i|
    // {
    //     let x_value = match x_values[i]
    //     {
    //         PortValue::Float( value ) => value,
    //         PortValue::Integer( value ) => value as f32,
    //         _ => panic!("ERROR, invalid value type"),
    //     };

    //     let y_value = match y_values[i]
    //     {
    //         PortValue::Float( value ) => value,
    //         PortValue::Integer( value ) => value as f32,
    //         _ => panic!("ERROR, invalid value type"),
    //     };

    //     (x_value as f64, y_value as f64).into()
    // }).collect();

    // Picture::from(plot).show_pdf(Engine::PdfLatex);

    // Some( Vec::new() )
}
