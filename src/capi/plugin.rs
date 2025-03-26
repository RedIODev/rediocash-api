
use std::{cell::Cell, ffi::{c_char, c_void, CStr, CString}};

use libloading::Library;

use crate::{command::{Command, Response, ResponseError, Status}, plugin::Plugin};

use super::api::{self, CommandType_COMMAND_INIT, CommandType_COMMAND_STATUS};


unsafe extern "C" {
    pub fn plugin_name(arg1: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn plugin_version(arg1: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn plugin_create() -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn plugin_destroy(arg1: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn plugin_free(arg1: *mut ::std::os::raw::c_void);
}


pub struct CPlugin {
    c_struct: *mut ::std::os::raw::c_void,
    name_buf: Cell<CString>,
    version_buf: Cell<CString>,
    fp_execute: unsafe fn (*const c_void, *const api::Command) -> api::ResponseResult,
    fp_name: unsafe fn (*const c_void) -> *const c_char,
    fp_version: unsafe fn (*const c_void) -> *const c_char,
    fp_destroy: unsafe fn (*mut c_void),
    fp_free: unsafe fn (*mut c_void),
    _lib: Library,
}

impl CPlugin {
    pub unsafe fn new(lib: Library) -> Result<CPlugin, libloading::Error> {
        unsafe {
            let c_struct = lib.get::<fn () -> *mut c_void>(b"plugin_create")?();
            Ok(CPlugin { c_struct, 
                name_buf: Cell::default(), 
                version_buf: Cell::default(),
                fp_execute: *lib.get(b"plugin_execute")?,
                fp_name: *lib.get(b"plugin_name")?,
                fp_version: *lib.get(b"plugin_version")?,
                fp_destroy: *lib.get(b"plugin_destroy")?,
                fp_free: *lib.get(b"plugin_free")?,
                _lib: lib
            })
        }
    }

    unsafe fn copy_c_str_nullable(&self, ptr: *const c_char) -> Option<String> {
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let str = CStr::from_ptr(ptr).to_str().map(str::to_owned).ok();
            (self.fp_free)(ptr as *mut c_void);
            str
        }
    }
    
    unsafe fn copy_c_str(&self, ptr: *const c_char) -> CString {
        if ptr.is_null() {
            return CString::default();
        }
        unsafe { 
            let str = CStr::from_ptr(ptr).to_owned();
            (self.fp_free)(ptr as *mut c_void);
            str
        }
    }

    unsafe fn from_c_response_result(&self, result: &api::ResponseResult) -> Result<Response, ResponseError> {
        unsafe {
            match result.isError {
                true => Err(ResponseError::from(&result.__bindgen_anon_1.error)),
                false => Ok(self.from_c_response(&result.__bindgen_anon_1.response))
            }
    
        }
    }

    unsafe fn from_c_response(&self, response: &api::Response) -> Response {
        unsafe {
            match response.type_ {
                api::CommandType_COMMAND_INIT => Response::Init(),
                api::CommandType_COMMAND_STATUS => {
                    let status_code = response.data.status.statusCode;
                    let status_msg = self.copy_c_str_nullable(response.data.status.message);
                    Response::Status(
                        Status::try_from(status_code)
                            .expect("Invalid Enum varient from C Plugin."), 
                        status_msg
                    )
                }
                _ => panic!("Invalid Enum varient from C Plugin.")
            }
        }
    }
}

impl Drop for CPlugin {
    fn drop(&mut self) {
        unsafe { 
            (self.fp_destroy)(self.c_struct);
        }
    }
}

impl Plugin for CPlugin {

    fn name(&self) -> &str {
        unsafe {
            if (*self.name_buf.as_ptr()).is_empty() {
                let ptr = (self.fp_name)(self.c_struct);
                self.name_buf.replace(self.copy_c_str(ptr));
            }
            c_string_as_str(&(*self.name_buf.as_ptr()))
        }
    }

    fn version(&self) -> &str {
        unsafe {
            if (*self.version_buf.as_ptr()).is_empty() {
                let ptr = (self.fp_version)(self.c_struct);
                self.version_buf.replace(self.copy_c_str(ptr));
            }
            c_string_as_str(&(*self.version_buf.as_ptr()))
        }
    }

    fn execute(&self, command: &crate::command::Command) -> Result<Response, ResponseError> {
        unsafe { self.from_c_response_result(&(self.fp_execute)(self.c_struct, &command.into()))}
    }
}

impl From<&Command> for api::Command {
    fn from(value: &Command) -> Self {
        match value {
            Command::Init() => api::Command { 
                type_: CommandType_COMMAND_INIT, 
                data: api::CommandData { bindgen_union_field: [] }
            },
            Command::Status => api::Command { 
                type_: CommandType_COMMAND_STATUS, 
                data: api::CommandData { bindgen_union_field: [] } 
            },
        }
    }
}

impl From<&api::ResponseError> for ResponseError {
    fn from(value: &api::ResponseError) -> Self {
        match value.type_ {
            api::ResponseErrorType_RESPONSE_ERROR_UNINIT => ResponseError::Uninit,
            api::ResponseErrorType_RESPONSE_ERROR_UNIMPLEMENTED => ResponseError::Unimplemented,
            _ => panic!("Invalid Enum varient from C Plugin.")
        }
    }
}


fn c_string_as_str(c_str:&CString) -> &str {
    c_str.as_c_str().to_str().expect("Invalid String from C Plugin.")
}

