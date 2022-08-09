#[cfg(test)]
mod me_tests {
    use log::LevelFilter;
    use rraw::auth::{PasswordAuthenticator, CodeAuthenticator, TokenAuthenticator};
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
    async fn create_client_by_pass() -> anyhow::Result<Client<PasswordAuthenticator>> {
        dotenv::dotenv()?;
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY_BY_PASS")?.as_str(),
            std::env::var("CLIENT_SECRET_BY_PASS")?.as_str(),
            std::env::var("REDDIT_USER")?.as_str(),
            std::env::var("PASSWORD")?.as_str(),
        );
        Ok(Client::login(arc, "RRAW Test (by u/KingTuxWH)").await?)
    }
    #[ignore]
    #[tokio::test]
    async fn me_test_by_pass() -> anyhow::Result<()> {
        init();
        let client = create_client_by_pass().await?;

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
    async fn test_inbox_by_pass() -> anyhow::Result<()> {
        init();
        let client = create_client_by_pass().await?;

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
    async fn create_client_by_code() -> anyhow::Result<Client<CodeAuthenticator>> {
        dotenv::dotenv()?;
        let arc = CodeAuthenticator::new(
            std::env::var("CLIENT_KEY_BY_CODE")?.as_str(),
            std::env::var("CLIENT_SECRET_BY_CODE")?.as_str(),
            std::env::var("CODE")?.as_str(),
            std::env::var("REDIRECT_URI")?.as_str(),
        );
        Ok(Client::login(arc, "RRAW Test (by u/KingTuxWH)").await?)
    }
    #[ignore]
    #[tokio::test]
    async fn me_test_by_code() -> anyhow::Result<()> {
        init();
        let client = create_client_by_code().await?;

        let me = client.me().await;

        assert!(me.is_ok());
        let me = me.unwrap();
        assert!(me.saved(None).await.is_ok());
        assert!(me.up_voted(None).await.is_ok());
        assert!(me.down_voted(None).await.is_ok());

        let r_t = client.refresh_token();
        if r_t.is_some() {
            println!("Refresh Token Is: {}", r_t.unwrap())
        } else {
            println!("Refresh Token Not Exist!")
        }

        return Ok(());
    }
    #[ignore]
    #[tokio::test]
    async fn test_inbox_by_code() -> anyhow::Result<()> {
        init();
        let client = create_client_by_code().await?;

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
    async fn create_client_by_token() -> anyhow::Result<Client<TokenAuthenticator>> {
        dotenv::dotenv()?;
        let arc = TokenAuthenticator::new(
            std::env::var("CLIENT_KEY_BY_TOKEN")?.as_str(),
            std::env::var("CLIENT_SECRET_BY_TOKEN")?.as_str(),
            std::env::var("REFRESH_TOKEN")?.as_str(),
        );
        Ok(Client::login(arc, "RRAW Test (by u/KingTuxWH)").await?)
    }
    #[ignore]
    #[tokio::test]
    async fn me_test_by_token() -> anyhow::Result<()> {
        init();
        let client = create_client_by_token().await?;

        let me = client.me().await;

        assert!(me.is_ok());
        let me = me.unwrap();
        assert!(me.saved(None).await.is_ok());
        assert!(me.up_voted(None).await.is_ok());
        assert!(me.down_voted(None).await.is_ok());

        let r_t = client.refresh_token();
        if r_t.is_some() {
            println!("Refresh Token Is: {}", r_t.unwrap())
        } else {
            println!("Refresh Token Not Exist!")
        }

        return Ok(());
    }
    #[ignore]
    #[tokio::test]
    async fn test_inbox_by_token() -> anyhow::Result<()> {
        init();
        let client = create_client_by_token().await?;

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
