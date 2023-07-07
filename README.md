
# RRAW ![crates.io](https://img.shields.io/crates/v/rraw.svg) [![Documentation](https://docs.rs/rraw/badge.svg)](https://docs.rs/rraw) [![Rust](https://github.com/wherkamp/rraw/actions/workflows/rust.yml/badge.svg)](https://github.com/wherkamp/rraw/actions/workflows/rust.yml)



**R**ust **R**eddit **A**PI **W**rapper is a basic Rust Wrapper for Reddit. It gives simple and easy use to common tasks done via the Reddit API. It also does very little processing of the Reddit API giving you near raw results from the Reddit API.

# End of Life

Due to recent changes to the Reddit API I no longer plan to be maintaining this library. 

# Features

- Support for Anonymous and Logged in Browsing 
- Async Backend powered by Tokio and Reqwest
- Raw Data results from Reddit API
- Made for Rust 2021

# How to get started

Here is a small crash course. However, please review the tests directory for more examples

```rust
    #[tokio::main]
    pub async fn main() -> anyhow::Result<()> {
    // Create a Client passing a Authenticator into it. Also give the Client a user agent
    let client =
        Client::login(AnonymousAuthenticator::new(), "RRAW Test (by u/KingTuxWH)").await?;
    /// Get the about page of the user
    let user = client.user("KingTuxWH").await?;
    /// About pages implement Display giving you their name
    println!("Username: {}", user.about);
    Ok(())
}
```