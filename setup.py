from __future__ import print_function
import os
from os import getenv, path
import subprocess
import shutil

DIRNAME, _ = path.split(path.abspath(__file__))


# use setuptools by default as per the official advice at:
# packaging.python.org/en/latest/current.html#packaging-tool-recommendations
use_setuptools = True
# set the environment variable USE_DISTUTILS=True to force the use of distutils
use_distutils = getenv('USE_DISTUTILS')
if use_distutils is not None:
    if use_distutils.lower() == 'true':
        use_setuptools = False
    else:
        print("Value {} for USE_DISTUTILS treated as False".\
              format(use_distutils))

from distutils.command.build import build as _build

if use_setuptools:
    try:
        from setuptools import setup
        from setuptools.command.install import install as _install
    except ImportError:
        use_setuptools = False

if not use_setuptools:
    from distutils.core import setup
    from distutils.command.install import install as _install

cargo_opts = ["build",  "--release"]


def move_libraries():
    target_path = path.join(DIRNAME, "target/release")
    lib_files = [f for f in os.listdir(target_path) if ".so" in f]
    print("move library files: {}".format(lib_files))
    for f in lib_files:
        shutil.move(path.join(target_path, f), path.join(DIRNAME, "python/gifts/", f))


def cargo_build():
    cargo_cmd = ["cargo"]
    cargo_cmd.extend(cargo_opts)
    print(cargo_cmd)
    if subprocess.call(cargo_cmd) != 0:
        raise EnvironmentError("error calling cargo")
    move_libraries()


class BuildWithcargo(_build):
    _build_opts = _build.user_options
    user_options = [
        ('define=', 'D',
         'cargo <var>:<type>=<value>'),
    ]
    user_options.extend(_build_opts)

    def initialize_options(self):
        _build.initialize_options(self)
        self.define = None

    def finalize_options(self):
        _build.finalize_options(self)
        # The argument parsing will result in self.define being a string, but
        # it has to be a list of 2-tuples.
        # Multiple symbols can be separated with semi-colons.
        if self.define:
            defines = self.define.split(';')
            self.define = [(s.strip(), None) if '=' not in s else
                           tuple(ss.strip() for ss in s.split('='))
                           for s in defines]
            cargo_opts.extend(self.define)

    def run(self):
        cargo_build()
        # can't use super() here because _build is an old style class in 2.7
        _build.run(self)

class InstallWithcargo(_install):
    _install_opts = _install.user_options
    user_options = [
        ('define=', 'D',
         'cargo <var>:<type>=<value>'),
    ]
    user_options.extend(_install_opts)

    def initialize_options(self):
        _install.initialize_options(self)
        self.define = None

    def finalize_options(self):
        _install.finalize_options(self)
        # The argument parsing will result in self.define being a string, but
        # it has to be a list of 2-tuples.
        # Multiple symbols can be separated with semi-colons.
        if self.define:
            defines = self.define.split(';')
            self.define = [(s.strip(), None) if '=' not in s else
                           tuple(ss.strip() for ss in s.split('='))
                           for s in defines]
            cargo_opts.extend(self.define)

    def run(self):
        # can't use super() here because _install is an old style class in 2.7
        _install.run(self)

long_description = '''GIFTS'''

setup(name = "gifts",
      version = "0.1.0",
      description = "The Gloria Imaging Fourier Transform Spectrometer library",
      long_description = "",
      author = "Thomas Latzko",
      author_email = "thomas.latzko@kit.edu",
      license = "MIT",
      url = "https://github.com/thomasl/gifts",
      packages = ["gifts"],
      package_dir = {'gifts': 'python/gifts'},
      package_data= {'' : ['libgifts.so']},
      cmdclass={
          'build' : BuildWithcargo,
          'install' : InstallWithcargo,
          }
  )
