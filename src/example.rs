use std::mem::MaybeUninit;

use crate::{command::{Command, Response, ResponseError, Status}, plugin::Plugin};


pub struct TestPlugin {
    status: Status,
    status_msg: Option<String>
}

impl Plugin for TestPlugin {
    fn name(&self) -> &str {
        "test plugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn execute(&self, command: &Command) -> Result<Response, ResponseError> {
        match command {
            Command::Status => Ok(Response::Status(self.status, self.status_msg.clone())),
            Command::Init() => Ok(Response::Init())
        }
    }
}

#[allow(unused)]
pub unsafe extern "C" fn create_plugin(plugin: &mut MaybeUninit<Box<dyn Plugin>>) {
    plugin.write(Box::new(TestPlugin { status: Status::Uninit, status_msg: None}));
}