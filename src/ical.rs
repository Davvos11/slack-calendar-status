use anyhow::Context;
use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeDelta, TimeZone, Utc};
use dotenv::dotenv;
use icalendar::{Calendar, CalendarComponent, Component, DatePerhapsTime, Event};
use now::DateTimeNow;
use rrule::{RRuleSet, Tz};

pub async fn get_events() -> anyhow::Result<Vec<CalendarItem>> {
    // Load env variables from .env
    dotenv().ok();
    let ical_url = dotenv::var("ICAL").expect("ICAL not set");
    // Download ical data
    let client = reqwest::Client::new();
    let response = client.get(ical_url).send().await?;
    let body = response.text().await?;

    // Get limits
    let start_week = Local::now().with_timezone(&Tz::LOCAL).beginning_of_week();
    let end_week = Local::now().with_timezone(&Tz::LOCAL).end_of_week();
    // Parse data
    let calendar: Calendar = body.parse().unwrap();
    let mut result = Vec::new();
    for component in &calendar.components {
        if let CalendarComponent::Event(event) = component {
            let Some(name) = event.get_summary() else {
                eprintln!("Item has no name");
                continue;
            };
            let start = match event.get_start() {
                None => {
                    eprintln!("Item has no start");
                    continue;
                }
                Some(DatePerhapsTime::Date(date)) => Some(get_datetime(date)),
                Some(DatePerhapsTime::DateTime(datetime)) => datetime.try_into_utc(),
            };
            let Some(start) = start else {
                eprintln!("Could not parse item start");
                continue;
            };
            let end = match event.get_end() {
                None => Some(start.end_of_day()),
                Some(DatePerhapsTime::Date(date)) => Some(get_datetime(date)),
                Some(DatePerhapsTime::DateTime(datetime)) => datetime.try_into_utc(),
            };
            let Some(end) = end else {
                eprintln!("Could not parse item end");
                continue;
            };

            let start = start.with_timezone(&Tz::LOCAL);
            let end = end.with_timezone(&Tz::LOCAL);
            let length = end - start;

            let recurrence = get_recurrence_string(event);
            let dates = match recurrence {
                Ok(Some(rule)) => rule
                    .after(start_week)
                    .before(end_week)
                    .all_unchecked()
                    .iter()
                    .map(|x| x.date_naive())
                    .collect(),
                Ok(None) => {
                    if start_week <= end && start <= end_week {
                        vec![start.date_naive()]
                    } else {
                        vec![]
                    }
                }
                Err(e) => {
                    eprintln!("{e:?}");
                    continue;
                }
            };

            if !dates.is_empty() {
                result.push(CalendarItem {
                    name: name.into(),
                    dates,
                    time: start.time(),
                    length,
                });
            }
        }
    }

    Ok(result)
}

#[derive(Debug)]
pub struct CalendarItem {
    pub name: String,
    pub dates: Vec<NaiveDate>,
    pub time: NaiveTime,
    pub length: TimeDelta,
}

fn get_datetime(date: NaiveDate) -> DateTime<Utc> {
    let datetime = date.and_time(Default::default());
    Utc.from_utc_datetime(&datetime)
}

fn get_recurrence_string(event: &Event) -> anyhow::Result<Option<RRuleSet>> {
    let props = event.properties();
    if let Some(rule) = props.get("RRULE") {
        let start = props.get("DTSTART").context("Missing DTSTART")?;

        let start_string: String = start.clone().try_into()?;
        let rule_string: String = rule.clone().try_into()?;
        let ful_rule = format!("{start_string}{rule_string}");
        let parsed_rule: RRuleSet = ful_rule.parse()?;

        Ok(Some(parsed_rule))
    } else {
        Ok(None)
    }
}
