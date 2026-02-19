use std::collections::{HashMap, HashSet};

use crate::{actions::Action, project::Project, settings::Settings, user_inputs::UserInputs, user_state::UserState};

mod windows;
use windows::Windows;

mod global_space;

mod menu_bar;

mod tab_viewer;
use tab_viewer::TabViewer;

mod viewport;
use viewport::{Viewport, VIEWPORT_REGISTRY};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Layout
{
    windows: Windows,
    viewports: HashMap<String, Box<dyn Viewport>>,
    docking_state: egui_dock::DockState<String>,
}

impl Layout
{
    pub fn new() -> Self
    {
        Self
        {
            windows: Windows::new(),
            viewports: HashMap::new(),
            docking_state: egui_dock::DockState::new(Vec::new()), 
        }
    }

    pub fn load() -> Self
    {
        let config_directory = directories::ProjectDirs::from("com", "empower", "empower-studio").expect("Could not find a config directory");
        let editor_state_path = config_directory.config_dir().join("studio_editor.json");

        let read_editor_state_result = std::fs::read_to_string(editor_state_path);

        match read_editor_state_result
        {
            Ok( editor_state_json ) =>
            {
                serde_json::from_str(&editor_state_json).unwrap()
            },
            Err(_) =>
            {
                Self::default_layout()
            },
        }
    }

    pub fn save(&self)
    {
        // @TODO, find a better way to match the save and load paths
        let config_directory = directories::ProjectDirs::from("com", "empower", "empower-studio").expect("Could not find a config directory");

        std::fs::create_dir(config_directory.config_dir());

        let file_path = config_directory.config_dir().join("studio_editor.json");
        let editor_json = serde_json::to_string_pretty(self).unwrap();

        let save_editor_state_result = std::fs::write(file_path, editor_json);

        match save_editor_state_result
        {
            Ok(_) => {},
            Err( error ) =>
            {
                println!("Error when saving editor state : {}",error.kind().to_string());
            },
        }
    }

    pub fn default_layout() -> Self
    {
        let mut new_default_layout = Self::new();
        
        let graph_viewport_name = new_default_layout.add_viewport("graph viewport");
        let content_browser_viewport_name = new_default_layout.add_viewport_without_docking_state("content browser viewport");
        let terminal_viewport_name = new_default_layout.add_viewport_without_docking_state("terminal viewport");

        // This is all to place the initial docking configuration
        let graph_viewport_index = new_default_layout.docking_state.find_tab(&graph_viewport_name).expect("Unable to find initial graph viewport tab");

        new_default_layout.docking_state.main_surface_mut().split_below(graph_viewport_index.1, 0.7, vec![content_browser_viewport_name, terminal_viewport_name]);

        new_default_layout
    }

    pub fn show_global_space(&mut self, ctx: &egui::Context, settings: &Settings, user_state: &UserState, user_inputs: &UserInputs, action_queue: &mut Vec<Action>)
    {
        global_space::show_global_space(ctx, user_state, user_inputs, action_queue);
    }

    pub fn show_menu_bar(&mut self, ctx: &egui::Context, action_queue: &mut Vec<Action>)
    {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| 
        {
            menu_bar::show_menu_bar(ui, action_queue);
        });
    }

    pub fn show_docking_space(&mut self, ctx: &egui::Context, mut project: &mut Project, settings: &Settings, user_state: &UserState, user_inputs: &UserInputs,action_queue: &mut Vec<Action>)
    {
        egui::CentralPanel::default()
        .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.0))
        .show(ctx, |ui| 
        {
            egui_dock::DockArea::new(&mut self.docking_state)
                .style({
                    let mut style = egui_dock::Style::from_egui(ctx.style().as_ref());
                    style.tab_bar.fill_tab_bar = true;
                    style
                })
                .show_close_buttons(true) // @TODO, add behavior here?
                .show_add_popup(true)
                .show_leaf_close_all_buttons(false)
                .show_leaf_collapse_buttons(false)
                .show_inside(
                    ui,
                    &mut TabViewer {
                        project: &mut project, // @TODO, find a way to get rid of this borrow
                        settings: settings,
                        viewports: &mut self.viewports,
                        user_state: user_state,
                        user_inputs: user_inputs,
                        action_queue,
                    },
                );
        });
    }

    fn adjust_viewport_name(&self, name: &'static str) -> String
    {
        let number_of_viewports_containing_the_name = self.viewports.iter().filter(|(viewport_name, _)| viewport_name.contains(name) ).count();

        if number_of_viewports_containing_the_name == 0
        {
            return String::from( name );
        }

        format!("{} ({})", name, number_of_viewports_containing_the_name)
    }

    pub fn add_viewport(&mut self, new_viewport_name: &'static str) -> String
    {
        let new_viewport_constructor = match VIEWPORT_REGISTRY.get(new_viewport_name)
        {
           Some( constructor ) => constructor,
           None => panic!("Tried to create a non-existing viewport name : {}", new_viewport_name), 
        };

        let adjusted_viewport_name = self.adjust_viewport_name( new_viewport_name );

        let new_viewport = new_viewport_constructor();

        self.viewports.insert(adjusted_viewport_name.clone(), new_viewport);
        self.docking_state.push_to_focused_leaf(adjusted_viewport_name.clone());

        adjusted_viewport_name
    }

    // @TODO, this function only has a very specific usecase, consider if it should be a bool in the add_viewport function instead
    pub fn add_viewport_without_docking_state(&mut self, new_viewport_name: &'static str) -> String
    {
        
        let new_viewport_constructor = match VIEWPORT_REGISTRY.get(new_viewport_name)
        {
           Some( constructor ) => constructor,
           None => panic!("Tried to create a non-existing viewport name : {}", new_viewport_name), 
        };

        let adjusted_viewport_name = self.adjust_viewport_name( new_viewport_name );

        let new_viewport = new_viewport_constructor();

        self.viewports.insert(adjusted_viewport_name.clone(), new_viewport);

        adjusted_viewport_name
    }
}
