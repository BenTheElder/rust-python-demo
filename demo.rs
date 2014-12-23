extern crate libc;

fn world() -> String {
	"world.".to_string()
}

#[no_mangle]
pub extern fn hello_world() -> *const libc::c_char {
	let s = "Hello ".to_string() + world().as_slice() + " (From rust)";
	unsafe{
		s.to_c_str().into_inner()
	}
}
