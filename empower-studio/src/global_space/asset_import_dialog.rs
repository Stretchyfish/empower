use std::{path::PathBuf, sync::Arc, thread::{self, JoinHandle}};

use egui::mutex::Mutex;
use empower_engine::distribution;
use rfd::FileDialog;

use crate::studio_context::{Log, StudioContext};

#[derive(Clone)]
pub struct AssetImportDialog
{
    task: Arc<Mutex<Option<JoinHandle<Option<PathBuf>>>>> // @TODO, this gets so complicated because of the clone, think in the future of a way to improve
}

impl AssetImportDialog
{
    pub fn new() -> Self
    {
        Self
        {
            task: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start_dialog(&mut self)
    {
        let mut task = self.task.lock(); // @TODO, this seems potentially very unsafe, investigate better approaches
        
        if task.is_some() // Don't want two dialogs spawned at once
        {
            return;
        }

        *task = Some( thread::spawn(move || {
            FileDialog::new()
            .set_title("import asset")
            .pick_file()
        }));
    }

    pub fn show(&mut self, studio_context: &mut StudioContext)
    {
        let mut task = self.task.lock();

        if task.is_none()
        {
            return;
        }

        let task_finished = task.as_ref().unwrap().is_finished();

        if !task_finished
        {
            return;
        }

        match task.take().unwrap().join()
        {
            Ok( result ) =>
            {
                match result
                {
                    Some( path ) =>
                    {
                        let import_asset_result = distribution::import_asset(studio_context.get_project_mut(), &path);

                        match import_asset_result
                        {
                            Ok(_) => {},
                            Err( error ) =>
                            {
                                studio_context.add_log( Log::info( error.as_str() ) );
                            },
                        }
                    },
                    None => {},
                }
            },
            Err(_) =>
            {
                studio_context.add_log( Log::info( "cannot read path" ) );
            },
        }
    }
}
