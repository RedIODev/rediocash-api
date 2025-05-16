use std::mem::MaybeUninit;

use crate::{command::{Command, Response, ResponseError, Status}, event::{Event, Events}, plugin::{InitData, Plugin}, CBox};


pub struct TestPlugin {
    pub events: Events
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
        *self.events.try_get_event_mut::<String, ()>("test").unwrap() += Self::test;
    }

}

impl TestPlugin {
    fn test(s: String) {
        println!("{s}")
    }

    pub fn fire(&self) {
        self.events.try_get_event_mut::<String, ()>("test").unwrap().notify("Test message".to_string());
    }
}

#[allow(unused)]
pub extern "C" fn load_plugin(data: InitData) -> CBox<dyn Plugin> {
    let boxed: Box<dyn Plugin> = Box::new(TestPlugin { events: *data.events.to_box() });
    boxed.into()
}

// #[unsafe(no_mangle)]
// pub fn load_rust_plugin(data: InitData) -> PluginWrapper {
//     PluginWrapper::new(Box::new(TestPlugin { events: unimplemented!()}))
// }

// 
// pub unsafe extern "C" fn create_plugin(plugin: &mut MaybeUninit<Box<dyn Plugin>>) {
//     plugin.write(Box::new(TestPlugin { status: Status::Uninit, status_msg: None}));
// }