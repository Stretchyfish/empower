use crate::node_graph::port::{PortCompatability, PortValue};
use pgfplots::{axis::plot::{Plot2D}, Engine, Picture};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct MathGraphState
{
    
}

impl MathGraphState
{
    pub fn new() -> Self
    {
        Self 
        { 

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

pub fn execute(inputs: Vec<&PortValue>) -> Vec<PortValue> 
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
        return Vec::new();
    }

    let mut plot = Plot2D::new();

    plot.coordinates = ( 0..y_values.len() )
    .into_iter()
    .map(|i|
    {
        let x_value = match x_values[i]
        {
            PortValue::Float( value ) => value,
            PortValue::Integer( value ) => value as f32,
            _ => panic!("ERROR, invalid value type"),
        };

        let y_value = match y_values[i]
        {
            PortValue::Float( value ) => value,
            PortValue::Integer( value ) => value as f32,
            _ => panic!("ERROR, invalid value type"),
        };

        (x_value as f64, y_value as f64).into()
    }).collect();

    Picture::from(plot).show_pdf(Engine::PdfLatex);

    Vec::new()
}
