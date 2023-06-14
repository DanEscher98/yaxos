#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![test_runner(yaxos::test_runner)]
#![reexport_test_harness_main = "test_main"]


extern crate alloc;
use core::panic::PanicInfo;


