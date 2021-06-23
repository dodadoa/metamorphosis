use tikv_client::{Config, RawClient, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let client = RawClient::new_with_config(
        vec!["127.0.0.1:2379"],
        config).await?;
    let key = "TiKV".as_bytes().to_owned();
    let value = "Works!".as_bytes().to_owned();

    client.put(key.clone(), value.clone()).await?;
    println!(
        "Put: {} => {}",
        std::str::from_utf8(&key).unwrap(),
        std::str::from_utf8(&value).unwrap()
    );

    let returned: Vec<u8> = client.get(key.clone()).await?
        .expect("Value should be present.").into();
    assert_eq!(returned, value);
    println!(
        "Get: {} => {}",
        std::str::from_utf8(&key).unwrap(),
        std::str::from_utf8(&value).unwrap()
    );
    Ok(())
}

