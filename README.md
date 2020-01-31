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

```cmd
SET VCPKG_DEFAULT_TRIPLET=x64-windows
```
To install tesseract

```cmd
REM from the vcpkg directory
.\vcpkg install tesseract
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