use crate::{event::Events, CBox};


pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn register_events(&mut self);
    fn subscribe_events(&mut self);
}

#[repr(C)]
pub struct InitData {
    pub global_events: CBox
}

impl Default for InitData {
    fn default() -> Self {
        Self { global_events: CBox::from(Box::<Events>::default()) }
    }
}

impl Clone for InitData {
    fn clone(&self) -> Self {
        
        Self { global_events: unsafe {self.global_events.cloned::<Events>()} }
    }
}

impl InitData {
    pub fn new (events: Events) -> InitData {
        Self { global_events: Box::new(events).into() }
    }
}