use actix_web::{web, App, HttpServer, Responder};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

async fn index() -> impl Responder {
    info!("Hello Tracing World!");
    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = Registry::default()
            .with(EnvFilter::new("debug,tracing_actix_web2=trace"))
            .with(fmt::layer().with_target(false));
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("listening at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(tracing_actix_web2::Tracer)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
