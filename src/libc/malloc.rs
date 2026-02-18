/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `malloc.h` memory management zones

use crate::{
    dyld::FunctionExports,
    environment::Environment,
    export_c_func,
    mem::{ConstPtr, GuestUSize, MutPtr, MutVoidPtr, Ptr, SafeRead},
};

#[repr(C, packed)]
#[allow(non_camel_case_types)]
pub struct malloc_zone_t {
    reserved1: MutVoidPtr,
    reserved2: MutVoidPtr,
    size: MutVoidPtr,
    malloc: MutVoidPtr,
    calloc: MutVoidPtr,
    valloc: MutVoidPtr,
    free: MutVoidPtr,
    realloc: MutVoidPtr,
    destroy: MutVoidPtr,
    zone_name: ConstPtr<u8>,
    batch_malloc: MutVoidPtr,
    batch_free: MutVoidPtr,
    introspect: MutVoidPtr,
    version: u32,
    memalign: MutVoidPtr,
}
unsafe impl SafeRead for malloc_zone_t {}

impl malloc_zone_t {
    pub fn new() -> malloc_zone_t {
        malloc_zone_t {
            reserved1: Ptr::null(),
            reserved2: Ptr::null(),
            size: Ptr::null(),
            malloc: Ptr::null(),
            calloc: Ptr::null(),
            valloc: Ptr::null(),
            free: Ptr::null(),
            realloc: Ptr::null(),
            destroy: Ptr::null(),
            zone_name: Ptr::null(),
            batch_malloc: Ptr::null(),
            batch_free: Ptr::null(),
            introspect: Ptr::null(),
            version: 0,
            memalign: Ptr::null(),
        }
    }
}

fn malloc_default_zone(env: &mut Environment) -> MutPtr<malloc_zone_t> {
    env.mem.get_default_zone()
}

fn malloc_create_zone(
    env: &mut Environment,
    start_size: GuestUSize,
    _flags: u32,
) -> MutPtr<malloc_zone_t> {
    env.mem.create_zone(start_size)
}

fn malloc_destroy_zone(env: &mut Environment, zone: MutPtr<malloc_zone_t>) {
    env.mem.destroy_zone(zone);
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(malloc_default_zone()),
    export_c_func!(malloc_create_zone(_, _)),
    export_c_func!(malloc_destroy_zone(_)),
];
