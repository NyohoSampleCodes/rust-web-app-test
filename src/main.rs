use actix_web::{App, HttpServer, Responder, web};

async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start server");
    
    let server = HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run();
        
    println!("Server started.");
    server.await?;
    println!("End.....");
    Ok(())
}
