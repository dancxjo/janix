#![no_std]
#![no_main]

// Thin binary wrapper; all real code (including `_start`) lives in the library crate.
extern crate kernel_aarch64;
