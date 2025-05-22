use std::ffi::CStr;

use rediocash_api::plugin::Plugin;

use crate::capi::CPlugin;



impl Plugin for CPlugin {
    fn name(&self) -> &str {
        let cstr = unsafe {self.vtable.name.expect("name fp can't be null!")(self.object)};
        unsafe { cstr_to_str(cstr) }.unwrap_or("Unknown")
    }

    fn version(&self) -> &str {
        let cstr = unsafe {self.vtable.version.expect("version fp can't be null!")(self.object)};
        unsafe { cstr_to_str(cstr) }.unwrap_or("0.0.0")
    }

    fn register_events(&mut self) {
        unsafe { self.vtable.register_events.expect("register_event fp can't be null!")(self.object) }
    }

    fn subscribe_events(&mut self) {
        unsafe { self.vtable.substribe_events.expect("subscribe_event fp can't be null!")(self.object) }
    }
}

unsafe fn cstr_to_str<'a>(cstr: *const i8) -> Option<&'a str> {
    if cstr.is_null() {
        return None;
    }

    let mut len = 0;
    unsafe {
        while (*cstr.add(len)) != 0 {
            len += 1;
        }
    }

    let slice = unsafe { std::slice::from_raw_parts(cstr as *const u8, len)};
    CStr::from_bytes_until_nul(slice)
                .ok()
                .map(|cstr| cstr.to_str().ok())
                .flatten()
}