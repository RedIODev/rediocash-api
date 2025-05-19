use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::ops::{AddAssign, SubAssign};
use std::sync::Arc;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use smallbox::{space, SmallBox, smallbox};


#[derive(Debug, Clone, Default)]
pub struct Events {
    events: Arc<RwLock<BTreeMap<String, SmallBox<dyn Any, space::S32>>>>
}

impl Events {

    pub fn register_event<A: 'static, R: 'static>(&self, name:impl Into<String>,  event: Event<A,R>) -> bool {
        let name = name.into();
        if self.events.read().contains_key(&name) {
            return false;
        }
        
        self.events.write().insert(name.into(), smallbox!(event));
        true
    }

    pub fn try_get_event<A: 'static, R: 'static>(&self, name: &str) -> Option<MappedRwLockReadGuard<'_, Event<A,R>>> {
        RwLockReadGuard::try_map(self.events.read(), |events| {
            events.get(name)
            .map(|event| event.downcast_ref())
            .flatten()
        })
        .ok()
    }

    pub fn try_get_event_mut<A: 'static, R: 'static>(&self, name: &str) -> Option<MappedRwLockWriteGuard<'_, Event<A,R>>> {
        RwLockWriteGuard::try_map(self.events.write(), |events| {
            events.get_mut(name)
            .map(|event| event.downcast_mut())
            .flatten()
        })
        .ok()
    }
}

pub trait Listener<A> {
    type Result;

    fn consume(&mut self, args: A) -> Self::Result;
}

impl<A, R, T> Listener<A> for T
where
    T: FnMut(A) -> R,
{
    type Result = R;

    fn consume(&mut self, args: A) -> Self::Result {
        self(args)
    }
}

pub type ListenerBox<A,R> = SmallBox<dyn Listener<A, Result = R>, space::S2>; 

#[derive(Default)]
pub struct Event<A, R> {
    listeners: BTreeMap<TypeId, ListenerBox<A,R>>,
}

impl<A: Clone, R> Event<A, R> {

    pub fn notify(&mut self, args: A) -> Vec<R> {
        let mut result = Vec::new();
        for listener in &mut self.listeners {
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
    where F: Listener<A,Result=R> + Any {
        let id = func.type_id();
        if self.listeners.contains_key(&id) {
            return false;
        }
        self.listeners.insert(id, smallbox!(func));
        true
    }

    pub fn unregister<F>(&mut self, func: &F) -> bool 
    where F: Listener<A,Result=R> + Any {
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
    F: Listener<A,Result=R> + Any,
{
    fn add_assign(&mut self, func: F) {
        self.register(func);
    }
}

impl<A, R, F> SubAssign<&F> for Event<A, R>
where
    F: Listener<A,Result=R> + Any,
{
    fn sub_assign(&mut self, func: &F) {
       self.unregister(func);
    }
}