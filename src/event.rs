use std::{error::Error, ops::{AddAssign, SubAssign}};

pub enum EventError {
    Generic(Box<dyn Error>),
    Compound(Vec<EventError>)
}

pub type Listener<Args, Res> = dyn Fn(Args) -> Result<Res, EventError>;

pub struct Event<Args, Res> {
    listeners: Vec<Box<Listener<Args, Res>>>
}


impl<Args: Clone, Res> Event<Args, Res> {
    pub fn new() -> Self {
        Self { listeners: Vec::new() }
    }

    pub fn send(&self, e: Args) -> Vec<Result<Res, EventError>> {
        let mut vec = Vec::new();
        for listener in &self.listeners {
            vec.push(listener(e.clone()));
        }
        vec
    }
}

impl<Args, Res, F> AddAssign<F> for Event<Args, Res> 
where 
    F: Fn(Args) -> Result<Res, EventError> + 'static
{
    fn add_assign(&mut self, func: F) {
        self.listeners.push(Box::new(func));
    }
}

impl<Args, Res, F> SubAssign<F> for Event<Args, Res> 
where 
    F: Fn(Args) -> Result<Res, EventError> + PartialEq<dyn Fn(Args) -> Result<Res, EventError>> + 'static  {
    fn sub_assign(&mut self, func: F) {
        self.listeners.retain(|f| func.eq(&**f))
    }
}


struct X {
    event: Event<i32, i32>
}

impl X {
    fn foo(i: i32) -> Result<i32, EventError> {
        Ok(i)
    }

    fn t(&mut self) {
        self.event += Self::foo;
        self.event -= Self::foo;
    }
}

