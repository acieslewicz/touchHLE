/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `mach_vm.h` and some other related functions

#![allow(non_camel_case_types)]

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::mem::{MutPtr, Ptr};

use crate::libc::mach::kern_return::{kern_return_t, KERN_SUCCESS};
use crate::libc::mach::port::mach_port_t;
use crate::libc::mach::types::{boolean_t, natural_t};

pub type vm_size_t = natural_t;
pub type vm_task_t = mach_port_t;
pub type vm_offset_t = u32;
pub type vm_address_t = vm_offset_t;
pub type vm_purgable_t = i32;

pub const PAGE_SIZE: vm_size_t = 4096;

pub fn vm_allocate(
    env: &mut Environment,
    _target_task: vm_task_t,
    address: MutPtr<vm_address_t>,
    size: vm_size_t,
    _anywhere: boolean_t,
) -> kern_return_t {
    // TODO: Error handling
    let allocated_address = env.mem.alloc(size);
    env.mem.write(address, allocated_address.to_bits());
    KERN_SUCCESS
}

pub fn vm_deallocate(
    env: &mut Environment,
    _target_task: vm_task_t,
    address: vm_address_t,
    size: vm_size_t,
) -> kern_return_t {
    let allocated_size = env.mem.malloc_size(Ptr::from_bits(address));

    if allocated_size != size {
        unimplemented!("Partial vm_deallocate not supported.")
    }

    env.mem.free(Ptr::from_bits(address));
    KERN_SUCCESS
}

pub fn vm_purgable_control(
    _env: &mut Environment,
    target_task: vm_task_t,
    address: vm_address_t,
    control: vm_purgable_t,
    _state: MutPtr<i32>,
) -> kern_return_t {
    log_dbg!(
        "TODO: Ignoring vm_purgable_control(task={}, addr=0x{:x}, control={})",
        target_task,
        address,
        control
    );
    KERN_SUCCESS
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(vm_allocate(_, _, _, _)),
    export_c_func!(vm_deallocate(_, _, _)),
    export_c_func!(vm_purgable_control(_, _, _, _)),
];
