#[cfg(test)]
mod me_tests {
    use log::LevelFilter;
    use rraw::auth::PasswordAuthenticator;
    use rraw::message::WhereMessage;
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
    async fn create_client() -> anyhow::Result<Client<PasswordAuthenticator>> {
        dotenv::dotenv()?;
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY")?.as_str(),
            std::env::var("CLIENT_SECRET")?.as_str(),
            std::env::var("REDDIT_USER")?.as_str(),
            std::env::var("PASSWORD")?.as_str(),
        );
        Ok(Client::login(arc, "RRAW Test (by u/KingTuxWH)").await?)
    }
    #[ignore]
    #[tokio::test]
    async fn me_test() -> anyhow::Result<()> {
        init();
        let client = create_client().await?;

        let me = client.me().await;

        assert!(me.is_ok());
        let me = me.unwrap();
        assert!(me.saved(None).await.is_ok());
        assert!(me.up_voted(None).await.is_ok());
        assert!(me.down_voted(None).await.is_ok());

        return Ok(());
    }
    #[ignore]
    #[tokio::test]
    async fn test_inbox() -> anyhow::Result<()> {
        init();
        let client = create_client().await?;

        let me = client.me().await?;

        me.get_messages(None, None).await.unwrap();
        me.get_messages(Some(WhereMessage::SENT), None)
            .await
            .unwrap();
        me.get_messages(Some(WhereMessage::Unread), None)
            .await
            .unwrap();
        return Ok(());
    }
}
