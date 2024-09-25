use anyhow::{Context, Result};
use clap::Parser;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use std::path::PathBuf;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to send
    #[arg(short, long)]
    message: String,

    /// The priority of the message (-2 to 2)
    #[arg(short, long, default_value = "0")]
    priority: i8,

    /// The path to an image to attach (optional)
    #[arg(short, long)]
    image: Option<PathBuf>,
}

#[derive(Serialize)]
struct PushoverRequest<'a> {
    token: &'a str,
    user: &'a str,
    message: &'a str,
    priority: i8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Read API token and user key from environment variables
    let api_token = env::var("PUSHOVER_API_TOKEN")
        .context("PUSHOVER_API_TOKEN environment variable not set")?;
    let user_key = env::var("PUSHOVER_USER_KEY")
        .context("PUSHOVER_USER_KEY environment variable not set")?;

    let client = reqwest::Client::new();
    let mut form = Form::new()
        .text("token", api_token)
        .text("user", user_key)
        .text("message", args.message)
        .text("priority", args.priority.to_string());

    if let Some(image_path) = args.image {
        let file_content = tokio::fs::read(&image_path)
            .await
            .context("Failed to read image file")?;
        let file_part = Part::bytes(file_content)
            .file_name(image_path.file_name().unwrap().to_string_lossy().to_string())
            .mime_str("image/jpeg")?;
        form = form.part("attachment", file_part);
    }

    let response = client
        .post("https://api.pushover.net/1/messages.json")
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Notification sent successfully!");
    } else {
        let error_text = response.text().await?;
        anyhow::bail!("Failed to send notification: {}", error_text);
    }

    Ok(())
}
