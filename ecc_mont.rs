#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn gnuk_malloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn bn256_add_uint(X: *mut bn256, A: *const bn256, w: uint32_t)
     -> uint32_t;
    #[no_mangle]
    static p25519: [bn256; 1];
    #[no_mangle]
    fn mod25638_add(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn mod25638_sub(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn mod25638_mul(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn mod25638_sqr(X: *mut bn256, A: *const bn256);
    #[no_mangle]
    fn mod25519_reduce(X: *mut bn256);
    #[no_mangle]
    fn mod_inv(X: *mut bn256, A: *const bn256, N: *const bn256);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn256 {
    pub word: [uint32_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pt {
    pub x: [bn256; 1],
    pub z: [bn256; 1],
}
/*                                                    -*- coding: utf-8 -*-
 * ecc-mont.c - Elliptic curve computation for
 *              the Montgomery curve: y^2 = x^3 + 486662*x^2 + x.
 *
 * Copyright (C) 2014, 2015 Free Software Initiative of Japan
 * Author: NIIBE Yutaka <gniibe@fsij.org>
 *
 * This file is a part of Gnuk, a GnuPG USB Token implementation.
 *
 * Gnuk is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Gnuk is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public
 * License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
/*
 * References:
 *
 * [1] D. J. Bernstein. Curve25519: new Diffie-Hellman speed records.
 *     Proceedings of PKC 2006, to appear.
 *     http://cr.yp.to/papers.html#curve25519. Date: 2006.02.09.
 *
 * [2] D. J. Bernstein. Can we avoid tests for zero in fast
 *     elliptic-curve arithmetic?
 *     http://cr.yp.to/papers.html#curvezero. Date: 2006.07.26.
 *
 */
/*
 * IMPLEMENTATION NOTE
 *
 * (0) We assume that the processor has no cache, nor branch target
 *     prediction.  Thus, we don't avoid indexing by secret value.
 *     We don't avoid conditional jump if both cases have same timing,
 *     either.
 *
 * (1) We use Radix-32 field arithmetic.  It's a representation like
 *     2^256-38, but it's more redundant.  For example, "1" can be
 *     represented in three ways in 256-bit: 1, 2^255-18, and
 *     2^256-37.
 *
 * (2) We use Montgomery double-and-add.
 *
 */
/*
 *
 * 121665 = 0x1db41
 *            1 1101 1011 0100 0001
 */
unsafe extern "C" fn mod25638_mul_121665(mut x: *mut bn256,
                                         mut a: *const bn256) {
    let mut s: *const uint32_t = 0 as *const uint32_t;
    let mut d: *mut uint32_t = 0 as *mut uint32_t;
    let mut w: uint32_t = 0;
    let mut c: uint32_t = 0;
    s = (*a).word.as_ptr();
    d = (*x).word.as_mut_ptr();
    memset(d as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    w = 121665 as libc::c_int as uint32_t;
    c =
        bn256_add_uint(x, x,
                       c.wrapping_mul(38 as libc::c_int as libc::c_uint));
    (*x).word[0 as libc::c_int as usize] =
        ((*x).word[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(c.wrapping_mul(38 as libc::c_int as
                                                           libc::c_uint)) as
            uint32_t as uint32_t;
}
/* *
 * @brief  Process Montgomery double-and-add
 *
 * With Q0, Q1, DIF (= Q0 - Q1), compute PRD = 2Q0, SUM = Q0 + Q1
 * Q0 and Q1 are clobbered.
 *
 */
unsafe extern "C" fn mont_d_and_a(mut prd: *mut pt, mut sum: *mut pt,
                                  mut q0: *mut pt, mut q1: *mut pt,
                                  mut dif_x: *const bn256) {
    mod25638_add((*sum).x.as_mut_ptr(), (*q1).x.as_mut_ptr(),
                 (*q1).z.as_mut_ptr());
    mod25638_sub((*q1).z.as_mut_ptr(), (*q1).x.as_mut_ptr(),
                 (*q1).z.as_mut_ptr());
    mod25638_add((*prd).x.as_mut_ptr(), (*q0).x.as_mut_ptr(),
                 (*q0).z.as_mut_ptr());
    mod25638_sub((*q0).z.as_mut_ptr(), (*q0).x.as_mut_ptr(),
                 (*q0).z.as_mut_ptr());
    mod25638_mul((*q1).x.as_mut_ptr(), (*q0).z.as_mut_ptr(),
                 (*sum).x.as_mut_ptr());
    mod25638_mul((*q1).z.as_mut_ptr(), (*prd).x.as_mut_ptr(),
                 (*q1).z.as_mut_ptr());
    mod25638_sqr((*q0).x.as_mut_ptr(), (*prd).x.as_mut_ptr());
    mod25638_sqr((*q0).z.as_mut_ptr(), (*q0).z.as_mut_ptr());
    mod25638_add((*sum).x.as_mut_ptr(), (*q1).x.as_mut_ptr(),
                 (*q1).z.as_mut_ptr());
    mod25638_sub((*q1).z.as_mut_ptr(), (*q1).x.as_mut_ptr(),
                 (*q1).z.as_mut_ptr());
    mod25638_mul((*prd).x.as_mut_ptr(), (*q0).x.as_mut_ptr(),
                 (*q0).z.as_mut_ptr());
    mod25638_sub((*q0).z.as_mut_ptr(), (*q0).x.as_mut_ptr(),
                 (*q0).z.as_mut_ptr());
    mod25638_sqr((*sum).x.as_mut_ptr(), (*sum).x.as_mut_ptr());
    mod25638_sqr((*sum).z.as_mut_ptr(), (*q1).z.as_mut_ptr());
    mod25638_mul_121665((*prd).z.as_mut_ptr(), (*q0).z.as_mut_ptr());
    mod25638_mul((*sum).z.as_mut_ptr(), (*sum).z.as_mut_ptr(), dif_x);
    mod25638_add((*prd).z.as_mut_ptr(), (*q0).x.as_mut_ptr(),
                 (*prd).z.as_mut_ptr());
    mod25638_mul((*prd).z.as_mut_ptr(), (*prd).z.as_mut_ptr(),
                 (*q0).z.as_mut_ptr());
}
/* *
 * @brief	RES  = x-coordinate of [n]Q
 *
 * @param N	Scalar N (three least significant bits are 000)
 * @param Q_X	x-coordinate of Q
 *
 */
unsafe extern "C" fn compute_nQ(mut res: *mut bn256, mut n: *const bn256,
                                mut q_x: *const bn256) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p0: [pt; 1] =
        [pt{x: [bn256{word: [0; 8],}; 1], z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut p1: [pt; 1] =
        [pt{x: [bn256{word: [0; 8],}; 1], z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut p0_: [pt; 1] =
        [pt{x: [bn256{word: [0; 8],}; 1], z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut p1_: [pt; 1] =
        [pt{x: [bn256{word: [0; 8],}; 1], z: [bn256{word: [0; 8],}; 1],}; 1];
    /* P0 = O = (1:0)  */
    memset((*p0.as_mut_ptr()).x.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*(*p0.as_mut_ptr()).x.as_mut_ptr()).word[0 as libc::c_int as usize] =
        1 as libc::c_int as uint32_t;
    memset((*p0.as_mut_ptr()).z.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<bn256>() as libc::c_ulong);
    /* P1 = (X:1) */
    memcpy((*p1.as_mut_ptr()).x.as_mut_ptr() as *mut libc::c_void,
           q_x as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memset((*p1.as_mut_ptr()).z.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*(*p1.as_mut_ptr()).z.as_mut_ptr()).word[0 as libc::c_int as usize] =
        1 as libc::c_int as uint32_t;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut u: uint32_t = (*n).word[(7 as libc::c_int - i) as usize];
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            let mut q0: *mut pt = 0 as *mut pt;
            let mut q1: *mut pt = 0 as *mut pt;
            let mut sum_n: *mut pt = 0 as *mut pt;
            let mut prd_n: *mut pt = 0 as *mut pt;
            if u & 0x80000000 as libc::c_uint != 0 {
                q0 = p1.as_mut_ptr();
                q1 = p0.as_mut_ptr();
                sum_n = p0_.as_mut_ptr();
                prd_n = p1_.as_mut_ptr()
            } else {
                q0 = p0.as_mut_ptr();
                q1 = p1.as_mut_ptr();
                sum_n = p1_.as_mut_ptr();
                prd_n = p0_.as_mut_ptr()
            }
            mont_d_and_a(prd_n, sum_n, q0, q1, q_x);
            if u & 0x40000000 as libc::c_int as libc::c_uint != 0 {
                q0 = p1_.as_mut_ptr();
                q1 = p0_.as_mut_ptr();
                sum_n = p0.as_mut_ptr();
                prd_n = p1.as_mut_ptr()
            } else {
                q0 = p0_.as_mut_ptr();
                q1 = p1_.as_mut_ptr();
                sum_n = p1.as_mut_ptr();
                prd_n = p0.as_mut_ptr()
            }
            mont_d_and_a(prd_n, sum_n, q0, q1, q_x);
            u <<= 2 as libc::c_int;
            j += 1
        }
        i += 1
    }
    /* We know the LSB of N is always 0.  Thus, result is always in P0.  */
  /*
   * p0->z may be zero here, but our mod_inv doesn't raise error for 0,
   * but returns 0 (like the implementation of z^(p-2)), thus, RES will
   * be 0 in that case, which is correct value.
   */
    mod_inv(res, (*p0.as_mut_ptr()).z.as_mut_ptr(),
            p25519.as_ptr()); /* Gx = 9 */
    mod25638_mul(res, res, (*p0.as_mut_ptr()).x.as_mut_ptr());
    mod25519_reduce(res);
}
#[no_mangle]
pub unsafe extern "C" fn ecdh_compute_public_25519(mut key_data:
                                                       *const uint8_t)
 -> *mut uint8_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut gx: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut k: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    memset(gx.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    gx[0 as libc::c_int as usize].word[0 as libc::c_int as usize] =
        9 as libc::c_int as uint32_t;
    memcpy(k.as_mut_ptr() as *mut libc::c_void,
           key_data as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    p =
        gnuk_malloc(::std::mem::size_of::<bn256>() as libc::c_ulong) as
            *mut uint8_t;
    if p.is_null() { return 0 as *mut uint8_t }
    compute_nQ(p as *mut bn256, k.as_mut_ptr(), gx.as_mut_ptr());
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ecdh_decrypt_curve25519(mut input: *const uint8_t,
                                                 mut output: *mut uint8_t,
                                                 mut key_data: *const uint8_t)
 -> libc::c_int {
    let mut q_x: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut k: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut shared: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    memcpy(q_x.as_mut_ptr() as *mut libc::c_void,
           input as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memcpy(k.as_mut_ptr() as *mut libc::c_void,
           key_data as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    compute_nQ(shared.as_mut_ptr(), k.as_mut_ptr(), q_x.as_mut_ptr());
    memcpy(output as *mut libc::c_void,
           shared.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    return 0 as libc::c_int;
}
