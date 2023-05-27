use chrono::{Local, Utc};
use octocrab::models::activity::Notification;
use octocrab::{Octocrab, Page};
use std::env::{args, var};
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

enum DateTimeFormat {
    Utc,
    Local,
}

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args: Vec<String> = args().collect();
    let mut datetime_format = DateTimeFormat::Local;
    if args[0] == "utc" {
        datetime_format = DateTimeFormat::Utc;
    }

    let token = var("GITHUB_NOTIFY_KEY").expect("GITHUB_NOTIFY_KEY env variable is required");
    let octo = Octocrab::builder().personal_token(token).build()?;

    loop {
        let current_datetime = match datetime_format {
            DateTimeFormat::Local => Local::now(),
            DateTimeFormat::Utc => Utc::now().into(),
        };

        println!("github_notify: Querying Github API at {}", current_datetime);

        let current_rate_limit = octo.ratelimit().get().await?;
        let minimum_remaining_rate = 2;
        if current_rate_limit.rate.remaining <= minimum_remaining_rate {
            eprintln!("github_notify: Cannot request more from Github API!");
        }

        let notification = octo.activity().notifications().list().send().await?;
        notify_desktop(&notification);

        thread::sleep(Duration::from_secs(60 * 10));
    }
}
