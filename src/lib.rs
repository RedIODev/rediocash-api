#![feature(ptr_metadata)]

use std::ptr::{DynMetadata, Pointee};


#[cfg(feature = "plugin")]
pub mod plugin;
#[cfg(feature = "loader")]
pub mod loader;
pub mod example;
pub mod event;


#[derive(Debug)]
#[repr(C)]
pub struct FatCBox(*mut (), *mut ());

impl<T> From<Box<T>> for FatCBox 
where T: ?Sized + Pointee<Metadata = DynMetadata<T>> {


    fn from(value: Box<T>) -> Self {
        let t = Box::leak(value) as *mut T;
        let (ptr, vtable) = t.to_raw_parts();
        let vtable = Box::new(vtable);
        let vtable = Box::leak(vtable) as * mut DynMetadata<T> as *mut ();
        Self(ptr, vtable)
    }
}

impl FatCBox {
    pub unsafe fn to_box<T>(self) -> Box<T> 
    where T: ?Sized + Pointee<Metadata = DynMetadata<T>> {
        unsafe {
            let vtable = Box::from_raw(self.1 as *mut DynMetadata<T>);
            let t = std::ptr::from_raw_parts_mut(self.0, *vtable);
            Box::from_raw(t)
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CBox(*mut ());

impl<T> From<Box<T>> for CBox {
    fn from(value: Box<T>) -> Self {
        Self(Box::leak(value) as *mut T as *mut ())
    }
}

impl CBox {
    pub unsafe fn to_box<T>(self) -> Box<T> {
        unsafe {
            Box::from_raw(self.0 as *mut T)
        }
    }
}

// impl<T:Clone + ?Sized> Clone for CBox {
//     fn clone(&self) -> Self {
//         let boxed = unsafe { Box::from_raw(self.0)};
//         boxed.clone().into()
//     }
// }

// impl<T:Default + ?Sized> Default for CBox {
//     fn default() -> Self {
//         Box::<T>::default().into()
//     }
// }