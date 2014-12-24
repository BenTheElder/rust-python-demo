import os
import sys
import glob
from ctypes import *

#get path to this file.
path = os.path.dirname(os.path.realpath(__file__))

#determine dynamic library extension based on platform
ext = ""
if sys.platform == "linux" or sys.platform == "linux2":
	ext = ".so"
elif sys.platform == "darwin":
	ext = ".dylib"
elif sys.platform == "win32":
    ext = ".dll"
else:
	print("Error: Unrecognized Platform.")
	sys.exit(-1)

#find rust library
f = glob.glob("%s/target/*%s"%(path,ext))
if not f:
	print("Error: Failed to find rust library. You may need to run `cargo build` first.")
	sys.exit(-1)

#load the library
demo = cdll.LoadLibrary(f[0])

#create a python string
py_string = "(And Python!)"
#convert to a ctypes c style string (null terminated char array pointer)
c_string = c_char_p(py_string)

#set the result type for the hello_world method to a c style string
demo.hello_world.restype = c_char_p
#set the arguments for hello_world to a c style string
demo.hello_world.argtypes = [c_char_p]

#call hello_world and print the result
print("demo.hello_world():\n\"%s\"" % demo.hello_world(c_string))


print("")


#create class to represent the test structure in the library
class DemoStruct(Structure):
	_fields_ = [("a", c_int), ("b", c_int)]

#set the result type for the test method
demo.get_demo_struct.restype = POINTER(DemoStruct)

#call get_demo_struct and keep the returned pointer
p_demo_struct = demo.get_demo_struct()
#get the struct behind the pointer
demo_struct = p_demo_struct.contents
#print the contents.
print("demo.get_demo_struct():\n{a: %d,b: %d}"%(demo_struct.a, demo_struct.b))

