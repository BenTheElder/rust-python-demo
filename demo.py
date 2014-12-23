import os
import sys
import glob
from ctypes import *

path = os.path.dirname(os.path.realpath(__file__))

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

f = glob.glob("%s/target/*%s"%(path,ext))
if not f:
	print("Error: Failed to find rust library. You may need to run `cargo build` first.")
	sys.exit(-1)

demo = cdll.LoadLibrary(f[0])

pystr = "(And Python!)"
c_s = c_char_p(pystr)

hello_world = demo.hello_world
hello_world.restype = c_char_p
hello_world.argtypes = [c_char_p]
print("%s" % hello_world(c_s))
