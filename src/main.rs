
mod repoversion;
mod configuration_manager;
mod caseless_properties;

use std::path::{PathBuf};
use crate::configuration_manager::ConfigurationManager;
use actix_files as fs;
use actix_session::{CookieSession, Session};
//use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};
use std::{env, io};
use actix_files::Files;


/// for sentry
#[get("/")]
async fn failing(_req: HttpRequest) -> Result<String, Error> {
    Err(io::Error::new(io::ErrorKind::Other, "An error happens here").into())
}

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let _guard = sentry::init(("https://b8679368c7a444a6b5c1acb823fb4f8d@o749414.ingest.sentry.io/5791371", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    // Sentry will capture this
//    panic!("Everything is on fire!");
    std::env::set_var("RUST_BACKTRACE", "1");



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
            // Enable the logger.
            .wrap(middleware::Logger::default())
            // We allow the visitor to see an index of the images at `/images`.
            .service(Files::new("/images", "static/images/").show_files_listing())
            .wrap(sentry_actix::Sentry::new())
            .service(failing)
            // Serve a tree of static files at the web root and specify the index file.
            // Note that the root path should always be defined as the last item. The paths are
            // resolved in the order they are defined. If this would be placed before the `/images`
            // path then the service for the static images would never be reached.
            .service(Files::new("/", "./web/phantombot/resources/web/").index_file("index.html"))
    })
        .bind("127.0.0.1:26000")?
        .run()
        .await




}

fn get_working_dir() -> std::io::Result<PathBuf>
{
    let path = env::current_dir();

    return path;

}
