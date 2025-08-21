use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;

use rocket::config::LogLevel;
use rocket::fs::NamedFile;
use rocket::response::content;

#[macro_use]
extern crate rocket;

#[launch]
fn video_server() -> _ {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let mut config = rocket::Config::release_default();

    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    config.port = port.parse().unwrap_or(8080);
    config.log_level = if cfg!(debug_assertions) {
        LogLevel::Normal
    } else {
        LogLevel::Critical
    };

    rocket::build()
        .configure(&config)
        .mount("/", routes![watch_video, get_video_segment])
        .mount("/api", routes![get_api_video, get_video_segment])
}

fn get_base_path() -> std::path::PathBuf {
    if cfg!(debug_assertions) {
        Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
    } else {
        env::current_exe().unwrap().parent().unwrap().to_path_buf()
    }
}

#[get("/watch?<v>")]
async fn get_api_video(v: &str) -> Option<NamedFile> {
    let base_path = get_base_path();
    let file_path = base_path.join("videos").join(v).join(format!("{v}.m3u8"));

    NamedFile::open(file_path).await.ok()
}

#[get("/watch?<v>")]
fn watch_video(v: &str) -> content::RawHtml<String> {
    let html_template = include_str!("assets/index.html").to_string();

    let html = html_template.replace("{{VIDEO_ID}}", v);
    content::RawHtml(html)
}

#[get("/<video_name>")]
async fn get_video_segment(video_name: &str) -> Option<NamedFile> {
    if video_name.len() < 15 && !video_name.ends_with(".ts") {
        return None;
    }
    let video_id = &video_name[..11];

    let base_path = get_base_path();
    let file_path = base_path.join("videos").join(video_id).join(video_name);

    NamedFile::open(file_path).await.ok()
}
