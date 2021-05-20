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
 * modp256k1.c -- modulo arithmetic for p256k1
 *
 * Copyright (C) 2014, 2016 Free Software Initiative of Japan
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
 * p256k1 =  2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
 */
/*
256      224      192      160      128       96       64       32        0
2^256
  1 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
2^256 - 2^32
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff 00000000
2^256 - 2^32 - 2^9
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffe00
2^256 - 2^32 - 2^9 - 2^8
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffd00
2^256 - 2^32 - 2^9 - 2^8 - 2^7
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc80
2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc40
2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc30
2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
  0 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe fffffc2f
*/
#[no_mangle]
pub static mut p256k1: bn256 =
    {
        let mut init =
            bn256{word:
                      [0xfffffc2f as libc::c_uint, 0xfffffffe as libc::c_uint,
                       0xffffffff as libc::c_uint, 0xffffffff as libc::c_uint,
                       0xffffffff as libc::c_uint, 0xffffffff as libc::c_uint,
                       0xffffffff as libc::c_uint,
                       0xffffffff as libc::c_uint],};
        init
    };
/*
 * Implementation Note.
 *
 * It's always modulo p256k1.
 *
 * Once, I tried redundant representation which caused wrong
 * calculation.  Implementation could be correct with redundant
 * representation, but it found that it's more expensive.
 *
 */
/* *
 * @brief  X = (A + B) mod p256k1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256k1_add(mut X: *mut bn256, mut A: *const bn256,
                                       mut B: *const bn256) {
    let mut cond: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    cond =
        (bn256_add(X, A, B) == 0 as libc::c_int as libc::c_uint) as
            libc::c_int as uint32_t;
    cond &= bn256_sub(tmp.as_mut_ptr(), X, &p256k1);
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
 * @brief  X = (A - B) mod p256
 */
#[no_mangle]
pub unsafe extern "C" fn modp256k1_sub(mut X: *mut bn256, mut A: *const bn256,
                                       mut B: *const bn256) {
    let mut borrow_0: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    borrow_0 = bn256_sub(X, A, B);
    bn256_add(tmp.as_mut_ptr(), X, &p256k1);
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
 * @brief  X = A mod p256k1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256k1_reduce(mut X: *mut bn256,
                                          mut A: *const bn512) {
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut carry: uint32_t = 0;
    let mut s0: uint32_t = 0;
    let mut s1: uint32_t = 0;
    /*
   * Suppose: P256K1 = 2^256 - CONST
   * Then, compute: W = A_low + A_high * CONST
   *                256-bit W0 = W mod 2^256
   *                64-bit (S1, S0) = W / 2^256
   * where: CONST = 2^32 + 2^9 + 2^8 + 2^7 + 2^6 + 2^4 + 1
   */
    /* W0 = A_low   */
  /* W7 = A_high  */
  /* W0 += W7 */
    carry =
        bn256_add(X,
                  &*(*A).word.as_ptr().offset(8 as libc::c_int as isize) as
                      *const uint32_t as *const bn256, A as *const bn256);
    /* W6 = W7 << 4 */
  /* W0 += W6 */
    bn256_shift(tmp.as_mut_ptr(),
                &*(*A).word.as_ptr().offset(8 as libc::c_int as isize) as
                    *const uint32_t as *const bn256, 4 as libc::c_int);
    carry =
        (carry as
             libc::c_uint).wrapping_add(bn256_add(X, X, tmp.as_mut_ptr())) as
            uint32_t as uint32_t;
    /* W5 = W6 << 2 */
  /* W0 += W5 */
    bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(), 2 as libc::c_int);
    carry =
        (carry as
             libc::c_uint).wrapping_add(bn256_add(X, X, tmp.as_mut_ptr())) as
            uint32_t as uint32_t;
    /* W4 = W5 << 1 */
  /* W0 += W4 */
    bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(), 1 as libc::c_int);
    carry =
        (carry as
             libc::c_uint).wrapping_add(bn256_add(X, X, tmp.as_mut_ptr())) as
            uint32_t as uint32_t;
    /* W3 = W4 << 1 */
  /* W0 += W3 */
    bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(), 1 as libc::c_int);
    carry =
        (carry as
             libc::c_uint).wrapping_add(bn256_add(X, X, tmp.as_mut_ptr())) as
            uint32_t as uint32_t;
    /* W2 = W3 << 1 */
  /* W0 += W2 */
    bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(), 1 as libc::c_int);
    carry =
        (carry as
             libc::c_uint).wrapping_add(bn256_add(X, X, tmp.as_mut_ptr())) as
            uint32_t as uint32_t;
    /* W1 = A_high << 32 */
  /* W0 += W1 */
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[12 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[11 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[10 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[9 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[8 as libc::c_int as usize];
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        0 as libc::c_int as uint32_t;
    carry =
        (carry as
             libc::c_uint).wrapping_add(bn256_add(X, X, tmp.as_mut_ptr())) as
            uint32_t as uint32_t;
    /* (S1, S0) = W / 2^256 */
    s0 = (*A).word[15 as libc::c_int as usize];
    carry =
        (carry as
             libc::c_uint).wrapping_add((s0 >>
                                             28 as
                                                 libc::c_int).wrapping_add(s0
                                                                               >>
                                                                               26
                                                                                   as
                                                                                   libc::c_int).wrapping_add(s0
                                                                                                                 >>
                                                                                                                 25
                                                                                                                     as
                                                                                                                     libc::c_int).wrapping_add(s0
                                                                                                                                                   >>
                                                                                                                                                   24
                                                                                                                                                       as
                                                                                                                                                       libc::c_int).wrapping_add(s0
                                                                                                                                                                                     >>
                                                                                                                                                                                     23
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_int))
            as uint32_t as uint32_t;
    carry = (carry as libc::c_uint).wrapping_add(s0) as uint32_t as uint32_t;
    s1 =
        if carry < s0 { 1 as libc::c_int } else { 0 as libc::c_int } as
            uint32_t;
    s0 = carry;
    /*
   * Compute: S:=(S02, S01, S00), S = (S1,S0)*CONST
   */
    let ref mut fresh0 = (*tmp.as_mut_ptr()).word[3 as libc::c_int as usize];
    *fresh0 = 0 as libc::c_int as uint32_t;
    let ref mut fresh1 = (*tmp.as_mut_ptr()).word[4 as libc::c_int as usize];
    *fresh1 = *fresh0;
    let ref mut fresh2 = (*tmp.as_mut_ptr()).word[5 as libc::c_int as usize];
    *fresh2 = *fresh1;
    let ref mut fresh3 = (*tmp.as_mut_ptr()).word[6 as libc::c_int as usize];
    *fresh3 = *fresh2;
    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize] = *fresh3;
    /* (S02, S01, S00) = (S1, S0) + (S1, S0)*2^32 */
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] = s0;
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] = s0.wrapping_add(s1);
    (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize] =
        s1.wrapping_add((if (*tmp.as_mut_ptr()).word[1 as libc::c_int as
                                                         usize] < s0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) as libc::c_uint);
    /* (S02, S01, S00) += (S1, S0)*2^9 */
    carry =
        (s0 >>
             23 as
                 libc::c_int).wrapping_add((*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    let ref mut fresh4 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh4 =
        (*fresh4 as
             libc::c_uint).wrapping_add((s1 >>
                                             23 as
                                                 libc::c_int).wrapping_add((if carry
                                                                                   <
                                                                                   (*tmp.as_mut_ptr()).word[1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                usize]
                                                                               {
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            } else {
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                            })
                                                                               as
                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (s1 << 9 as libc::c_int).wrapping_add(carry);
    let ref mut fresh5 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh5 =
        (*fresh5 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    let ref mut fresh6 = (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize];
    *fresh6 =
        (*fresh6 as libc::c_uint).wrapping_add(s0 << 9 as libc::c_int) as
            uint32_t as uint32_t;
    carry =
        if (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] <
               s0 << 9 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int } as uint32_t;
    let ref mut fresh7 = (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh7 =
        (*fresh7 as libc::c_uint).wrapping_add(carry) as uint32_t as uint32_t;
    let ref mut fresh8 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh8 =
        (*fresh8 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    /* (S02, S01, S00) += (S1, S0)*2^8 */
    carry =
        (s0 >>
             24 as
                 libc::c_int).wrapping_add((*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    let ref mut fresh9 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh9 =
        (*fresh9 as
             libc::c_uint).wrapping_add((s1 >>
                                             24 as
                                                 libc::c_int).wrapping_add((if carry
                                                                                   <
                                                                                   (*tmp.as_mut_ptr()).word[1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                usize]
                                                                               {
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            } else {
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                            })
                                                                               as
                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (s1 << 8 as libc::c_int).wrapping_add(carry);
    let ref mut fresh10 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh10 =
        (*fresh10 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    let ref mut fresh11 = (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize];
    *fresh11 =
        (*fresh11 as libc::c_uint).wrapping_add(s0 << 8 as libc::c_int) as
            uint32_t as uint32_t;
    carry =
        if (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] <
               s0 << 8 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int } as uint32_t;
    let ref mut fresh12 = (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh12 =
        (*fresh12 as libc::c_uint).wrapping_add(carry) as uint32_t as
            uint32_t;
    let ref mut fresh13 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh13 =
        (*fresh13 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    /* (S02, S01, S00) += (S1, S0)*2^7 */
    carry =
        (s0 >>
             25 as
                 libc::c_int).wrapping_add((*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    let ref mut fresh14 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh14 =
        (*fresh14 as
             libc::c_uint).wrapping_add((s1 >>
                                             25 as
                                                 libc::c_int).wrapping_add((if carry
                                                                                   <
                                                                                   (*tmp.as_mut_ptr()).word[1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                usize]
                                                                               {
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            } else {
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                            })
                                                                               as
                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (s1 << 7 as libc::c_int).wrapping_add(carry);
    let ref mut fresh15 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh15 =
        (*fresh15 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    let ref mut fresh16 = (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize];
    *fresh16 =
        (*fresh16 as libc::c_uint).wrapping_add(s0 << 7 as libc::c_int) as
            uint32_t as uint32_t;
    carry =
        if (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] <
               s0 << 7 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int } as uint32_t;
    let ref mut fresh17 = (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh17 =
        (*fresh17 as libc::c_uint).wrapping_add(carry) as uint32_t as
            uint32_t;
    let ref mut fresh18 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh18 =
        (*fresh18 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    /* (S02, S01, S00) += (S1, S0)*2^6 */
    carry =
        (s0 >>
             26 as
                 libc::c_int).wrapping_add((*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    let ref mut fresh19 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh19 =
        (*fresh19 as
             libc::c_uint).wrapping_add((s1 >>
                                             26 as
                                                 libc::c_int).wrapping_add((if carry
                                                                                   <
                                                                                   (*tmp.as_mut_ptr()).word[1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                usize]
                                                                               {
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            } else {
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                            })
                                                                               as
                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (s1 << 6 as libc::c_int).wrapping_add(carry);
    let ref mut fresh20 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh20 =
        (*fresh20 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    let ref mut fresh21 = (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize];
    *fresh21 =
        (*fresh21 as libc::c_uint).wrapping_add(s0 << 6 as libc::c_int) as
            uint32_t as uint32_t;
    carry =
        if (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] <
               s0 << 6 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int } as uint32_t;
    let ref mut fresh22 = (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh22 =
        (*fresh22 as libc::c_uint).wrapping_add(carry) as uint32_t as
            uint32_t;
    let ref mut fresh23 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh23 =
        (*fresh23 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    /* (S02, S01, S00) += (S1, S0)*2^4 */
    carry =
        (s0 >>
             28 as
                 libc::c_int).wrapping_add((*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    let ref mut fresh24 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh24 =
        (*fresh24 as
             libc::c_uint).wrapping_add((s1 >>
                                             28 as
                                                 libc::c_int).wrapping_add((if carry
                                                                                   <
                                                                                   (*tmp.as_mut_ptr()).word[1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                usize]
                                                                               {
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            } else {
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                            })
                                                                               as
                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (s1 << 4 as libc::c_int).wrapping_add(carry);
    let ref mut fresh25 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh25 =
        (*fresh25 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    let ref mut fresh26 = (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize];
    *fresh26 =
        (*fresh26 as libc::c_uint).wrapping_add(s0 << 4 as libc::c_int) as
            uint32_t as uint32_t;
    carry =
        if (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] <
               s0 << 4 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int } as uint32_t;
    let ref mut fresh27 = (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh27 =
        (*fresh27 as libc::c_uint).wrapping_add(carry) as uint32_t as
            uint32_t;
    let ref mut fresh28 = (*tmp.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh28 =
        (*fresh28 as
             libc::c_uint).wrapping_add(if (*tmp.as_mut_ptr()).word[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                               < carry {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int } as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    /* W0 += S */
    modp256k1_add(X, X, tmp.as_mut_ptr());
    carry = bn256_sub(tmp.as_mut_ptr(), X, &p256k1);
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
/* *
 * @brief  X = (A * B) mod p256k1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256k1_mul(mut X: *mut bn256, mut A: *const bn256,
                                       mut B: *const bn256) {
    let mut AB: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    bn256_mul(AB.as_mut_ptr(), A, B);
    modp256k1_reduce(X, AB.as_mut_ptr());
}
/* *
 * @brief  X = A * A mod p256k1
 */
#[no_mangle]
pub unsafe extern "C" fn modp256k1_sqr(mut X: *mut bn256,
                                       mut A: *const bn256) {
    let mut AA: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    bn256_sqr(AA.as_mut_ptr(), A);
    modp256k1_reduce(X, AA.as_mut_ptr());
}
/* *
 * @brief  X = (A << shift) mod p256k1
 * @note   shift < 32
 */
#[no_mangle]
pub unsafe extern "C" fn modp256k1_shift(mut X: *mut bn256,
                                         mut A: *const bn256,
                                         mut shift: libc::c_int) {
    let mut carry: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    carry = bn256_shift(X, A, shift);
    if shift < 0 as libc::c_int { return }
    memset(tmp.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        carry.wrapping_add(carry << 9 as libc::c_int);
    (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize] =
        carry.wrapping_add(((*tmp.as_mut_ptr()).word[0 as libc::c_int as
                                                         usize] <
                                carry << 9 as libc::c_int) as libc::c_int as
                               libc::c_uint).wrapping_add(carry >>
                                                              23 as
                                                                  libc::c_int);
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*tmp.as_mut_ptr()).word[0 as libc::c_int as
                                     usize].wrapping_add(carry <<
                                                             8 as
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
                                                                  8 as
                                                                      libc::c_int)
                                                             as libc::c_int as
                                                             libc::c_uint).wrapping_add(carry
                                                                                            >>
                                                                                            24
                                                                                                as
                                                                                                libc::c_int);
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*tmp.as_mut_ptr()).word[0 as libc::c_int as
                                     usize].wrapping_add(carry <<
                                                             7 as
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
                                                                  7 as
                                                                      libc::c_int)
                                                             as libc::c_int as
                                                             libc::c_uint).wrapping_add(carry
                                                                                            >>
                                                                                            25
                                                                                                as
                                                                                                libc::c_int);
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*tmp.as_mut_ptr()).word[0 as libc::c_int as
                                     usize].wrapping_add(carry <<
                                                             6 as
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
                                                                  6 as
                                                                      libc::c_int)
                                                             as libc::c_int as
                                                             libc::c_uint).wrapping_add(carry
                                                                                            >>
                                                                                            26
                                                                                                as
                                                                                                libc::c_int);
    (*tmp.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*tmp.as_mut_ptr()).word[0 as libc::c_int as
                                     usize].wrapping_add(carry <<
                                                             4 as
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
                                                                  4 as
                                                                      libc::c_int)
                                                             as libc::c_int as
                                                             libc::c_uint).wrapping_add(carry
                                                                                            >>
                                                                                            28
                                                                                                as
                                                                                                libc::c_int);
    modp256k1_add(X, X, tmp.as_mut_ptr());
}
