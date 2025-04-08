use crate::ical::get_events;
use crate::slack::{Slack, UserPrefs};
use chrono::Datelike;

mod ical;
mod slack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let events = get_events().await?;

    let mut prefs = UserPrefs::new();
    for event in events {
        if event.name != "Werk" {
            continue;
        }

        let end = event.time + event.length;
        for date in &event.dates {
            prefs.set_day(date.weekday(), event.time.into(), end.into());
        }
    }

    let slack = Slack::new();
    let status = slack.set_notification_schedule(&prefs).await?;
    dbg!(status);
    Ok(())
}
