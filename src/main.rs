mod configuration_manager;

use std::env;
use std::path::{PathBuf};

fn main()
{
    let path = get_working_dir();
    println!("The working directory is: {:?}" , path);
}

fn get_working_dir() -> std::io::Result<PathBuf>
{
    let path = env::current_dir();

    return path;

}
