use crate::studio_context::StudioContext;

pub fn show(ui: &mut egui::Ui, _: &mut StudioContext)
{
    ui.checkbox(&mut true, "debug mode");

    if true
    {
        ui.add( egui::Slider::new( &mut 0.0, 0.0..=10.0 ).text("delay") );
    }
}

