#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* 32-byte random bytes */
    #[no_mangle]
    fn random_bytes_get() -> *const uint8_t;
    #[no_mangle]
    fn random_bytes_free(p: *const uint8_t);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
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
 * bn.c -- 256-bit (and 512-bit) bignum calculation
 *
 * Copyright (C) 2011, 2013, 2014 Free Software Initiative of Japan
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
#[no_mangle]
pub unsafe extern "C" fn bn256_add(mut X: *mut bn256, mut A: *const bn256,
                                   mut B: *const bn256) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut v: uint32_t = 0;
    let mut carry: uint32_t = 0 as libc::c_int as uint32_t;
    let mut px: *mut uint32_t = 0 as *mut uint32_t;
    let mut pa: *const uint32_t = 0 as *const uint32_t;
    let mut pb: *const uint32_t = 0 as *const uint32_t;
    px = (*X).word.as_mut_ptr();
    pa = (*A).word.as_ptr();
    pb = (*B).word.as_ptr();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        v = *pb;
        *px = (*pa).wrapping_add(carry);
        carry = (*px < carry) as libc::c_int as uint32_t;
        *px = (*px as libc::c_uint).wrapping_add(v) as uint32_t as uint32_t;
        carry =
            (carry as
                 libc::c_uint).wrapping_add((*px < v) as libc::c_int as
                                                libc::c_uint) as uint32_t as
                uint32_t;
        px = px.offset(1);
        pa = pa.offset(1);
        pb = pb.offset(1);
        i += 1
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_sub(mut X: *mut bn256, mut A: *const bn256,
                                   mut B: *const bn256) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut v: uint32_t = 0;
    let mut borrow: uint32_t = 0 as libc::c_int as uint32_t;
    let mut px: *mut uint32_t = 0 as *mut uint32_t;
    let mut pa: *const uint32_t = 0 as *const uint32_t;
    let mut pb: *const uint32_t = 0 as *const uint32_t;
    px = (*X).word.as_mut_ptr();
    pa = (*A).word.as_ptr();
    pb = (*B).word.as_ptr();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut borrow0: uint32_t = (*pa < borrow) as libc::c_int as uint32_t;
        v = *pb;
        *px = (*pa).wrapping_sub(borrow);
        borrow =
            ((*px < v) as libc::c_int as libc::c_uint).wrapping_add(borrow0);
        *px = (*px as libc::c_uint).wrapping_sub(v) as uint32_t as uint32_t;
        px = px.offset(1);
        pa = pa.offset(1);
        pb = pb.offset(1);
        i += 1
    }
    return borrow;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_add_uint(mut X: *mut bn256,
                                        mut A: *const bn256, mut w: uint32_t)
 -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut carry: uint32_t = w;
    let mut px: *mut uint32_t = 0 as *mut uint32_t;
    let mut pa: *const uint32_t = 0 as *const uint32_t;
    px = (*X).word.as_mut_ptr();
    pa = (*A).word.as_ptr();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *px = (*pa).wrapping_add(carry);
        carry = (*px < carry) as libc::c_int as uint32_t;
        px = px.offset(1);
        pa = pa.offset(1);
        i += 1
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_sub_uint(mut X: *mut bn256,
                                        mut A: *const bn256, mut w: uint32_t)
 -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut borrow: uint32_t = w;
    let mut px: *mut uint32_t = 0 as *mut uint32_t;
    let mut pa: *const uint32_t = 0 as *const uint32_t;
    px = (*X).word.as_mut_ptr();
    pa = (*A).word.as_ptr();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut borrow0: uint32_t = (*pa < borrow) as libc::c_int as uint32_t;
        *px = (*pa).wrapping_sub(borrow);
        borrow = borrow0;
        px = px.offset(1);
        pa = pa.offset(1);
        i += 1
    }
    return borrow;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_mul(mut X: *mut bn512, mut A: *const bn256,
                                   mut B: *const bn256) {
    let mut s: *const uint32_t = 0 as *const uint32_t;
    let mut d: *mut uint32_t = 0 as *mut uint32_t;
    let mut w: uint32_t = 0;
    let mut c: uint32_t = 0;
    memset((*X).word.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<uint32_t>() as
                libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong));
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[0 as libc::c_int as usize];
    *d = c;
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[1 as libc::c_int as usize];
    *d = c;
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(2 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[2 as libc::c_int as usize];
    *d = c;
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(3 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[3 as libc::c_int as usize];
    *d = c;
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(4 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[4 as libc::c_int as usize];
    *d = c;
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(5 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[5 as libc::c_int as usize];
    *d = c;
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(6 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[6 as libc::c_int as usize];
    *d = c;
    s = (*A).word.as_ptr();
    d =
        &mut *(*X).word.as_mut_ptr().offset(7 as libc::c_int as isize) as
            *mut uint32_t;
    w = (*B).word[7 as libc::c_int as usize];
    *d = c;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_sqr(mut X: *mut bn512, mut A: *const bn256) {
    let mut i: libc::c_int = 0;
    memset((*X).word.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn512>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut wij: *mut uint32_t =
            &mut *(*X).word.as_mut_ptr().offset((i * 2 as libc::c_int) as
                                                    isize) as *mut uint32_t;
        let mut xj: *const uint32_t =
            &*(*A).word.as_ptr().offset(i as isize) as *const uint32_t;
        let fresh0 = xj;
        xj = xj.offset(1);
        let mut x_i: uint32_t = *fresh0;
        let mut c: uint32_t = 0;
        /* (C,R4,R5) := w_i_i + x_i*x_i; w_i_i := R5; */
        /* R5 := w_i_i; */
        /* R9 := 0, the constant ZERO from here.  */
        /* (C,R4,R5) := (C,R4) + w_i_j + 2*x_i*x_j; */
        /* (C,R4,R6) := (C,R4) + w_i_j + 2*x_i*x_j; */
        /* */
        /* (C,R4,R5) := (C,R4) + w_i_j + 2*x_i*x_j; */
        if i < 8 as libc::c_int - 1 as libc::c_int { *wij = c }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn bn256_shift(mut X: *mut bn256, mut A: *const bn256,
                                     mut shift: libc::c_int) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut carry: uint32_t = 0 as libc::c_int as uint32_t;
    let mut next_carry: uint32_t = 0;
    if shift > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            next_carry = (*A).word[i as usize] >> 32 as libc::c_int - shift;
            (*X).word[i as usize] = (*A).word[i as usize] << shift | carry;
            carry = next_carry;
            i += 1
        }
    } else {
        shift = -shift;
        i = 8 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            next_carry =
                (*A).word[i as usize] &
                    (((1 as libc::c_int) << shift) - 1 as libc::c_int) as
                        libc::c_uint;
            (*X).word[i as usize] =
                (*A).word[i as usize] >> shift |
                    carry << 32 as libc::c_int - shift;
            carry = next_carry;
            i -= 1
        }
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_is_zero(mut X: *const bn256) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        r &=
            ((*X).word[i as usize] == 0 as libc::c_int as libc::c_uint) as
                libc::c_int;
        i += 1
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_is_even(mut X: *const bn256) -> libc::c_int {
    return ((*X).word[0 as libc::c_int as usize] &
                1 as libc::c_int as libc::c_uint == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_is_ge(mut A: *const bn256, mut B: *const bn256)
 -> libc::c_int {
    let mut borrow: uint32_t = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    borrow = bn256_sub(tmp.as_mut_ptr(), A, B);
    return (borrow == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bn256_cmp(mut A: *const bn256, mut B: *const bn256)
 -> libc::c_int {
    let mut borrow: uint32_t = 0;
    let mut is_zero: libc::c_int = 0;
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    borrow = bn256_sub(tmp.as_mut_ptr(), A, B);
    is_zero = bn256_is_zero(tmp.as_mut_ptr());
    return if is_zero != 0 {
               0 as libc::c_int
           } else if borrow != 0 {
               -(1 as libc::c_int)
           } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn bn256_random(mut X: *mut bn256) {
    let mut rand: *const uint8_t = random_bytes_get();
    (*X).word[7 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(7 as libc::c_int as isize);
    (*X).word[6 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(6 as libc::c_int as isize);
    (*X).word[5 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(5 as libc::c_int as isize);
    (*X).word[4 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(4 as libc::c_int as isize);
    (*X).word[3 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(3 as libc::c_int as isize);
    (*X).word[2 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(2 as libc::c_int as isize);
    (*X).word[1 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(1 as libc::c_int as isize);
    (*X).word[0 as libc::c_int as usize] =
        *(rand as *mut uint32_t).offset(0 as libc::c_int as isize);
    random_bytes_free(rand);
}
