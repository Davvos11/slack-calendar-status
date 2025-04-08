# Slack Calendar status
Set your Slack notification schedule and out-of-office status based on your calendar.

This uses ICAL to get your calendar events, although it has only been tested with ICAL links from Google Calendar.

## Setup
First, create a `.env` file:
```shell
cp .env.dist .env
```
Then, fill in all the values:
- `SLACK_WORKSPACE`: the `company` part in `company.slack.com`
- `SLACK_TOKEN`: the `token` from a Slack request (see below)
- `SLACK_COOKIE`: the `d` cookie from a Slack request (see below)
- `WORK_EVENT`: the title of events in your calendar where you want notifications.
- `ICAL`: ICAL link for the calendar (i.e. `https://calendar.google.com/calendar/ical/.../.../basic.ics`)

## Slack token and cookie
To obtain the token and cookie value from Slack, go to company.slack.com in your browser, open Developer Tools
(<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>I</kbd> or <kbd>F12</kbd>) and click on the Network tab.

Now, open the Preferences in Slack and click on the corresponding request to `https://company.slack.com/api/megaphone.notifications.list`
(or any other Slack API request will do).

Go to the Cookies tab for this request and copy the value for `d` into `SLACK_COOKIE`. Mine starts with `xoxd-` and is quiet long.

Then, go to the Request tab for this request and copy the value for `token` into `SLACK_TOKEN`. Mine starts with `x0xc-` and is quiet long.

## Usage
Now, just run this project, for example using `cargo run --release`

You can also use `cargo build --release` to create an executable to use in e.g. a cron job.

## Example:

