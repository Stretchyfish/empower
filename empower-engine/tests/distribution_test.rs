use empower_engine::{distribution::TEMP_PROJECT_LOCATION, project::{Project, ProjectState}};


#[test]
fn test_saving_project() -> std::io::Result<()> 
{
    let project_after_saving_location = TEMP_PROJECT_LOCATION.join("empower_save_test1"); // They need unique names, otherwise the file operation will conflict in testing due to being parallel
    if project_after_saving_location.exists()
    {
        std::fs::remove_dir_all( &project_after_saving_location )?; // Just in case there has already been done a test
    }
    
    let mut project = empower_engine::project::Project::with_name("empower_save_test1");

    let save_result = project.save( Some( TEMP_PROJECT_LOCATION.to_path_buf() ) );

    assert_eq!(save_result.unwrap(), project_after_saving_location );

    Ok(())
}

#[test]
fn test_loading_project() -> std::io::Result<()>
{
    let project_after_saving_location = TEMP_PROJECT_LOCATION.join("empower_load_test1"); // They need unique names, otherwise the file operation will conflict in testing due to being parallel
    if project_after_saving_location.exists()
    {
        std::fs::remove_dir_all( &project_after_saving_location )?; // Just in case there has already been done a test
    }

    let save_location = 
    {
        let mut project = empower_engine::project::Project::with_name("empower_load_test1");
        project.save( Some( TEMP_PROJECT_LOCATION.to_path_buf() ) ).unwrap()
    };

    let project = Project::load( &save_location ).unwrap();

    assert_eq!(project.name, "empower_load_test1".to_string());
    assert_eq!(project.state, ProjectState::Saved( project_after_saving_location ) );
    
    Ok(())
}
