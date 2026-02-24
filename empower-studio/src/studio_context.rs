use std::collections::VecDeque;

pub mod project;
use project::Project;

mod settings;
use settings::Settings;

pub mod layout;
use layout::Layout;

mod requests;
use requests::Request;

mod cache;
use cache::Cache;

mod windows;
use windows::Windows;

mod user_state;
use user_state::UserState;

pub struct StudioContext
{
    project: Project,
    settings: Settings,

    windows: Windows,
    layout: Layout,

    pub cache: Cache, // @TODO, find a different way
    user_state: UserState,

    requests: VecDeque<Request>,
}

impl StudioContext
{
    pub fn new() -> Self
    {
        Self
        {
            project: Project::new(),
            settings: Settings {},

            windows: Windows::new(),
            layout: Layout::load(),

            cache: Cache::new(),
            user_state: UserState::Idle,

            requests: VecDeque::new(),
        }
    }

    fn save(&self)
    {
        self.layout.save();
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

    pub fn request_load_project(&mut self)
    {
        self.requests.push_back( Request::LoadProject );
    }

    pub fn get_layout_clone(&self) -> Layout
    {
        self.layout.clone()
    }

    pub fn set_layout(&mut self, layout: Layout)
    {
        self.layout = layout;
    }

    pub fn get_project_mut(&mut self) -> &mut Project // @TODO, THIS IS ONLY TEMPORARY!
    {
        &mut self.project
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
            Request::SaveProject => { self.project.save(); },
            Request::LoadProject => { 
                self.project.load(); 
                self.cache.add_previous_project( self.project.location.clone() );
            },
        };
    }
}
