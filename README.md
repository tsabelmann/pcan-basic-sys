# pcan-basic-sys

[![crates](https://img.shields.io/crates/v/pcan-basic-sys.svg)](https://crates.io/crates/pcan-basic-sys)
[![Documentation](https://img.shields.io/docsrs/pcan-basic-sys.svg)](https://docs.rs/pcan-basic-sys)
[![Crate License](https://img.shields.io/crates/l/pcan-basic-sys.svg)](https://crates.io/crates/pcan-basic-sys)
[![Dependency Status](https://deps.rs/repo/github/tsabelmann/pcan-basic-sys/status.svg)](https://deps.rs/repo/github/tsabelmann/pcan-basic-sys)

Rust bindings for version `V4.6.0.600` of the [PCAN-Basic API](https://www.peak-system.com/PCAN-Basic.239.0.html) provided by [PEAK-System Technik GmbH](https://www.peak-system.com/).

## Installation

**Disclaimer**:
> pcan-basic-sys is a Rust crate targeting the Windows platform. Other platforms, i.e., macOS and Linux are not supported.

### Windows

- Install the [PCAN-Basic API](https://www.peak-system.com/quick/DrvSetup)
- Install Rust
- Install a Rust Windows toolchain. Install at least one of the following toolchains: 
    * `rustup add toolchain i686-pc-windows-gnu`
    * `rustup add toolchain x86_64-pc-windows-gnu`
    * `rustup add toolchain i686-pc-windows-msvc`
    * `rustup add toolchain x86_64-pc-windows-msvc`
- Use `pcan-basic-sys` as a dependency

If you choose `MSVC` you need to install the **Visual Studio** toolchain.

<!-- ## [Documentation]

[Documentation]: https://docs.rs/pcan-basic-sys
[PEAK-System Technik GmbH]: http://www.peak-system.com
[PCAN-Basic API]: http://www.peak-system.com/fileadmin/media/files/pcan-basic.zip -->

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