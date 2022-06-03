
mod framework;

extern crate libc;
use libc::c_void;

pub use framework::Framework;
pub use framework::ChannelRx;
pub use framework::ChannelTx;

#[repr(C, align(8))]
pub struct BufferStruct {
    pub ptr : *const u8,
    pub size : usize,
}

#[repr(u32)]
pub enum ConfigStatus {
    OK,
    ERROR,
    INTERFACEUPDATE, // Notify the caller that the public interface has changed
}

#[repr(u32)]
pub enum RuntimeStatus {
    OK,
    ERROR,
}

pub type ConfigStatusCallback  = unsafe extern "C" fn(obj : *mut c_void) -> ConfigStatus;
pub type RuntimeStatusCallback = unsafe extern "C" fn(obj : *mut c_void) -> RuntimeStatus;

pub type VoidCallback = unsafe extern "C" fn(obj : *mut c_void);
pub type SizeCallback = unsafe extern "C" fn(size : usize) -> *mut u8;

pub trait BaseModel {
    fn config(&mut self) -> ConfigStatus;
    fn init(&mut self, interface : &mut Box<dyn Framework>) -> RuntimeStatus;
    fn step(&mut self, interface : &mut Box<dyn Framework>) -> RuntimeStatus;
    fn pause(&mut self) -> RuntimeStatus;
    fn stop(&mut self) -> RuntimeStatus;

    fn msg_get(&self, id : BufferStruct, cb : SizeCallback) -> u32;
    fn msg_set(&mut self, id : BufferStruct, data : BufferStruct) -> u32;
    fn get_ptr(&self, id : BufferStruct) -> *const u8;
}


/// Used for wrapping models that come from other language, like C++ and Fortran
pub struct BaseModelExternal {
    pub obj : *mut c_void,
    pub config_fn : ConfigStatusCallback,
    pub init_fn : RuntimeStatusCallback,
    pub step_fn : RuntimeStatusCallback,
    pub pause_fn : RuntimeStatusCallback,
    pub stop_fn : RuntimeStatusCallback,
    pub destructor_fn : VoidCallback,
}

impl BaseModel for BaseModelExternal {
    fn config(&mut self) -> ConfigStatus {
        unsafe { (self.config_fn)(self.obj) }
    }
    fn init(&mut self, _interface : &mut Box<dyn Framework>) -> RuntimeStatus {
        unsafe { (self.init_fn)(self.obj) }
    }
    fn step(&mut self, _interface : &mut Box<dyn Framework>) -> RuntimeStatus {
        unsafe { (self.step_fn)(self.obj) }
    }
    fn pause(&mut self) -> RuntimeStatus {
        unsafe { (self.pause_fn)(self.obj) }
    }
    fn stop(&mut self) -> RuntimeStatus {
        unsafe { (self.stop_fn)(self.obj) }
    }
    fn msg_get(&self, _id : BufferStruct, _cb : SizeCallback) -> u32 {
        1
    }
    fn msg_set(&mut self, _id : BufferStruct, _data : BufferStruct) -> u32 {
        1
    }
    fn get_ptr(&self, _id : BufferStruct) -> *const u8 {
        0 as *const u8
    }
}

impl Drop for BaseModelExternal {
    fn drop(&mut self) {
        unsafe { (self.destructor_fn)(self.obj); }
    }
}

unsafe impl Send for BaseModelExternal {}

#[no_mangle]
pub extern "C" fn meta_get(ptr : *mut c_void, id : BufferStruct, cb : SizeCallback) -> u32 {
    let app : Box<Box<dyn BaseModel + Send>> = unsafe { Box::from_raw(ptr as *mut Box<dyn BaseModel + Send>) };
    let stat = (*app).msg_get(id, cb);
    Box::into_raw(app); // release ownership of the box
    stat
}

#[no_mangle]
pub extern "C" fn meta_set(ptr : *mut c_void, id : BufferStruct, data : BufferStruct) -> u32 {
    let mut app : Box<Box<dyn BaseModel + Send>> = unsafe { Box::from_raw(ptr as *mut Box<dyn BaseModel + Send>) };
    let stat = (*app).msg_set(id, data);
    Box::into_raw(app); // release ownership of the box
    stat
}
#[no_mangle]
pub extern "C" fn get_ptr(ptr : *mut c_void, id : BufferStruct) -> *const u8 {
    let app : Box<Box<dyn BaseModel + Send>> = unsafe { Box::from_raw(ptr as *mut Box<dyn BaseModel + Send>) };
    let p = (*app).get_ptr(id);
    Box::into_raw(app); // release ownership of the box
    p
}
