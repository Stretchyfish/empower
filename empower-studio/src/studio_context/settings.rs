mod developer_settings;
pub use developer_settings::DeveloperSettings;

mod project_settings;
pub use project_settings::ProjectSettings;

mod export_settings;
pub use export_settings::ExportSettings;

const CONFIG_DIRECTORY_PROJECT_NAME: &'static str = "empower-studio"; // @TODO, this created in multiple files, should be more global
const CONFIG_SETTINGS_FILE_NAME: &'static str = "settings.json";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings
{
    pub project_settings: ProjectSettings,
    pub developer_settings: DeveloperSettings,
    pub export_settings: ExportSettings,
}

impl Settings
{
    pub fn new() -> Self
    {
        Self
        {
            project_settings: ProjectSettings::new(),
            developer_settings: DeveloperSettings::default(),
            export_settings: ExportSettings::new(),
        }
    }

    pub fn save(&self)
    {
        // @TODO, find a better way to match the save and load paths
        let config_directory = directories::ProjectDirs::from("com", "empower", CONFIG_DIRECTORY_PROJECT_NAME).expect("Could not find a config directory");

        // @TODO, this work is done multiple times, maybe define it better
        let created_config_directory = std::fs::create_dir(config_directory.config_dir());
        match created_config_directory
        {
            Ok(_) => {},
            Err( error ) => match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("Failing to save settings because : {}", error.kind().to_string());
                }
            },
        }

        let file_path = config_directory.config_dir().join(CONFIG_SETTINGS_FILE_NAME);
        let settings_json = serde_json::to_string_pretty(self).unwrap();

        let save_settings_result = std::fs::write(file_path, settings_json);

        match save_settings_result
        {
            Ok(_) => {},
            Err( error ) =>
            {
                println!("Error when saving cache : {}",error.kind().to_string());
            },
        }
    }

    pub fn load() -> Self
    {
        let config_directory = directories::ProjectDirs::from("com", "empower", CONFIG_DIRECTORY_PROJECT_NAME).expect("Could not find a config directory");
        let settings_path = config_directory.config_dir().join(CONFIG_SETTINGS_FILE_NAME);

        let read_settings_result = std::fs::read_to_string(settings_path);

        match read_settings_result // If it fails to load, use new
        {
            Ok( settings_json ) =>
            {
                let settings_json_result = serde_json::from_str(&settings_json);

                match settings_json_result 
                {
                    Ok( settings ) => { return settings; },
                    Err( error ) => { println!("Error when reading stored cache, using default instead, error: {}", error.to_string()); },
                };
            },
            Err( error ) => { println!("Error when reading stored cache, using default instead, error: {}", error.to_string()); },
        };

        Self::new()
    }
}
