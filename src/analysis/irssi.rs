extern crate time;

pub struct Irssi;

use super::Matcher;
use super::event::Event;

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
