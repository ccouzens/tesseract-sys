# tesseract-sys

Rust bindings for [Tesseract](https://github.com/tesseract-ocr/tesseract)

## Building

This links to the C libraries [leptonica](https://github.com/danbloomberg/leptonica) and tesseract.

On Ubuntu and derivatives the additional dependencies can be installed by running:

```bash
sudo apt-get install libleptonica-dev libtesseract-dev clang
```

On Fedora 30 the additional dependencies can be installed by running:

```bash
sudo dnf install leptonica-devel tesseract-devel clang
```

On Termux 2019 (Android, Android on Chromebooks) the additional dependencies can be installed by running:

```bash
pkg install libclang leptonica-dev tesseract-dev
```

### Building on Windows

On Windows, this library uses Microsoft's [vcpkg](https://github.com/microsoft/vcpkg) to provide tesseract.

Please install [vcpkg](https://github.com/microsoft/vcpkg) and **set up user wide integration** or [vcpkg crate](https://crates.io/crates/vcpkg) won't be able to find a library.
By default vcpkg installs 32 bit libraries. If you need 64 bit libraries then set following environment variable

To install tesseract

```cmd
REM from the vcpkg directory

REM 32 bit
.\vcpkg install tesseract:x86-windows

REM 64 bit
.\vcpkg install tesseract:x64-windows
```

vcpkg allows building either dynamically or statically linked application

if you prefer dynamic linking

```cmd
SET VCPKGRS_DYNAMIC=true
```

for statically linked libraries

```cmd
SET RUSTFLAGS=-Ctarget-feature=+crt-static
```

To run the tests please download the [English trained data](https://github.com/tesseract-ocr/tessdata/blob/master/eng.traineddata) to this directory and set

```cmd
SET TESSDATA_PREFIX=.
```
