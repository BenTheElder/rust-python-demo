extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::vec::Vec;
use std::str;

// simple data structure to pass to python
#[repr(C)] // c memory layout
pub struct DemoStruct {
	a: i32,
	b: i32
}


#[no_mangle] // do not mangle the method name so we can load it from python
pub extern fn get_demo_struct() -> *const DemoStruct {
	// return a pointer to a copy of our simple demo structure
	&DemoStruct{a: 0, b: 1337}
}

// create a type for a c style char*
pub type Cstr = *const libc::c_char;

// a structure containing a length and an array of c-strings
#[repr(C)] // c memory layout
pub struct CstrArr {
	len: u32, // length of the data pointer
	data: *const Cstr // c string array
}

// return an array of c-strings with a length
#[no_mangle] // do not mangle the method name so we can load it from python
pub extern fn get_array_of_strings() -> *const CstrArr {
	// normal rust vector of strings
	let rust_vec = vec!["Hello","world","from","rust"];
	// copy over rust strings to ffi c-strings
	let mut c_vec = Vec::with_capacity(rust_vec.len());
	for i in 0..rust_vec.len() {
		c_vec.push(CString::new(rust_vec[i]).unwrap().as_ptr());
	}
	// put into our c-struct length/pointer pair and return
	&CstrArr {
		len: c_vec.len() as u32,
		data: c_vec.as_ptr(),
	}
}


// take in a c-string and combine it with a rust string to produce a new c-string
#[no_mangle] // do not mangle the method name so we can load it from python
pub extern fn hello_world(c: *const libc::c_char) -> Cstr {
	// normal rust string
	let mut s = "Hello world (From rust). ".to_string();
	// this part is unsafe, we have to convert from a pointer to an ffi::Cstr
	// and then back to an ffi::CString pointer
	unsafe {
		let cs = CStr::from_ptr(c);
		s = s + str::from_utf8(cs.to_bytes()).unwrap();
		CString::new(s).unwrap().as_ptr()
	}
}
