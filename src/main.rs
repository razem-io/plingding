mod config;

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;
use config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to send (optional when using --watch)
    #[arg(short, long)]
    message: Option<String>,

    /// The priority of the message (-2 to 2)
    #[arg(short, long, default_value = "0")]
    priority: i8,

    /// The path to an image to attach (optional)
    #[arg(short, long)]
    image: Option<PathBuf>,

    /// The command to watch (optional)
    #[arg(short, long)]
    watch: Option<String>,
}

#[derive(Serialize)]
struct PushoverRequest<'a> {
    token: &'a str,
    user: &'a str,
    message: &'a str,
    priority: i8,
}

fn execute_command(command: &str) -> Result<bool> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .context("Failed to execute command")?;

    Ok(output.status.success())
}

async fn send_notification(client: &reqwest::Client, config: &Config, message: &str, priority: i8, image: Option<PathBuf>) -> Result<()> {
    let mut form = Form::new()
        .text("token", config.api_key.clone())
        .text("user", config.user_key.clone())
        .text("message", message.to_string())
        .text("priority", priority.to_string());

    if let Some(image_path) = image {
        let file_content = tokio::fs::read(&image_path)
            .await
            .context("Failed to read image file")?;
        let file_part = Part::bytes(file_content)
            .file_name(image_path.file_name().unwrap().to_string_lossy().to_string())
            .mime_str("image/jpeg")?;
        form = form.part("attachment", file_part);
    }

    let response = client
        .post(&config.base_url)
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;

    let client = reqwest::Client::new();

    if let Some(command) = args.watch {
        println!("Watching command: {}", command);
        let success = execute_command(&command)?;
        let mut message = if success {
            format!("Successfully executed: {}", command)
        } else {
            format!("Failed to execute: {}", command)
        };

        if let Some(additional_msg) = args.message {
            message.push_str("\n");
            message.push_str(&additional_msg);
        }

        send_notification(&client, &config, &message, args.priority, args.image).await?;
    } else {
        let message = args.message.unwrap_or_else(|| "Plingding!".to_string());
        send_notification(&client, &config, &message, args.priority, args.image).await?;
    }

    Ok(())
}
