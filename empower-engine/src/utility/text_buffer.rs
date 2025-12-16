#[derive(Default, Clone)]
pub struct TextBuffer
{
    pub lines: Vec<String>
}

impl TextBuffer
{
    pub fn new() -> Self
    {
        TextBuffer { lines: Vec::new() }
    }

    pub fn add_text(&mut self, text: &String)
    {
        let cloned_text = text.clone();

        if self.lines.is_empty()
        {
            self.lines.push(cloned_text);
            return;
        }

        let newest_line= self.lines.last_mut().unwrap();
        *newest_line += cloned_text.as_str();
    }

    pub fn add_line(&mut self, line: &String)
    {
        self.lines.push(line.clone());
    }
}