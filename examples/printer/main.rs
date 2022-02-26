use aht10::unix::Aht10Client;
use aht10::Aht10;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    serve().await
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let mut aht = Aht10Client::new_with_path_and_address_hex(
        env!("I2C_DEVICE_PATH"),
        env!("I2C_DEVICE_ADDRESS"),
    )?;

    aht.initialize()?;
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    aht.normalize()?;
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        if let Err(_) = aht.trigger_measure() {
            continue;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(75)).await;

        if let Ok(re) = aht.measure() {
            info!("temp: {}, hum: {}", re.temp(), re.hum());
        }
    }
}
