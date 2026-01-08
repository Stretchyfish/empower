use std::collections::HashMap;

use crate::NodeGraphKey;

#[derive(Clone)]
pub struct WindowManager // @TODO, pub might not be needed
{
    pub main_window_key: Option<NodeGraphKey>, // @TODO, make these private
    pub windows: HashMap<NodeGraphKey, String>,
}

impl WindowManager
{
    pub fn new() -> Self
    {
        Self
        {
            main_window_key: None,
            windows: HashMap::new(),
        }
    }

    pub fn create_window(&mut self, node_key: &NodeGraphKey, node_name: &str)
    {
        if self.windows.contains_key(node_key)
        {
            return;
        }

        if self.main_window_key == None
        {
            self.main_window_key = Some ( *node_key );
        }

        let new_name = self.adjust_window_name(node_name);
        self.windows.insert(*node_key, new_name);
    }

    pub fn get_windows(&self) -> HashMap<NodeGraphKey, String>
    {
        self.windows.clone()
    }

    pub fn remove_window(&mut self, node_key: &NodeGraphKey)
    {
        if *node_key == self.main_window_key.unwrap_or(0) // This should never possibly fail, but set to 0 for safety
        {
            self.main_window_key = None;
        }
        
        self.windows.remove(node_key);
    }

    pub fn clear_windows(&mut self)
    {
        self.main_window_key = None;
        self.windows.clear();
    }

    fn adjust_window_name(&self, node_name: &str) -> String
    {
        let mut number_of_windows_with_same_name = 0;
        for text in self.windows.values()
        {
            if text.contains(node_name)
            {
                number_of_windows_with_same_name += 1;
            }
        }

        if number_of_windows_with_same_name == 0
        {
            return String::from( node_name );
        }

        format!("{} ({})", node_name, number_of_windows_with_same_name)
    } 
}
