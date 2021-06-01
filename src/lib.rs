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
    use crate::responses::RedditType;
    use crate::responses::RedditType::{Comment, Link};

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
        let subreddit = x.unwrap();
        println!("{}", subreddit.data.title.unwrap());
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
        for x in x.data.children {
            match x.data {
                Comment(comment) => {
                    println!("Comment {:?}", comment.body);

                }
                Link(link) => {
                    println!("Link {:?}", link.name);

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
        let response = user.about().await.unwrap();
        println!("{}", response.data.name);
    }
}
