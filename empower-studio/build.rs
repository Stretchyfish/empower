use std::{env, fs};
use std::path::PathBuf;

fn main()
{
    // let executable_directory = env::current_exe().unwrap().parent().unwrap().to_path_buf();


    let working_directory = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // @TODO, add safety check here
    
    let profile = env::var("PROFILE").unwrap(); 
    let target_dir = working_directory.join("../target").join(profile);

    // println!("cargo:warning=target dir: {}", target_dir.to_string_lossy().to_string());

    let resources_directory = target_dir.join("resources");
    let create_resource_directory_result = fs::create_dir(resources_directory.clone());

    match create_resource_directory_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {
                    println!("cargo:warning=Resources already exists");
                },
                _ => { panic!("cargo:warning=Error when creating resource directory: {}", error.kind().to_string()); },
            }
        },
    }

    // @TODO, have it fetch the once from different platforms
    // @TODO, add exe if the platform is windows
    let runtime_original_location = target_dir.join("empower-application");

    let _ = fs::create_dir(resources_directory.join("empower-application"));
    let runtime_new_location = resources_directory.join("empower-application").join("empower-application");

    let copy_runtime_result = fs::copy(runtime_original_location, runtime_new_location);

    match copy_runtime_result 
    {
        Ok(_) => {},
        Err( error ) =>
        {
            match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {
                    println!("cargo:warning=Runtime already exists");
                },
                _ => { panic!("cargo:warning=Error when copying runtime: {}", error.kind().to_string()); },
            }
        },
    }
    
    // println!("cargo:warning=Created resource folder at : {}", resources_directory.to_string_lossy().to_string());
    
    
    // let application_platform_build_configs = [
    //     ("Linux", "x86_64-unknown-linux-gnu", "application")
    // ];

    // let path = std::env::current_dir().unwrap().to_string_lossy().to_string();

    // println!("Path: {}", path);

    // for ( platform, _target, _executable_name ) in application_platform_build_configs
    // {
    //     let status = Command::new("cargo").args(["build", "--release", "-p", "empower-application"]).status().expect("Failed to build application"); 

    //     if !status.success()
    //     {
    //         panic!("Failed to build runtime for {}", platform);
    //     }

        // let current_directory = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        
    // }
}
