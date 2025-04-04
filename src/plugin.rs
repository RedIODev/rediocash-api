use crate::{command::{Command, Response, ResponseError}, event::Events};

pub trait Plugin: PartialOrd {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn register_events(&self, events: &mut Events);
    fn subscribe_events(&self, events: &mut Events);
}