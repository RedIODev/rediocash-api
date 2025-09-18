#![feature(ptr_metadata)]
#![feature(box_vec_non_null)]

use std::{ffi::{c_char, c_void, CStr}, ptr::{DynMetadata, NonNull, Pointee, Thin}};


#[cfg(feature = "plugin")]
pub mod plugin;
#[cfg(feature = "loader")]
pub mod loader;
pub mod example;
pub mod event;


#[derive(Debug)]
#[repr(C)]
pub struct FatCBox(NonNull<()>, NonNull<()>);

impl<T> From<Box<T>> for FatCBox 
where T: ?Sized + Pointee<Metadata = DynMetadata<T>> {


    fn from(value: Box<T>) -> Self {
        let t = NonNull::from_ref(Box::leak(value));
        let (ptr, vtable) = t.to_raw_parts();
        let vtable = Box::new(vtable);
        let vtable = NonNull::from_ref(Box::leak(vtable)).cast();
        Self(ptr, vtable)
    }
}

impl FatCBox {
    pub unsafe fn to_box<T>(self) -> Box<T> 
    where T: ?Sized + Pointee<Metadata = DynMetadata<T>> {
        unsafe {
            let vtable = Box::from_non_null(self.1.cast());
            let t = NonNull::from_raw_parts(self.0, *vtable);
            Box::from_non_null(t)
        }
    }

    pub fn raw(self) -> (NonNull<()>, NonNull<()>) {
        (self.0, self.1)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CBox(NonNull<()>);

impl<T> From<Box<T>> for CBox {
    fn from(value: Box<T>) -> Self {
        Self(NonNull::from_ref(Box::leak(value)).cast())
    }
}

impl CBox {
    pub unsafe fn to_box<T: Thin>(self) -> Box<T> {
        unsafe {
            Box::from_non_null(self.0.cast())
        }
    }

    pub unsafe fn from_raw(ptr: *mut()) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    pub unsafe fn cloned<T: Clone + Thin>(&self) -> Self {
        let boxed = unsafe { Box::<T>::from_non_null(self.0.cast())};
                 boxed.clone().into()
    }

    pub fn raw(self) -> NonNull<()> {
        self.0
    }
}

type CDeallocFp = unsafe extern "C" fn(*mut c_void);

pub unsafe fn takeown_cstring(string: *const c_char, dealloc_string_fp: CDeallocFp) -> String {
    let cstr = unsafe { CStr::from_ptr(string) }.to_string_lossy().into_owned();
    unsafe { dealloc_string_fp(string as *mut c_void) };
    cstr
}