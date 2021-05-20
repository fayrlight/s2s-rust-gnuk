#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type chx_pq;
    pub type chx_thread;
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_create(flags_and_prio: uint32_t, stack_addr: uintptr_t,
                      stack_size: size_t,
                      thread_entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      _: *mut libc::c_void) -> chopstx_t;
    #[no_mangle]
    fn chopstx_usec_wait(usec: uint32_t);
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_mutex_init(mutex: *mut chopstx_mutex_t);
    #[no_mangle]
    fn chopstx_mutex_lock(mutex: *mut chopstx_mutex_t);
    #[no_mangle]
    fn chopstx_mutex_unlock(mutex: *mut chopstx_mutex_t);
    #[no_mangle]
    fn chopstx_join(_: chopstx_t, _: *mut *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn chopstx_setpriority(_: chopstx_prio_t) -> chopstx_prio_t;
    #[no_mangle]
    fn eventflag_init(ev: *mut eventflag);
    #[no_mangle]
    fn eventflag_wait(ev: *mut eventflag) -> eventmask_t;
    #[no_mangle]
    fn eventflag_signal(ev: *mut eventflag, m: eventmask_t);
    #[no_mangle]
    static mut vector: [handler; 16];
    #[no_mangle]
    fn adc_init() -> libc::c_int;
    #[no_mangle]
    static ccid_state_p: *mut ccid_state;
    #[no_mangle]
    static mut auth_status: uint8_t;
    #[no_mangle]
    fn flash_put_data_internal(p: *const uint8_t, hw: uint16_t);
    #[no_mangle]
    static gnuk_string_serial: [uint8_t; 0];
    #[no_mangle]
    static mut _regnual_start: uint8_t;
    #[no_mangle]
    static mut __heap_end__: [uint8_t; 0];
    #[no_mangle]
    fn usb_lld_shutdown();
    #[no_mangle]
    fn random_init();
    #[no_mangle]
    fn random_fini();
    #[no_mangle]
    static mut __process1_stack_base__: [uint8_t; 0];
    #[no_mangle]
    static mut __process1_stack_size__: [uint8_t; 0];
    #[no_mangle]
    fn ccid_thread(arg: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    static mut bDeviceState: uint32_t;
    /*
 * Malloc for Gnuk.
 *
 * Each memory chunk has header with size information.
 * The size of chunk is at least 16.
 *
 * Free memory is managed by FREE_LIST.
 *
 * When it is managed in FREE_LIST, three pointers, ->NEXT, ->PREV,
 * and ->NEIGHBOR is used.  NEXT and PREV is to implement doubly
 * linked list.  NEIGHBOR is to link adjacent memory chunk to be
 * reclaimed to system.
 */
    #[no_mangle]
    static mut __heap_base__: [uint8_t; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
/*
 * chopstx.h - Threads and only threads.
 *
 * Copyright (C) 2013, 2016, 2017, 2018  Flying Stone Technology
 * Author: NIIBE Yutaka <gniibe@fsij.org>
 *
 * This file is a part of Chopstx, a thread library for embedded.
 *
 * Chopstx is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Chopstx is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * As additional permission under GNU GPL version 3 section 7, you may
 * distribute non-source form of the Program without the copy of the
 * GNU GPL normally required by section 4, provided you inform the
 * receipents of GNU GPL by a written offer.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_qh {
    pub next: *mut chx_pq,
    pub prev: *mut chx_pq,
}
pub type chopstx_t = uintptr_t;
pub type chopstx_prio_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_spinlock {
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_mtx {
    pub q: chx_qh,
    pub lock: chx_spinlock,
    pub owner: *mut chx_thread,
    pub list: *mut chx_mtx,
}
/* nothing for uniprocessor.  */
pub type chopstx_mutex_t = chx_mtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_cond {
    pub q: chx_qh,
    pub lock: chx_spinlock,
}
pub type chopstx_cond_t = chx_cond;
pub type eventmask_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eventflag {
    pub flags: eventmask_t,
    pub mask: eventmask_t,
    pub mutex: chopstx_mutex_t,
    pub cond: chopstx_cond_t,
}
pub type handler = Option<unsafe extern "C" fn() -> ()>;
pub type nonreturn_handler1
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> !>;
/* CCID thread */
/* USB Rx data available  */
/* OpenPGP Execution finished */
/* CCID Tx finished  */
/* OpenPGPcard thread */
/* Maximum cmd apdu data is key import 24+4+256+256 (proc_key_import) */
/* without header */
/* Maximum res apdu data is public key 5+9+512 (gpg_do_public_key) */
/* without trailer */
/* USB buffer size of LL (Low-level): size of single Bulk transaction */
pub type ccid_state = libc::c_uint;
/* Exec requested */
/* ICC Thread Terminated */
pub const CCID_STATE_EXEC_REQUESTED: ccid_state = 7;
/* APDU Sent Partially */
pub const CCID_STATE_EXITED: ccid_state = 6;
/* APDU Received Partially */
pub const CCID_STATE_SEND: ccid_state = 5;
/* Busy4 */
pub const CCID_STATE_RECEIVE: ccid_state = 4;
/* Waiting APDU */
/* Busy1, Busy2, Busy3, Busy5 */
pub const CCID_STATE_EXECUTE: ccid_state = 3;
/* Initial */
pub const CCID_STATE_WAIT: ccid_state = 2;
/* No card available */
pub const CCID_STATE_START: ccid_state = 1;
pub const CCID_STATE_NOCARD: ccid_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCB {
    pub CPUID: uint32_t,
    pub ICSR: uint32_t,
    pub VTOR: uint32_t,
    pub AIRCR: uint32_t,
    pub SCR: uint32_t,
    pub CCR: uint32_t,
    pub SHP: [uint8_t; 12],
    pub SHCSR: uint32_t,
    pub CFSR: uint32_t,
    pub HFSR: uint32_t,
    pub DFSR: uint32_t,
    pub MMAR: uint32_t,
    pub BFAR: uint32_t,
    pub AFSR: uint32_t,
    pub PFR: [uint32_t; 2],
    pub DFR: uint32_t,
    pub AFR: uint32_t,
    pub MMFR: [uint32_t; 4],
    pub ISAR: [uint32_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mem_head {
    pub size: uint32_t,
    pub next: *mut mem_head,
    pub prev: *mut mem_head,
    pub neighbor: *mut mem_head,
}
#[inline]
unsafe extern "C" fn unique_device_id() -> *const uint8_t {
    /* STM32F103 has 96-bit unique device identifier */
    let mut addr: *const uint8_t =
        0x1ffff7e8 as libc::c_int as *const uint8_t;
    return addr;
}
#[inline]
unsafe extern "C" fn set_led(mut on: libc::c_int) {
    let mut func: Option<unsafe extern "C" fn(_: libc::c_int) -> ()> =
        ::std::mem::transmute::<handler,
                                Option<unsafe extern "C" fn(_: libc::c_int)
                                           ->
                                               ()>>(vector[2 as libc::c_int as
                                                               usize]);
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(on);
}
#[inline]
unsafe extern "C" fn flash_unlock() {
    Some((*vector.as_mut_ptr().offset(3 as libc::c_int as
                                          isize)).expect("non-null function pointer")).expect("non-null function pointer")();
}
#[inline]
unsafe extern "C" fn flash_erase_all_and_exec(mut entry:
                                                  Option<unsafe extern "C" fn()
                                                             -> ()>) -> ! {
    let mut func: nonreturn_handler1 =
        ::std::mem::transmute::<handler,
                                nonreturn_handler1>(vector[9 as libc::c_int as
                                                               usize]);
    Some(func.expect("non-null function pointer")).expect("non-null function pointer")(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                                                          ->
                                                                                                                              ()>,
                                                                                                               *mut libc::c_void>(entry));
}
// Initialized in run_static_initializers
static mut SCB: *mut SCB = 0 as *const SCB as *mut SCB;
unsafe extern "C" fn device_initialize_once() {
    let mut p: *const uint8_t =
        &*gnuk_string_serial.as_ptr().offset((2 as libc::c_int +
                                                  11 as libc::c_int *
                                                      2 as libc::c_int) as
                                                 isize) as *const uint8_t;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int ==
           0xff as libc::c_int &&
           *p.offset(1 as libc::c_int as isize) as libc::c_int ==
               0xff as libc::c_int &&
           *p.offset(2 as libc::c_int as isize) as libc::c_int ==
               0xff as libc::c_int &&
           *p.offset(3 as libc::c_int as isize) as libc::c_int ==
               0xff as libc::c_int {
        /*
       * This is the first time invocation.
       * Setup serial number by unique device ID.
       */
        let mut u: *const uint8_t =
            unique_device_id().offset(8 as libc::c_int as isize);
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut b: uint8_t = *u.offset((3 as libc::c_int - i) as isize);
            let mut nibble: uint8_t = 0;
            nibble = (b as libc::c_int >> 4 as libc::c_int) as uint8_t;
            nibble =
                (nibble as libc::c_int +
                     if nibble as libc::c_int >= 10 as libc::c_int {
                         ('A' as i32) - 10 as libc::c_int
                     } else { '0' as i32 }) as uint8_t;
            flash_put_data_internal(&*p.offset((i * 4 as libc::c_int) as
                                                   isize),
                                    nibble as uint16_t);
            nibble = (b as libc::c_int & 0xf as libc::c_int) as uint8_t;
            nibble =
                (nibble as libc::c_int +
                     if nibble as libc::c_int >= 10 as libc::c_int {
                         ('A' as i32) - 10 as libc::c_int
                     } else { '0' as i32 }) as uint8_t;
            flash_put_data_internal(&*p.offset((i * 4 as libc::c_int +
                                                    2 as libc::c_int) as
                                                   isize),
                                    nibble as uint16_t);
            i += 1
        }
    };
}
static mut fatal_code: uint8_t = 0;
static mut led_event: eventflag =
    eventflag{flags: 0,
              mask: 0,
              mutex:
                  chopstx_mutex_t{q:
                                      chx_qh{next:
                                                 0 as *const chx_pq as
                                                     *mut chx_pq,
                                             prev:
                                                 0 as *const chx_pq as
                                                     *mut chx_pq,},
                                  lock: chx_spinlock{},
                                  owner:
                                      0 as *const chx_thread as
                                          *mut chx_thread,
                                  list: 0 as *const chx_mtx as *mut chx_mtx,},
              cond:
                  chopstx_cond_t{q:
                                     chx_qh{next:
                                                0 as *const chx_pq as
                                                    *mut chx_pq,
                                            prev:
                                                0 as *const chx_pq as
                                                    *mut chx_pq,},
                                 lock: chx_spinlock{},},};
unsafe extern "C" fn display_fatal_code() {
    loop  {
        set_led(1 as libc::c_int);
        chopstx_usec_wait((25 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(0 as libc::c_int);
        chopstx_usec_wait((75 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(1 as libc::c_int);
        chopstx_usec_wait((25 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(0 as libc::c_int);
        chopstx_usec_wait((75 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(1 as libc::c_int);
        chopstx_usec_wait((25 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(0 as libc::c_int);
        chopstx_usec_wait((200 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(1 as libc::c_int);
        if fatal_code as libc::c_int & 1 as libc::c_int != 0 {
            chopstx_usec_wait((100 as libc::c_int * 1000 as libc::c_int) as
                                  uint32_t);
        } else {
            chopstx_usec_wait((25 as libc::c_int * 1000 as libc::c_int) as
                                  uint32_t);
        }
        set_led(0 as libc::c_int);
        chopstx_usec_wait((75 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(1 as libc::c_int);
        if fatal_code as libc::c_int & 2 as libc::c_int != 0 {
            chopstx_usec_wait((100 as libc::c_int * 1000 as libc::c_int) as
                                  uint32_t);
        } else {
            chopstx_usec_wait((25 as libc::c_int * 1000 as libc::c_int) as
                                  uint32_t);
        }
        set_led(0 as libc::c_int);
        chopstx_usec_wait((75 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(1 as libc::c_int);
        chopstx_usec_wait((200 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
        set_led(0 as libc::c_int);
        chopstx_usec_wait((75 as libc::c_int * 1000 as libc::c_int *
                               10 as libc::c_int) as uint32_t);
    };
}
static mut led_inverted: uint8_t = 0;
unsafe extern "C" fn emit_led(mut on_time: libc::c_int,
                              mut off_time: libc::c_int) {
    set_led((led_inverted == 0) as libc::c_int);
    chopstx_usec_wait(on_time as uint32_t);
    set_led(led_inverted as libc::c_int);
    chopstx_usec_wait(off_time as uint32_t);
}
unsafe extern "C" fn display_status_code() {
    let mut ccid_state: ccid_state = *ccid_state_p;
    if ccid_state as libc::c_uint ==
           CCID_STATE_START as libc::c_int as libc::c_uint {
        emit_led(100 as libc::c_int * 1000 as libc::c_int,
                 200 as libc::c_int * 1000 as libc::c_int);
    } else {
        /* OpenPGP card thread is running */
        emit_led(if auth_status as libc::c_int & 0x4 as libc::c_int != 0 {
                     (100 as libc::c_int) * 1000 as libc::c_int
                 } else { (25 as libc::c_int) * 1000 as libc::c_int },
                 75 as libc::c_int * 1000 as libc::c_int);
        emit_led(if auth_status as libc::c_int & 0x2 as libc::c_int != 0 {
                     (100 as libc::c_int) * 1000 as libc::c_int
                 } else { (25 as libc::c_int) * 1000 as libc::c_int },
                 75 as libc::c_int * 1000 as libc::c_int);
        emit_led(if auth_status as libc::c_int & 0x1 as libc::c_int != 0 {
                     (100 as libc::c_int) * 1000 as libc::c_int
                 } else { (25 as libc::c_int) * 1000 as libc::c_int },
                 75 as libc::c_int * 1000 as libc::c_int);
        if ccid_state as libc::c_uint ==
               CCID_STATE_WAIT as libc::c_int as libc::c_uint {
            chopstx_usec_wait((200 as libc::c_int * 1000 as libc::c_int *
                                   2 as libc::c_int) as uint32_t);
        } else {
            chopstx_usec_wait((75 as libc::c_int * 1000 as libc::c_int) as
                                  uint32_t);
            emit_led(if ccid_state as libc::c_uint ==
                            CCID_STATE_RECEIVE as libc::c_int as libc::c_uint
                        {
                         (100 as libc::c_int) * 1000 as libc::c_int
                     } else { (25 as libc::c_int) * 1000 as libc::c_int },
                     200 as libc::c_int * 1000 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn led_blink(mut spec: libc::c_int) {
    if spec == 64 as libc::c_int || spec == 128 as libc::c_int {
        led_inverted = (spec == 64 as libc::c_int) as libc::c_int as uint8_t;
        spec = 16 as libc::c_int
    }
    eventflag_signal(&mut led_event, spec as eventmask_t);
}
unsafe extern "C" fn calculate_regnual_entry_address(mut addr: *const uint8_t)
 -> uint32_t {
    let mut p: *const uint8_t = addr.offset(4 as libc::c_int as isize);
    let mut v: uint32_t =
        (*p.offset(0 as libc::c_int as isize) as libc::c_int +
             ((*p.offset(1 as libc::c_int as isize) as libc::c_int) <<
                  8 as libc::c_int) +
             ((*p.offset(2 as libc::c_int as isize) as libc::c_int) <<
                  16 as libc::c_int) +
             ((*p.offset(3 as libc::c_int as isize) as libc::c_int) <<
                  24 as libc::c_int)) as uint32_t;
    v =
        (v as
             libc::c_uint).wrapping_sub(0x20001400 as libc::c_int as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    v =
        (v as libc::c_uint).wrapping_add(addr as uint32_t) as uint32_t as
            uint32_t;
    return v;
}
/*
 * Entry point.
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut entry: uint32_t = 0;
    let mut ccid_thd: chopstx_t = 0;
    gnuk_malloc_init();
    flash_unlock();
    device_initialize_once();
    adc_init();
    eventflag_init(&mut led_event);
    random_init();
    ccid_thd =
        chopstx_create(3 as libc::c_int as uint32_t,
                       __process1_stack_base__.as_mut_ptr() as uint32_t as
                           uintptr_t,
                       __process1_stack_size__.as_mut_ptr() as uint32_t as
                           size_t,
                       Some(ccid_thread as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> *mut libc::c_void),
                       0 as *mut libc::c_void);
    chopstx_setpriority(5 as libc::c_int as chopstx_prio_t);
    while !(bDeviceState != 0 as libc::c_int as libc::c_uint) {
        chopstx_usec_wait((250 as libc::c_int * 1000 as libc::c_int) as
                              uint32_t);
    }
    loop  {
        let mut m: eventmask_t = 0;
        m = eventflag_wait(&mut led_event);
        match m {
            1 => {
                emit_led(100 as libc::c_int * 1000 as libc::c_int,
                         200 as libc::c_int * 1000 as libc::c_int);
            }
            2 => {
                emit_led(50 as libc::c_int * 1000 as libc::c_int,
                         50 as libc::c_int * 1000 as libc::c_int);
                emit_led(50 as libc::c_int * 1000 as libc::c_int,
                         200 as libc::c_int * 1000 as libc::c_int);
            }
            4 => { display_status_code(); }
            8 => { display_fatal_code(); }
            16 => { set_led(led_inverted as libc::c_int); }
            32 => { break ; }
            _ => {
                emit_led(25 as libc::c_int * 1000 as libc::c_int,
                         200 as libc::c_int * 1000 as libc::c_int);
            }
        }
    }
    random_fini();
    set_led(1 as libc::c_int);
    usb_lld_shutdown();
    /* Finish application.  */
    chopstx_join(ccid_thd, 0 as *mut *mut libc::c_void);
    /* Set vector */
    ::std::ptr::write_volatile(&mut (*SCB).VTOR as *mut uint32_t,
                               &mut _regnual_start as *mut uint8_t as
                                   uint32_t);
    entry = calculate_regnual_entry_address(&mut _regnual_start);
    /* Leave Gnuk to exec reGNUal */
    flash_erase_all_and_exec(::std::mem::transmute::<libc::intptr_t,
                                                     Option<unsafe extern "C" fn()
                                                                ->
                                                                    ()>>(entry
                                                                             as
                                                                             libc::intptr_t));
}
#[no_mangle]
pub unsafe extern "C" fn fatal(mut code: uint8_t) -> ! {
    extern "C" {
        #[link_name = "_write"]
        fn _write_0(s: *const libc::c_char, len: libc::c_int);
    }
    ::std::ptr::write_volatile(&mut fatal_code as *mut uint8_t, code);
    eventflag_signal(&mut led_event, 8 as libc::c_int as eventmask_t);
    _write_0(b"fatal\r\n\x00" as *const u8 as *const libc::c_char,
             7 as libc::c_int);
    loop  { };
}
static mut heap_p: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut malloc_mtx: chopstx_mutex_t =
    chopstx_mutex_t{q:
                        chx_qh{next: 0 as *const chx_pq as *mut chx_pq,
                               prev: 0 as *const chx_pq as *mut chx_pq,},
                    lock: chx_spinlock{},
                    owner: 0 as *const chx_thread as *mut chx_thread,
                    list: 0 as *const chx_mtx as *mut chx_mtx,};
/* backlink to neighbor */
static mut free_list: *mut mem_head = 0 as *const mem_head as *mut mem_head;
unsafe extern "C" fn gnuk_malloc_init() {
    chopstx_mutex_init(&mut malloc_mtx);
    heap_p = __heap_base__.as_mut_ptr();
    free_list = 0 as *mut mem_head;
}
unsafe extern "C" fn sbrk(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = heap_p as *mut libc::c_void;
    if (__heap_end__.as_mut_ptr().wrapping_offset_from(heap_p) as libc::c_long
            as size_t) < size {
        return 0 as *mut libc::c_void
    }
    heap_p = heap_p.offset(size as isize);
    return p;
}
unsafe extern "C" fn remove_from_free_list(mut m: *mut mem_head) {
    if !(*m).prev.is_null() {
        (*(*m).prev).next = (*m).next
    } else { free_list = (*m).next }
    if !(*m).next.is_null() { (*(*m).next).prev = (*m).prev };
}
#[no_mangle]
pub unsafe extern "C" fn gnuk_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut m: *mut mem_head = 0 as *mut mem_head;
    let mut m0: *mut mem_head = 0 as *mut mem_head;
    size =
        size.wrapping_add(::std::mem::size_of::<uint32_t>() as
                              libc::c_ulong).wrapping_add(16 as libc::c_int as
                                                              libc::c_ulong).wrapping_sub(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)
            & !(16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    chopstx_mutex_lock(&mut malloc_mtx);
    m = free_list;
    loop  {
        if m.is_null() {
            m = sbrk(size) as *mut mem_head;
            if !m.is_null() { (*m).size = size as uint32_t }
            break ;
        } else if (*m).size as libc::c_ulong == size {
            remove_from_free_list(m);
            m0 = free_list;
            while !m0.is_null() {
                if (*m0).neighbor == m {
                    (*m0).neighbor = 0 as *mut mem_head
                } else { m0 = (*m0).next }
            }
            break ;
        } else { m = (*m).next }
    }
    chopstx_mutex_unlock(&mut malloc_mtx);
    if m.is_null() {
        return m as *mut libc::c_void
    } else {
        return (m as
                    *mut libc::c_void).offset(::std::mem::size_of::<uint32_t>()
                                                  as libc::c_ulong as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn gnuk_free(mut p: *mut libc::c_void) {
    let mut m: *mut mem_head =
        p.offset(-(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize)) as *mut mem_head;
    let mut m0: *mut mem_head = 0 as *mut mem_head;
    chopstx_mutex_lock(&mut malloc_mtx);
    m0 = free_list;
    (*m).neighbor = 0 as *mut mem_head;
    while !m0.is_null() {
        if (m as *mut libc::c_void).offset((*m).size as isize) ==
               m0 as *mut libc::c_void {
            (*m0).neighbor = m
        } else if (m0 as *mut libc::c_void).offset((*m0).size as isize) ==
                      m as *mut libc::c_void {
            (*m).neighbor = m0
        }
        m0 = (*m0).next
    }
    if (m as *mut libc::c_void).offset((*m).size as isize) ==
           heap_p as *mut libc::c_void {
        let mut mn: *mut mem_head = (*m).neighbor;
        heap_p = heap_p.offset(-((*m).size as isize));
        while !mn.is_null() {
            heap_p = heap_p.offset(-((*mn).size as isize));
            remove_from_free_list(mn);
            mn = (*mn).neighbor
        }
    } else {
        (*m).next = free_list;
        (*m).prev = 0 as *mut mem_head;
        if !free_list.is_null() { (*free_list).prev = m }
        free_list = m
    }
    chopstx_mutex_unlock(&mut malloc_mtx);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    SCB =
        (0xe000e000 as
             libc::c_uint).wrapping_add(0xd00 as libc::c_int as libc::c_uint)
            as *mut SCB
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
