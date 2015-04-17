extern crate time;

use std::fmt;

use self::Event::*;

pub enum Event {
    /// Time, nick and contents
    Message(time::Tm, String, String),
    /// Time, kicked, kicker, reason
    Kick(time::Tm, String, String, String),
}

impl fmt::Display for Event {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Message(t, ref nick, ref cont) => {
                write!(fmt, "{} <{}> {}", t.rfc822z(), nick, cont)
            },
            Kick(t, ref kicked, ref kicker, ref reason) => {
                write!(fmt, "{} {} kicked {}: {}", t.rfc822z(), kicker, kicked, reason)
            }
        }
    }
}
