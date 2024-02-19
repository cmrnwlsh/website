use actix_files::NamedFile;
use actix_files::Files;
use actix_web::*;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use website::app::*;
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        ssl_builder
        .set_private_key_file("./cert/privkey.pem", SslFiletype::PEM)
        .unwrap();
    ssl_builder.set_certificate_chain_file("./cert/fullchain.pem").unwrap();
    println!("listening on https://{}", &addr);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info")); 

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let hsts =RedirectHttps::with_hsts(StrictTransportSecurity::recommended());
        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .route(
                "/favicon.ico",
                web::get().to(|| async { NamedFile::open("./target/site/favicon.ico") }),
            )
            .service(Files::new("/pkg", "target/site/pkg"))
            .service(Files::new("/assets", "target/site"))
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(hsts)
    })
    .bind_openssl(&addr, ssl_builder)?
    .run()
    .await
}


