#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    /* NULL */
    #[no_mangle]
    fn gnuk_malloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn gnuk_free(_: *mut libc::c_void);
    #[no_mangle]
    fn compute_kP_p256k1(X: *mut ac, K: *const bn256, P: *const ac)
     -> libc::c_int;
    #[no_mangle]
    fn compute_kG_p256k1(X: *mut ac, K: *const bn256) -> libc::c_int;
    #[no_mangle]
    fn ecdsa_p256k1(r: *mut bn256, s: *mut bn256, z: *const bn256,
                    d: *const bn256);
    #[no_mangle]
    fn check_secret_p256k1(q: *const bn256, d1: *mut bn256) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
/*
 * stdlib.h replacement to replace malloc functions
 */
pub type size_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn256 {
    pub word: [uint32_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ac {
    pub x: [bn256; 1],
    pub y: [bn256; 1],
}
#[no_mangle]
pub unsafe extern "C" fn ecdsa_sign_p256k1(mut hash: *const uint8_t,
                                           mut output: *mut uint8_t,
                                           mut key_data: *const uint8_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0; /* skip '04' */
    let mut r: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut s: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut z: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut d: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    p = d.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *p.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize) =
            *key_data.offset(i as isize);
        i += 1
    }
    p = z.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *p.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize) =
            *hash.offset(i as isize);
        i += 1
    }
    ecdsa_p256k1(r.as_mut_ptr(), s.as_mut_ptr(), z.as_mut_ptr(),
                 d.as_mut_ptr());
    p = r.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 =
            *p.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize);
        i += 1
    }
    p = s.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 =
            *p.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize);
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ecc_compute_public_p256k1(mut key_data:
                                                       *const uint8_t)
 -> *mut uint8_t {
    let mut p0: *mut uint8_t = 0 as *mut uint8_t;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut p1: *mut uint8_t = 0 as *mut uint8_t;
    let mut q: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut k: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut i: libc::c_int = 0;
    p0 =
        gnuk_malloc((32 as libc::c_int * 2 as libc::c_int) as size_t) as
            *mut uint8_t;
    if p0.is_null() { return 0 as *mut uint8_t }
    p = k.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *p.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize) =
            *key_data.offset(i as isize);
        i += 1
    }
    if compute_kG_p256k1(q.as_mut_ptr(), k.as_mut_ptr()) < 0 as libc::c_int {
        gnuk_free(p0 as *mut libc::c_void);
        return 0 as *mut uint8_t
    }
    p = p0;
    p1 = (*q.as_mut_ptr()).x.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 =
            *p1.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize);
        i += 1
    }
    p1 = (*q.as_mut_ptr()).y.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 =
            *p1.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize);
        i += 1
    }
    return p0;
}
#[no_mangle]
pub unsafe extern "C" fn ecdh_decrypt_p256k1(mut input: *const uint8_t,
                                             mut output: *mut uint8_t,
                                             mut key_data: *const uint8_t)
 -> libc::c_int {
    let mut k: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut X: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut P: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut i: libc::c_int = 0;
    let mut p0: *mut uint8_t = 0 as *mut uint8_t;
    let mut p1: *const uint8_t = 0 as *const uint8_t;
    let mut r: libc::c_int = 0;
    p0 = k.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *p0.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize) =
            *key_data.offset(i as isize);
        i += 1
    }
    p1 = input.offset(1 as libc::c_int as isize);
    p0 = (*P.as_mut_ptr()).x.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let fresh4 = p1;
        p1 = p1.offset(1);
        *p0.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize) =
            *fresh4;
        i += 1
    }
    p0 = (*P.as_mut_ptr()).y.as_mut_ptr() as *mut uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let fresh5 = p1;
        p1 = p1.offset(1);
        *p0.offset((32 as libc::c_int - i - 1 as libc::c_int) as isize) =
            *fresh5;
        i += 1
    }
    r = compute_kP_p256k1(X.as_mut_ptr(), k.as_mut_ptr(), P.as_mut_ptr());
    if r == 0 as libc::c_int {
        p0 = output;
        p1 = (*X.as_mut_ptr()).x.as_mut_ptr() as *const uint8_t;
        let fresh6 = p0;
        p0 = p0.offset(1);
        *fresh6 = 4 as libc::c_int as uint8_t;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            let fresh7 = p0;
            p0 = p0.offset(1);
            *fresh7 =
                *p1.offset((32 as libc::c_int - i - 1 as libc::c_int) as
                               isize);
            i += 1
        }
        p1 = (*X.as_mut_ptr()).y.as_mut_ptr() as *const uint8_t;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            let fresh8 = p0;
            p0 = p0.offset(1);
            *fresh8 =
                *p1.offset((32 as libc::c_int - i - 1 as libc::c_int) as
                               isize);
            i += 1
        }
    }
    return r;
}
/* *
 * @brief Check if a secret d0 is valid or not
 *
 * @param D0	scalar D0: secret
 * @param D1	scalar D1: secret candidate N-D0
 *
 * Return 0 on error.
 * Return -1 when D1 should be used as the secret
 * Return 1 when D0 should be used as the secret
 */
#[no_mangle]
pub unsafe extern "C" fn ecc_check_secret_p256k1(mut d0: *const uint8_t,
                                                 mut d1: *mut uint8_t)
 -> libc::c_int {
    return check_secret_p256k1(d0 as *const bn256, d1 as *mut bn256);
}
/*
 * call-ec_p256k1.c - interface between Gnuk and Elliptic curve over
 *                    GF(p256k1)
 *
 * Copyright (C) 2014 Free Software Initiative of Japan
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
