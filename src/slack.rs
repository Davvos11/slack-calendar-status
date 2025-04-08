use anyhow::Context;
use chrono::{DateTime, Local, NaiveTime, Timelike, Weekday};
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::{header, multipart, Client, IntoUrl};
use serde::{Serialize, Serializer};
use std::borrow::Cow;

pub struct Slack {
    client: Client,
    token: String,
    headers: HeaderMap,
}
impl Slack {
    pub fn new() -> Self {
        // Load env variables from .env
        dotenv().ok();
        let token = std::env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set");
        let cookie = std::env::var("SLACK_COOKIE").expect("SLACK_COOKIE not set");
        // Set up client
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            format!("d={cookie};")
                .parse()
                .expect("Invalid SLACK_COOKIE"),
        );
        let client = Client::new();
        Self {
            client,
            token,
            headers,
        }
    }

    async fn request(
        &self,
        url: impl IntoUrl,
        title: impl Into<Cow<'static, str>>,
        body: &impl Serialize,
    ) -> anyhow::Result<String> {
        let body = serde_json::to_string(body)?;
        let form = multipart::Form::new()
            .text("token", self.token.clone())
            .text(title, body);

        let response = self
            .client
            .post(url)
            .headers(self.headers.clone())
            .multipart(form)
            .send()
            .await
            .context("Could not set notification schedule")?;
        let result = response.text().await.context("Could not parse response")?;
        Ok(result)
    }

    pub async fn set_notification_schedule(&self, schedule: &UserPrefs) -> anyhow::Result<String> {
        self.request(
            "https://vcampusk.slack.com/api/users.prefs.set",
            "prefs",
            schedule,
        )
        .await
    }

    pub async fn set_status(&self, status: &UserProfile) -> anyhow::Result<String> {
        self.request(
            "https://vcampusk.slack.com/api/users.profile.set",
            "profile",
            status,
        )
        .await
    }
}

#[derive(Serialize, Default)]
pub struct UserPrefs {
    dnd_enabled: bool,
    dnd_days: DndDays,
    dnd_before_monday: Option<Time>,
    dnd_after_monday: Option<Time>,
    dnd_enabled_monday: DndEnabled,
    dnd_before_tuesday: Option<Time>,
    dnd_after_tuesday: Option<Time>,
    dnd_enabled_tuesday: DndEnabled,
    dnd_before_wednesday: Option<Time>,
    dnd_after_wednesday: Option<Time>,
    dnd_enabled_wednesday: DndEnabled,
    dnd_before_thursday: Option<Time>,
    dnd_after_thursday: Option<Time>,
    dnd_enabled_thursday: DndEnabled,
    dnd_before_friday: Option<Time>,
    dnd_after_friday: Option<Time>,
    dnd_enabled_friday: DndEnabled,
    dnd_before_saturday: Option<Time>,
    dnd_after_saturday: Option<Time>,
    dnd_enabled_saturday: DndEnabled,
    dnd_before_sunday: Option<Time>,
    dnd_after_sunday: Option<Time>,
    dnd_enabled_sunday: DndEnabled,
}

impl UserPrefs {
    pub fn new() -> Self {
        Self {
            dnd_enabled: true,
            ..Default::default()
        }
    }

    pub fn set_day(&mut self, day: Weekday, from: Time, to: Time) -> &mut UserPrefs {
        match day {
            Weekday::Mon => {
                self.dnd_enabled_monday = DndEnabled::Partial;
                self.dnd_before_monday = Some(from);
                self.dnd_after_monday = Some(to);
            }
            Weekday::Tue => {
                self.dnd_enabled_tuesday = DndEnabled::Partial;
                self.dnd_before_tuesday = Some(from);
                self.dnd_after_tuesday = Some(to);
            }
            Weekday::Wed => {
                self.dnd_enabled_wednesday = DndEnabled::Partial;
                self.dnd_before_wednesday = Some(from);
                self.dnd_after_wednesday = Some(to);
            }
            Weekday::Thu => {
                self.dnd_enabled_thursday = DndEnabled::Partial;
                self.dnd_before_thursday = Some(from);
                self.dnd_after_thursday = Some(to);
            }
            Weekday::Fri => {
                self.dnd_enabled_friday = DndEnabled::Partial;
                self.dnd_before_friday = Some(from);
                self.dnd_after_friday = Some(to);
            }
            Weekday::Sat => {
                self.dnd_enabled_saturday = DndEnabled::Partial;
                self.dnd_before_saturday = Some(from);
                self.dnd_after_saturday = Some(to);
            }
            Weekday::Sun => {
                self.dnd_enabled_sunday = DndEnabled::Partial;
                self.dnd_before_sunday = Some(from);
                self.dnd_after_sunday = Some(to);
            }
        }
        self
    }
}

#[derive(Serialize)]
pub struct UserProfile {
    status_emoji: String,
    status_expiration: u64,
    status_text: String,
}

impl UserProfile {
    pub fn out_of_office(until: DateTime<Local>) -> Self {
        Self {
            status_emoji: String::from(":no_entry:"),
            status_expiration: until.timestamp() as u64,
            status_text: String::from("Out of office"),
        }
    }

    pub fn in_office() -> Self {
        Self {
            status_emoji: String::new(),
            status_expiration: 0,
            status_text: String::new(),
        }
    }
}
#[derive(Serialize, Default)]
#[serde(rename_all = "snake_case")]
enum DndDays {
    EveryDay,
    Weekdays,
    #[default]
    Custom,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "snake_case")]
enum DndEnabled {
    Partial,
    #[default]
    AllDay,
}

#[derive(Default)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
}

impl Into<Time> for NaiveTime {
    fn into(self) -> Time {
        Time {
            hours: self.hour() as u8,
            minutes: self.minute() as u8,
        }
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format!("{:02}:{:02}", self.hours, self.minutes))
    }
}
