#[derive(Default, Clone)]
pub struct EmpowerResult
{
    text_output: Vec<String>,
    pub compilation_duration: f32, // Time to compile // @TODO, make this private
}

impl EmpowerResult
{
    pub fn new() -> Self
    {
        Self
        {
           text_output: Vec::new(),
           compilation_duration: 0.0, 
        }
    }

    pub fn add_line(&mut self, line: String)
    {
        self.text_output.push(line);
    }

    pub fn add_text(&mut self, text: String)
    {
        if self.text_output.len() == 0
        {
            self.text_output.push(String::new());
        }

        let last_line = self.text_output.last_mut().expect("Terminal viewport tried to add to last line, but no last line existed");
        last_line.push_str(text.as_str());
    }

    pub fn get_lines(&self) -> Vec<String> // @TODO, change this name
    {
        self.text_output.clone()
    }
}