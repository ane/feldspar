use super::churn::Matcher;
use super::event::Event;
use time;

use regex::Regex;

static TimeStamp: Regex = regex!(r"\d{2}:\d{2}");
static Mode: Regex = regex!(r"[@&%+!\s]");
static Nick: Regex = regex!(r"[A-Za-z\[\]\\`_\^\{\|\}]+");

pub struct Irssi;

fn parse_timestamp(ts: &str) -> Option<time::Tm> {
    match ts.split(':').collect::<Vec<&str>>().as_slice() {
        [a, b] => {
            let (h, m) = (a.parse::<i32>(), b.parse::<i32>());
            match (h.ok(), m.ok()) {
                (Some(hours), Some(min)) => {
                    let mut ts = time::empty_tm();
                    ts.tm_hour = hours;

                    ts.tm_min = min;
                    Some(ts)
                },
                _ => None,
            }
        },
        _ => None,
    }
}

impl Matcher for Irssi {
    fn regular(&self, input: &str) -> Option<Event> {
        let raw = format!(r"^({})\s<{}({})>\s(.*)$", TimeStamp, Mode, Nick);
        let re = Regex::new(&raw).unwrap();
        re.captures(input).and_then(|caps| {
            match [caps.at(1), caps.at(2), caps.at(3)] {
                [Some(tstamp), Some(nick), Some(cont)] => {
                    parse_timestamp(tstamp).and_then(|ts| {
                        Some(Event::Message(ts, nick.to_string(), cont.to_string()))
                    })
                },
                _ => None,
            }
        })
    }

    fn kick(&self, input: &str) -> Option<Event> {
        match input {
            "hahaha" => {
                Some(Event::Kick(time::now_utc(), String::from_str("herp"), String::from_str("derp"), String::from_str("go away")))
            },
            _ => None
        }
    }
}
