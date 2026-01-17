use std::collections::{HashSet, VecDeque};

use crate::NodeGraphKey;

#[derive(Clone)]
pub struct Task
{
    pub nodes_to_setup: VecDeque<NodeGraphKey>,
    pub nodes_to_update: HashSet<NodeGraphKey>,
}
