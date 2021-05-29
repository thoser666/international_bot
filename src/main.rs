//mod configuration_manager;
//mod repoversion;

mod repoversion;
mod configuration_manager;
mod caseless_properties;

use std::env;
use std::path::{PathBuf};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use crate::configuration_manager::ConfigurationManager;


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let _guard = sentry::init(("https://b8679368c7a444a6b5c1acb823fb4f8d@o749414.ingest.sentry.io/5791371", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    // Sentry will capture this
    panic!("Everything is on fire!");



    let path = get_working_dir();
    println!("The working directory is: {:?}" , path);

    // read command-line parameter
    let _args: Vec<String> = env::args().collect();

    // how many args?
    let args: Vec<String> = env::args().collect();




    // Constants
    let version = "--version";
    let v = "--v";

    // if we have commandline args
    if args.len() > 1
    {
        let arg = &_args[1];

        if !_args.is_empty()
        {
            match arg.to_string()
            {
                version =>
                    {
                        // TODO write Internationalbotversion and exit Program
                        println!("--Version");
                    }
                _v =>
                    {
                        // TODO write Internationalbotversion and exit Program
                        println!("--v");
                    }
            }
        }
    }

    let config =  ConfigurationManager::new();
    let start_properties = config.get_configuration();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind(("127.0.0.1", 26000))?
        .run()
        .await

//    println!("{:?}", _args);

}

fn get_working_dir() -> std::io::Result<PathBuf>
{
    let path = env::current_dir();

    return path;

}
