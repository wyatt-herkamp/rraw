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

    let subreddit = client.subreddit("askreddit").await;

    assert!(subreddit.is_ok());
    let data = subreddit.unwrap();
    for (id, value) in data.subreddit.other.iter() {
        println!("{id}: {value:?}");
    }
    return Ok(());
}
