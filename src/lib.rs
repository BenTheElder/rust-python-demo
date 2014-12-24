extern crate libc;
use std::c_str::CString;

#[repr(C)]
pub struct DemoStruct {
	a: i32,
	b: i32
}


#[no_mangle]
pub extern fn get_demo_struct() -> *mut DemoStruct {
	&mut DemoStruct{a: 0, b: 1337}
}


#[no_mangle]
pub extern fn hello_world(c: *const libc::c_char) -> *const libc::c_char {
	let mut s = "Hello world (From rust). ".to_string();
	unsafe {
		let cs = CString::new(c, false);
		s = s + cs.as_str().unwrap();
		s.to_c_str().into_inner()
	}
}
