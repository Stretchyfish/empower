use std::collections::VecDeque;

#[derive(Default, Clone)]
pub struct TextBuffer
{
    pub max_lines: usize,
    pub lines: VecDeque<String>,
    pub line_counter: usize,
}

impl TextBuffer
{
    pub fn new() -> Self
    {
        TextBuffer { max_lines: 1000, lines: VecDeque::new(), line_counter: 0 }
    }

    pub fn add_text(&mut self, text: &String)
    {
        let cloned_text = text.clone();

        if self.lines.is_empty()
        {
            self.lines.push_back(cloned_text);
            return;
        }

        let newest_line = self.lines.get_mut(self.lines.len()).unwrap();
        *newest_line += cloned_text.as_str();
    }

    pub fn add_line(&mut self, line: &String)
    {
        if (self.lines.len() > self.max_lines)
        {
            self.lines.pop_front();
        }
        
        self.lines.push_back(line.clone());
        self.line_counter += 1;
    }

    pub fn clear(&mut self)
    {
        self.line_counter = 0;
        self.lines.clear();
    }
}
