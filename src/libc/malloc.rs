/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `malloc.h` memory management zones

use std::{cell::OnceCell, collections::HashMap};

use crate::{
    dyld::FunctionExports,
    environment::Environment,
    export_c_func,
    mem::{AllocatorID, ConstPtr, GuestUSize, MutPtr, MutVoidPtr, Ptr, SafeRead},
};

#[derive(Default)]
pub struct MallocZones {
    default_zone: OnceCell<MutPtr<malloc_zone_t>>,
    zone_to_allocator: HashMap<MutPtr<malloc_zone_t>, AllocatorID>,
}

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
    if env.malloc_zones.default_zone.get().is_none() {
        let zone = env.mem.alloc_and_write(malloc_zone_t::new());
        env.malloc_zones.default_zone.set(zone).unwrap();
        assert!(env
            .malloc_zones
            .zone_to_allocator
            .insert(zone, env.mem.get_default_allocator())
            .is_none());
    }

    *env.malloc_zones.default_zone.get().unwrap()
}

fn malloc_create_zone(
    env: &mut Environment,
    start_size: GuestUSize,
    _flags: u32,
) -> MutPtr<malloc_zone_t> {
    let zone = env.mem.alloc_and_write(malloc_zone_t::new());
    let allocator = env.mem.create_allocator(start_size);
    assert!(env
        .malloc_zones
        .zone_to_allocator
        .insert(zone, allocator)
        .is_none());
    zone
}

fn malloc_destroy_zone(env: &mut Environment, zone: MutPtr<malloc_zone_t>) {
    env.mem.free(zone.cast());
    let allocator = env
        .malloc_zones
        .zone_to_allocator
        .remove(&zone)
        .expect("Zone {zone:?} does not map to an allocator");
    env.mem.destroy_allocator(allocator);
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(malloc_default_zone()),
    export_c_func!(malloc_create_zone(_, _)),
    export_c_func!(malloc_destroy_zone(_)),
];
