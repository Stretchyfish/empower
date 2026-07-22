use std::{env, fs};
use std::path::PathBuf;

fn main()
{
    println!("cargo:rerun-if-changed=.cargo_build_timestamp"); // A hack to make it always use the build

    let working_directory = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let profile = match env::var("PROFILE") 
    {
        Ok( profile ) => profile,
        Err( error ) => panic!("Failed ot get profile during build: {}", error),
    };

    // println!("cargo:Warning=Profile: {}", profile);

    let target = match std::env::var("TARGET")
    {
        Ok( target ) => target,
        Err( error ) => panic!("Failed ot get target during build: {}", error),
    };

    // println!("cargo:Warning=Target: {}", target);

    let target_dir = working_directory.join("../target").join(profile);

    let resources_directory = target_dir.join("resources");
    let create_resource_directory_result = fs::create_dir(resources_directory.clone());

    match create_resource_directory_result
    {
        Ok(_) =>
        {
            // println!("cargo:warning=Created resource folder at : {}", resources_directory.to_string_lossy().to_string());
        },
        Err( error ) =>
        {
            match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {},
                _ => { panic!("cargo:warning=Error when creating resource directory: {}", error.kind().to_string()); },
            }
        },
    }

    let runtime_directory = resources_directory.join("runtime");
    let create_runtime_directory_result = fs::create_dir(runtime_directory.clone());

    match create_runtime_directory_result
    {
        Ok(_) =>
        {
            // println!("cargo:warning=Created runtime folder at : {}", runtime_directory.to_string_lossy().to_string());
        },
        Err( error ) =>
        {
            match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {},
                _ => { panic!("cargo:warning=Error when creating runtime directory: {}", error.kind().to_string()); },
            }
        },
    }

    // @TODO, have it fetch the once from different platforms
    // @TODO, add exe if the platform is windows
    let runtime_original_location = if cfg!(target_os = "windows")
    {
        target_dir.join("empower-application.exe")
    }
    else
    {
        target_dir.join("empower-application")
    };

    if !runtime_original_location.exists()
    {
        println!("cargo:warning=No runtime could be found at {}, make sure empower-application is build for exporting.", runtime_original_location.to_string_lossy().to_string());
        return;
    }

    let new_runtime_directory_name = format!("empower-application-{}", target);
    let _ = fs::create_dir(runtime_directory.join(new_runtime_directory_name.clone()));

    let runtime_new_location = if cfg!(target_os = "windows")
    {
        runtime_directory.join(new_runtime_directory_name).join("empower-application.exe")
    }
    else
    {
        runtime_directory.join(new_runtime_directory_name).join("empower-application")
    };

    let copy_runtime_result = fs::copy(runtime_original_location, runtime_new_location);

    match copy_runtime_result 
    {
        Ok(_) =>
        {
            // println!("cargo:warning=Moved runtime for platform: {}, into: {}", target, runtime_directory.to_string_lossy().to_string());
        },
        Err( error ) =>
        {
            match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {
                    println!("cargo:warning=Runtime already exists");
                },
                std::io::ErrorKind::NotFound => {
                    println!("cargo:warning=Runtime not found");
                },
                _ => { panic!("cargo:warning=Error when copying runtime: {}", error.kind().to_string()); },
            }
        },
    }
}
