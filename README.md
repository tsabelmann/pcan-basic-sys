# pcan-basic-sys

[![crates](https://img.shields.io/crates/v/pcan-basic-sys.svg)](https://crates.io/crates/pcan-basic-sys)
[![Documentation](https://img.shields.io/docsrs/pcan-basic-sys.svg)](https://docs.rs/pcan-basic-sys)
[![Crate License](https://img.shields.io/crates/l/pcan-basic-sys.svg)](https://crates.io/crates/pcan-basic-sys)
[![Dependency Status](https://deps.rs/repo/github/tsabelmann/pcan-basic-sys/status.svg)](https://deps.rs/repo/github/tsabelmann/pcan-basic-sys)

Rust bindings for version `V4.6.0.600` of the [PCAN-Basic API](https://www.peak-system.com/PCAN-Basic.239.0.html) provided by the [PEAK-System Technik GmbH](https://www.peak-system.com/).

## Installation

### Windows

- Install the [PCAN-Basic API](https://www.peak-system.com/quick/DrvSetup) driver
- Install Rust
- Install a Rust Windows toolchain. Install at least one of the following toolchains: 
    * `rustup toolchain install stable-i686-pc-windows-gnu`
    * `rustup toolchain install stable-x86_64-pc-windows-gnu`
    * `rustup toolchain install stable-i686-pc-windows-msvc`
    * `rustup toolchain install stable-x86_64-pc-windows-msvc`
- Use `pcan-basic-sys` as a dependency

If you choose `MSVC` you need to install the **Visual Studio** toolchain.

### Linux

- Install the [PCAN-Basic API](http://www.peak-system.com/fileadmin/media/linux/files/peak-linux-driver-8.14.0.tar.gz) driver
- Install Rust
- Install a Rust Linux toolchaib. Install at least on of the following toolchains:
  - `rustup toolchain install stable-i686-unknown-linux-gnu`
  - `rustup toolchain install stable-x86_64-unknown-linux-gnu`
- Use `pcan-basic-sys` as a dependency



## License / Terms of Usage

The source code of this project is licensed under the MIT/Apache-2.0 license. This implies that you are free to use, share, and adapt it. However, please give appropriate credit by citing the project.

> Please read the [End User License Agreement](https://www.peak-system.com/quick/eula) of the 
> company PEAK-System Technik GmbH at:
> 
> PEAK-System Technik GmbH grants the right to the customer to use the files in
> this software package as long as this is done in connection with original
> hardware by PEAK-System or OEM hardware coming from PEAK-System. It is NOT
> allowed to use any of these files (even not parts) with third-party hardware.
> 
> If you are not sure whether you have acquired an appropriate license with the
> used hardware, please contact our technical support team (support@peak-system.com).

## Contact

If you have problems using the software, find mistakes, or have general questions please use the [issue tracker](https://github.com/tsabelmann/pcan-basic-sys/issues) to contact us.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT/Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contributors

* [Christopher Woodall](https://github.com/cwoodall)
* [Tim Lucas Sabelmann](https://github.com/tsabelmann)