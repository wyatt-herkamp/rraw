use log::LevelFilter;
use rraw::auth::AnonymousAuthenticator;
use rraw::Client;

fn init() {
    if let Err(error) = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Debug)
        .try_init()
    {
        println!("Logger Failed to Init Error: {}", error);
    }
}

#[ignore]
#[tokio::test]
async fn generic() -> anyhow::Result<()> {
    init();
    let client = Client::login(AnonymousAuthenticator::new(), "RRAW Test (by u/KingTuxWH)").await?;

    let domains = client.domain("rust-lang.org", None).await;
    assert!(domains.is_ok());
    let data = domains.unwrap().data;
    assert!(data.children.len() > 0);
    return Ok(());
}
