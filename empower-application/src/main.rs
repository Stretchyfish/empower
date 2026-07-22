use std::env;
use empower_engine::{compiler::Program, runtime};

fn main()
{
    let current_directory = env::current_exe().unwrap().parent().unwrap().to_path_buf();

    let program_path = current_directory.join("program.json");

    let program_string = std::fs::read_to_string(&program_path);

    println!("Program : {:?}", program_string);

    match program_string
    {
        Ok(_) => {},
        Err( error ) => 
        {
            panic!("Error when loading graph: {}, at path: {}", error.kind().to_string(), program_path.to_string_lossy().to_string());
        },
    }

    let program = Program::from_json( &program_string.unwrap() ).unwrap();
    let _ = runtime::run_gui(program);
}
