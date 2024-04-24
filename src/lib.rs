use std::env;
use std::fs::File;
use std::io::Write;

mod all_strings;
mod imp_functions;

pub fn run_command_in_backend() -> std::io::Result<()> {
    // Check if the "backend" directory exists
    if !std::path::Path::new("backend").exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "backend directory does not exist",
        ));
    }

    // Change current working directory to "backend"
    env::set_current_dir("backend")?;

    imp_functions::execute_npm_command("init", "-y")?;
    imp_functions::execute_npm_command("install", "express")?;
    imp_functions::execute_npm_command("install", "mongoose")?;
    imp_functions::execute_npm_command("install", "cors")?;
    imp_functions::execute_npm_command("install", "dotenv")?;

    let folders = vec![
        "connection",
        "config",
        "controller",
        "migrations",
        "models",
        "routers",
        "server",
        "utils",
    ];

    // Create each folder
    for folder in &folders {
        imp_functions::create_folder(folder)?;
    }

    //---------------------------------------------------------------------------------------------------

    let file_path = format!("../backend/.gitignore");
    let mut file = File::create(file_path)?;

    file.write_all(crate::all_strings::strings::GITIGNORE_CONTENT.as_bytes())?;

    //--------------------------------------------------------------------------------------------------

    let env_file_path = format!("../backend/.env");
    let mut env_file = File::create(env_file_path)?;

    env_file.write_all(crate::all_strings::strings::ENV_CONTENT.as_bytes())?;

    //--------------------------------------------------------------------------------------------------

    let conn_file_path = format!("../backend/connection/mongoDB.js");
    let mut conn_file = File::create(conn_file_path)?;

    conn_file.write_all(crate::all_strings::strings::MONGODB_CONTENT.as_bytes())?;

    //--------------------------------------------------------------------------------------------------

    let model_file_path = format!("../backend/models/studentModel.js");
    let mut model_file = File::create(model_file_path)?;

    model_file.write_all(crate::all_strings::strings::STUDENT_MODEL_CONTENT.as_bytes())?;

    //--------------------------------------------------------------------------------------------------

    let router_file_path1 = format!("../backend/routers/setStudentRoute.js");
    let router_file_path2 = format!("../backend/routers/getHelloWorldRoute.js");
    let mut route_file1 = File::create(router_file_path1)?;
    let mut route_file2 = File::create(router_file_path2)?;

    route_file1.write_all(crate::all_strings::strings::STUDENT_ROUTE_CONTENT.as_bytes())?;
    route_file2.write_all(crate::all_strings::strings::HELLO_WORLD_ROUTE_CONTENT.as_bytes())?;

    //--------------------------------------------------------------------------------------------------

    let server_file_path = format!("../backend/server/server.js");

    let mut server_file = File::create(server_file_path)?;

    server_file.write_all(crate::all_strings::strings::SERVER_CONTENT.as_bytes())?;

    Ok(())
}
