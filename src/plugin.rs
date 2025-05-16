use crate::{event::Events, CBox};


pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn register_events(&mut self);
    fn subscribe_events(&mut self);
}

//define clear rust to rust api and clear rust to C api

// pub type LoadFunc = unsafe extern "C" fn(InitData) -> Box<dyn Plugin>;

// pub const LOAD_FUNC_NAME: &'static [u8] = b"load_plugin";

#[derive(Clone, Default)]
#[repr(C)]
pub struct InitData {
    pub events: CBox<Events>
}

impl InitData {
    pub fn new (events: Events) -> InitData {
        Self { events: Box::new(events).into() }
    }
}