use chrono::{DateTime, Local, Utc};
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

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args: Vec<String> = args().collect();
    let mut datetime_format = "";

    if let Some(dtf) = args.get(1) {
        datetime_format = dtf;
    }

    let github_api_key =
        var("GITHUB_NOTIFY_KEY").expect("GITHUB_NOTIFY_KEY env variable is required");
    let octo = Octocrab::builder().personal_token(github_api_key).build()?;

    loop {
        println!(
            "github_notify: Querying Github API at {}",
            if datetime_format == "utc" {
                Utc::now().to_string()
            } else {
                Local::now().to_string()
            }
        );

        let current_rate_limit = octo.ratelimit().get().await?;
        let minimum_remaining_rate = 2;
        if current_rate_limit.rate.remaining <= minimum_remaining_rate {
            eprintln!("github_notify: Cannot request more from Github API!");
        }

        let github_notification = octo.activity().notifications().list().send().await?;
        notify_desktop(&github_notification);

        thread::sleep(Duration::from_secs(60 * 10));
    }
}
