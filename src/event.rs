use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::{AddAssign, SubAssign};
use std::rc::Rc;

use smallbox::{space, SmallBox, smallbox};

// pub enum EventError {
//     Generic(Box<dyn Error>),
//     Compound(Vec<EventError>),
// }

pub struct Events {
    events: BTreeMap<String, SmallBox<dyn Any, space::S32>>
}

impl Events {
    pub fn register_event<A: 'static, R: 'static>(&mut self, name:impl Into<String>,  event: Rc<Event<A,R>>) -> bool {
        let name = name.into();
        if self.events.contains_key(&name) {
            return false;
        }
        
        self.events.insert(name.into(), smallbox!(event));
        true
    }

    pub fn try_get_event<A: 'static, R: 'static>(&mut self, name: &str) -> Option<&mut Event<A, R>> {
        self.events
                .get_mut(name)
                .map(|event| event.downcast_mut::<Rc<Event<A,R>>>())
                .flatten()
                .map(|rc| &mut **rc)
    }
}

pub trait Listener<A> {
    type Result;

    fn consume(&self, args: A) -> Self::Result;
}

impl<A, R, T> Listener<A> for T
where
    T: Fn(A) -> R,
{
    type Result = R;

    fn consume(&self, args: A) -> Self::Result {
        self(args)
    }
}

pub type ListenerBox<A,R> = SmallBox<dyn Listener<A, Result = R>, space::S2>; 

#[derive(Default)]
pub struct Event<A, R> {
    listeners: BTreeMap<TypeId, ListenerBox<A,R>>,
}

impl<A: Clone, R> Event<A, R> {

    pub fn notify(&self, args: A) -> Vec<R> {
        let mut result = Vec::new();
        for listener in &self.listeners {
            result.push(listener.1.consume(args.clone()));
        }
        result
    }
}

impl<A,R> Event<A,R> {

    pub fn new() -> Self {
        Self {
            listeners: BTreeMap::new(),
        }
    }

    pub fn register<F>(&mut self, func: F) -> bool 
    where F: Fn(A) -> R + Any {
        let id = func.type_id();
        if self.listeners.contains_key(&id) {
            return false;
        }
        self.listeners.insert(id, smallbox!(func));
        true
    }

    pub fn unregister<F>(&mut self, func: &F) -> bool 
    where F: Fn(A) -> R + Any {
        self.listeners.remove(&func.type_id()).is_some()

    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }

    pub fn remove(&mut self, type_id: TypeId) -> Option<ListenerBox<A,R>> {
        self.listeners.remove(&type_id)
    }

    pub fn len(&self) -> usize {
        self.listeners.len()
    }
}

impl<A, R, F> AddAssign<F> for Event<A, R>
where
    F: Fn(A) -> R + Any,
{
    fn add_assign(&mut self, func: F) {
        self.register(func);
    }
}

impl<A, R, F> SubAssign<&F> for Event<A, R>
where
    F: Fn(A) -> R + Any,
{
    fn sub_assign(&mut self, func: &F) {
       self.unregister(func);
    }
}

// struct AnyOrd<T: Any>(pub T);

// impl<T: Any> From<T> for AnyOrd<T> {
//     fn from(value: T) -> Self {
//         AnyOrd(value)
//     }
// }

// impl<T: Any> PartialEq for AnyOrd<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.type_id().eq(&other.type_id())
//     }
// }

// impl<T: Any> Eq for AnyOrd<T> {}

// impl<T: Any> PartialOrd for AnyOrd<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.0.type_id().partial_cmp(&other.0.type_id())
//     }
// }

// impl<T: Any> Ord for AnyOrd<T> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.0.type_id().cmp(&other.0.type_id())
//     }
// }
