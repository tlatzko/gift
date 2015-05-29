import ctypes
import os
dirname, filename = os.path.split(os.path.abspath(__file__))
lib_path = os.path.join(dirname, "libgifts.so")
print(lib_path)
lib = ctypes.CDLL(lib_path)


def test_rust():
    lib.run_pipe();
