use super::churn::Matcher;
use super::event::Event;
use time;

use regex::Regex;

static TimeStamp: Regex = regex!(r"\d{2}:\d{2}");
static Mode: Regex = regex!(r"[@&%+!\s]");
static Nick: Regex = regex!(r"[A-Za-z\[\]\\`_\^\{\|\}]+");

pub struct Irssi;

impl Matcher for Irssi {
    fn regular(&self, input: &str) -> Option<Event> {
        let raw = format!(r"^{}\s<{}({})>\s(.*)$", TimeStamp, Mode, Nick);
        let re = Regex::new(&raw).unwrap();
        match re.captures(input) {
            Some(caps) => {
                match [caps.at(1), caps.at(2)] {
                    [Some(nick), Some(cont)] => {
                        Some(Event::Message(time::now_utc(), nick.to_string(), cont.to_string()))
                    }
                    _ => None,
                }
            },
            None => None,
        }
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
