#[allow(missing_copy_implementations)]
extern crate libc;
use std::c_str::CString;
use std::vec::Vec;

#[repr(C)]
pub struct DemoStruct {
	a: i32,
	b: i32
}

//just to make rustc happy, we don't actually use this.
impl Copy for DemoStruct {}

#[no_mangle]
pub extern fn get_demo_struct() -> *const DemoStruct {
	&DemoStruct{a: 0, b: 1337}
}



pub type Cstr = *const libc::c_char;

#[repr(C)]
pub struct CstrArr {
	len: u32,
	data: *const Cstr
}

#[no_mangle]
pub extern fn get_array_of_strings() -> *const CstrArr {
	let rust_vec = vec!["Hello","world","from","rust"];
	unsafe {
		&CstrArr {
			len: rust_vec.len().to_u32().unwrap(),
			data: Vec::from_fn(rust_vec.len(),
				|s| rust_vec[s].to_c_str().into_inner()).as_ptr()
		}
	}
}



#[no_mangle]
pub extern fn hello_world(c: *const libc::c_char) -> Cstr {
	let mut s = "Hello world (From rust). ".to_string();
	unsafe {
		let cs = CString::new(c, false);
		s = s + cs.as_str().unwrap();
		s.to_c_str().into_inner()
	}
}
