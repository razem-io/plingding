mod config;

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;
use config::{Config, PushProvider};

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

    /// The providers to use (comma-separated list)
    #[arg(short, long)]
    providers: Option<String>,
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

async fn send_notification(client: &reqwest::Client, provider: &PushProvider, message: &str, priority: i8, image: Option<&PathBuf>) -> Result<()> {
    match provider.provider_type.as_str() {
        "pushover" => send_pushover_notification(client, provider, message, priority, image).await,
        "ntfy" => send_ntfy_notification(client, provider, message, priority, image).await,
        _ => Err(anyhow::anyhow!("Unsupported provider type for {}: {}", provider.name, provider.provider_type)),
    }
}

async fn send_pushover_notification(client: &reqwest::Client, provider: &PushProvider, message: &str, priority: i8, image: Option<&PathBuf>) -> Result<()> {
    let mut form = Form::new()
        .text("token", provider.api_key.clone())
        .text("user", provider.user_key.clone().unwrap_or_default())
        .text("message", message.to_string())
        .text("priority", priority.to_string());

    if let Some(image_path) = image {
        let file_content = tokio::fs::read(image_path)
            .await
            .context("Failed to read image file")?;
        let file_part = Part::bytes(file_content)
            .file_name(image_path.file_name().unwrap().to_string_lossy().to_string())
            .mime_str("image/jpeg")?;
        form = form.part("attachment", file_part);
    }

    let base_url = provider.base_url.as_deref().unwrap_or("https://api.pushover.net/1/messages.json");
    let response = client
        .post(base_url)
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Notification sent successfully to Pushover ({})", provider.name);
    } else {
        let error_text = response.text().await?;
        anyhow::bail!("Failed to send notification to Pushover ({}): {}", provider.name, error_text);
    }

    Ok(())
}

async fn send_ntfy_notification(client: &reqwest::Client, provider: &PushProvider, message: &str, priority: i8, image: Option<&PathBuf>) -> Result<()> {
    let ntfy_priority = match priority {
        -2 => "min",
        -1 => "low",
        0 => "default",
        1 => "high",
        2 => "max",
        _ => "default",
    };

    let base_url = provider.base_url.as_ref().context("Base URL is required for ntfy provider")?;
    let mut request = client
        .post(base_url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .header("Priority", ntfy_priority)
        .body(message.to_string());

    if let Some(image_path) = image {
        let file_content = tokio::fs::read(image_path)
            .await
            .context("Failed to read image file")?;
        request = request.header("Filename", image_path.file_name().unwrap().to_string_lossy().to_string());
        request = request.body(file_content);
    }

    let response = request.send().await?;

    if response.status().is_success() {
        println!("Notification sent successfully to ntfy ({})", provider.name);
    } else {
        let error_text = response.text().await?;
        anyhow::bail!("Failed to send notification to ntfy ({}): {}", provider.name, error_text);
    }

    Ok(())
}

fn select_providers(config: &Config, cli_providers: Option<&str>) -> Vec<&PushProvider> {
    match cli_providers {
        Some(providers) => providers
            .split(',')
            .filter_map(|name| config.get_provider(name.trim()))
            .collect(),
        None => config.get_default_providers(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;

    let client = reqwest::Client::new();

    let providers = select_providers(&config, args.providers.as_deref());
    if providers.is_empty() {
        anyhow::bail!("No valid providers specified");
    }

    let message = if let Some(command) = args.watch {
        println!("Watching command: {}", command);
        let success = execute_command(&command)?;
        let mut msg = if success {
            format!("Successfully executed: {}", command)
        } else {
            format!("Failed to execute: {}", command)
        };

        if let Some(additional_msg) = args.message {
            msg.push_str("\n");
            msg.push_str(&additional_msg);
        }
        msg
    } else {
        args.message.unwrap_or_else(|| "Plingding!".to_string())
    };

    for provider in providers {
        send_notification(&client, provider, &message, args.priority, args.image.as_ref()).await?;
    }

    Ok(())
}
