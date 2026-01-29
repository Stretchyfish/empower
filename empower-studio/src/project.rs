
pub struct Project
{
    name: String,
}

impl Project
{
    pub fn new() -> Self
    {
        Self
        {
            name: String::from("Untitled*"),
        }
    }

    pub fn save(&mut self, name: String)
    {
        self.name = name;
    }

    pub fn load()
    {
        
    }
}
