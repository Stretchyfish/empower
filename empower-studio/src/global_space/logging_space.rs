use std::{collections::VecDeque, time::Instant};

use crate::studio_context::{Log, LogLevel};

const MAX_LOG_AGE: f32 = 3.0;
const LOGGING_VERTICAL_TEXT_GAP: f32 = 25.0;

pub fn show(logs: &VecDeque<Log>, ui: &mut egui::Ui)
{
    let log_layer = egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("log_layer"), 
    );

    let painter = ui.layer_painter(log_layer);

    let mut vertical_offset = 0.0;

    let time_now = Instant::now();
    for log in logs
    {
        if time_now > log.creation_time + std::time::Duration::from_secs_f32(MAX_LOG_AGE) 
        {
            continue;
        }

        let log_color = match log.level
        {
            LogLevel::Info => egui::Color32::YELLOW,
            LogLevel::Warning => egui::Color32::RED,
            LogLevel::News => egui::Color32::GREEN,
        };

        painter.text(
            egui::pos2(10.0, 80.0 + vertical_offset),
            egui::Align2::LEFT_TOP,
            format!("{}", log.text),
            egui::FontId::monospace(14.0),
            log_color,
        );

        vertical_offset += LOGGING_VERTICAL_TEXT_GAP;
    }
}
