
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
//use actix_utils::mpsc;

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
/*#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("web/phantombot/resources/web/index.html")))
}
*/
/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// response body
/*async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(web::Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}
*/
/// handler with path parameters like `/user/{name}/`
/*async fn with_param(
    req: HttpRequest,
    web::Path((name,)): web::Path<(String,)>,
) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", name))
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let _guard = sentry::init(("https://b8679368c7a444a6b5c1acb823fb4f8d@o749414.ingest.sentry.io/5791371", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    // Sentry will capture this
//    panic!("Everything is on fire!");



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

/*    HttpServer::new(|| {
        App::new()
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register favicon
            .service(favicon)
            // register simple route, handle all methods
            .service(welcome)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            // async response body
            .service(
                web::resource("/async-body/{name}").route(web::get().to(response_body)),
            )
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // redirect
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "web/phantombot/resources/web/index.html")
                    .finish()
            })))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
 */
    HttpServer::new(|| {
        App::new()
            // Enable the logger.
            .wrap(middleware::Logger::default())
            // We allow the visitor to see an index of the images at `/images`.
            .service(Files::new("/images", "static/images/").show_files_listing())
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
