#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, register_tool)]
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
    fn bn256_add_uint(X: *mut bn256, A: *const bn256, w: uint32_t)
     -> uint32_t;
    #[no_mangle]
    fn bn256_sub_uint(X: *mut bn256, A: *const bn256, w: uint32_t)
     -> uint32_t;
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
 * mod25638.c -- modulo arithmetic of 2^256-38 for 2^255-19 field
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
/*
 * The field is \Z/(2^255-19)
 *
 * We use radix-32.  During computation, it's not reduced to 2^255-19,
 * but it is represented in 256-bit (it is redundant representation),
 * that is, something like 2^256-38.
 *
 * The idea is, keeping within 256-bit until it will be converted to
 * affine coordinates.
 */
/*
256      224      192      160      128       96       64       32        0
2^256
  1 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
2^256 - 16
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffff0
2^256 - 16 - 2
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffee
2^256 - 16 - 2 - 1
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffed
*/
#[no_mangle]
pub static mut p25519: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0xffffffed as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0x7fffffff as libc::c_int as uint32_t],};
         init
     }];
/*
 * Implementation Note.
 *
 * It's not always modulo n25638.  The representation is redundant
 * during computation.  For example, when we add the number - 1 and 1,
 * it won't overflow to 2^256, and the result is represented within
 * 256-bit.
 */
/* *
 * @brief  X = (A + B) mod 2^256-38
 */
#[no_mangle]
pub unsafe extern "C" fn mod25638_add(mut X: *mut bn256, mut A: *const bn256,
                                      mut B: *const bn256) {
    let mut carry: uint32_t = 0;
    carry = bn256_add(X, A, B);
    carry =
        bn256_add_uint(X, X,
                       carry.wrapping_mul(38 as libc::c_int as libc::c_uint));
    (*X).word[0 as libc::c_int as usize] =
        ((*X).word[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(carry.wrapping_mul(38 as libc::c_int
                                                               as
                                                               libc::c_uint))
            as uint32_t as uint32_t;
}
/* *
 * @brief  X = (A - B) mod 2^256-38
 */
#[no_mangle]
pub unsafe extern "C" fn mod25638_sub(mut X: *mut bn256, mut A: *const bn256,
                                      mut B: *const bn256) {
    let mut borrow: uint32_t = 0;
    borrow = bn256_sub(X, A, B);
    borrow =
        bn256_sub_uint(X, X,
                       borrow.wrapping_mul(38 as libc::c_int as
                                               libc::c_uint));
    (*X).word[0 as libc::c_int as usize] =
        ((*X).word[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_sub(borrow.wrapping_mul(38 as libc::c_int
                                                                as
                                                                libc::c_uint))
            as uint32_t as uint32_t;
}
/* *
 * @brief  X = A mod 2^256-38
 *
 * Note that the second argument is not "const bn512 *".
 * A is modified during the computation of modulo.
 *
 * It's not precisely modulo 2^256-38 for all cases,
 * but result may be redundant.
 */
unsafe extern "C" fn mod25638_reduce(mut X: *mut bn256, mut A: *mut bn512) {
    let mut s: *const uint32_t = 0 as *const uint32_t;
    let mut d: *mut uint32_t = 0 as *mut uint32_t;
    let mut w: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut c0: uint32_t = 0;
    s =
        &mut *(*A).word.as_mut_ptr().offset(8 as libc::c_int as isize) as
            *mut uint32_t;
    d =
        &mut *(*A).word.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut uint32_t;
    w = 38 as libc::c_int as uint32_t;
    *d = c;
    c0 =
        (*A).word[8 as libc::c_int as
                      usize].wrapping_mul(38 as libc::c_int as libc::c_uint);
    d =
        &mut *(*X).word.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut uint32_t;
    s =
        &mut *(*A).word.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut uint32_t;
    (*X).word[0 as libc::c_int as usize] =
        ((*X).word[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(c.wrapping_mul(38 as libc::c_int as
                                                           libc::c_uint)) as
            uint32_t as uint32_t;
}
/* *
 * @brief  X = (A * B) mod 2^256-38
 */
#[no_mangle]
pub unsafe extern "C" fn mod25638_mul(mut X: *mut bn256, mut A: *const bn256,
                                      mut B: *const bn256) {
    let mut tmp: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    bn256_mul(tmp.as_mut_ptr(), A, B);
    mod25638_reduce(X, tmp.as_mut_ptr());
}
/* *
 * @brief  X = A * A mod 2^256-38
 */
#[no_mangle]
pub unsafe extern "C" fn mod25638_sqr(mut X: *mut bn256,
                                      mut A: *const bn256) {
    let mut tmp: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    bn256_sqr(tmp.as_mut_ptr(), A);
    mod25638_reduce(X, tmp.as_mut_ptr());
}
/* *
 * @brief  X = (A << shift) mod 2^256-38
 * @note   shift < 32
 */
#[no_mangle]
pub unsafe extern "C" fn mod25638_shift(mut X: *mut bn256,
                                        mut A: *const bn256,
                                        mut shift: libc::c_int) {
    let mut carry: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    carry = bn256_shift(X, A, shift);
    if shift < 0 as libc::c_int { return }
    memset(tmp.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        carry << 1 as libc::c_int;
    /* tmp->word[1] = (carry >> 31);  always zero.  */
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*tmp.as_mut_ptr()).word[0 as libc::c_int as
                                     usize].wrapping_add(carry <<
                                                             2 as
                                                                 libc::c_int);
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (((*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] <
              carry << 2 as libc::c_int) as libc::c_int as
             libc::c_uint).wrapping_add(carry >> 30 as libc::c_int);
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*tmp.as_mut_ptr()).word[0 as libc::c_int as
                                     usize].wrapping_add(carry <<
                                                             5 as
                                                                 libc::c_int);
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*tmp.as_mut_ptr()).word[1 as libc::c_int as
                                     usize].wrapping_add(((*tmp.as_mut_ptr()).word[0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                              <
                                                              carry <<
                                                                  5 as
                                                                      libc::c_int)
                                                             as libc::c_int as
                                                             libc::c_uint).wrapping_add(carry
                                                                                            >>
                                                                                            27
                                                                                                as
                                                                                                libc::c_int);
    mod25638_add(X, X, tmp.as_mut_ptr());
}
/*
 * @brief  X = A mod 2^255-19
 *
 * It's precisely modulo 2^255-19 (unlike mod25638_reduce).
 */
#[no_mangle]
pub unsafe extern "C" fn mod25519_reduce(mut X: *mut bn256) {
    let mut q: uint32_t = 0; /* dummy */
    let mut r0: [bn256; 1] = [bn256{word: [0; 8],}; 1]; /* dummy */
    let mut r1: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut flag: libc::c_int = 0;
    memcpy(r0.as_mut_ptr() as *mut libc::c_void, X as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    q =
        (*r0.as_mut_ptr()).word[7 as libc::c_int as usize] >>
            31 as libc::c_int;
    let ref mut fresh0 = (*r0.as_mut_ptr()).word[7 as libc::c_int as usize];
    *fresh0 &= 0x7fffffff as libc::c_int as libc::c_uint;
    if q != 0 {
        bn256_add_uint(r0.as_mut_ptr(), r0.as_mut_ptr(),
                       19 as libc::c_int as uint32_t);
        q =
            (*r0.as_mut_ptr()).word[7 as libc::c_int as usize] >>
                31 as libc::c_int;
        let ref mut fresh1 =
            (*r0.as_mut_ptr()).word[7 as libc::c_int as usize];
        *fresh1 &= 0x7fffffff as libc::c_int as libc::c_uint;
        if q != 0 {
            bn256_add_uint(r1.as_mut_ptr(), r0.as_mut_ptr(),
                           19 as libc::c_int as uint32_t);
            q =
                (*r1.as_mut_ptr()).word[7 as libc::c_int as usize] >>
                    31 as libc::c_int;
            let ref mut fresh2 =
                (*r1.as_mut_ptr()).word[7 as libc::c_int as usize];
            *fresh2 &= 0x7fffffff as libc::c_int as libc::c_uint;
            flag = 0 as libc::c_int
        } else { flag = 1 as libc::c_int }
    } else {
        bn256_add_uint(r1.as_mut_ptr(), r0.as_mut_ptr(),
                       19 as libc::c_int as uint32_t);
        q =
            (*r1.as_mut_ptr()).word[7 as libc::c_int as usize] >>
                31 as libc::c_int;
        let ref mut fresh3 =
            (*r1.as_mut_ptr()).word[7 as libc::c_int as usize];
        *fresh3 &= 0x7fffffff as libc::c_int as libc::c_uint;
        if q != 0 { flag = 2 as libc::c_int } else { flag = 3 as libc::c_int }
    }
    if flag != 0 {
        bn256_add_uint(r1.as_mut_ptr(), r0.as_mut_ptr(),
                       19 as libc::c_int as uint32_t);
        q =
            (*r1.as_mut_ptr()).word[7 as libc::c_int as usize] >>
                31 as libc::c_int;
        let ref mut fresh4 =
            (*r1.as_mut_ptr()).word[7 as libc::c_int as usize];
        *fresh4 &= 0x7fffffff as libc::c_int as libc::c_uint;
        if q != 0 {
            memcpy(X as *mut libc::c_void,
                   r1.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<bn256>() as libc::c_ulong);
        } else {
            memcpy(X as *mut libc::c_void,
                   r0.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<bn256>() as libc::c_ulong);
        }
    } else if q != 0 {
        asm!("" : : "r" (q) : "memory" : "volatile");
        memcpy(X as *mut libc::c_void, r1.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
        asm!("" : : "r" (q) : "memory" : "volatile")
    } else {
        memcpy(X as *mut libc::c_void, r1.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    };
}
