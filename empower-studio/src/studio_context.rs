mod layout;
use std::{collections::VecDeque, path::PathBuf};

use empower_engine::{assets::AssetId, compiler::{self, CompileResult, CompileSettings}, distribution, executor::{Executor, ExecutorSettings}, project::Project};
use layout::Layout;

mod request;
use once_cell::sync::Lazy;
use request::Request;

mod windows;
pub use windows::Windows;

mod settings;
pub use settings::Settings;

mod cache;
pub use cache::Cache;

mod user_state;
pub use user_state::UserState;

use crate::docking_space::Viewport;

static CONFIG_DIRECTORY: Lazy<directories::ProjectDirs> = Lazy::new(|| {
    directories::ProjectDirs::from("com", "empower", "empower-studio").expect("Could not find a config directory")
});

pub struct StudioContext
{
    project: Project,
    
    layout: Layout,
    settings: Settings,

    executor_settings: ExecutorSettings,
    executor: Option<Executor>,

    compile_result: Option<CompileResult>,

    windows: Windows,
    user_state: UserState,

    requests: VecDeque<Request>,

    cache: Cache,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        StudioContext
        {
            project: Project::new(),
            
            layout: Layout::load(),
            settings: Settings::new(),

            executor_settings: ExecutorSettings::new_debug_mode(),
            executor: None,
            compile_result: None,

            windows: Windows::new(),
            user_state: UserState::None,

            requests: VecDeque::new(),

            cache: Cache::load(),
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

    pub fn request_new_viewport(&mut self, viewport: Viewport )
    {
        self.requests.push_back( Request::AddViewport { viewport: viewport});
    }

    pub fn request_new_viewport_at_first_docking_leaf(&mut self, viewport: Viewport )
    {
        self.requests.push_back( Request::AddViewportAtFirstLeaf { viewport: viewport} );
    }

    pub fn request_add_or_focus_graph_viewport(&mut self, graph_id: AssetId )
    {
        self.requests.push_back( Request::AddOrFocusGraphViewport { graph_id: graph_id });
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

        self.layout.save();
        self.cache.save();
    }

    fn load_studio(&mut self)
    {
        self.layout = Layout::load();
        self.cache = Cache::load();
    }

    pub fn request_save_studio(&mut self)
    {
        self.requests.push_back( Request::SaveStudio );
    }

    pub fn request_load_studio(&mut self)
    {
        self.requests.push_back( Request::LoadStudio );
    }

    pub fn get_project(&self) -> &Project
    {
        &self.project
    }

    pub fn get_project_mut(&mut self) -> &mut Project
    {
        &mut self.project
    }

    pub fn get_windows_mut(&mut self) -> &mut Windows
    {
        &mut self.windows
    }

    pub fn get_settings(&self) -> &Settings
    {
        &self.settings
    }

    pub fn get_windows(&mut self) -> Windows
    {
        self.windows.clone()
    }

    pub fn set_windows(&mut self, windows: Windows)
    {
        self.windows = windows;
    }

    pub fn get_settings_mut_and_project(&mut self) -> (&mut Settings, &Project)
    {
        (&mut self.settings, &self.project)
    }

    pub fn request_compile(&mut self)
    {
        self.requests.push_back( Request::Compile );
    }

    pub fn is_executing(&self) -> bool
    {
        if self.executor.is_some()
        {
            return self.executor.as_ref().unwrap().is_running();
        }

        false
    }

    pub fn request_compile_and_start_execute(&mut self)
    {
        self.requests.push_back( Request::Compile ); // @TODO, need to add checks to make sure it even needs to compile!
        self.requests.push_back( Request::StartExecute );
    }

    pub fn request_stop_execute(&mut self)
    {
        self.requests.push_back( Request::StopExecute );
    }

    fn compile(&mut self)
    {
        let compile_result = compiler::compile(&mut self.project, &CompileSettings::new_with_meta() );

        if let Err(e) = compile_result
        {
            println!("Failed to compile: {}", e);
            self.executor_settings.outputs.as_mut().unwrap().push( e.to_string() ); // @TODO, find a proper way to do logging
            return;
        }

        self.compile_result = Some( compile_result.unwrap() );
    }

    pub fn get_compile_result(&self) -> &Option<CompileResult>
    {
        &self.compile_result
    }

    fn start_execution(&mut self)
    {
        if self.compile_result.is_none()
        {
            self.executor_settings.outputs.as_mut().unwrap().push( "Tried to start execution without a compiled program".to_string() ); // @TODO, find a proper solution for this
            // panic!("Tried to start execution without a compiled program");
            return;
        }

        self.executor_settings.clear_cache();

        self.executor = Some( Executor::new(self.compile_result.as_ref().unwrap().program.clone(), self.executor_settings.clone()) ); // @TODO, decide if this is the desired behavior, or if it should ".take()" the program.
    }

    fn stop_execution(&mut self)
    {
        self.executor = None;
    }

    pub fn get_executor(&self) -> &Option<Executor>
    {
        &self.executor
    }

    pub fn get_executor_mut(&mut self) -> &mut Option<Executor>
    {
        &mut self.executor
    }

    pub fn request_save_project(&mut self)
    {
        self.requests.push_back( Request::SaveProject );
    }

    pub fn request_save_project_as(&mut self)
    {
        self.requests.push_back( Request::SaveProjectAs );
    }

    pub fn request_load_project(&mut self)
    {
        self.requests.push_back( Request::LoadProject );
    }

    pub fn request_export_project(&mut self, config: distribution::ExportConfig)
    {
        self.requests.push_back( Request::ExportProject { config } );
    }

    pub fn get_cache(&self) -> &Cache
    {
        &self.cache
    }

    pub fn request_load_specific_project(&mut self, project_path: PathBuf)
    {
        self.requests.push_back( Request::LoadSpecificProject { project_path });
    }

    pub fn set_user_state(&mut self, new_user_state: UserState) // @TODO, decide if this should be done with a request
    {
        self.user_state = new_user_state;
    }

    pub fn get_user_state(&self) -> &UserState
    {
        &self.user_state
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
            Request::AddViewport { viewport } => { self.layout.add_viewport( viewport ); },
            Request::AddOrFocusGraphViewport { graph_id } => { self.layout.add_or_focus_graph_viewport(graph_id); },
            Request::AddViewportAtFirstLeaf { viewport } => { self.layout.add_viewport_at_first_leaf( viewport ); },
            Request::SaveStudio => { self.save_studio(); },
            Request::SaveProject =>
            {
                let _ = self.project.save();
                self.cache.add_previous_project(self.project.location.clone());
            },
            Request::SaveProjectAs => { self.windows.project_name_panel.activate_show(self.project.name.clone()); }, // This window will have the user set the projects name before calling regular save again
            Request::LoadProject => {  self.project.load(); },
            Request::LoadSpecificProject { project_path } => { self.project.load_specific_path(project_path); },
            Request::LoadStudio => { self.load_studio(); },
            Request::Compile => { self.compile(); },
            Request::StartExecute => { self.start_execution(); },
            Request::StopExecute => { self.stop_execution(); },
            Request::ExportProject { config } => {

                let _ = self.project.save();
                self.compile(); // @TODO, double check, this behavior might appear twice

                if self.compile_result.is_none()
                {
                    panic!("Cannot export project, as it is not build yet");
                }
                
                let _ = distribution::export(&self.compile_result.as_ref().unwrap().program, &config);
            },
        }
    }
}
