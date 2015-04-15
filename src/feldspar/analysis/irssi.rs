use super::churn::Matcher;
use super::event::Event;

use regex::Regex;

static TimeStamp : regex::Regex = regex!(r"\d{2}:\d{2}");
static Mode : regex::Regex = regex!(r"@&%+!\s");
static Nick : regex::Regex = regex!(r"A-Za-z\[\]\\`_\^\{\|\}");

pub struct Irssi;

impl Matcher for Irssi {
    fn regular(&self, input: &str) -> Option<Event> {
        match input {
            "hehehe" => {
                let t = time::now_utc();
                Some(Event::Message(t, "ding", "dong"))
            },
            _ => None
        }
    }

    fn kick(&self, input: &str) -> Option<Event> {
        match input {
            "hahaha" => {
                Some(Event::Kick(time::now_utc(), "bip", "bap", "i don't like you"))
            },
            _ => None
        }
    }
}
