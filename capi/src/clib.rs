use std::{ffi::{c_char, CStr}, os::raw::c_void, ptr::NonNull};

use rediocash_api::{event::{Event, Events}, plugin::{InitData, Plugin}, CBox, FatCBox};

use crate::capi::{deallocate_event_data, CEvent, CEventMut, CEvents, CInitData, CPlugin};


#[unsafe(no_mangle)]
pub extern "C" fn plugin_new(plugin: CPlugin) -> FatCBox {
    let plugin: Box<dyn Plugin> = Box::new(plugin);
    plugin.into()
}

#[unsafe(no_mangle)]
pub extern "C" fn unpack_init_data(data: InitData) -> CInitData {
    CInitData { 
        events: CEvents { inner: data.global_events.raw().cast().as_ptr()} 
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn register_event(mut events: CEvents, name: *const c_char) -> bool {
    let events = unsafe { events.events() };
    let cstr = unsafe { takeown_cstring(name) };
    events.register_event(cstr, Event::<(), ()>::new()) //C types
}

#[unsafe(no_mangle)]
pub extern "C" fn get_event(mut events: CEvents, name: *const c_char) -> CEvent {
    let events = unsafe { events.events() };
    let cstr = unsafe { takeown_cstring(name) };
    let event = events.get_raw_event(&cstr).unwrap();
    let boxed_event = CBox::from(Box::new(event));
    CEvent { event: boxed_event.raw().cast().as_ptr() }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_event_mut(mut events: CEvents, name: *const c_char) -> CEventMut {
    let events = unsafe { events.events() };
    let cstr = unsafe { takeown_cstring(name) };
    let event = events.get_raw_event_mut(&cstr).unwrap();
    let boxed_event = CBox::from(Box::new(event));
    CEventMut { event: boxed_event.raw().cast().as_ptr() }
}


impl CEvents {
    //takes ownership of the Events object to ensure single ownership of temporary raw pointer. 
    //This means this function can only be called once on any CEvents object.
    unsafe fn events(&mut self) -> Box<Events> {
        let ptr = NonNull::new(self.inner).expect("Nullpointer not allowed!").cast();
        self.inner = std::ptr::null_mut();
        unsafe {Box::from_raw(ptr.as_ptr())}
    }
}

unsafe fn takeown_cstring(string: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(string) }.to_string_lossy().into_owned();
    unsafe { deallocate_event_data(string as *mut c_void) };
    cstr
}