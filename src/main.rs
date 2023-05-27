use chrono::{DateTime, Local};
use notify_rust::Notification;
use octocrab::Octocrab;
use std::env::var;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = var("GITHUB_NOTIFY_KEY").expect("GITHUB_NOTIFY_KEY env variable is required");
    let octo = Octocrab::builder().personal_token(token).build()?;

    loop {
        thread::sleep(Duration::from_secs(60 * 10));
        let now: DateTime<Local> = Local::now();
        println!("github_notify: Querying Github API at {}", now);

        let current_rate_limit = octo.ratelimit().get().await?;
        if current_rate_limit.rate.remaining <= 2 {
            eprintln!("github_notify: Cannot request more from Github API!");
        }

        let notification = octo.activity().notifications().list().send().await?;

        if notification.items.len() > 0 {
            Notification::new()
                .summary("New Github notification!")
                .body(notification.items[0].reason.as_str())
                .icon("firefox")
                .show()
                .expect("Failed to launch notification!");
        }
    }
}
