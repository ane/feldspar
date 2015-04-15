extern crate time;

use std::fmt;

use self::Event::*;

pub enum Event {
    /// Time, nick and contents
    Message(time::Tm, &'static str, &'static str),
    /// Time, kicked, kicker, reason
    Kick(time::Tm, &'static str, &'static str, &'static str)
}

impl fmt::Display for Event {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Message(t, nick, cont) => {
                write!(fmt, "{} <{}> {}", t.rfc822z(), nick, cont)
            },
            Kick(t, kicked, kicker, reason) => {
                write!(fmt, "{} {} kicked {}: {}", t.rfc822z(), kicker, kicked, reason)
            }
        }
    }
}
