use std::{alloc::Layout, ffi::{c_char, CStr, CString}, os::raw::c_void};

use rediocash_api::{event::{Event, Events}, plugin::{InitData, Plugin}, FatCBox};

use crate::capi::{deallocate_event_data, CEvents, CInitData, CPlugin, PluginVtable};


#[unsafe(no_mangle)]
pub extern "C" fn plugin_new(plugin: CPlugin) -> FatCBox {
    let plugin: Box<dyn Plugin> = Box::new(plugin);
    plugin.into()
}

#[unsafe(no_mangle)]
pub extern "C" fn unpack_init_data(data: InitData) -> CInitData {
    CInitData { 
        events: CEvents { inner: unsafe { data.global_events.raw() as *mut c_void}} 
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn register_event(events: CEvents, name: *const c_char) -> bool {
    let events = unsafe { Box::from_raw(events.inner as *mut Events) };
    let cstr = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
    unsafe { deallocate_event_data(name as *mut c_void) };
    events.register_event(cstr, Event::<*const c_void, *const c_void>::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn get_event(events: CEvents, name: *const c_char) -> bool {
    
}
