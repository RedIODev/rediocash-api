use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::ops::{AddAssign, SubAssign};
use std::sync::Arc;
use derive_enum_from_into::EnumFrom;
use dyn_serde::{Deserializer, Serialize};
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};


use smallbox::{space, SmallBox, smallbox};


#[derive(Clone, Default)]
pub struct Events {
    events: Arc<RwLock<BTreeMap<String, SmallBox<dyn RawEvent, space::S32>>>>
}

impl Events {

    pub fn register_event(&self, name:impl Into<String>,  event: Event<impl EventArgs, impl EventResult>) -> bool {
        let name = name.into();
        if self.events.read().contains_key(&name) {
            return false;
        }
        
        self.events.write().insert(name.into(), smallbox!(event));
        true
    }

    pub fn get_raw_event(&self, name: &str) -> Option<MappedRwLockReadGuard<'_, SmallBox<dyn RawEvent, space::S32>>> {
        RwLockReadGuard::try_map(self.events.read(), 
                |events| events.get(name))
                .ok()
    }

    pub fn get_raw_event_mut(&self, name: &str) -> Option<MappedRwLockWriteGuard<'_, SmallBox<dyn RawEvent, space::S32>>> {
        RwLockWriteGuard::try_map(self.events.write(),
                |events| events.get_mut(name))
                .ok()
    }

    pub fn try_get_event<A: EventArgs, R: EventResult>(&self, name: &str) -> Option<MappedRwLockReadGuard<'_, Event<A,R>>> {
        MappedRwLockReadGuard::try_map(self.get_raw_event(name)?,
                |e| e.any().downcast_ref())
                .ok()
    }

    pub fn try_get_event_mut<A: EventArgs, R: EventResult>(&self, name: &str) -> Option<MappedRwLockWriteGuard<'_, Event<A,R>>> {
        MappedRwLockWriteGuard::try_map(self.get_raw_event_mut(name)?,
                |e| e.any_mut().downcast_mut())
                .ok()
    }

    

}

pub trait EventArgs: 'static + Clone + for<'a> serde::Deserialize<'a> {}
impl<T> EventArgs for T where T: 'static + Clone + for<'a> serde::Deserialize<'a> {}
pub trait EventResult: 'static + serde::Serialize {}
impl<T> EventResult for T where T: 'static + serde::Serialize {}

type CListenerFp = unsafe extern "C" fn(()) -> (); //C Args & Result types

pub trait RawEvent: Any {
    fn notify_c(&mut self, args: &mut dyn Deserializer) -> Vec<Box<dyn Serialize>>;

    // fn consume_c(&mut self, args: &mut dyn Deserializer) -> Box<dyn Serialize>;

    fn register_c(&mut self, fp: CListenerFp) -> bool;

    fn unregister_c(&mut self, fp: CListenerFp) -> bool;

    fn clear(&mut self);

    fn remove(&mut self, listener_id: ListernerId) -> bool;

    fn len(&self) -> usize;
}

impl dyn RawEvent {
    pub fn any(&self) -> &dyn Any {
        self
    }

    pub fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<A: EventArgs, R: EventResult> RawEvent for Event<A,R> {
    fn notify_c(&mut self, args: &mut dyn Deserializer) -> Vec<Box<dyn Serialize>> {
        let a = args.deserialize::<A>().unwrap();
        self.notify(a)
                .into_iter()
                .map(|r| Box::new(r) as Box<dyn Serialize>)
                .collect()
    }

    // fn consume_c(&mut self, args: &mut dyn Deserializer) -> Box<dyn Serialize> {
    //     todo!()
    // }
    
    fn register_c(&mut self, fp: CListenerFp) -> bool {
        let id = ListernerId::Fp(fp as usize);
        if self.listeners.contains_key(&id) {
            return false;
        }
        self.listeners.insert(id, smallbox!(CListener {fp}));
        true
    }
    
    fn unregister_c(&mut self, fp: CListenerFp) -> bool {
        self.remove(ListernerId::Fp(fp as usize))
    }

    fn clear(&mut self) {
        self.listeners.clear();
    }

    fn remove(&mut self, listener_id: ListernerId) -> bool {
        self.listeners.remove(&listener_id).is_some()
    }

    fn len(&self) -> usize {
        self.listeners.len()
    }
}

pub trait Listener<A, R> {
    fn consume(&mut self, args: A) -> R;
}

impl<A, R, T> Listener<A, R> for T
where
    T: FnMut(A) -> R,
{

    fn consume(&mut self, args: A) -> R {
        self(args)
    }
}

struct CListener {
    fp: CListenerFp
}

impl<A,R> Listener<A, R> for CListener {
    fn consume(&mut self, args: A) -> R {
        todo!()
    }
}


pub type ListenerBox<A,R> = SmallBox<dyn Listener<A, R>, space::S2>; 

#[derive(PartialEq, PartialOrd, Eq, Ord, EnumFrom)]
pub enum ListernerId {
    TypeId(TypeId),
    Fp(usize)
}

#[derive(Default)]
pub struct Event<A:Clone, R> { 
    listeners: BTreeMap<ListernerId, ListenerBox<A,R>>,
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

impl<A:Clone + EventArgs, R: EventResult> Event<A,R> {

    pub fn new() -> Self {
        Self {
            listeners: BTreeMap::new(),
        }
    }

    pub fn register<F>(&mut self, func: F) -> bool 
    where F: Listener<A, R> + Any {
        let id = func.type_id().into();
        if self.listeners.contains_key(&id) {
            return false;
        }
        self.listeners.insert(id, smallbox!(func));
        true
    }

    pub fn unregister<F>(&mut self, func: &F) -> bool 
    where F: Listener<A, R> + Any {
        self.remove(func.type_id().into())
    }


}

impl<A:Clone + EventArgs, R: EventResult, F> AddAssign<F> for Event<A, R>
where
    F: Listener<A, R> + Any,
{
    fn add_assign(&mut self, func: F) {
        self.register(func);
    }
}

impl<A:Clone + EventArgs, R: EventResult, F> SubAssign<&F> for Event<A, R>
where
    F: Listener<A, R> + Any,
{
    fn sub_assign(&mut self, func: &F) {
       self.unregister(func);
    }
}