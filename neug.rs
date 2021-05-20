#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type chx_pq;
    pub type chx_thread;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_create(flags_and_prio: uint32_t, stack_addr: uintptr_t,
                      stack_size: size_t,
                      thread_entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      _: *mut libc::c_void) -> chopstx_t;
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_mutex_init(mutex: *mut chopstx_mutex_t);
    #[no_mangle]
    fn chopstx_mutex_lock(mutex: *mut chopstx_mutex_t);
    #[no_mangle]
    fn chopstx_mutex_unlock(mutex: *mut chopstx_mutex_t);
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_cond_init(cond: *mut chopstx_cond_t);
    #[no_mangle]
    fn chopstx_cond_wait(cond: *mut chopstx_cond_t,
                         mutex: *mut chopstx_mutex_t);
    #[no_mangle]
    fn chopstx_cond_signal(cond: *mut chopstx_cond_t);
    #[no_mangle]
    fn chopstx_join(_: chopstx_t, _: *mut *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn adc_start();
    #[no_mangle]
    fn adc_stop();
    #[no_mangle]
    static mut adc_buf: [uint32_t; 64];
    #[no_mangle]
    fn adc_start_conversion(offset: libc::c_int, count: libc::c_int);
    #[no_mangle]
    fn adc_wait_completion() -> libc::c_int;
    #[no_mangle]
    fn sha256_start(ctx: *mut sha256_context);
    #[no_mangle]
    fn sha256_finish(ctx: *mut sha256_context, output: *mut libc::c_uchar);
    #[no_mangle]
    fn sha256_process(ctx: *mut sha256_context);
    #[no_mangle]
    static mut __process2_stack_base__: [uint8_t; 0];
    #[no_mangle]
    static mut __process2_stack_size__: [uint8_t; 0];
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
/*
 * Ring buffer, filled by generator, consumed by neug_get routine.
 */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct rng_rb {
    pub buf: *mut uint32_t,
    pub m: chopstx_mutex_t,
    pub data_available: chopstx_cond_t,
    pub space_available: chopstx_cond_t,
    pub head: uint8_t,
    pub tail: uint8_t,
    pub size: uint8_t,
    #[bitfield(name = "full", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "empty", ty = "libc::c_uint", bits = "1..=1")]
    pub full_empty: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_context {
    pub total: [uint32_t; 2],
    pub state: [uint32_t; 8],
    pub wbuf: [uint32_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CRC {
    pub DR: uint32_t,
    pub IDR: uint8_t,
    pub RESERVED0: uint8_t,
    pub RESERVED1: uint16_t,
    pub CR: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RCC {
    pub CR: uint32_t,
    pub CFGR: uint32_t,
    pub CIR: uint32_t,
    pub APB2RSTR: uint32_t,
    pub APB1RSTR: uint32_t,
    pub AHBENR: uint32_t,
    pub APB2ENR: uint32_t,
    pub APB1ENR: uint32_t,
    pub BDCR: uint32_t,
    pub CSR: uint32_t,
}
#[inline]
unsafe extern "C" fn unique_device_id() -> *const uint8_t {
    /* STM32F103 has 96-bit unique device identifier */
    let mut addr: *const uint8_t =
        0x1ffff7e8 as libc::c_int as *const uint8_t;
    return addr;
}
static mut CRC: *mut CRC =
    unsafe {
        (0x40000000 as libc::c_int + 0x20000 as libc::c_int +
             0x3000 as libc::c_int) as *mut CRC
    };
static mut RCC: *mut RCC =
    unsafe {
        (0x40000000 as libc::c_int + 0x20000 as libc::c_int +
             0x1000 as libc::c_int) as *mut RCC
    };
/*
 * neug.c - true random number generation
 *
 * Copyright (C) 2011, 2012, 2013, 2016
 *               Free Software Initiative of Japan
 * Author: NIIBE Yutaka <gniibe@fsij.org>
 *
 * This file is a part of NeuG, a True Random Number Generator
 * implementation based on quantization error of ADC (for STM32F103).
 *
 * NeuG is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * NeuG is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public
 * License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
static mut mode_mtx: chopstx_mutex_t =
    chopstx_mutex_t{q:
                        chx_qh{next: 0 as *const chx_pq as *mut chx_pq,
                               prev: 0 as *const chx_pq as *mut chx_pq,},
                    lock: chx_spinlock{},
                    owner: 0 as *const chx_thread as *mut chx_thread,
                    list: 0 as *const chx_mtx as *mut chx_mtx,};
static mut mode_cond: chopstx_cond_t =
    chopstx_cond_t{q:
                       chx_qh{next: 0 as *const chx_pq as *mut chx_pq,
                              prev: 0 as *const chx_pq as *mut chx_pq,},
                   lock: chx_spinlock{},};
static mut sha256_ctx_data: sha256_context =
    sha256_context{total: [0; 2], state: [0; 8], wbuf: [0; 16],};
static mut sha256_output: [uint32_t; 8] = [0; 8];
static mut ep_round: uint8_t = 0;
/*
 * Hash_df initial string:
 *
 *  Initial five bytes are:
 *    1,          : counter = 1
 *    0, 0, 1, 0  : no_of_bits_returned (in big endian)
 *
 *  Then, three-byte from noise source follows.
 *
 *  One-byte was used in the previous turn, and we have three bytes in
 *  CRC->DR.
 */
unsafe extern "C" fn ep_fill_initial_string() {
    let mut v: uint32_t = (*CRC).DR;
    let mut b1: uint8_t = 0;
    let mut b2: uint8_t = 0;
    let mut b3: uint8_t = 0;
    b3 = (v >> 24 as libc::c_int) as uint8_t;
    b2 = (v >> 16 as libc::c_int) as uint8_t;
    b1 = (v >> 8 as libc::c_int) as uint8_t;
    noise_source_continuous_test(b1);
    noise_source_continuous_test(b2);
    noise_source_continuous_test(b3);
    adc_buf[0 as libc::c_int as usize] = 0x1000001 as libc::c_int as uint32_t;
    adc_buf[1 as libc::c_int as usize] = v & 0xffffff00 as libc::c_uint;
}
unsafe extern "C" fn ep_init(mut mode: libc::c_int) {
    if mode == 1 as libc::c_int {
        ep_round = 3 as libc::c_int as uint8_t;
        adc_start_conversion(0 as libc::c_int, 32 as libc::c_int);
    } else if mode == 2 as libc::c_int {
        ep_round = 4 as libc::c_int as uint8_t;
        adc_start_conversion(0 as libc::c_int,
                             32 as libc::c_int / 4 as libc::c_int);
    } else {
        ep_round = 0 as libc::c_int as uint8_t;
        ep_fill_initial_string();
        adc_start_conversion(2 as libc::c_int, 56 as libc::c_int);
    };
}
unsafe extern "C" fn ep_fill_wbuf_v(mut i: libc::c_int, mut test: libc::c_int,
                                    mut v: uint32_t) {
    if test != 0 {
        let mut b0: uint8_t = 0;
        let mut b1: uint8_t = 0;
        let mut b2: uint8_t = 0;
        let mut b3: uint8_t = 0;
        b3 = (v >> 24 as libc::c_int) as uint8_t;
        b2 = (v >> 16 as libc::c_int) as uint8_t;
        b1 = (v >> 8 as libc::c_int) as uint8_t;
        b0 = v as uint8_t;
        noise_source_continuous_test_word(b0, b1, b2, b3);
    }
    sha256_ctx_data.wbuf[i as usize] = v;
}
/* Here, we assume a little endian architecture.  */
unsafe extern "C" fn ep_process(mut mode: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0; /* First byte of CRC->DR is used here.  */
    let mut n: libc::c_int =
        0; /* The rest three-byte of
					  CRC->DR is used here.  */
    let mut v: uint32_t = 0;
    if ep_round as libc::c_int == 0 as libc::c_int {
        sha256_start(&mut sha256_ctx_data);
        sha256_ctx_data.wbuf[0 as libc::c_int as usize] =
            adc_buf[0 as libc::c_int as usize];
        sha256_ctx_data.wbuf[1 as libc::c_int as usize] =
            adc_buf[1 as libc::c_int as usize];
        i = 0 as libc::c_int;
        while i < 56 as libc::c_int / 4 as libc::c_int {
            ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                       adc_buf[(i * 4 as libc::c_int +
                                                    2 as libc::c_int) as
                                                   usize]);
            ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                       adc_buf[(i * 4 as libc::c_int +
                                                    3 as libc::c_int) as
                                                   usize]);
            ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                       adc_buf[(i * 4 as libc::c_int +
                                                    4 as libc::c_int) as
                                                   usize]);
            ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                       adc_buf[(i * 4 as libc::c_int +
                                                    5 as libc::c_int) as
                                                   usize]);
            v = (*CRC).DR;
            ep_fill_wbuf_v(i + 2 as libc::c_int, 1 as libc::c_int, v);
            i += 1
        }
        adc_start_conversion(0 as libc::c_int, 64 as libc::c_int);
        sha256_process(&mut sha256_ctx_data);
        ep_round = ep_round.wrapping_add(1);
        return 0 as libc::c_int
    } else {
        if ep_round as libc::c_int == 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 64 as libc::c_int / 4 as libc::c_int {
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int) as
                                                       usize]);
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int +
                                                        1 as libc::c_int) as
                                                       usize]);
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int +
                                                        2 as libc::c_int) as
                                                       usize]);
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int +
                                                        3 as libc::c_int) as
                                                       usize]);
                v = (*CRC).DR;
                ep_fill_wbuf_v(i, 1 as libc::c_int, v);
                i += 1
            }
            adc_start_conversion(0 as libc::c_int,
                                 17 as libc::c_int + 3 as libc::c_int);
            sha256_process(&mut sha256_ctx_data);
            ep_round = ep_round.wrapping_add(1);
            return 0 as libc::c_int
        } else {
            if ep_round as libc::c_int == 2 as libc::c_int {
                i = 0 as libc::c_int;
                while i < 17 as libc::c_int / 4 as libc::c_int {
                    ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                   *mut uint32_t,
                                               adc_buf[(i * 4 as libc::c_int)
                                                           as usize]);
                    ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                   *mut uint32_t,
                                               adc_buf[(i * 4 as libc::c_int +
                                                            1 as libc::c_int)
                                                           as usize]);
                    ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                   *mut uint32_t,
                                               adc_buf[(i * 4 as libc::c_int +
                                                            2 as libc::c_int)
                                                           as usize]);
                    ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                   *mut uint32_t,
                                               adc_buf[(i * 4 as libc::c_int +
                                                            3 as libc::c_int)
                                                           as usize]);
                    v = (*CRC).DR;
                    ep_fill_wbuf_v(i, 1 as libc::c_int, v);
                    i += 1
                }
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int) as
                                                       usize]);
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int +
                                                        1 as libc::c_int) as
                                                       usize]);
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int +
                                                        2 as libc::c_int) as
                                                       usize]);
                ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t,
                                           adc_buf[(i * 4 as libc::c_int +
                                                        3 as libc::c_int) as
                                                       usize]);
                v = (*CRC).DR & 0xff as libc::c_int as libc::c_uint;
                noise_source_continuous_test(v as uint8_t);
                sha256_ctx_data.wbuf[i as usize] = v;
                ep_init(0 as libc::c_int);
                n = 32 as libc::c_int / 2 as libc::c_int;
                memcpy((sha256_ctx_data.wbuf.as_mut_ptr() as
                            *mut uint8_t).offset(17 as libc::c_int as isize)
                           as *mut libc::c_void,
                       sha256_output.as_mut_ptr() as *const libc::c_void,
                       n as libc::c_ulong);
                sha256_ctx_data.total[0 as libc::c_int as usize] =
                    (5 as libc::c_int + 140 as libc::c_int + n) as uint32_t;
                sha256_finish(&mut sha256_ctx_data,
                              sha256_output.as_mut_ptr() as *mut uint8_t);
                return (32 as libc::c_int as
                            libc::c_ulong).wrapping_div(::std::mem::size_of::<uint32_t>()
                                                            as libc::c_ulong)
                           as libc::c_int
            } else {
                if ep_round as libc::c_int == 3 as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int / 4 as libc::c_int {
                        ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                       *mut uint32_t,
                                                   adc_buf[(i *
                                                                4 as
                                                                    libc::c_int)
                                                               as usize]);
                        ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                       *mut uint32_t,
                                                   adc_buf[(i *
                                                                4 as
                                                                    libc::c_int
                                                                +
                                                                1 as
                                                                    libc::c_int)
                                                               as usize]);
                        ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                       *mut uint32_t,
                                                   adc_buf[(i *
                                                                4 as
                                                                    libc::c_int
                                                                +
                                                                2 as
                                                                    libc::c_int)
                                                               as usize]);
                        ::std::ptr::write_volatile(&mut (*CRC).DR as
                                                       *mut uint32_t,
                                                   adc_buf[(i *
                                                                4 as
                                                                    libc::c_int
                                                                +
                                                                3 as
                                                                    libc::c_int)
                                                               as usize]);
                        v = (*CRC).DR;
                        ep_fill_wbuf_v(i, 1 as libc::c_int, v);
                        i += 1
                    }
                    ep_init(mode);
                    return 32 as libc::c_int / 4 as libc::c_int
                } else {
                    if ep_round as libc::c_int == 4 as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < 32 as libc::c_int / 4 as libc::c_int {
                            v = adc_buf[i as usize];
                            ep_fill_wbuf_v(i, 0 as libc::c_int, v);
                            i += 1
                        }
                        ep_init(mode);
                        return 32 as libc::c_int / 4 as libc::c_int
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ep_output(mut mode: libc::c_int) -> *const uint32_t {
    if mode != 0 {
        return sha256_ctx_data.wbuf.as_mut_ptr()
    } else { return sha256_output.as_mut_ptr() };
}
#[no_mangle]
pub static mut neug_err_state: uint8_t = 0;
#[no_mangle]
pub static mut neug_err_cnt: uint16_t = 0;
#[no_mangle]
pub static mut neug_err_cnt_rc: uint16_t = 0;
#[no_mangle]
pub static mut neug_err_cnt_p64: uint16_t = 0;
#[no_mangle]
pub static mut neug_err_cnt_p4k: uint16_t = 0;
#[no_mangle]
pub static mut neug_rc_max: uint16_t = 0;
#[no_mangle]
pub static mut neug_p64_max: uint16_t = 0;
#[no_mangle]
pub static mut neug_p4k_max: uint16_t = 0;
unsafe extern "C" fn noise_source_cnt_max_reset() {
    neug_err_cnt_p4k = 0 as libc::c_int as uint16_t;
    neug_err_cnt_p64 = neug_err_cnt_p4k;
    neug_err_cnt_rc = neug_err_cnt_p64;
    neug_err_cnt = neug_err_cnt_rc;
    neug_p4k_max = 0 as libc::c_int as uint16_t;
    neug_p64_max = neug_p4k_max;
    neug_rc_max = neug_p64_max;
}
unsafe extern "C" fn noise_source_error_reset() {
    neug_err_state = 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn noise_source_error(mut err: uint32_t) {
    neug_err_state = (neug_err_state as libc::c_uint | err) as uint8_t;
    neug_err_cnt = neug_err_cnt.wrapping_add(1);
    if err & 1 as libc::c_int as libc::c_uint != 0 {
        neug_err_cnt_rc = neug_err_cnt_rc.wrapping_add(1)
    }
    if err & 2 as libc::c_int as libc::c_uint != 0 {
        neug_err_cnt_p64 = neug_err_cnt_p64.wrapping_add(1)
    }
    if err & 4 as libc::c_int as libc::c_uint != 0 {
        neug_err_cnt_p4k = neug_err_cnt_p4k.wrapping_add(1)
    };
}
static mut rct_a: uint8_t = 0;
static mut rct_b: uint8_t = 0;
unsafe extern "C" fn repetition_count_test(mut sample: uint8_t) {
    if rct_a as libc::c_int == sample as libc::c_int {
        rct_b = rct_b.wrapping_add(1);
        if rct_b as libc::c_int >= 9 as libc::c_int {
            noise_source_error(1 as libc::c_int as uint32_t);
        }
        if rct_b as libc::c_int > neug_rc_max as libc::c_int {
            neug_rc_max = rct_b as uint16_t
        }
    } else { rct_a = sample; rct_b = 1 as libc::c_int as uint8_t };
}
unsafe extern "C" fn repetition_count_test_word(mut b0: uint8_t,
                                                mut b1: uint8_t,
                                                mut b2: uint8_t,
                                                mut b3: uint8_t) {
    if rct_a as libc::c_int == b0 as libc::c_int {
        rct_b = rct_b.wrapping_add(1)
    } else { rct_a = b0; rct_b = 1 as libc::c_int as uint8_t }
    if rct_a as libc::c_int == b1 as libc::c_int {
        rct_b = rct_b.wrapping_add(1)
    } else { rct_a = b1; rct_b = 1 as libc::c_int as uint8_t }
    if rct_a as libc::c_int == b2 as libc::c_int {
        rct_b = rct_b.wrapping_add(1)
    } else { rct_a = b2; rct_b = 1 as libc::c_int as uint8_t }
    if rct_a as libc::c_int == b3 as libc::c_int {
        rct_b = rct_b.wrapping_add(1)
    } else { rct_a = b3; rct_b = 1 as libc::c_int as uint8_t }
    if rct_b as libc::c_int >= 9 as libc::c_int {
        noise_source_error(1 as libc::c_int as uint32_t);
    }
    if rct_b as libc::c_int > neug_rc_max as libc::c_int {
        neug_rc_max = rct_b as uint16_t
    };
}
static mut ap64t_a: uint8_t = 0;
static mut ap64t_b: uint8_t = 0;
static mut ap64t_s: uint8_t = 0;
unsafe extern "C" fn adaptive_proportion_64_test(mut sample: uint8_t) {
    let fresh0 = ap64t_s;
    ap64t_s = ap64t_s.wrapping_add(1);
    if fresh0 as libc::c_int >= 64 as libc::c_int {
        ap64t_a = sample;
        ap64t_s = 1 as libc::c_int as uint8_t;
        ap64t_b = 0 as libc::c_int as uint8_t
    } else if ap64t_a as libc::c_int == sample as libc::c_int {
        ap64t_b = ap64t_b.wrapping_add(1);
        if ap64t_b as libc::c_int > 18 as libc::c_int {
            noise_source_error(2 as libc::c_int as uint32_t);
        }
        if ap64t_b as libc::c_int > neug_p64_max as libc::c_int {
            neug_p64_max = ap64t_b as uint16_t
        }
    };
}
unsafe extern "C" fn adaptive_proportion_64_test_word(mut b0: uint8_t,
                                                      mut b1: uint8_t,
                                                      mut b2: uint8_t,
                                                      mut b3: uint8_t) {
    if ap64t_s as libc::c_int >= 64 as libc::c_int {
        ap64t_a = b0;
        ap64t_s = 4 as libc::c_int as uint8_t;
        ap64t_b = 0 as libc::c_int as uint8_t
    } else {
        ap64t_s = (ap64t_s as libc::c_int + 4 as libc::c_int) as uint8_t;
        if ap64t_a as libc::c_int == b0 as libc::c_int {
            ap64t_b = ap64t_b.wrapping_add(1)
        }
    }
    if ap64t_a as libc::c_int == b1 as libc::c_int {
        ap64t_b = ap64t_b.wrapping_add(1)
    }
    if ap64t_a as libc::c_int == b2 as libc::c_int {
        ap64t_b = ap64t_b.wrapping_add(1)
    }
    if ap64t_a as libc::c_int == b3 as libc::c_int {
        ap64t_b = ap64t_b.wrapping_add(1)
    }
    if ap64t_b as libc::c_int > 18 as libc::c_int {
        noise_source_error(2 as libc::c_int as uint32_t);
    }
    if ap64t_b as libc::c_int > neug_p64_max as libc::c_int {
        neug_p64_max = ap64t_b as uint16_t
    };
}
static mut ap4096t_a: uint8_t = 0;
static mut ap4096t_b: uint16_t = 0;
static mut ap4096t_s: uint16_t = 0;
unsafe extern "C" fn adaptive_proportion_4096_test(mut sample: uint8_t) {
    let fresh1 = ap4096t_s;
    ap4096t_s = ap4096t_s.wrapping_add(1);
    if fresh1 as libc::c_int >= 4096 as libc::c_int {
        ap4096t_a = sample;
        ap4096t_s = 1 as libc::c_int as uint16_t;
        ap4096t_b = 0 as libc::c_int as uint16_t
    } else if ap4096t_a as libc::c_int == sample as libc::c_int {
        ap4096t_b = ap4096t_b.wrapping_add(1);
        if ap4096t_b as libc::c_int > 315 as libc::c_int {
            noise_source_error(4 as libc::c_int as uint32_t);
        }
        if ap4096t_b as libc::c_int > neug_p4k_max as libc::c_int {
            neug_p4k_max = ap4096t_b
        }
    };
}
unsafe extern "C" fn adaptive_proportion_4096_test_word(mut b0: uint8_t,
                                                        mut b1: uint8_t,
                                                        mut b2: uint8_t,
                                                        mut b3: uint8_t) {
    if ap4096t_s as libc::c_int >= 4096 as libc::c_int {
        ap4096t_a = b0;
        ap4096t_s = 4 as libc::c_int as uint16_t;
        ap4096t_b = 0 as libc::c_int as uint16_t
    } else {
        ap4096t_s = (ap4096t_s as libc::c_int + 4 as libc::c_int) as uint16_t;
        if ap4096t_a as libc::c_int == b0 as libc::c_int {
            ap4096t_b = ap4096t_b.wrapping_add(1)
        }
    }
    if ap4096t_a as libc::c_int == b1 as libc::c_int {
        ap4096t_b = ap4096t_b.wrapping_add(1)
    }
    if ap4096t_a as libc::c_int == b2 as libc::c_int {
        ap4096t_b = ap4096t_b.wrapping_add(1)
    }
    if ap4096t_a as libc::c_int == b3 as libc::c_int {
        ap4096t_b = ap4096t_b.wrapping_add(1)
    }
    if ap4096t_b as libc::c_int > 315 as libc::c_int {
        noise_source_error(4 as libc::c_int as uint32_t);
    }
    if ap4096t_b as libc::c_int > neug_p4k_max as libc::c_int {
        neug_p4k_max = ap4096t_b
    };
}
unsafe extern "C" fn noise_source_continuous_test(mut noise: uint8_t) {
    repetition_count_test(noise);
    adaptive_proportion_64_test(noise);
    adaptive_proportion_4096_test(noise);
}
unsafe extern "C" fn noise_source_continuous_test_word(mut b0: uint8_t,
                                                       mut b1: uint8_t,
                                                       mut b2: uint8_t,
                                                       mut b3: uint8_t) {
    repetition_count_test_word(b0, b1, b2, b3);
    adaptive_proportion_64_test_word(b0, b1, b2, b3);
    adaptive_proportion_4096_test_word(b0, b1, b2, b3);
}
unsafe extern "C" fn rb_init(mut rb: *mut rng_rb, mut p: *mut uint32_t,
                             mut size: uint8_t) {
    (*rb).buf = p;
    (*rb).size = size;
    chopstx_mutex_init(&mut (*rb).m);
    chopstx_cond_init(&mut (*rb).data_available);
    chopstx_cond_init(&mut (*rb).space_available);
    (*rb).tail = 0 as libc::c_int as uint8_t;
    (*rb).head = (*rb).tail;
    (*rb).set_full(0 as libc::c_int as libc::c_uint);
    (*rb).set_empty(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn rb_add(mut rb: *mut rng_rb, mut v: uint32_t) {
    let fresh2 = (*rb).tail;
    (*rb).tail = (*rb).tail.wrapping_add(1);
    *(*rb).buf.offset(fresh2 as isize) = v;
    if (*rb).tail as libc::c_int == (*rb).size as libc::c_int {
        (*rb).tail = 0 as libc::c_int as uint8_t
    }
    if (*rb).tail as libc::c_int == (*rb).head as libc::c_int {
        (*rb).set_full(1 as libc::c_int as libc::c_uint)
    }
    (*rb).set_empty(0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn rb_del(mut rb: *mut rng_rb) -> uint32_t {
    let fresh3 = (*rb).head;
    (*rb).head = (*rb).head.wrapping_add(1);
    let mut v: uint32_t = *(*rb).buf.offset(fresh3 as isize);
    if (*rb).head as libc::c_int == (*rb).size as libc::c_int {
        (*rb).head = 0 as libc::c_int as uint8_t
    }
    if (*rb).head as libc::c_int == (*rb).tail as libc::c_int {
        (*rb).set_empty(1 as libc::c_int as libc::c_uint)
    }
    (*rb).set_full(0 as libc::c_int as libc::c_uint);
    return v;
}
#[no_mangle]
pub static mut neug_mode: uint8_t = 0;
static mut rng_should_terminate: libc::c_int = 0;
static mut rng_thread: chopstx_t = 0;
/* *
 * @brief Random number generation thread.
 */
unsafe extern "C" fn rng(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut rb: *mut rng_rb = arg as *mut rng_rb;
    let mut mode: libc::c_int = neug_mode as libc::c_int;
    rng_should_terminate = 0 as libc::c_int;
    chopstx_mutex_init(&mut mode_mtx);
    chopstx_cond_init(&mut mode_cond);
    /* Enable ADCs */
    adc_start();
    ep_init(mode);
    while rng_should_terminate == 0 {
        let mut err: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        err = adc_wait_completion();
        chopstx_mutex_lock(&mut mode_mtx);
        if err != 0 || mode != neug_mode as libc::c_int {
            mode = neug_mode as libc::c_int;
            noise_source_cnt_max_reset();
            /* Discarding data available, re-initiate from the start.  */
            ep_init(mode);
            chopstx_cond_signal(&mut mode_cond);
            chopstx_mutex_unlock(&mut mode_mtx);
        } else {
            chopstx_mutex_unlock(&mut mode_mtx);
            n = ep_process(mode);
            if !(n != 0) { continue ; }
            let mut i: libc::c_int = 0;
            let mut vp: *const uint32_t = 0 as *const uint32_t;
            if neug_err_state as libc::c_int != 0 as libc::c_int &&
                   (mode == 0 as libc::c_int || mode == 1 as libc::c_int) {
                /* Don't use the result and do it again.  */
                noise_source_error_reset();
            } else {
                vp = ep_output(mode);
                chopstx_mutex_lock(&mut (*rb).m);
                while (*rb).full() != 0 {
                    chopstx_cond_wait(&mut (*rb).space_available,
                                      &mut (*rb).m);
                }
                i = 0 as libc::c_int;
                while i < n {
                    let fresh4 = vp;
                    vp = vp.offset(1);
                    rb_add(rb, *fresh4);
                    if (*rb).full() != 0 { break ; }
                    i += 1
                }
                chopstx_cond_signal(&mut (*rb).data_available);
                chopstx_mutex_unlock(&mut (*rb).m);
            }
        }
    }
    adc_stop();
    return 0 as *mut libc::c_void;
}
static mut the_ring_buffer: rng_rb =
    rng_rb{buf: 0 as *const uint32_t as *mut uint32_t,
           m:
               chopstx_mutex_t{q:
                                   chx_qh{next:
                                              0 as *const chx_pq as
                                                  *mut chx_pq,
                                          prev:
                                              0 as *const chx_pq as
                                                  *mut chx_pq,},
                               lock: chx_spinlock{},
                               owner:
                                   0 as *const chx_thread as *mut chx_thread,
                               list: 0 as *const chx_mtx as *mut chx_mtx,},
           data_available:
               chopstx_cond_t{q:
                                  chx_qh{next:
                                             0 as *const chx_pq as
                                                 *mut chx_pq,
                                         prev:
                                             0 as *const chx_pq as
                                                 *mut chx_pq,},
                              lock: chx_spinlock{},},
           space_available:
               chopstx_cond_t{q:
                                  chx_qh{next:
                                             0 as *const chx_pq as
                                                 *mut chx_pq,
                                         prev:
                                             0 as *const chx_pq as
                                                 *mut chx_pq,},
                              lock: chx_spinlock{},},
           head: 0,
           tail: 0,
           size: 0,
           full_empty: [0; 1],
           c2rust_padding: [0; 4],};
/* Sample data directly.         */
/* *
 * @brief Initialize NeuG.
 */
#[no_mangle]
pub unsafe extern "C" fn neug_init(mut buf: *mut uint32_t,
                                   mut size: uint8_t) {
    let mut u: *const uint32_t = unique_device_id() as *const uint32_t;
    let mut rb: *mut rng_rb = &mut the_ring_buffer;
    let mut i: libc::c_int = 0;
    ::std::ptr::write_volatile(&mut (*RCC).AHBENR as *mut uint32_t,
                               (::std::ptr::read_volatile::<uint32_t>(&(*RCC).AHBENR
                                                                          as
                                                                          *const uint32_t)
                                    as libc::c_uint |
                                    0x40 as libc::c_int as libc::c_uint) as
                                   uint32_t as uint32_t);
    ::std::ptr::write_volatile(&mut (*CRC).CR as *mut uint32_t,
                               0x1 as libc::c_int as uint32_t);
    /*
   * This initialization ensures that it generates different sequence
   * even if all physical conditions are same.
   */
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let fresh5 = u;
        u = u.offset(1);
        ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t, *fresh5);
        i += 1
    }
    neug_mode = 0 as libc::c_int as uint8_t;
    rb_init(rb, buf, size);
    rng_thread =
        chopstx_create(2 as libc::c_int as uint32_t,
                       __process2_stack_base__.as_mut_ptr() as uint32_t as
                           uintptr_t,
                       __process2_stack_size__.as_mut_ptr() as uint32_t as
                           size_t,
                       Some(rng as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> *mut libc::c_void),
                       rb as *mut libc::c_void);
}
/* *
 * @breif Flush random bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn neug_flush() {
    let mut rb: *mut rng_rb = &mut the_ring_buffer;
    chopstx_mutex_lock(&mut (*rb).m);
    while (*rb).empty() == 0 { rb_del(rb); }
    chopstx_cond_signal(&mut (*rb).space_available);
    chopstx_mutex_unlock(&mut (*rb).m);
}
/* *
 * @brief  Wakes up RNG thread to generate random numbers.
 */
#[no_mangle]
pub unsafe extern "C" fn neug_kick_filling() {
    let mut rb: *mut rng_rb = &mut the_ring_buffer;
    chopstx_mutex_lock(&mut (*rb).m);
    if (*rb).full() == 0 { chopstx_cond_signal(&mut (*rb).space_available); }
    chopstx_mutex_unlock(&mut (*rb).m);
}
/* *
 * @brief  Get random word (32-bit) from NeuG.
 * @detail With NEUG_KICK_FILLING, it wakes up RNG thread.
 *         With NEUG_NO_KICK, it doesn't wake up RNG thread automatically,
 *         it is needed to call neug_kick_filling later.
 */
#[no_mangle]
pub unsafe extern "C" fn neug_get(mut kick: libc::c_int) -> uint32_t {
    let mut rb: *mut rng_rb = &mut the_ring_buffer;
    let mut v: uint32_t = 0;
    chopstx_mutex_lock(&mut (*rb).m);
    while (*rb).empty() != 0 {
        chopstx_cond_wait(&mut (*rb).data_available, &mut (*rb).m);
    }
    v = rb_del(rb);
    if kick != 0 { chopstx_cond_signal(&mut (*rb).space_available); }
    chopstx_mutex_unlock(&mut (*rb).m);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn neug_get_nonblock(mut p: *mut uint32_t)
 -> libc::c_int {
    let mut rb: *mut rng_rb = &mut the_ring_buffer;
    let mut r: libc::c_int = 0 as libc::c_int;
    chopstx_mutex_lock(&mut (*rb).m);
    if (*rb).empty() != 0 {
        r = -(1 as libc::c_int);
        chopstx_cond_signal(&mut (*rb).space_available);
    } else { *p = rb_del(rb) }
    chopstx_mutex_unlock(&mut (*rb).m);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn neug_consume_random(mut proc_0:
                                                 Option<unsafe extern "C" fn(_:
                                                                                 uint32_t,
                                                                             _:
                                                                                 libc::c_int)
                                                            -> ()>)
 -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut rb: *mut rng_rb = &mut the_ring_buffer;
    chopstx_mutex_lock(&mut (*rb).m);
    while (*rb).empty() == 0 {
        let mut v: uint32_t = 0;
        v = rb_del(rb);
        proc_0.expect("non-null function pointer")(v, i);
        i += 1
    }
    chopstx_cond_signal(&mut (*rb).space_available);
    chopstx_mutex_unlock(&mut (*rb).m);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn neug_wait_full() {
    let mut rb: *mut rng_rb = &mut the_ring_buffer;
    chopstx_mutex_lock(&mut (*rb).m);
    while (*rb).full() == 0 {
        chopstx_cond_wait(&mut (*rb).data_available, &mut (*rb).m);
    }
    chopstx_mutex_unlock(&mut (*rb).m);
}
#[no_mangle]
pub unsafe extern "C" fn neug_fini() {
    rng_should_terminate = 1 as libc::c_int;
    neug_get(1 as libc::c_int);
    chopstx_join(rng_thread, 0 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn neug_mode_select(mut mode: uint8_t) {
    if neug_mode as libc::c_int == mode as libc::c_int { return }
    neug_wait_full();
    chopstx_mutex_lock(&mut mode_mtx);
    neug_mode = mode;
    neug_flush();
    chopstx_cond_wait(&mut mode_cond, &mut mode_mtx);
    chopstx_mutex_unlock(&mut mode_mtx);
    neug_wait_full();
    neug_flush();
}
