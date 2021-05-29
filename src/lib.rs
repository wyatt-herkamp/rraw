mod auth;
mod me;
mod responses;
mod subreddit;
mod user;
mod utils;

#[cfg(test)]
mod tests {
    use tokio;

    use crate::auth::{AnonymousAuthenticator, PasswordAuthenticator};
    use crate::me::Me;
    use crate::responses::RedditType::{Comment, Link};
    use crate::responses::RedditType;

    #[tokio::test]
    async fn anon_subreddit_tests() {
        let me = Me::login(
            AnonymousAuthenticator::new(),
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
            .await
            .unwrap();
        let subreddit = me.subreddit("memes".to_string());
        let x = subreddit.about().await;
        let subreddit1 = x.unwrap();
        println!("{}", subreddit1.data.title.unwrap());
    }

    #[tokio::test]
    async fn user_saved() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(
            arc,
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
            .await
            .unwrap();
        let user = me.user("KingTuxWH".to_string());
        let x = user.saved(None).await.unwrap();
        for x in x.data.safe_children() {
            match x {
                Comment(comment) => {
                    println!("Comment {:?}", comment.unwrap().body);

                }
                Link(link) => {
                    println!("Link {:?}", link.unwrap().name);

                }
                _ => {}
            }

        }
    }

    #[tokio::test]
    async fn anon_user_tests() {
        let me = Me::login(
            AnonymousAuthenticator::new(),
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
            .await
            .unwrap();
        let user = me.user("KingTuxWH".to_string());
        let about = user.about().await.unwrap();
        println!("{}", about.data.name);
    }
}
