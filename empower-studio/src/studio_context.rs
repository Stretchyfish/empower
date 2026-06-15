mod layout;
use std::collections::VecDeque;

use empower_engine::{compiler::{self, InstructionSet, Program, release_compile}, project::Project};
use layout::Layout;

mod request;
use once_cell::sync::Lazy;
use request::Request;

mod windows;
pub use windows::Windows;

mod settings;
pub use settings::Settings;

use crate::user_state::{UserAction, UserState};

static CONFIG_DIRECTORY: Lazy<directories::ProjectDirs> = Lazy::new(|| {
    directories::ProjectDirs::from("com", "empower", "empower-studio").expect("Could not find a config directory")
});

pub struct StudioContext
{
    project: Project,
    
    layout: Layout,
    settings: Settings,

    program: Option<Program>,
    user_state: Option<UserState>,

    windows: Windows,

    requests: VecDeque<Request>,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        StudioContext
        {
            project: Project::new(),
            
            layout: Layout::load(&CONFIG_DIRECTORY.config_dir()),
            settings: Settings::new(),

            program: None,
            user_state: None,

            windows: Windows::new(),

            requests: VecDeque::new(),
        }
    }

    pub fn get_layout_clone(&self) -> Layout
    {
        self.layout.clone()
    }

    pub fn set_layout(&mut self, layout: Layout)
    {
        self.layout = layout;
    }

    pub fn request_default_layout(&mut self)
    {
        self.requests.push_back( Request::DefaultLayout );
    }

    pub fn request_new_viewport(&mut self, new_viewport_name: &'static str)
    {
        self.requests.push_back( Request::AddViewport { name: new_viewport_name });
    }

    fn save_studio(&mut self)
    {
        let created_config_directory = std::fs::create_dir(CONFIG_DIRECTORY.config_dir()); // In case there isn't already a config directory, one is created
        match created_config_directory
        {
            Ok(_) => {},
            Err( error ) => match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("Failing to create config directory: {}", error.kind().to_string());
                }
            },
        }

        self.layout.save(&CONFIG_DIRECTORY.config_dir());
    }

    fn load_studio(&mut self)
    {
        self.layout = Layout::load(&CONFIG_DIRECTORY.config_dir());
    }

    pub fn request_save_studio(&mut self)
    {
        self.requests.push_back( Request::SaveStudio );
    }

    pub fn request_load_studio(&mut self)
    {
        self.requests.push_back( Request::LoadStudio );
    }

    pub fn get_project_mut(&mut self) -> &mut Project
    {
        &mut self.project
    }

    pub fn get_project_mut_and_borrow_user_state(&mut self) -> (&mut Project, &Option<UserState>) // This is a helper function to overcome borrower limitations
    {
        (&mut self.project, &self.user_state)
    }

    pub fn request_user_state_change(&mut self, layer_or_viewport: String, new_action: UserAction)
    {
        self.requests.push_back( Request::UserStateChange { layer_or_viewport, new_action } );
    }

    pub fn request_user_state_clear(&mut self)
    {
        self.requests.push_back( Request::UserStateClear );
    }

    pub fn get_user_state(&self) -> &Option<UserState>
    {
        &self.user_state
    }

    pub fn get_windows_mut(&mut self) -> &mut Windows
    {
        &mut self.windows
    }

    pub fn get_settings(&self) -> &Settings
    {
        &self.settings
    }

    pub fn get_windows_and_settings_mut_and_borrow_user_state(&mut self) -> (&mut Windows, &mut Settings, &Option<UserState>)
    {
        (&mut self.windows, &mut self.settings, &self.user_state)
    }

    pub fn request_compile(&mut self)
    {
        self.requests.push_back( Request::Compile );
    }

    fn compile(&mut self)
    {
        let program = compiler::release_compile(&self.project);

        if let Err(e) = program 
        {
            println!("Failed to compile: {}", e);
        }

        self.program = Some( program.unwrap() );
    }

    pub fn get_program(&self) -> &Option<Program>
    {
        &self.program
    }

    pub fn process_requests(&mut self)
    {
        if self.requests.is_empty()
        {
            return;
        }

        let request_to_process = self.requests.pop_front().unwrap(); // Save to do, due to check above

        match request_to_process
        {
            Request::DefaultLayout => { self.layout = Layout::default_layout() },
            Request::AddViewport { name } => { self.layout.add_viewport( name ); },
            Request::SaveStudio => { self.save_studio(); },
            Request::LoadStudio => { self.load_studio(); },
            Request::UserStateClear => { self.user_state = None; },
            Request::UserStateChange { layer_or_viewport, new_action } => { self.user_state = Some( UserState::from(layer_or_viewport, new_action) )},
            Request::Compile => { self.compile(); },
        }
    }
}
