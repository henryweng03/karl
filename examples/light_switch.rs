use serde_json;
use karl::net::KarlAPI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = KarlAPI::new();
    let data = api.get_triggered().await?.unwrap();
    let slots = serde_json::from_slice(&data[..])?;
    match slots {
        serde_json::Value::Object(map) => {
            match map.get("state").unwrap() {
                serde_json::Value::String(state) => {
                    if state == &"on".to_string() {
                        api.push("state", vec![1]).await?;
                    } else if state == &"off".to_string() {
                        api.push("state", vec![0]).await?;
                    }
                },
                _ => {},
            }
        }
        _ => {},
    }
    Ok(())
}