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
    fn bn256_sub(X: *mut bn256, A: *const bn256, B: *const bn256) -> uint32_t;
    #[no_mangle]
    fn bn256_is_zero(X: *const bn256) -> libc::c_int;
    #[no_mangle]
    fn bn256_cmp(A: *const bn256, B: *const bn256) -> libc::c_int;
    #[no_mangle]
    fn mod_inv(X: *mut bn256, A: *const bn256, N: *const bn256);
    #[no_mangle]
    static p256r1: bn256;
    #[no_mangle]
    fn modp256r1_add(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn modp256r1_sub(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn modp256r1_mul(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn modp256r1_sqr(X: *mut bn256, A: *const bn256);
    #[no_mangle]
    fn modp256r1_shift(X: *mut bn256, A: *const bn256, shift: libc::c_int);
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpc {
    pub x: [bn256; 1],
    pub y: [bn256; 1],
    pub z: [bn256; 1],
}
/*
 * jpc.c -- arithmetic on Jacobian projective coordinates.
 *
 * Copyright (C) 2011, 2013 Free Software Initiative of Japan
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
/* *
 * @brief	X = 2 * A
 *
 * @param X	Destination JPC
 * @param A	JPC
 */
#[no_mangle]
pub unsafe extern "C" fn jpc_double_p256r1(mut X: *mut jpc,
                                           mut A: *const jpc) {
    let mut a: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut b: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut c: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut tmp0: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut d: *mut bn256 = 0 as *mut bn256;
    if bn256_is_zero((*A).z.as_ptr()) != 0 {
        /* A is infinite */
        return
    }
    d = (*X).x.as_mut_ptr();
    modp256r1_sqr(a.as_mut_ptr(), (*A).y.as_ptr());
    memcpy(b.as_mut_ptr() as *mut libc::c_void,
           a.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    modp256r1_mul(a.as_mut_ptr(), a.as_mut_ptr(), (*A).x.as_ptr());
    modp256r1_shift(a.as_mut_ptr(), a.as_mut_ptr(), 2 as libc::c_int);
    modp256r1_sqr(b.as_mut_ptr(), b.as_mut_ptr());
    modp256r1_shift(b.as_mut_ptr(), b.as_mut_ptr(), 3 as libc::c_int);
    modp256r1_sqr(tmp0.as_mut_ptr(), (*A).z.as_ptr());
    modp256r1_sub(c.as_mut_ptr(), (*A).x.as_ptr(), tmp0.as_mut_ptr());
    modp256r1_add(tmp0.as_mut_ptr(), tmp0.as_mut_ptr(), (*A).x.as_ptr());
    modp256r1_mul(tmp0.as_mut_ptr(), tmp0.as_mut_ptr(), c.as_mut_ptr());
    modp256r1_shift(c.as_mut_ptr(), tmp0.as_mut_ptr(), 1 as libc::c_int);
    modp256r1_add(c.as_mut_ptr(), c.as_mut_ptr(), tmp0.as_mut_ptr());
    modp256r1_sqr(d, c.as_mut_ptr());
    modp256r1_shift(tmp0.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int);
    modp256r1_sub(d, d, tmp0.as_mut_ptr());
    modp256r1_mul((*X).z.as_mut_ptr(), (*A).y.as_ptr(), (*A).z.as_ptr());
    modp256r1_shift((*X).z.as_mut_ptr(), (*X).z.as_mut_ptr(),
                    1 as libc::c_int);
    modp256r1_sub(tmp0.as_mut_ptr(), a.as_mut_ptr(), d);
    modp256r1_mul(tmp0.as_mut_ptr(), c.as_mut_ptr(), tmp0.as_mut_ptr());
    modp256r1_sub((*X).y.as_mut_ptr(), tmp0.as_mut_ptr(), b.as_mut_ptr());
}
/* *
 * @brief	X = A + B
 *
 * @param X	Destination JPC
 * @param A	JPC
 * @param B	AC
 * @param MINUS if 1 subtraction, addition otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn jpc_add_ac_signed_p256r1(mut X: *mut jpc,
                                                  mut A: *const jpc,
                                                  mut B: *const ac,
                                                  mut minus: libc::c_int) {
    let mut a: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut b: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut c: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut d: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    if bn256_is_zero((*A).z.as_ptr()) != 0 {
        /* A is infinite */
        memcpy((*X).x.as_mut_ptr() as *mut libc::c_void,
               (*B).x.as_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
        if minus != 0 {
            memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
                   (*B).y.as_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<bn256>() as libc::c_ulong);
            bn256_sub((*X).y.as_mut_ptr(), &p256r1, (*B).y.as_ptr());
        } else {
            memcpy((*X).y.as_mut_ptr() as *mut libc::c_void,
                   (*B).y.as_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<bn256>() as libc::c_ulong);
            bn256_sub(tmp.as_mut_ptr(), &p256r1, (*B).y.as_ptr());
        }
        memset((*X).z.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
        (*(*X).z.as_mut_ptr()).word[0 as libc::c_int as usize] =
            1 as libc::c_int as uint32_t;
        return
    }
    modp256r1_sqr(a.as_mut_ptr(), (*A).z.as_ptr());
    memcpy(b.as_mut_ptr() as *mut libc::c_void,
           a.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    modp256r1_mul(a.as_mut_ptr(), a.as_mut_ptr(), (*B).x.as_ptr());
    modp256r1_mul(b.as_mut_ptr(), b.as_mut_ptr(), (*A).z.as_ptr());
    if minus != 0 {
        bn256_sub(c.as_mut_ptr(), &p256r1, (*B).y.as_ptr());
        modp256r1_mul(b.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    } else {
        bn256_sub(tmp.as_mut_ptr(), &p256r1, (*B).y.as_ptr());
        modp256r1_mul(b.as_mut_ptr(), b.as_mut_ptr(), (*B).y.as_ptr());
    }
    if bn256_cmp((*A).x.as_ptr(), a.as_mut_ptr()) == 0 as libc::c_int &&
           bn256_cmp((*A).y.as_ptr(), b.as_mut_ptr()) == 0 as libc::c_int {
        jpc_double_p256r1(X, A);
        return
    }
    modp256r1_sub(c.as_mut_ptr(), a.as_mut_ptr(), (*A).x.as_ptr());
    modp256r1_sub(d.as_mut_ptr(), b.as_mut_ptr(), (*A).y.as_ptr());
    modp256r1_mul((*X).z.as_mut_ptr(), (*A).z.as_ptr(), c.as_mut_ptr());
    modp256r1_sqr(a.as_mut_ptr(), c.as_mut_ptr());
    modp256r1_mul(b.as_mut_ptr(), a.as_mut_ptr(), c.as_mut_ptr());
    modp256r1_mul(c.as_mut_ptr(), (*A).x.as_ptr(), a.as_mut_ptr());
    modp256r1_sqr((*X).x.as_mut_ptr(), d.as_mut_ptr());
    memcpy(a.as_mut_ptr() as *mut libc::c_void,
           c.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    modp256r1_shift(c.as_mut_ptr(), c.as_mut_ptr(), 1 as libc::c_int);
    modp256r1_add(c.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr());
    modp256r1_sub((*X).x.as_mut_ptr(), (*X).x.as_mut_ptr(), c.as_mut_ptr());
    modp256r1_sub(c.as_mut_ptr(), a.as_mut_ptr(), (*X).x.as_mut_ptr());
    modp256r1_mul(c.as_mut_ptr(), c.as_mut_ptr(), d.as_mut_ptr());
    modp256r1_mul(a.as_mut_ptr(), (*A).y.as_ptr(), b.as_mut_ptr());
    modp256r1_sub((*X).y.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr());
}
/* *
 * @brief	X = A + B
 *
 * @param X	Destination JPC
 * @param A	JPC
 * @param B	AC
 */
#[no_mangle]
pub unsafe extern "C" fn jpc_add_ac_p256r1(mut X: *mut jpc, mut A: *const jpc,
                                           mut B: *const ac) {
    jpc_add_ac_signed_p256r1(X, A, B, 0 as libc::c_int);
}
/* *
 * @brief	X = convert A
 *
 * @param X	Destination AC
 * @param A	JPC
 *
 * Return -1 on error (infinite).
 * Return 0 on success.
 */
#[no_mangle]
pub unsafe extern "C" fn jpc_to_ac_p256r1(mut X: *mut ac, mut A: *const jpc)
 -> libc::c_int {
    let mut z_inv: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut z_inv_sqr: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    if bn256_is_zero((*A).z.as_ptr()) != 0 { return -(1 as libc::c_int) }
    mod_inv(z_inv.as_mut_ptr(), (*A).z.as_ptr(), &p256r1);
    modp256r1_sqr(z_inv_sqr.as_mut_ptr(), z_inv.as_mut_ptr());
    modp256r1_mul(z_inv.as_mut_ptr(), z_inv.as_mut_ptr(),
                  z_inv_sqr.as_mut_ptr());
    modp256r1_mul((*X).x.as_mut_ptr(), (*A).x.as_ptr(),
                  z_inv_sqr.as_mut_ptr());
    modp256r1_mul((*X).y.as_mut_ptr(), (*A).y.as_ptr(), z_inv.as_mut_ptr());
    return 0 as libc::c_int;
}
