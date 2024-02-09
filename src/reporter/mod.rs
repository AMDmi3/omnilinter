pub mod json;
pub mod stdout;

use crate::location::MatchLocation;

pub trait Reporter {
    fn report(&mut self, location: &MatchLocation, message: &str);

    fn flush(&self) {}
}
