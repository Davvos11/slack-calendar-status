use crate::ical::get_events;
use crate::slack::{Slack, UserPrefs, UserProfile};
use chrono::{Datelike, Local, TimeZone, Utc};
use std::process::exit;
use dotenv::dotenv;

mod ical;
mod slack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load env variables from .env
    dotenv().ok();
    let work_event = dotenv::var("WORK_EVENT").expect("WORK_EVENT not set");
    
    // Load events
    let events = get_events().await?;

    let mut prefs = UserPrefs::new();
    let now = Local::now();
    let mut next = None;
    let mut current = false;
    
    // Filter and process events
    for event in events {
        if event.name != work_event {
            continue;
        }

        let end = event.time + event.length;
        for date in &event.dates {
            prefs.set_day(date.weekday(), event.time.into(), end.into());
            if let Some(datetime) = Local.from_local_datetime(&date.and_time(event.time)).single() {
                if datetime > now && (next.is_none() || datetime < next.unwrap()) {
                    next = Some(datetime);
                }
                if datetime <= now && (datetime + event.length) >= now {
                    current = true;
                }
            }
        }
    }

    let profile = if current {
        Some(UserProfile::in_office())
    } else {
        next.map(UserProfile::out_of_office)
    };

    // Send to Slack
    let slack = Slack::new();
    let result = slack.set_notification_schedule(&prefs).await?;
    println!("{result}");
    if let Some(profile) = profile {
        let result = slack.set_status(&profile).await?;
        println!("{result}");
    }
    Ok(())
}
