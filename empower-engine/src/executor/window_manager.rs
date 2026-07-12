use std::collections::HashSet;

pub struct WindowManager
{
    pub window_names: HashSet<String>,
}

impl WindowManager
{
    pub fn new() -> Self
    {
        Self
        {
            window_names: HashSet::new(),
        }
    }

    pub fn remove_window(&mut self, window_name: &String) -> bool
    {
        self.window_names.remove(window_name)
    }

    pub fn add_new_window(&mut self, window_name: String) -> String
    {
        let new_window_name = self.adjust_window_name(&window_name);
        self.window_names.insert(new_window_name.clone());

        new_window_name
    }

    fn adjust_window_name(&self, name: &String) -> String
    {
        let number_of_windows_containing_the_name = self.window_names.iter().filter(|window_name| window_name.contains( name ) ).count();

        if number_of_windows_containing_the_name == 0
        {
            return String::from( name );
        }

        format!("{} ({})", name, number_of_windows_containing_the_name)
    }
}
