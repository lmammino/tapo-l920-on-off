use std::env;
use tapo::ApiClient;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let tapo_device_ip = env::var("TAPO_DEVICE_IP")?;

    let device = ApiClient::new(tapo_username, tapo_password)?
        .l920(tapo_device_ip)
        .await?;

    let device_info = device.get_device_info().await?;
    info!(
        "Connected to {} - {} ({}). State is {}",
        device_info.model,
        device_info.nickname,
        device_info.device_id,
        match device_info.device_on {
            true => "on",
            false => "off",
        }
    );

    info!("{:?}", device_info);

    if device_info.overheated {
        warn!("Device is overheated");
    }

    if device_info.device_on {
        device.off().await?;
        info!("Device turned off");
    } else {
        device.on().await?;
        info!("Device turned on");
    }

    Ok(())
}
