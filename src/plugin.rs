
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn register_events(&mut self);
    fn subscribe_events(&mut self);
}