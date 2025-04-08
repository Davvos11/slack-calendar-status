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

Calendar has the following events with the title "Werk" (i.e. `WORK_EVENT` setting)

![Screenshot_20250408_190140](https://github.com/user-attachments/assets/0b515782-c00b-46c7-8005-609e7e36d20b)

After running, Slack notification schedule is set to:

![Screenshot_20250408_190213](https://github.com/user-attachments/assets/0334b7e9-6c54-49c4-ad9b-6ebb15ce378f)

And my Slack status is set to:

![Screenshot_20250408_190225](https://github.com/user-attachments/assets/7ee4d441-7998-4bb3-a253-ca757beea33e)
