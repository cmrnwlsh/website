use actix_files::NamedFile;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use website::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .route("/favicon.ico", web::get().to(|| async {
                NamedFile::open("./target/site/favicon.ico")
            }))
            .service(Files::new("/pkg", "target/site/pkg"))
            .service(Files::new("/assets", "target/site"))
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(middleware::Compress::default())
    })
        .bind(&addr)?
        .run()
        .await
}


