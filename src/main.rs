mod configuration_manager;

use std::env;
use std::path::{PathBuf};

fn main()
{
    let path = get_working_dir();
    println!("The working directory is: {:?}" , path);

    // read command-line parameter
    let _args: Vec<String> = env::args().collect();
    let arg = &_args[1];


    // Constants
    let version = "--version";
    let v = "--v";


    if !_args.is_empty()
    {
        match arg.to_string()
        {
            version =>
                {
                    // TODO write Phantombotversion and exit Program
                    println!("--Version");
                }
            v =>
                {
                    // TODO write Phantombotversion and exit Program
                    println!("--v");
                }
        }

    }
    println!("{:?}", _args);

}

fn get_working_dir() -> std::io::Result<PathBuf>
{
    let path = env::current_dir();

    return path;

}
