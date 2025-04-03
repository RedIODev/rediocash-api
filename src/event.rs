use std::{cmp::Ordering, collections::BTreeSet, error::Error, ops::{AddAssign, SubAssign}, ptr::fn_addr_eq};

pub enum EventError {
    Generic(Box<dyn Error>),
    Compound(Vec<EventError>)
}

pub enum Listener<A,R = ()> {
    Fp(fn (A) -> R),
    Other(Box<dyn Fn(A) -> R>)
}

impl<A,R> Listener<A, R> {
    fn call(&self, args: A) -> R {
        match self {
            Listener::Fp(fp) => fp(args),
            Listener::Other(other) => other(args),
        }
    }
}

impl<A,R> PartialEq for Listener<A,R> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fp(f1), Self::Fp(f2)) => fn_addr_eq(*f1, *f2),
            _ => false
        }
    }
}

impl<A,R> Eq for Listener<A,R> {}

impl<A,R> PartialOrd for Listener<A,R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<A,R> Ord for Listener<A,R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }
        Ordering::Greater
        
    }
}

impl<A,R> From<Box<dyn Fn(A)->R>> for Listener<A,R> {
    fn from(value: Box<dyn Fn(A)->R>) -> Self {
        Listener::Other(value)
    }
}

pub struct Event<A,R = ()> {
    pub listeners: BTreeSet<Listener<A,R>>
}

impl<A: Clone, R> Event<A, R> {
    pub fn new() -> Self {
        Self { listeners: BTreeSet::new() }
    }

    pub fn send(&self, args: A) -> Vec<R> {
        let mut result = Vec::new();
        for listener in &self.listeners {
            result.push(listener.call(args.clone()));
        }
        result
    }

    pub fn insert<F>(&mut self, func: F) 
    where F: Into<Listener<A,R>> {
        self.listeners.insert(func.into());
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}

impl<A,R> AddAssign<fn(A)->R> for Event<A,R> {

    ///
    /// 
    /// Might behave unexpected see core::ptr::fn_addr_eq
    /// 
    fn add_assign(&mut self, func: fn(A)->R) {
        self.listeners.insert(Listener::Fp(func));
    }
}

impl<A,R> SubAssign<fn(A)->R> for Event<A,R> {
    
    ///
    /// 
    /// Might behave unexpected see core::ptr::fn_addr_eq
    /// 
    fn sub_assign(&mut self, func: fn(A)->R) {
        let _x: bool = self.listeners.remove(&Listener::Fp(func));
        println!("{_x}")
    }
}