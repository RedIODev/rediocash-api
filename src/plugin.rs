use crate::command::{Command, Response, ResponseError};

pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, command: &Command) -> Result<Response, ResponseError>;

}