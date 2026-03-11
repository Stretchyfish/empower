use std::{collections::VecDeque, path::PathBuf};

pub mod project;
use empower_engine::{NodeGraphKey, runtime::EmpowerExecutor, utility::{log_buffer::LogBuffer}};
use project::Project;

mod settings;
pub use settings::Settings;

pub mod layout;
use layout::Layout;

mod requests;
use requests::Request;

mod cache;
use cache::Cache;

mod windows;
use windows::Windows;

use crate::studio_context::project::ProjectState;

pub struct StudioContext
{
    project: Project,
    settings: Settings,

    windows: Windows,
    layout: Layout,

    cache: Cache, 

    executor: EmpowerExecutor,

    requests: VecDeque<Request>,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            project: Project::new(),
            settings: Settings::load(),

            windows: Windows::new(),
            layout: Layout::load(),

            cache: Cache::load(),

            executor: EmpowerExecutor::new(true, false),

            requests: VecDeque::new(),
        }
    }

    fn save(&self) // @TODO, might need a better name, or needs atleast to seperate temporary saving from project
    {
        self.layout.save();
        self.cache.save();
        self.settings.save();
    }

    pub fn request_save(&mut self) // @TODO, this should probably be a request instead
    {
        self.requests.push_back( Request::Save );
    }

    pub fn request_save_layout(&mut self)
    {
        self.requests.push_back( Request::SaveLayout );
    }

    pub fn request_load_layout(&mut self)
    {
        self.requests.push_back( Request::LoadLayout );
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

    pub fn request_load_specific_project(&mut self, project_path: PathBuf)
    {
        self.requests.push_back( Request::LoadSpecificProject { project_path });
    }

    pub fn get_layout_clone(&self) -> Layout
    {
        self.layout.clone()
    }

    pub fn set_layout(&mut self, layout: Layout)
    {
        self.layout = layout;
    }

    pub fn get_cache(&self) -> &Cache
    {
        &self.cache
    }

    pub fn get_windows_clone(&self) -> Windows // @TODO, find a better way to do this
    {
        self.windows.clone()
    }

    pub fn set_windows(&mut self, windows: Windows)
    {
        self.windows = windows;
    }

    pub fn project_is_temporary(&self) -> bool
    {
        match self.project.state
        {
            ProjectState::Temporary => true,
            ProjectState::Saved => false,
        }
    }

    pub fn get_project_mut(&mut self) -> &mut Project // @TODO, THIS IS ONLY TEMPORARY!
    {
        &mut self.project
    }

    pub fn get_settings_mut(&mut self) -> &mut Settings
    {
        &mut self.settings
    }

    pub fn get_settings_clone(&self) -> Settings
    {
        self.settings.clone()
    }

    pub fn exeucutor_is_running(&self) -> bool
    {
        self.executor.is_running()
    }

    pub fn request_executor_start(&mut self)
    {
        self.requests.push_back( Request::StartExecution );
    }

    pub fn request_executor_start_from(&mut self, node_key: NodeGraphKey)
    {
        self.requests.push_back( Request::StartExecutionFrom { node_key });
    }

    pub fn request_executor_stop(&mut self)
    {
        self.requests.push_back( Request::StopExeuction );
    }

    pub fn execute_node_graph(&mut self, ctx: &egui::Context)
    {
        self.executor.execute_node_graph(&mut self.project.graph_editor.node_graph, Some( ctx ));
    }

    pub fn get_execution_log(&self) -> &LogBuffer
    {
        &self.executor.logs
    }

    pub fn set_settings(&mut self, settings: Settings)
    {
        self.settings = settings;
    }

    pub fn request_new_layout(&mut self)
    {
        self.requests.push_back( Request::NewLayout );
    }

    pub fn request_default_layout(&mut self)
    {
        self.requests.push_back( Request::DefaultLayout );
    }

    pub fn request_new_viewport(&mut self, new_viewport_name: &'static str)
    {
        self.requests.push_back( Request::AddViewport { name: new_viewport_name });
    }

    pub fn has_request(&self) -> bool
    {
        !self.requests.is_empty()
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
            Request::NewLayout => { self.layout = Layout::new(); },
            Request::DefaultLayout => { self.layout = Layout::default_layout(); },
            Request::AddViewport { name } => { self.layout.add_viewport( name ); },
            Request::Save => { self.save(); },
            Request::SaveLayout => { self.layout.save(); },
            Request::LoadLayout => { self.layout = Layout::load(); },
            Request::SaveProject => { 
                self.project.save(); 
                self.cache.add_previous_project(self.project.location.clone());
            },
            Request::SaveProjectAs =>
            {
                self.windows.project_name_window.activate_show(self.project.name.clone()); // This window will have the user set the projects name before calling regular save again
            },
            Request::LoadProject => { 

                let project_path = rfd::FileDialog::new()
                                                    .set_title("Find project to load")
                                                    .set_can_create_directories(true)
                                                    .pick_folder();

                if project_path.is_none()
                {
                    println!("Failed to get folder path!"); // @TODO, in the future, handle this error properly!
                    return;
                }

                self.project.load(project_path.unwrap()); 
                self.cache.add_previous_project( self.project.location.clone() );
            },
            Request::LoadSpecificProject { project_path } => { self.project.load(project_path); },
            Request::StartExecution => { self.executor.start_node_graph( &mut self.project.graph_editor.node_graph ); },
            Request::StartExecutionFrom { node_key } => { self.executor.start_node_graph_from_entry(&mut self.project.graph_editor.node_graph, &node_key); },
            Request::StopExeuction => { self.executor.stop_node_graph(); },
        };
    }
}
