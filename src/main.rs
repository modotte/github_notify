use notify_rust::Notification;
use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_NOTIFY_KEY").expect("GITHUB_TOKEN env variable is required");
    let octo = Octocrab::builder().personal_token(token).build()?;
    let current_rate_limit = octo.ratelimit().get().await?;
    if current_rate_limit.rate.remaining <= 2 {
        eprintln!("Cannot request more from Github API!");
    }

    let notifications = octo.activity().notifications().list().send().await?;

    println!("{:?}", notifications.items);

    Ok(())
}
