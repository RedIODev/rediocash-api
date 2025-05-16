use std::ops::Deref;


pub mod command;
#[cfg(feature = "plugin")]
pub mod plugin;
#[cfg(feature = "loader")]
pub mod loader;
pub mod example;
#[cfg(feature = "capi")]
pub mod capi;

pub mod event;

//mod linkedlist;

#[derive(Debug)]
#[repr(C)]
pub struct CBox<T: ?Sized>(*mut T);

impl<T:?Sized> From<Box<T>> for CBox<T> {
    fn from(value: Box<T>) -> Self {
        Self(Box::leak(value))
    }
}

impl<T:?Sized> CBox<T> {
    pub fn to_box(self) -> Box<T> {
        unsafe {Box::from_raw(self.0)}
    }
}

impl<T:Clone + ?Sized> Clone for CBox<T> {
    fn clone(&self) -> Self {
        let boxed = unsafe { Box::from_raw(self.0)};
        boxed.clone().into()
    }
}

impl<T:Default + ?Sized> Default for CBox<T> {
    fn default() -> Self {
        Box::<T>::default().into()
    }
}