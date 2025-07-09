pub struct TerminalViewport
{
   pub title: String, 
   pub lines: Vec<String>,
}

impl TerminalViewport
{
    pub fn new(title: String) -> Self
    {
        Self 
        { 
            title,
            lines: Vec::new(),
        } 
    }

    pub fn add_line(&mut self, line: String)
    {
        self.lines.push(line);
    }

    pub fn add_text(&mut self, text: String)
    {
        if self.lines.len() == 0
        {
            self.lines.push(String::new());
        }

        let last_line = self.lines.last_mut().expect("Terminal viewport tried to add to last line, but no last line existed");
        last_line.push_str(text.as_str());
    }

}

pub fn show(ui: &mut egui::Ui, terminal_viewport: &mut TerminalViewport)
{
    ui.horizontal_top(|ui|
    {
        if ui.button("Clear").clicked()
        {
            terminal_viewport.lines.clear();
        }

        if ui.button("Add text").clicked()
        {
            terminal_viewport.add_text(String::from("test something"));
        }

        if ui.button("Add line").clicked()
        {
            terminal_viewport.add_line(String::from("test something"));
        }
    });

    egui::ScrollArea::vertical()
    .id_salt(egui::Id::from(terminal_viewport.title.clone()))
    .auto_shrink(false)
    .stick_to_bottom(true)
    .show(ui, |ui|
    {
        for line in terminal_viewport.lines.iter()
        {
            let line_text = format!("{}", line);
            ui.label(line_text); 
        }
    });
}