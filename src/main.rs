use chrono::{DateTime, Local};
use octocrab::models::activity::Notification;
use octocrab::{Octocrab, Page};
use std::env::var;
use std::{thread, time::Duration};

fn notify_desktop(github_notification: &Page<Notification>) {
    if !github_notification.items.is_empty() {
        notify_rust::Notification::new()
            .summary("There are unread Github notifications!")
            .body(github_notification.items[0].reason.as_str())
            .icon("firefox")
            .show()
            .expect("github_notify: Cannot launch desktop notification!");
    }
}

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = var("GITHUB_NOTIFY_KEY").expect("GITHUB_NOTIFY_KEY env variable is required");
    let octo = Octocrab::builder().personal_token(token).build()?;

    loop {
        thread::sleep(Duration::from_secs(60 * 10));
        let current_datetime: DateTime<Local> = Local::now();
        println!("github_notify: Querying Github API at {}", current_datetime);

        let current_rate_limit = octo.ratelimit().get().await?;
        if current_rate_limit.rate.remaining <= 2 {
            eprintln!("github_notify: Cannot request more from Github API!");
        }

        let notification = octo.activity().notifications().list().send().await?;
        notify_desktop(&notification);
    }
}
