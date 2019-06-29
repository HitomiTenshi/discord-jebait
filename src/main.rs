use actix_files as fs;
use actix_web::{http::header, web, App, HttpRequest, HttpServer, Result};

fn jebaited(req: HttpRequest) -> Result<fs::NamedFile> {
    if let Some(user_agent) = req.headers().get(header::USER_AGENT) {
        if let Ok(user_agent_str) = user_agent.to_str() {
            if user_agent_str.contains("Discordbot") {
                return Ok(fs::NamedFile::open("praying-jesus.jpg")?);
            }
        }
    }

    Ok(fs::NamedFile::open("praying-jesus.webm")?)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/praying-jesus.jpg").to(jebaited)))
        .bind("127.0.0.1:54299")?
        .run()
}