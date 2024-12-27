use actix_web::{App, HttpServer, Responder, web};

async fn hello() -> impl Responder {
    println!("Hello world is going to be shown...");
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start server");
    
    let server = HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("0.0.0.0:8080")?
        .run();
        
    println!("Server started.");
    server.await?;
    println!("End.....");
    Ok(())
}
