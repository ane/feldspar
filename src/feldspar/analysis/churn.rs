use super::event::*;

pub trait Matcher {
    fn regular(&self, input: &str) -> Option<Event>;
    fn kick(&self, input: &str) -> Option<Event>;
}

pub fn match_line<M: Matcher>(m: &M, input: &str) -> Option<Event> {
    m.regular(input)
        .or(m.kick(input))
}
