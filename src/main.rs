use crate::slack::{Day, Slack, Time, UserPrefs};

mod slack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let slack = Slack::new();
    let prefs = UserPrefs::new().set_day(
        Day::Friday,
        Time {
            hours: 01,
            minutes: 30,
        },
        Time {
            hours: 20,
            minutes: 00,
        },
    );
    let status = slack.set_notification_schedule(&prefs).await?;
    dbg!(status);
    Ok(())
}
