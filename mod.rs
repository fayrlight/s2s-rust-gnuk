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
    fn bn256_shift(X: *mut bn256, A: *const bn256, shift: libc::c_int)
     -> uint32_t;
    #[no_mangle]
    fn bn256_is_even(X: *const bn256) -> libc::c_int;
    #[no_mangle]
    fn bn256_is_ge(A: *const bn256, B: *const bn256) -> libc::c_int;
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
 * mod.c -- modulo arithmetic
 *
 * Copyright (C) 2011, 2014 Free Software Initiative of Japan
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
 * @brief X = A mod B (using MU=(1<<(256)+MU_lower)) (Barret reduction)
 *
 */
#[no_mangle]
pub unsafe extern "C" fn mod_reduce(mut X: *mut bn256, mut A: *const bn512,
                                    mut B: *const bn256,
                                    mut MU_lower: *const bn256) {
    let mut q: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut q_big: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    let mut tmp: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    let mut carry: uint32_t = 0;
    let mut borrow_next: uint32_t = 0;
    memset(q.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*q.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize];
    bn256_mul(tmp.as_mut_ptr(), q.as_mut_ptr(), MU_lower);
    let ref mut fresh0 = (*tmp.as_mut_ptr()).word[8 as libc::c_int as usize];
    *fresh0 =
        (*fresh0 as
             libc::c_uint).wrapping_add((*A).word[15 as libc::c_int as usize])
            as uint32_t as uint32_t;
    carry =
        ((*tmp.as_mut_ptr()).word[8 as libc::c_int as usize] <
             (*A).word[15 as libc::c_int as usize]) as libc::c_int as
            uint32_t;
    let ref mut fresh1 = (*tmp.as_mut_ptr()).word[9 as libc::c_int as usize];
    *fresh1 =
        (*fresh1 as libc::c_uint).wrapping_add(carry) as uint32_t as uint32_t;
    (*q.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[14 as libc::c_int as usize];
    (*q.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[13 as libc::c_int as usize];
    (*q.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[12 as libc::c_int as usize];
    (*q.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[11 as libc::c_int as usize];
    (*q.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[10 as libc::c_int as usize];
    (*q.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[9 as libc::c_int as usize];
    (*q.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[8 as libc::c_int as usize];
    (*q.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[7 as libc::c_int as usize];
    bn256_mul(q_big.as_mut_ptr(), q.as_mut_ptr(), MU_lower);
    bn256_add(&mut *(*q_big.as_mut_ptr()).word.as_mut_ptr().offset(8 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                  as *mut uint32_t as *mut bn256,
              &mut *(*q_big.as_mut_ptr()).word.as_mut_ptr().offset(8 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                  as *mut uint32_t as *mut bn256, q.as_mut_ptr());
    (*q.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*q_big.as_mut_ptr()).word[9 as libc::c_int as
                                       usize].wrapping_add((*tmp.as_mut_ptr()).word[1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        usize]);
    carry =
        ((*q.as_mut_ptr()).word[0 as libc::c_int as usize] <
             (*tmp.as_mut_ptr()).word[1 as libc::c_int as usize]) as
            libc::c_int as uint32_t;
    (*q.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*q_big.as_mut_ptr()).word[10 as libc::c_int as
                                       usize].wrapping_add(carry);
    carry =
        ((*q.as_mut_ptr()).word[1 as libc::c_int as usize] < carry) as
            libc::c_int as uint32_t;
    let ref mut fresh2 = (*q.as_mut_ptr()).word[1 as libc::c_int as usize];
    *fresh2 =
        (*fresh2 as
             libc::c_uint).wrapping_add((*tmp.as_mut_ptr()).word[2 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry =
        (carry as
             libc::c_uint).wrapping_add(((*q.as_mut_ptr()).word[1 as
                                                                    libc::c_int
                                                                    as usize]
                                             <
                                             (*tmp.as_mut_ptr()).word[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize])
                                            as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    (*q.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*q_big.as_mut_ptr()).word[11 as libc::c_int as
                                       usize].wrapping_add(carry);
    carry =
        ((*q.as_mut_ptr()).word[2 as libc::c_int as usize] < carry) as
            libc::c_int as uint32_t;
    let ref mut fresh3 = (*q.as_mut_ptr()).word[2 as libc::c_int as usize];
    *fresh3 =
        (*fresh3 as
             libc::c_uint).wrapping_add((*tmp.as_mut_ptr()).word[3 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry =
        (carry as
             libc::c_uint).wrapping_add(((*q.as_mut_ptr()).word[2 as
                                                                    libc::c_int
                                                                    as usize]
                                             <
                                             (*tmp.as_mut_ptr()).word[3 as
                                                                          libc::c_int
                                                                          as
                                                                          usize])
                                            as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    (*q.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*q_big.as_mut_ptr()).word[12 as libc::c_int as
                                       usize].wrapping_add(carry);
    carry =
        ((*q.as_mut_ptr()).word[3 as libc::c_int as usize] < carry) as
            libc::c_int as uint32_t;
    let ref mut fresh4 = (*q.as_mut_ptr()).word[3 as libc::c_int as usize];
    *fresh4 =
        (*fresh4 as
             libc::c_uint).wrapping_add((*tmp.as_mut_ptr()).word[4 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry =
        (carry as
             libc::c_uint).wrapping_add(((*q.as_mut_ptr()).word[3 as
                                                                    libc::c_int
                                                                    as usize]
                                             <
                                             (*tmp.as_mut_ptr()).word[4 as
                                                                          libc::c_int
                                                                          as
                                                                          usize])
                                            as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    (*q.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*q_big.as_mut_ptr()).word[13 as libc::c_int as
                                       usize].wrapping_add(carry);
    carry =
        ((*q.as_mut_ptr()).word[4 as libc::c_int as usize] < carry) as
            libc::c_int as uint32_t;
    let ref mut fresh5 = (*q.as_mut_ptr()).word[4 as libc::c_int as usize];
    *fresh5 =
        (*fresh5 as
             libc::c_uint).wrapping_add((*tmp.as_mut_ptr()).word[5 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry =
        (carry as
             libc::c_uint).wrapping_add(((*q.as_mut_ptr()).word[4 as
                                                                    libc::c_int
                                                                    as usize]
                                             <
                                             (*tmp.as_mut_ptr()).word[5 as
                                                                          libc::c_int
                                                                          as
                                                                          usize])
                                            as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    (*q.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*q_big.as_mut_ptr()).word[14 as libc::c_int as
                                       usize].wrapping_add(carry);
    carry =
        ((*q.as_mut_ptr()).word[5 as libc::c_int as usize] < carry) as
            libc::c_int as uint32_t;
    let ref mut fresh6 = (*q.as_mut_ptr()).word[5 as libc::c_int as usize];
    *fresh6 =
        (*fresh6 as
             libc::c_uint).wrapping_add((*tmp.as_mut_ptr()).word[6 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry =
        (carry as
             libc::c_uint).wrapping_add(((*q.as_mut_ptr()).word[5 as
                                                                    libc::c_int
                                                                    as usize]
                                             <
                                             (*tmp.as_mut_ptr()).word[6 as
                                                                          libc::c_int
                                                                          as
                                                                          usize])
                                            as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    (*q.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*q_big.as_mut_ptr()).word[15 as libc::c_int as
                                       usize].wrapping_add(carry);
    carry =
        ((*q.as_mut_ptr()).word[6 as libc::c_int as usize] < carry) as
            libc::c_int as uint32_t;
    let ref mut fresh7 = (*q.as_mut_ptr()).word[6 as libc::c_int as usize];
    *fresh7 =
        (*fresh7 as
             libc::c_uint).wrapping_add((*tmp.as_mut_ptr()).word[7 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry =
        (carry as
             libc::c_uint).wrapping_add(((*q.as_mut_ptr()).word[6 as
                                                                    libc::c_int
                                                                    as usize]
                                             <
                                             (*tmp.as_mut_ptr()).word[7 as
                                                                          libc::c_int
                                                                          as
                                                                          usize])
                                            as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    (*q.as_mut_ptr()).word[7 as libc::c_int as usize] = carry;
    let ref mut fresh8 = (*q.as_mut_ptr()).word[7 as libc::c_int as usize];
    *fresh8 =
        (*fresh8 as
             libc::c_uint).wrapping_add((*tmp.as_mut_ptr()).word[8 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry =
        ((*q.as_mut_ptr()).word[7 as libc::c_int as usize] <
             (*tmp.as_mut_ptr()).word[8 as libc::c_int as usize]) as
            libc::c_int as uint32_t;
    memset(q_big.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn512>() as libc::c_ulong);
    (*q_big.as_mut_ptr()).word[8 as libc::c_int as usize] =
        (*A).word[8 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[7 as libc::c_int as usize] =
        (*A).word[7 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[6 as libc::c_int as usize] =
        (*A).word[6 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[5 as libc::c_int as usize] =
        (*A).word[5 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[4 as libc::c_int as usize] =
        (*A).word[4 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[3 as libc::c_int as usize] =
        (*A).word[3 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[2 as libc::c_int as usize] =
        (*A).word[2 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[1 as libc::c_int as usize] =
        (*A).word[1 as libc::c_int as usize];
    (*q_big.as_mut_ptr()).word[0 as libc::c_int as usize] =
        (*A).word[0 as libc::c_int as usize];
    bn256_mul(tmp.as_mut_ptr(), q.as_mut_ptr(), B);
    let ref mut fresh9 = (*tmp.as_mut_ptr()).word[8 as libc::c_int as usize];
    *fresh9 =
        (*fresh9 as
             libc::c_uint).wrapping_add(carry.wrapping_mul((*B).word[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]))
            as uint32_t as uint32_t;
    let ref mut fresh10 = (*tmp.as_mut_ptr()).word[9 as libc::c_int as usize];
    *fresh10 = 0 as libc::c_int as uint32_t;
    let ref mut fresh11 =
        (*tmp.as_mut_ptr()).word[10 as libc::c_int as usize];
    *fresh11 = *fresh10;
    let ref mut fresh12 =
        (*tmp.as_mut_ptr()).word[11 as libc::c_int as usize];
    *fresh12 = *fresh11;
    let ref mut fresh13 =
        (*tmp.as_mut_ptr()).word[12 as libc::c_int as usize];
    *fresh13 = *fresh12;
    let ref mut fresh14 =
        (*tmp.as_mut_ptr()).word[13 as libc::c_int as usize];
    *fresh14 = *fresh13;
    let ref mut fresh15 =
        (*tmp.as_mut_ptr()).word[14 as libc::c_int as usize];
    *fresh15 = *fresh14;
    (*tmp.as_mut_ptr()).word[15 as libc::c_int as usize] = *fresh15;
    carry =
        bn256_sub(X,
                  &mut *(*q_big.as_mut_ptr()).word.as_mut_ptr().offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                      as *mut uint32_t as *mut bn256,
                  &mut *(*tmp.as_mut_ptr()).word.as_mut_ptr().offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                      as *mut uint32_t as *mut bn256);
    borrow_next =
        ((*q_big.as_mut_ptr()).word[8 as libc::c_int as usize] < carry) as
            libc::c_int as uint32_t;
    let ref mut fresh16 =
        (*q_big.as_mut_ptr()).word[8 as libc::c_int as usize];
    *fresh16 =
        (*fresh16 as libc::c_uint).wrapping_sub(carry) as uint32_t as
            uint32_t;
    borrow_next =
        (borrow_next as
             libc::c_uint).wrapping_add(((*q_big.as_mut_ptr()).word[8 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                             <
                                             (*tmp.as_mut_ptr()).word[8 as
                                                                          libc::c_int
                                                                          as
                                                                          usize])
                                            as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    let ref mut fresh17 =
        (*q_big.as_mut_ptr()).word[8 as libc::c_int as usize];
    *fresh17 =
        (*fresh17 as
             libc::c_uint).wrapping_sub((*tmp.as_mut_ptr()).word[8 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
            as uint32_t as uint32_t;
    carry = (*q_big.as_mut_ptr()).word[8 as libc::c_int as usize];
    if carry != 0 {
        carry =
            (carry as libc::c_uint).wrapping_sub(bn256_sub(X, X, B)) as
                uint32_t as uint32_t
    } else { bn256_sub(q.as_mut_ptr(), X, B); }
    if carry != 0 {
        carry =
            (carry as libc::c_uint).wrapping_sub(bn256_sub(X, X, B)) as
                uint32_t as uint32_t
    } else { bn256_sub(q.as_mut_ptr(), X, B); }
    carry = bn256_sub(q.as_mut_ptr(), X, B);
    if carry != 0 {
        memcpy(q.as_mut_ptr() as *mut libc::c_void, X as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    } else {
        memcpy(X as *mut libc::c_void, q.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<bn256>() as libc::c_ulong);
    };
}
/* *
 * @brief C = X^(-1) mod N
 *
 * Assume X and N are co-prime (or N is prime).
 * NOTE: If X==0, it return 0.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn mod_inv(mut C: *mut bn256, mut X: *const bn256,
                                 mut N: *const bn256) {
    let mut u: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut v: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut A: [bn256; 1] =
        [{
             let mut init =
                 bn256{word:
                           [1 as libc::c_int as uint32_t,
                            0 as libc::c_int as uint32_t,
                            0 as libc::c_int as uint32_t,
                            0 as libc::c_int as uint32_t,
                            0 as libc::c_int as uint32_t,
                            0 as libc::c_int as uint32_t,
                            0 as libc::c_int as uint32_t,
                            0 as libc::c_int as uint32_t],};
             init
         }];
    let mut carry: uint32_t = 0;
    let mut n: libc::c_int =
        3 as libc::c_int * 256 as libc::c_int - 2 as libc::c_int;
    memset(C as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memcpy(u.as_mut_ptr() as *mut libc::c_void, X as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memcpy(v.as_mut_ptr() as *mut libc::c_void, N as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    loop  {
        let fresh18 = n;
        n = n - 1;
        if !(fresh18 != 0) { break ; }
        let mut c: libc::c_int =
            (bn256_is_even(u.as_mut_ptr()) << 1 as libc::c_int) +
                bn256_is_even(v.as_mut_ptr());
        match c {
            3 => {
                bn256_shift(u.as_mut_ptr(), u.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(A.as_mut_ptr()) != 0 {
                    bn256_add(tmp.as_mut_ptr(), A.as_mut_ptr(), N);
                    carry = 0 as libc::c_int as uint32_t
                } else {
                    carry = bn256_add(A.as_mut_ptr(), A.as_mut_ptr(), N)
                }
                bn256_shift(A.as_mut_ptr(), A.as_mut_ptr(),
                            -(1 as libc::c_int));
                let ref mut fresh19 =
                    (*A.as_mut_ptr()).word[7 as libc::c_int as usize];
                *fresh19 |= carry.wrapping_mul(0x80000000 as libc::c_uint);
                bn256_shift(v.as_mut_ptr(), v.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(C) != 0 {
                    bn256_add(tmp.as_mut_ptr(), C, N);
                    carry = 0 as libc::c_int as uint32_t
                } else { carry = bn256_add(C, C, N) }
                bn256_shift(C, C, -(1 as libc::c_int));
                (*C).word[7 as libc::c_int as usize] |=
                    carry.wrapping_mul(0x80000000 as libc::c_uint);
                if bn256_is_ge(tmp.as_mut_ptr(), tmp.as_mut_ptr()) != 0 {
                    bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                              tmp.as_mut_ptr());
                    carry =
                        bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    if carry != 0 {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    } else { bn256_add(tmp.as_mut_ptr(), A.as_mut_ptr(), N); }
                } else {
                    bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                              tmp.as_mut_ptr());
                    carry =
                        bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    if carry != 0 {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    } else {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N);
                    }
                }
            }
            1 => {
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(tmp.as_mut_ptr()) != 0 {
                    bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N);
                    carry = 0 as libc::c_int as uint32_t
                } else {
                    carry = bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N)
                }
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                let ref mut fresh20 =
                    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize];
                *fresh20 |= carry.wrapping_mul(0x80000000 as libc::c_uint);
                bn256_shift(v.as_mut_ptr(), v.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(C) != 0 {
                    bn256_add(tmp.as_mut_ptr(), C, N);
                    carry = 0 as libc::c_int as uint32_t
                } else { carry = bn256_add(C, C, N) }
                bn256_shift(C, C, -(1 as libc::c_int));
                (*C).word[7 as libc::c_int as usize] |=
                    carry.wrapping_mul(0x80000000 as libc::c_uint);
                if bn256_is_ge(tmp.as_mut_ptr(), tmp.as_mut_ptr()) != 0 {
                    bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                              tmp.as_mut_ptr());
                    carry =
                        bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    if carry != 0 {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    } else { bn256_add(tmp.as_mut_ptr(), A.as_mut_ptr(), N); }
                } else {
                    bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                              tmp.as_mut_ptr());
                    carry =
                        bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    if carry != 0 {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    } else {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N);
                    }
                }
            }
            2 => {
                bn256_shift(u.as_mut_ptr(), u.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(A.as_mut_ptr()) != 0 {
                    bn256_add(tmp.as_mut_ptr(), A.as_mut_ptr(), N);
                    carry = 0 as libc::c_int as uint32_t
                } else {
                    carry = bn256_add(A.as_mut_ptr(), A.as_mut_ptr(), N)
                }
                bn256_shift(A.as_mut_ptr(), A.as_mut_ptr(),
                            -(1 as libc::c_int));
                let ref mut fresh21 =
                    (*A.as_mut_ptr()).word[7 as libc::c_int as usize];
                *fresh21 |= carry.wrapping_mul(0x80000000 as libc::c_uint);
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(tmp.as_mut_ptr()) != 0 {
                    bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N);
                    carry = 0 as libc::c_int as uint32_t
                } else {
                    carry = bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N)
                }
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                let ref mut fresh22 =
                    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize];
                *fresh22 |= carry.wrapping_mul(0x80000000 as libc::c_uint);
                if bn256_is_ge(tmp.as_mut_ptr(), tmp.as_mut_ptr()) != 0 {
                    bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                              tmp.as_mut_ptr());
                    carry =
                        bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    if carry != 0 {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    } else { bn256_add(tmp.as_mut_ptr(), A.as_mut_ptr(), N); }
                } else {
                    bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                              tmp.as_mut_ptr());
                    carry =
                        bn256_sub(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    if carry != 0 {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                                  tmp.as_mut_ptr());
                    } else {
                        bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N);
                    }
                }
            }
            0 => {
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(tmp.as_mut_ptr()) != 0 {
                    bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N);
                    carry = 0 as libc::c_int as uint32_t
                } else {
                    carry = bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N)
                }
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                let ref mut fresh23 =
                    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize];
                *fresh23 |= carry.wrapping_mul(0x80000000 as libc::c_uint);
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                if bn256_is_even(tmp.as_mut_ptr()) != 0 {
                    bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N);
                    carry = 0 as libc::c_int as uint32_t
                } else {
                    carry = bn256_add(tmp.as_mut_ptr(), tmp.as_mut_ptr(), N)
                }
                bn256_shift(tmp.as_mut_ptr(), tmp.as_mut_ptr(),
                            -(1 as libc::c_int));
                let ref mut fresh24 =
                    (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize];
                *fresh24 |= carry.wrapping_mul(0x80000000 as libc::c_uint);
                if bn256_is_ge(u.as_mut_ptr(), v.as_mut_ptr()) != 0 {
                    bn256_sub(u.as_mut_ptr(), u.as_mut_ptr(), v.as_mut_ptr());
                    carry = bn256_sub(A.as_mut_ptr(), A.as_mut_ptr(), C);
                    if carry != 0 {
                        bn256_add(A.as_mut_ptr(), A.as_mut_ptr(), N);
                    } else { bn256_add(tmp.as_mut_ptr(), A.as_mut_ptr(), N); }
                } else {
                    bn256_sub(v.as_mut_ptr(), v.as_mut_ptr(), u.as_mut_ptr());
                    carry = bn256_sub(C, C, A.as_mut_ptr());
                    if carry != 0 {
                        bn256_add(C, C, N);
                    } else { bn256_add(tmp.as_mut_ptr(), C, N); }
                }
            }
            _ => { }
        }
    };
}
