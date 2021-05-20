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
    fn bn256_add(X: *mut bn256, A: *const bn256, B: *const bn256) -> uint32_t;
    #[no_mangle]
    fn bn256_sub(X: *mut bn256, A: *const bn256, B: *const bn256) -> uint32_t;
    #[no_mangle]
    fn bn256_mul(X: *mut bn512, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn bn256_sqr(X: *mut bn512, A: *const bn256);
    #[no_mangle]
    fn bn256_shift(X: *mut bn256, A: *const bn256, shift: libc::c_int)
     -> uint32_t;
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
pub struct bn512 {
    pub word: [uint32_t; 16],
}
/*
 * modp256r1.c -- modulo arithmetic for p256r1
 *
 * Copyright (C) 2011, 2013, 2014, 2016
 *               Free Software Initiative of Japan
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
 * p256 =  2^256 - 2^224 + 2^192 + 2^96 - 1
 */
/*
256      224      192      160      128       96       64       32        0
2^256
  1 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
2^256 - 2^224
  0 ffffffff 00000000 00000000 00000000 00000000 00000000 00000000 00000000
2^256 - 2^224 + 2^192
  0 ffffffff 00000001 00000000 00000000 00000000 00000000 00000000 00000000
2^256 - 2^224 + 2^192 + 2^96
  0 ffffffff 00000001 00000000 00000000 00000001 00000000 00000000 00000000
2^256 - 2^224 + 2^192 + 2^96 - 1
  0 ffffffff 00000001 00000000 00000000 00000000 ffffffff ffffffff ffffffff
*/
#[no_mangle]
pub static mut p256r1: bn256 =
    {
        let mut init =
            bn256{word:
                      [0xffffffff as libc::c_uint, 0xffffffff as libc::c_uint,
                       0xffffffff as libc::c_uint,
                       0 as libc::c_int as uint32_t,
                       0 as libc::c_int as uint32_t,
                       0 as libc::c_int as uint32_t,
                       0x1 as libc::c_int as uint32_t,
                       0xffffffff as libc::c_uint],};
        init
    };
/*
 * Implementation Note.
 *
 * It's always modulo p256r1.
 *
 * Once, I tried redundant representation which caused wrong
 * calculation.  Implementation could be correct with redundant
 * representation, but it found that it's more expensive.
 *
 */
/* *
 * @brief  X = (A + B) mod p256r1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256r1_add(mut X: *mut bn256, mut A: *const bn256,
                                       mut B: *const bn256) {
    let mut cond: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    cond =
        (bn256_add(X, A, B) == 0 as libc::c_int as libc::c_uint) as
            libc::c_int as uint32_t;
    cond &= bn256_sub(tmp.as_mut_ptr(), X, &p256r1);
    if cond != 0 {
        /* No-carry AND borrow */
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
               tmp.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(X as *mut libc::c_void,
               tmp.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    };
}
/* *
 * @brief  X = (A - B) mod p256r1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256r1_sub(mut X: *mut bn256, mut A: *const bn256,
                                       mut B: *const bn256) {
    let mut borrow_0: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    borrow_0 = bn256_sub(X, A, B);
    bn256_add(tmp.as_mut_ptr(), X, &p256r1);
    if borrow_0 != 0 {
        memcpy(X as *mut libc::c_void,
               tmp.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
               tmp.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    };
}
/* *
 * @brief  X = A mod p256r1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256r1_reduce(mut X: *mut bn256,
                                          mut A: *const bn512) {
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut tmp0: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut borrow_0: uint32_t = 0;
    (*X).word[7 as libc::c_int as usize] =
        (*A).word[7 as libc::c_int as usize];
    (*X).word[6 as libc::c_int as usize] =
        (*A).word[6 as libc::c_int as usize];
    (*X).word[5 as libc::c_int as usize] =
        (*A).word[5 as libc::c_int as usize];
    (*X).word[4 as libc::c_int as usize] =
        (*A).word[4 as libc::c_int as usize];
    (*X).word[3 as libc::c_int as usize] =
        (*A).word[3 as libc::c_int as usize];
    (*X).word[2 as libc::c_int as usize] =
        (*A).word[2 as libc::c_int as usize];
    (*X).word[1 as libc::c_int as usize] =
        (*A).word[1 as libc::c_int as usize];
    (*X).word[0 as libc::c_int as usize] =
        (*A).word[0 as libc::c_int as usize];
    borrow_0 = bn256_sub(tmp0.as_mut_ptr(), X, &p256r1);
    if borrow_0 != 0 {
        memcpy(tmp0.as_mut_ptr() as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(X as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    }
    /* X = S1 */
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[12 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[11 as libc::c_int as usize];
    let ref mut fresh0 = (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize];
    *fresh0 = 0 as libc::c_int as uint32_t;
    let ref mut fresh1 = (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh1 = *fresh0;
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] = *fresh1;
    /* X += 2 * S2 */
    modp256r1_add(X, X, tmp.as_mut_ptr());
    modp256r1_add(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        0 as libc::c_int as uint32_t;
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[12 as libc::c_int as usize];
    let ref mut fresh2 = (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize];
    *fresh2 = 0 as libc::c_int as uint32_t;
    let ref mut fresh3 = (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh3 = *fresh2;
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] = *fresh3;
    /* X += 2 * S3 */
    modp256r1_add(X, X, tmp.as_mut_ptr());
    modp256r1_add(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    let ref mut fresh4 = (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize];
    *fresh4 = 0 as libc::c_int as uint32_t;
    let ref mut fresh5 = (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize];
    *fresh5 = *fresh4;
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] = *fresh5;
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[10 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[9 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[8 as libc::c_int as usize];
    /* X += S4 */
    modp256r1_add(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[8 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[11 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[10 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[9 as libc::c_int as usize];
    borrow_0 = bn256_sub(tmp0.as_mut_ptr(), tmp.as_mut_ptr(), &p256r1);
    if borrow_0 != 0 {
        memcpy(tmp0.as_mut_ptr() as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    }
    /* X += S5 */
    modp256r1_add(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[10 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[8 as libc::c_int as usize];
    let ref mut fresh6 = (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize];
    *fresh6 = 0 as libc::c_int as uint32_t;
    let ref mut fresh7 = (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize];
    *fresh7 = *fresh6;
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] = *fresh7;
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[12 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[11 as libc::c_int as usize];
    borrow_0 = bn256_sub(tmp0.as_mut_ptr(), tmp.as_mut_ptr(), &p256r1);
    if borrow_0 != 0 {
        memcpy(tmp0.as_mut_ptr() as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    }
    /* X -= S6 */
    modp256r1_sub(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[11 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[9 as libc::c_int as usize];
    let ref mut fresh8 = (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize];
    *fresh8 = 0 as libc::c_int as uint32_t;
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] = *fresh8;
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[12 as libc::c_int as usize];
    borrow_0 = bn256_sub(tmp0.as_mut_ptr(), tmp.as_mut_ptr(), &p256r1);
    if borrow_0 != 0 {
        memcpy(tmp0.as_mut_ptr() as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
               tmp0.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    }
    /* X -= S7 */
    modp256r1_sub(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[12 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        0 as libc::c_int as uint32_t;
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[10 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[9 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[8 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    /* X -= S8 */
    modp256r1_sub(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        0 as libc::c_int as uint32_t;
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[11 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[10 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[9 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        0 as libc::c_int as uint32_t;
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    /* X -= S9 */
    modp256r1_sub(X, X, tmp.as_mut_ptr());
    borrow_0 = bn256_sub(tmp.as_mut_ptr(), X, &p256r1);
    if borrow_0 != 0 {
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
               X as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(X as *mut libc::c_void,
               tmp.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    };
}
/* *
 * @brief  X = (A * B) mod p256r1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256r1_mul(mut X: *mut bn256, mut A: *const bn256,
                                       mut B: *const bn256) {
    let mut AB: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    bn256_mul(AB.as_mut_ptr(), A, B);
    modp256r1_reduce(X, AB.as_mut_ptr());
}
/* *
 * @brief  X = A * A mod p256r1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256r1_sqr(mut X: *mut bn256,
                                       mut A: *const bn256) {
    let mut AA: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    bn256_sqr(AA.as_mut_ptr(), A);
    modp256r1_reduce(X, AA.as_mut_ptr());
}
/* *
 * @brief  X = (A << shift) mod p256r1
 * @note   shift < 32
 */
#[no_mangle]
pub unsafe extern "C" fn modp256r1_shift(mut X: *mut bn256,
                                         mut A: *const bn256,
                                         mut shift: libc::c_int) {
    let mut carry: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    carry = bn256_shift(X, A, shift);
    if shift < 0 as libc::c_int { return }
    memset(tmp.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] = carry;
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] = carry;
    modp256r1_add(X, X, tmp.as_mut_ptr());
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        0 as libc::c_int as uint32_t;
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        0 as libc::c_int as uint32_t;
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] = carry;
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] = carry;
    modp256r1_sub(X, X, tmp.as_mut_ptr());
    carry = bn256_sub(tmp.as_mut_ptr(), X, &p256r1);
    if carry != 0 {
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
               X as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(X as *mut libc::c_void,
               tmp.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    };
}
