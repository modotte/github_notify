use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octo = Octocrab::builder().personal_token(token).build()?;
    let thread = octocrab::instance()
        .activity()
        .notifications()
        .list()
        .send()
        .await?;

    println!("{:?}", thread.items);

    Ok(())
}
