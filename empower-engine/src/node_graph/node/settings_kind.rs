pub enum SettingsKind<'a>
{
    Text { label: &'a str, value: &'a mut String },
    Slider { label: &'a str, value: &'a mut f32, min: f32, max: f32 },
    Choice { label: &'a str, value: &'a mut String, options: Vec<&'a str> },
}
