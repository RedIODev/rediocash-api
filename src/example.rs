use std::mem::MaybeUninit;

use crate::{command::{Command, Response, ResponseError, Status}, event::{Event, Events}, plugin::Plugin};


pub struct TestPlugin {
    events: Events
}

impl Plugin for TestPlugin {
    fn name(&self) -> &str {
        "test plugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn register_events(&mut self) {
        self.events.register_event("test", Event::<String,()>::new());
    }
    
    fn subscribe_events(&mut self) {
        *self.events.try_get_event_mut::<String, ()>("test").unwrap() += |s| Self::test(s);
    }

}

impl TestPlugin {
    fn test(s: String) {
        println!("{s}")
    }
}

#[allow(unused, improper_ctypes_definitions)]
pub extern "C" fn create(events: Events) -> Box<dyn Plugin> {
    Box::new(TestPlugin { events })
}

// 
// pub unsafe extern "C" fn create_plugin(plugin: &mut MaybeUninit<Box<dyn Plugin>>) {
//     plugin.write(Box::new(TestPlugin { status: Status::Uninit, status_msg: None}));
// }