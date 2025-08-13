use crate::node_graph::NodeGraphKey;

#[derive(Default, Clone)]
pub struct Port
{
    pub key: NodeGraphKey, 

}

impl Port
{
    pub fn new(key: NodeGraphKey) -> Self
    {
        Self 
        {  
            key
        }
    }
}