use actix_web::{App, HttpServer, Responder, web};
use opentelemetry::{
    trace::{Span, Tracer, TracerProvider as _},
};
use opentelemetry_sdk::trace::TracerProvider;

async fn hello() -> impl Responder {
    println!("Hello world is going to be shown...");
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let provider = TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("my_tracer");

    let mut span = tracer.start("server_laungh");
        
    println!("Start server");

    let server = HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("0.0.0.0:8080")?
        .run();

    span.end();
    
    println!("Server started.");
    server.await?;
    println!("End.....");
    //provider.shutdown().expect("TracerProvider should shutdown successfully")
    Ok(())
}
