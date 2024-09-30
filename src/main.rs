use anyhow::{Context, Result};
use clap::Parser;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use std::path::PathBuf;
use std::env;
use std::process::Command;

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

async fn send_notification(client: &reqwest::Client, api_token: &str, user_key: &str, message: &str, priority: i8, image: Option<PathBuf>) -> Result<()> {
    let mut form = Form::new()
        .text("token", api_token.to_string())
        .text("user", user_key.to_string())
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Read API token and user key from environment variables
    let api_token = env::var("PUSHOVER_API_TOKEN")
        .context("PUSHOVER_API_TOKEN environment variable not set")?;
    let user_key = env::var("PUSHOVER_USER_KEY")
        .context("PUSHOVER_USER_KEY environment variable not set")?;

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

        send_notification(&client, &api_token, &user_key, &message, args.priority, args.image).await?;
    } else {
        let message = args.message.unwrap_or_else(|| "Plingding!".to_string());
        send_notification(&client, &api_token, &user_key, &message, args.priority, args.image).await?;
    }

    Ok(())
}
