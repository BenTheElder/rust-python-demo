extern crate libc;
use std::c_str::CString;


fn world() -> String {
	"world.".to_string()
}

#[no_mangle]
pub extern fn hello_world(c: *const libc::c_char) -> *const libc::c_char {
	unsafe{
		let cs = CString::new(c, false);
		let mut s = "Hello ".to_string() + world().as_slice() + " (From rust) ";
		s = s + cs.as_str().unwrap();
		s.to_c_str().into_inner()
	}
}
