#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn gnuk_malloc(_: size_t) -> *mut libc::c_void;
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
    fn mod_inv(X: *mut bn256, A: *const bn256, N: *const bn256);
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
    fn sha512_start(ctx: *mut sha512_context);
    #[no_mangle]
    fn sha512_finish(ctx: *mut sha512_context, output: *mut libc::c_uchar);
    #[no_mangle]
    fn sha512_update(ctx: *mut sha512_context, input: *const libc::c_uchar,
                     ilen: libc::c_uint);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct bn512 {
    pub word: [uint32_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_context {
    pub total: [uint64_t; 2],
    pub state: [uint64_t; 8],
    pub wbuf: [uint64_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptc {
    pub x: [bn256; 1],
    pub y: [bn256; 1],
    pub z: [bn256; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ac {
    pub x: [bn256; 1],
    pub y: [bn256; 1],
}
/*                                                    -*- coding: utf-8 -*-
 * ecc-edwards.c - Elliptic curve computation for
 *                 the twisted Edwards curve: -x^2 + y^2 = 1 + d*x^2*y^2
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
 * References:
 *
 * [1] Daniel J. Bernstein, Niels Duif, Tanja Lange, Peter Schwabe, Bo-Yin Yang.
 *     High-speed high-security signatures.
 *     Journal of Cryptographic Engineering 2 (2012), 77--89.
 *     http://cr.yp.to/papers.html#ed25519
 *
 * [2] Daniel J. Bernstein, Peter Birkner, Marc Joye, Tanja Lange,
 *     Christiane Peters.
 *     Twisted Edwards curves.
 *     Pages 389--405 in Progress in cryptology---AFRICACRYPT 2008.
 *     http://cr.yp.to/papers.html#twisted
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
 * (2) We use fixed base comb multiplication.  Scalar is 252-bit.
 *     There are various possible choices for 252 = 2 * 2 * 3 * 3 * 7.
 *     Current choice of total size is 3KB.  We use three tables, and
 *     a table has 16 points (3 * 1KB).
 *
 *     Window size W = 4-bit, E = 21.
 *                                                       <--21-bit-
 *                                             <---42-bit----------
 *     [        ][########][////////][        ][########][////////]
 *                                   <-------63-bit----------------
 *                         <-----------84-bit----------------------
 *               <--------------105-bit----------------------------
 *
 *     [        ][########][////////][        ][########][////////]
 *                                                                 <-126-bit-
 *                                                       <-147-bit-
 *                                             <----168-bit--------
 *
 *                                   <-------189-bit---------------
 *                         <----------210-bit----------------------
 *               <-------------231-bit-----------------------------
 */
/*
 * Identity element: (0,1)
 * Negation: -(x,y) = (-x,y)
 *
 * d: -0x2DFC9311D490018C7338BF8688861767FF8FF5B2BEBE27548A14B235ECA6874A
 * order:
 *     0x1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3ED
 * Gx: 0x216936D3CD6E53FEC0A4E231FDD6DC5C692CC7609525A7B2C9562D608F25D51A
 * Gy: 0x6666666666666666666666666666666666666666666666666666666666666658
 */
/* d + 2^255 - 19 */
static mut coefficient_d: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0x135978a3 as libc::c_int as uint32_t,
                        0x75eb4dca as libc::c_int as uint32_t,
                        0x4141d8ab as libc::c_int as uint32_t,
                        0x700a4d as libc::c_int as uint32_t,
                        0x7779e898 as libc::c_int as uint32_t,
                        0x8cc74079 as libc::c_uint,
                        0x2b6ffe73 as libc::c_int as uint32_t,
                        0x52036cee as libc::c_int as uint32_t],};
         init
     }];
unsafe extern "C" fn mod25519_is_neg(mut a: *const bn256) -> libc::c_int {
    return ((*a).word[0 as libc::c_int as usize] &
                1 as libc::c_int as libc::c_uint) as libc::c_int;
}
/* *
 * @brief  X = 2 * A
 *
 * Compute (X3 : Y3 : Z3) = 2 * (X1 : Y1 : Z1)
 */
unsafe extern "C" fn point_double(mut X: *mut ptc, mut A: *const ptc) {
    let mut b: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut d: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut e: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    /* Compute: B = (X1 + Y1)^2 */
    mod25638_add(b.as_mut_ptr(), (*A).x.as_ptr(), (*A).y.as_ptr());
    mod25638_sqr(b.as_mut_ptr(), b.as_mut_ptr());
    /* Compute: C = X1^2        : E      */
    mod25638_sqr(e.as_mut_ptr(), (*A).x.as_ptr());
    /* Compute: D = Y1^2             */
    mod25638_sqr(d.as_mut_ptr(), (*A).y.as_ptr());
    /* E = aC; where a = -1 */
  /* Compute: D - E = D + C : Y3_tmp */
    mod25638_add((*X).y.as_mut_ptr(), e.as_mut_ptr(), d.as_mut_ptr());
    /* Compute: -F = -(E + D) = C - D; where a = -1 : E */
    mod25638_sub(e.as_mut_ptr(), e.as_mut_ptr(), d.as_mut_ptr());
    /* Compute: H = Z1^2        : D     */
    mod25638_sqr(d.as_mut_ptr(), (*A).z.as_ptr());
    /* Compute: -J = 2*H - F    : D     */
    mod25638_add(d.as_mut_ptr(), d.as_mut_ptr(), d.as_mut_ptr());
    mod25638_add(d.as_mut_ptr(), d.as_mut_ptr(), e.as_mut_ptr());
    /* Compute: X3 = (B-C-D)*J = -J*(C+D-B) = -J*(Y3_tmp-B)  */
    mod25638_sub((*X).x.as_mut_ptr(), (*X).y.as_mut_ptr(), b.as_mut_ptr());
    mod25638_mul((*X).x.as_mut_ptr(), (*X).x.as_mut_ptr(), d.as_mut_ptr());
    /* Compute: Y3 = -F*(D-E) = -F*Y3_tmp            */
    mod25638_mul((*X).y.as_mut_ptr(), (*X).y.as_mut_ptr(), e.as_mut_ptr());
    /* Z3 = -F*-J             */
    mod25638_mul((*X).z.as_mut_ptr(), e.as_mut_ptr(), d.as_mut_ptr());
}
/* *
 * @brief	X = A + B
 *
 * @param X	Destination PTC
 * @param A	PTC
 * @param B	AC
 *
 * Compute: (X3 : Y3 : Z3) = (X1 : Y1 : Z1) + (X2 : Y2 : 1)
 */
unsafe extern "C" fn point_add(mut X: *mut ptc, mut A: *const ptc,
                               mut B: *const ac) {
    let mut c: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut d: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut e: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    /* Compute: C = X1 * X2 */
    mod25638_mul(c.as_mut_ptr(), (*A).x.as_ptr(), (*B).x.as_ptr());
    /* Compute: D = Y1 * Y2 */
    mod25638_mul(d.as_mut_ptr(), (*A).y.as_ptr(), (*B).y.as_ptr());
    /* Compute: E = d * C * D */
    mod25638_mul(e.as_mut_ptr(), c.as_mut_ptr(), d.as_mut_ptr());
    mod25638_mul(e.as_mut_ptr(), coefficient_d.as_ptr(), e.as_mut_ptr());
    /* Compute: C_1 = C + D */
    mod25638_add(c.as_mut_ptr(), c.as_mut_ptr(), d.as_mut_ptr());
    /* Compute: D_1 = Z1^2 : B */
    mod25638_sqr(d.as_mut_ptr(), (*A).z.as_ptr());
    /* tmp = D_1 - E : F */
    mod25638_sub(tmp.as_mut_ptr(), d.as_mut_ptr(), e.as_mut_ptr());
    /* D_2 = D_1 + E : G */
    mod25638_add(d.as_mut_ptr(), d.as_mut_ptr(), e.as_mut_ptr());
    /* X3_final = Z1 * tmp * ((X1 + Y1) * (X2 + Y2) - C_1) */
    mod25638_add((*X).x.as_mut_ptr(), (*A).x.as_ptr(), (*A).y.as_ptr());
    mod25638_add(e.as_mut_ptr(), (*B).x.as_ptr(), (*B).y.as_ptr());
    mod25638_mul(e.as_mut_ptr(), (*X).x.as_mut_ptr(), e.as_mut_ptr());
    mod25638_sub(e.as_mut_ptr(), e.as_mut_ptr(), c.as_mut_ptr());
    mod25638_mul(e.as_mut_ptr(), tmp.as_mut_ptr(), e.as_mut_ptr());
    mod25638_mul((*X).x.as_mut_ptr(), (*A).z.as_ptr(), e.as_mut_ptr());
    /* Y3_final = Z1 * D_2 * C_1 */
    mod25638_mul(c.as_mut_ptr(), d.as_mut_ptr(), c.as_mut_ptr());
    mod25638_mul((*X).y.as_mut_ptr(), (*A).z.as_ptr(), c.as_mut_ptr());
    /* Z3_final = tmp * D_2 */
    mod25638_mul((*X).z.as_mut_ptr(), tmp.as_mut_ptr(), d.as_mut_ptr());
    /* A = Z1 */
  /* B = A^2 */
  /* C = X1 * X2 */
  /* D = Y1 * Y2 */
  /* E = d * C * D */
  /* F = B - E */
  /* G = B + E */
  /* X3 = A * F * ((X1 + Y1) * (X2 + Y2) - C - D) */
  /* Y3 = A * G * (D - aC); where a = -1 */
  /* Z3 = F * G */
}
/* *
 * @brief	X = convert A
 *
 * @param X	Destination AC
 * @param A	PTC
 *
 * (X1:Y1:Z1) represents the affine point (x=X1/Z1, y=Y1/Z1)
 */
unsafe extern "C" fn point_ptc_to_ac(mut X: *mut ac, mut A: *const ptc) {
    let mut z_inv: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    /*
   * A->z may be bigger than p25519, or two times bigger than p25519.
   * But this is no problem for computation of mod_inv.
   */
    mod_inv(z_inv.as_mut_ptr(), (*A).z.as_ptr(), p25519.as_ptr());
    mod25638_mul((*X).x.as_mut_ptr(), (*A).x.as_ptr(), z_inv.as_mut_ptr());
    mod25519_reduce((*X).x.as_mut_ptr());
    mod25638_mul((*X).y.as_mut_ptr(), (*A).y.as_ptr(), z_inv.as_mut_ptr());
    mod25519_reduce((*X).y.as_mut_ptr());
}
static mut precomputed_KG: [ac; 16] =
    [{
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t],};
                         init
                     }],
                y:
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
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x8f25d51a as libc::c_uint,
                                        0xc9562d60 as libc::c_uint,
                                        0x9525a7b2 as libc::c_uint,
                                        0x692cc760 as libc::c_int as uint32_t,
                                        0xfdd6dc5c as libc::c_uint,
                                        0xc0a4e231 as libc::c_uint,
                                        0xcd6e53fe as libc::c_uint,
                                        0x216936d3 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x66666658 as libc::c_int as uint32_t,
                                        0x66666666 as libc::c_int as uint32_t,
                                        0x66666666 as libc::c_int as uint32_t,
                                        0x66666666 as libc::c_int as uint32_t,
                                        0x66666666 as libc::c_int as uint32_t,
                                        0x66666666 as libc::c_int as uint32_t,
                                        0x66666666 as libc::c_int as uint32_t,
                                        0x66666666 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x3713af22 as libc::c_int as uint32_t,
                                        0xac7137bd as libc::c_uint,
                                        0xac634604 as libc::c_uint,
                                        0x25ed77a4 as libc::c_int as uint32_t,
                                        0xa815e038 as libc::c_uint,
                                        0xce0d0064 as libc::c_uint,
                                        0xbca90151 as libc::c_uint,
                                        0x41c030f as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x780f989 as libc::c_int as uint32_t,
                                        0xe9b33fcf as libc::c_uint,
                                        0x3d4445e7 as libc::c_int as uint32_t,
                                        0xe4e97c2a as libc::c_uint,
                                        0x655e5c16 as libc::c_int as uint32_t,
                                        0xc67dc71c as libc::c_uint,
                                        0xee43fb7a as libc::c_uint,
                                        0x72467625 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x3ee99893 as libc::c_int as uint32_t,
                                        0x76a19171 as libc::c_int as uint32_t,
                                        0x7ba9b065 as libc::c_int as uint32_t,
                                        0xe647edd9 as libc::c_uint,
                                        0x6aeae260 as libc::c_int as uint32_t,
                                        0x31f39299 as libc::c_int as uint32_t,
                                        0x5f4a9bb2 as libc::c_int as uint32_t,
                                        0x6d9e4545 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x94cae280 as libc::c_uint,
                                        0xc41433da as libc::c_uint,
                                        0x79061211 as libc::c_int as uint32_t,
                                        0x8e842de8 as libc::c_uint,
                                        0xa259dc8a as libc::c_uint,
                                        0xaab95e0b as libc::c_uint,
                                        0x99013cd0 as libc::c_uint,
                                        0x28bd5fc3 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x7d23ea24 as libc::c_int as uint32_t,
                                        0x59e22c56 as libc::c_int as uint32_t,
                                        0x460850e as libc::c_int as uint32_t,
                                        0x1e745a88 as libc::c_int as uint32_t,
                                        0xda13ef4b as libc::c_uint,
                                        0x4583ff4c as libc::c_int as uint32_t,
                                        0x95083f85 as libc::c_uint,
                                        0x1f13202c as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x90275f48 as libc::c_uint,
                                        0xad42025c as libc::c_uint,
                                        0xb55c4778 as libc::c_uint,
                                        0x85087e as libc::c_int as uint32_t,
                                        0xfdfd7ffa as libc::c_uint,
                                        0xf21109e7 as libc::c_uint,
                                        0x6c381b7e as libc::c_int as uint32_t,
                                        0x66336d35 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd00851f2 as libc::c_uint,
                                        0xaa9476ab as libc::c_uint,
                                        0x4a61600b as libc::c_int as uint32_t,
                                        0xe7838534 as libc::c_uint,
                                        0x1a52df87 as libc::c_int as uint32_t,
                                        0xde65625 as libc::c_int as uint32_t,
                                        0xbd675870 as libc::c_uint,
                                        0x5f0dd494 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe23493ba as libc::c_uint,
                                        0xf20aec1b as libc::c_uint,
                                        0x3414b0a8 as libc::c_int as uint32_t,
                                        0x8f7f2741 as libc::c_uint,
                                        0xa80e1eb6 as libc::c_uint,
                                        0x497e74bd as libc::c_int as uint32_t,
                                        0xe9365b15 as libc::c_uint,
                                        0x1648eaac as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x4ac2b69 as libc::c_int as uint32_t,
                                        0x5b78dcec as libc::c_int as uint32_t,
                                        0x32001a73 as libc::c_int as uint32_t,
                                        0xecdb66ce as libc::c_uint,
                                        0xb34cf697 as libc::c_uint,
                                        0xb75832f4 as libc::c_uint,
                                        0x3a2bce94 as libc::c_int as uint32_t,
                                        0x7aaf57c5 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x60fdfc6f as libc::c_int as uint32_t,
                                        0xb32ed2ce as libc::c_uint,
                                        0x757924c6 as libc::c_int as uint32_t,
                                        0x77bf20be as libc::c_int as uint32_t,
                                        0x48742dd1 as libc::c_int as uint32_t,
                                        0xaebd15dd as libc::c_uint,
                                        0x55d38439 as libc::c_int as uint32_t,
                                        0x6311bb16 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x42ff5c97 as libc::c_int as uint32_t,
                                        0x139cdd73 as libc::c_int as uint32_t,
                                        0xdbd82964 as libc::c_uint,
                                        0xee4c359e as libc::c_uint,
                                        0x70611a3f as libc::c_int as uint32_t,
                                        0x91c1cd94 as libc::c_uint,
                                        0x8075dbcb as libc::c_uint,
                                        0x1d0c34f6 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5f931219 as libc::c_int as uint32_t,
                                        0x43eaa549 as libc::c_int as uint32_t,
                                        0xa23d35a6 as libc::c_uint,
                                        0x3737aba7 as libc::c_int as uint32_t,
                                        0x46f167bb as libc::c_int as uint32_t,
                                        0x54b1992f as libc::c_int as uint32_t,
                                        0xb74a9944 as libc::c_uint,
                                        0x1a11f3c as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xba46b161 as libc::c_uint,
                                        0x67a5310e as libc::c_int as uint32_t,
                                        0xd9d67f6c as libc::c_uint,
                                        0x790f8527 as libc::c_int as uint32_t,
                                        0x2f6cc814 as libc::c_int as uint32_t,
                                        0x359c5b5f as libc::c_int as uint32_t,
                                        0x7786383d as libc::c_int as uint32_t,
                                        0x7b6a5565 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x663ab0d3 as libc::c_int as uint32_t,
                                        0xf1431b60 as libc::c_uint,
                                        0x9995826 as libc::c_int as uint32_t,
                                        0x14a32d8f as libc::c_int as uint32_t,
                                        0xeddb8571 as libc::c_uint,
                                        0x61d526f6 as libc::c_int as uint32_t,
                                        0xeac739a as libc::c_int as uint32_t,
                                        0xcb7acea as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x4a2d009f as libc::c_int as uint32_t,
                                        0x5eb1a697 as libc::c_int as uint32_t,
                                        0xd8df987a as libc::c_uint,
                                        0xdacb43b4 as libc::c_uint,
                                        0x8397f958 as libc::c_uint,
                                        0x4870f214 as libc::c_int as uint32_t,
                                        0x8a175fbb as libc::c_uint,
                                        0x5aa0c67c as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x78887db3 as libc::c_int as uint32_t,
                                        0x27dbbd4c as libc::c_int as uint32_t,
                                        0x64e322ab as libc::c_int as uint32_t,
                                        0xe327b707 as libc::c_uint,
                                        0x7cbe4e3b as libc::c_int as uint32_t,
                                        0x87e293fa as libc::c_uint,
                                        0xbda72395 as libc::c_uint,
                                        0x17040799 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x99d1e696 as libc::c_uint,
                                        0xc833a5a2 as libc::c_uint,
                                        0x2d9d5877 as libc::c_int as uint32_t,
                                        0x969bff8e as libc::c_uint,
                                        0x2216fa67 as libc::c_int as uint32_t,
                                        0x383a533a as libc::c_int as uint32_t,
                                        0x684d3925 as libc::c_int as uint32_t,
                                        0x338bbe0a as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd6cfb491 as libc::c_uint,
                                        0x35b5aae8 as libc::c_int as uint32_t,
                                        0xaa12f3f8 as libc::c_uint,
                                        0x4a588279 as libc::c_int as uint32_t,
                                        0x2e30380e as libc::c_int as uint32_t,
                                        0xa7c2e708 as libc::c_uint,
                                        0x9e4b3d62 as libc::c_uint,
                                        0x69f13e09 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x27f1cd56 as libc::c_int as uint32_t,
                                        0xec0dc2ef as libc::c_uint,
                                        0xdb11cc97 as libc::c_uint,
                                        0x1af11548 as libc::c_int as uint32_t,
                                        0x9ebc7613 as libc::c_uint,
                                        0xb642f86a as libc::c_uint,
                                        0xcb77c3b9 as libc::c_uint,
                                        0x5ce45e73 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x3eddd6de as libc::c_int as uint32_t,
                                        0x5d128786 as libc::c_int as uint32_t,
                                        0x4859eab7 as libc::c_int as uint32_t,
                                        0x16f9a6b4 as libc::c_int as uint32_t,
                                        0xd8782345 as libc::c_uint,
                                        0x55c53916 as libc::c_int as uint32_t,
                                        0xdb7b202a as libc::c_uint,
                                        0x6b1dfa87 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x19e30528 as libc::c_int as uint32_t,
                                        0x2461a8ed as libc::c_int as uint32_t,
                                        0x665cfb1c as libc::c_int as uint32_t,
                                        0xaf756bf9 as libc::c_uint,
                                        0x3a6e8673 as libc::c_int as uint32_t,
                                        0xfcafd1d as libc::c_int as uint32_t,
                                        0x45d10f48 as libc::c_int as uint32_t,
                                        0xd264435 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5431db67 as libc::c_int as uint32_t,
                                        0x543fd4c6 as libc::c_int as uint32_t,
                                        0x60932432 as libc::c_int as uint32_t,
                                        0xc153a5b3 as libc::c_uint,
                                        0xd2119aa4 as libc::c_uint,
                                        0x41d5b8eb as libc::c_int as uint32_t,
                                        0x8b09b6a5 as libc::c_uint,
                                        0x36bd9ab4 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x21e06738 as libc::c_int as uint32_t,
                                        0x6d39f935 as libc::c_int as uint32_t,
                                        0x3765dd86 as libc::c_int as uint32_t,
                                        0x4e6a7c59 as libc::c_int as uint32_t,
                                        0xa4730880 as libc::c_uint,
                                        0xefc0dd80 as libc::c_uint,
                                        0x4079fe2f as libc::c_int as uint32_t,
                                        0x40617e56 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x921439b9 as libc::c_uint,
                                        0xbc83cdff as libc::c_uint,
                                        0x98833c09 as libc::c_uint,
                                        0xd5cccc06 as libc::c_uint,
                                        0xda13cdcb as libc::c_uint,
                                        0xe315c425 as libc::c_uint,
                                        0x67ff5370 as libc::c_int as uint32_t,
                                        0x37bc6e84 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xf643b5f5 as libc::c_uint,
                                        0x65e7f028 as libc::c_int as uint32_t,
                                        0xffbf5a8 as libc::c_int as uint32_t,
                                        0x5b0d4831 as libc::c_int as uint32_t,
                                        0xf4085f62 as libc::c_uint,
                                        0xf540498 as libc::c_int as uint32_t,
                                        0xdb7bd1b as libc::c_int as uint32_t,
                                        0x6f0bb035 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x9733742c as libc::c_uint,
                                        0x51f65571 as libc::c_int as uint32_t,
                                        0xf513409f as libc::c_uint,
                                        0x2fc047a0 as libc::c_int as uint32_t,
                                        0x355facf6 as libc::c_int as uint32_t,
                                        0x7f45010 as libc::c_int as uint32_t,
                                        0x3a989a9c as libc::c_int as uint32_t,
                                        0x5cd416a9 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x748f2a67 as libc::c_int as uint32_t,
                                        0xbdd7208 as libc::c_int as uint32_t,
                                        0x415b7f7f as libc::c_int as uint32_t,
                                        0xcf0b80b as libc::c_int as uint32_t,
                                        0x57aa0119 as libc::c_int as uint32_t,
                                        0x44afdd5f as libc::c_int as uint32_t,
                                        0x430dc946 as libc::c_int as uint32_t,
                                        0x5d68802 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x1a60eeb2 as libc::c_int as uint32_t,
                                        0x420c46e5 as libc::c_int as uint32_t,
                                        0x665024f5 as libc::c_int as uint32_t,
                                        0xc60a9b33 as libc::c_uint,
                                        0x48c51347 as libc::c_int as uint32_t,
                                        0x37520265 as libc::c_int as uint32_t,
                                        0xa21bfb as libc::c_int as uint32_t,
                                        0x6f4be0af as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     }];
static mut precomputed_2E_KG: [ac; 16] =
    [{
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t],};
                         init
                     }],
                y:
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
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x199c4f7d as libc::c_int as uint32_t,
                                        0xec314ac0 as libc::c_uint,
                                        0xb2ebaaf9 as libc::c_uint,
                                        0x66a39c16 as libc::c_int as uint32_t,
                                        0xedd4d15f as libc::c_uint,
                                        0xab1c92b8 as libc::c_uint,
                                        0x57d9eada as libc::c_int as uint32_t,
                                        0x482a4cdf as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x6e4eb04b as libc::c_int as uint32_t,
                                        0xbd513b11 as libc::c_uint,
                                        0x25e4fd6a as libc::c_int as uint32_t,
                                        0x3f115fa5 as libc::c_int as uint32_t,
                                        0x14519298 as libc::c_int as uint32_t,
                                        0xb3c5fc6 as libc::c_int as uint32_t,
                                        0x81c2f7a8 as libc::c_uint,
                                        0x7391de43 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x1254fe02 as libc::c_int as uint32_t,
                                        0xa57dca18 as libc::c_uint,
                                        0x6da34368 as libc::c_int as uint32_t,
                                        0xa56a2a14 as libc::c_uint,
                                        0x63e7328e as libc::c_int as uint32_t,
                                        0x44c6e34f as libc::c_int as uint32_t,
                                        0xca63ab3e as libc::c_uint,
                                        0x3f748617 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x7dc1641e as libc::c_int as uint32_t,
                                        0x5a13dc52 as libc::c_int as uint32_t,
                                        0xee4e9ca1 as libc::c_uint,
                                        0x4cbb2899 as libc::c_int as uint32_t,
                                        0x1ba9acee as libc::c_int as uint32_t,
                                        0x3938a289 as libc::c_int as uint32_t,
                                        0x420fc47b as libc::c_int as uint32_t,
                                        0xfed89e6 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x49cbad08 as libc::c_int as uint32_t,
                                        0x3c193f32 as libc::c_int as uint32_t,
                                        0x15e80ef5 as libc::c_int as uint32_t,
                                        0xdda71ef1 as libc::c_uint,
                                        0x9d128c33 as libc::c_uint,
                                        0xda44186c as libc::c_uint,
                                        0xbf98c24f as libc::c_uint,
                                        0x54183ede as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x93d165c1 as libc::c_uint,
                                        0x2cb483f7 as libc::c_int as uint32_t,
                                        0x177f44aa as libc::c_int as uint32_t,
                                        0x51762ace as libc::c_int as uint32_t,
                                        0xb4ab035d as libc::c_uint,
                                        0xb3fe651b as libc::c_uint,
                                        0xa0b0d4e5 as libc::c_uint,
                                        0x426c99c3 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xef3f3fb1 as libc::c_uint,
                                        0xb3fcf4d8 as libc::c_uint,
                                        0x65060a0 as libc::c_int as uint32_t,
                                        0x7052292b as libc::c_int as uint32_t,
                                        0x24240b15 as libc::c_int as uint32_t,
                                        0x18795ff8 as libc::c_int as uint32_t,
                                        0x9989ffcc as libc::c_uint,
                                        0x13aea184 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc2b81f44 as libc::c_uint,
                                        0x1930c101 as libc::c_int as uint32_t,
                                        0x10600555 as libc::c_int as uint32_t,
                                        0x672d6ca4 as libc::c_int as uint32_t,
                                        0x1b25e570 as libc::c_int as uint32_t,
                                        0xfbddbff2 as libc::c_uint,
                                        0x8ca12b70 as libc::c_uint,
                                        0x884949c as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x564bbf as libc::c_int as uint32_t,
                                        0x9983a033 as libc::c_uint,
                                        0xde61b72d as libc::c_uint,
                                        0x95587d25 as libc::c_uint,
                                        0xeb17ad71 as libc::c_uint,
                                        0xb6719dfb as libc::c_uint,
                                        0xc0bc3517 as libc::c_uint,
                                        0x46871ad0 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe95a6693 as libc::c_uint,
                                        0xb034fb61 as libc::c_uint,
                                        0x76eabad9 as libc::c_int as uint32_t,
                                        0x5b0d8d18 as libc::c_int as uint32_t,
                                        0x884785dc as libc::c_uint,
                                        0xad295dd0 as libc::c_uint,
                                        0x74a1276a as libc::c_int as uint32_t,
                                        0x359debad as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe89fb5ca as libc::c_uint,
                                        0x2e5a2686 as libc::c_int as uint32_t,
                                        0x5656c6c5 as libc::c_int as uint32_t,
                                        0xd3d200ba as libc::c_uint,
                                        0x9c969001 as libc::c_uint,
                                        0xef4c051e as libc::c_uint,
                                        0x2cb45f4 as libc::c_int as uint32_t,
                                        0xd4ea946 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x76d6e506 as libc::c_int as uint32_t,
                                        0xa6f8a422 as libc::c_uint,
                                        0x63209e23 as libc::c_int as uint32_t,
                                        0x454c768f as libc::c_int as uint32_t,
                                        0x2b372386 as libc::c_int as uint32_t,
                                        0x5c12fd04 as libc::c_int as uint32_t,
                                        0xdbfee11f as libc::c_uint,
                                        0x1aedbd3e as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xdbf569 as libc::c_int as uint32_t,
                                        0x700ab50f as libc::c_int as uint32_t,
                                        0xd335b313 as libc::c_uint,
                                        0x9553643c as libc::c_uint,
                                        0xa17dc97e as libc::c_uint,
                                        0xeea9bddf as libc::c_uint,
                                        0x3350a2bd as libc::c_int as uint32_t,
                                        0xd12fe3d as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xa16a3dee as libc::c_uint,
                                        0xe5ac35fe as libc::c_uint,
                                        0xf81950c3 as libc::c_uint,
                                        0x4ae4664a as libc::c_int as uint32_t,
                                        0x3dbbf921 as libc::c_int as uint32_t,
                                        0x75c63df4 as libc::c_int as uint32_t,
                                        0x2958a5a6 as libc::c_int as uint32_t,
                                        0x545b109c as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xa61b29c as libc::c_int as uint32_t,
                                        0xd7a52a98 as libc::c_uint,
                                        0x65aca9ee as libc::c_int as uint32_t,
                                        0xe21e0acb as libc::c_uint,
                                        0x5985dcbe as libc::c_int as uint32_t,
                                        0x57a69c0f as libc::c_int as uint32_t,
                                        0xeb87a534 as libc::c_uint,
                                        0x3c0c1e7b as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x6384bd2f as libc::c_int as uint32_t,
                                        0xf0a0b50d as libc::c_uint,
                                        0xc6939e4b as libc::c_uint,
                                        0xff349a34 as libc::c_uint,
                                        0x6e2f1973 as libc::c_int as uint32_t,
                                        0x922c4554 as libc::c_uint,
                                        0xf1347631 as libc::c_uint,
                                        0x74e826b2 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xa655803c as libc::c_uint,
                                        0xd7eaa066 as libc::c_uint,
                                        0x38292c5c as libc::c_int as uint32_t,
                                        0x9504e76 as libc::c_int as uint32_t,
                                        0x2c874953 as libc::c_int as uint32_t,
                                        0xe298a02e as libc::c_uint,
                                        0x8932b73f as libc::c_uint,
                                        0x225093ed as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe69c3efd as libc::c_uint,
                                        0xf93e2b4d as libc::c_uint,
                                        0x8a87c799 as libc::c_uint,
                                        0xa2cbd5fc as libc::c_uint,
                                        0x85dba986 as libc::c_uint,
                                        0xdf41da94 as libc::c_uint,
                                        0xccee8edc as libc::c_uint,
                                        0x36fe85e7 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x7d742813 as libc::c_int as uint32_t,
                                        0x78df7dc5 as libc::c_int as uint32_t,
                                        0x4a193e64 as libc::c_int as uint32_t,
                                        0x333bcc6d as libc::c_int as uint32_t,
                                        0x6a966d2d as libc::c_int as uint32_t,
                                        0x8242aa25 as libc::c_uint,
                                        0x4cd36d32 as libc::c_int as uint32_t,
                                        0x3500a94 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x580505d7 as libc::c_int as uint32_t,
                                        0xd5d110fc as libc::c_uint,
                                        0xfa11e1e9 as libc::c_uint,
                                        0xb2f47e16 as libc::c_uint,
                                        0x6eab6b4 as libc::c_int as uint32_t,
                                        0xd0030f92 as libc::c_uint,
                                        0x62c91d46 as libc::c_int as uint32_t,
                                        0x2dc80d5f as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x2a75e492 as libc::c_int as uint32_t,
                                        0x5788b01a as libc::c_int as uint32_t,
                                        0xbae31352 as libc::c_uint,
                                        0x992acf54 as libc::c_uint,
                                        0x8159db27 as libc::c_uint,
                                        0x4591b980 as libc::c_int as uint32_t,
                                        0xd3d84740 as libc::c_uint,
                                        0x36c6533c as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x103883b5 as libc::c_int as uint32_t,
                                        0xc44c7c00 as libc::c_uint,
                                        0x515d0820 as libc::c_int as uint32_t,
                                        0x10329423 as libc::c_int as uint32_t,
                                        0x71b9dc16 as libc::c_int as uint32_t,
                                        0xbd306903 as libc::c_uint,
                                        0xf88f8d32 as libc::c_uint,
                                        0x7edd5a95 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5523d7 as libc::c_int as uint32_t,
                                        0xfd63b1ac as libc::c_uint,
                                        0xad70dd21 as libc::c_uint,
                                        0x74482e0d as libc::c_int as uint32_t,
                                        0x2b56105 as libc::c_int as uint32_t,
                                        0x67c9d9d0 as libc::c_int as uint32_t,
                                        0x5971b456 as libc::c_int as uint32_t,
                                        0x4d318012 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x841106df as libc::c_uint,
                                        0xdc9a6f6d as libc::c_uint,
                                        0xa326987f as libc::c_uint,
                                        0x7c52ed9d as libc::c_int as uint32_t,
                                        0x607ea0 as libc::c_int as uint32_t,
                                        0x4dbeaa6f as libc::c_int as uint32_t,
                                        0x6959e688 as libc::c_int as uint32_t,
                                        0x115c221d as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc80f7c16 as libc::c_uint,
                                        0xf8718464 as libc::c_uint,
                                        0xe9930634 as libc::c_uint,
                                        0x5dc8f40 as libc::c_int as uint32_t,
                                        0xc2e9d5f4 as libc::c_uint,
                                        0xefa699bb as libc::c_uint,
                                        0x21da209 as libc::c_int as uint32_t,
                                        0x2469e813 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc602a3c4 as libc::c_uint,
                                        0x75c02845 as libc::c_int as uint32_t,
                                        0xa200f9d as libc::c_int as uint32_t,
                                        0x49d1b2ce as libc::c_int as uint32_t,
                                        0x2fb3ec8f as libc::c_int as uint32_t,
                                        0xd21b75e4 as libc::c_uint,
                                        0xd72a7545 as libc::c_uint,
                                        0x10dd726a as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x63ef1a6c as libc::c_int as uint32_t,
                                        0xeda58527 as libc::c_uint,
                                        0x51705e0 as libc::c_int as uint32_t,
                                        0xb3fc0e72 as libc::c_uint,
                                        0x44f1161f as libc::c_int as uint32_t,
                                        0xbda6f3ee as libc::c_uint,
                                        0xf339efe5 as libc::c_uint,
                                        0x7680aebf as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xb1b070a7 as libc::c_uint,
                                        0xe8d3fd01 as libc::c_uint,
                                        0xdbfbaaa0 as libc::c_uint,
                                        0xc3ff7dbf as libc::c_uint,
                                        0xa320c916 as libc::c_uint,
                                        0xd81ef6f2 as libc::c_uint,
                                        0x62a3b54d as libc::c_int as uint32_t,
                                        0x3e22a1fb as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xb1fa18c8 as libc::c_uint,
                                        0xcdbb9187 as libc::c_uint,
                                        0xcb483a17 as libc::c_uint,
                                        0x8ddb5f6b as libc::c_uint,
                                        0xea49af98 as libc::c_uint,
                                        0xc0a880b9 as libc::c_uint,
                                        0xf2dfddd0 as libc::c_uint,
                                        0x53bf600b as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x9e25b164 as libc::c_uint,
                                        0x4217404c as libc::c_int as uint32_t,
                                        0xafb74aa7 as libc::c_uint,
                                        0xfabf06ee as libc::c_uint,
                                        0x2b9f233c as libc::c_int as uint32_t,
                                        0xb17712ae as libc::c_uint,
                                        0xd0eb909e as libc::c_uint,
                                        0x71f0b344 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     }];
static mut precomputed_4E_KG: [ac; 16] =
    [{
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t,
                                        0 as libc::c_int as uint32_t],};
                         init
                     }],
                y:
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
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe388a820 as libc::c_uint,
                                        0xbb6ec091 as libc::c_uint,
                                        0x5182278a as libc::c_int as uint32_t,
                                        0xa928b283 as libc::c_uint,
                                        0xa9a6eb83 as libc::c_uint,
                                        0x2259174d as libc::c_int as uint32_t,
                                        0x45500054 as libc::c_int as uint32_t,
                                        0x184b48cb as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x26e77c33 as libc::c_int as uint32_t,
                                        0xfe324dba as libc::c_uint,
                                        0x83faf453 as libc::c_uint,
                                        0x6679a5e3 as libc::c_int as uint32_t,
                                        0x2380ef73 as libc::c_int as uint32_t,
                                        0xdd60c268 as libc::c_uint,
                                        0x3dc33a9 as libc::c_int as uint32_t,
                                        0x3ee0e07a as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xce974493 as libc::c_uint,
                                        0x403aff28 as libc::c_int as uint32_t,
                                        0x9bf6f5c4 as libc::c_uint,
                                        0x84076bf4 as libc::c_uint,
                                        0xecd898fb as libc::c_uint,
                                        0xec57038c as libc::c_uint,
                                        0xb663ed49 as libc::c_uint,
                                        0x2898ffaa as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xf335163d as libc::c_uint,
                                        0xf4b3bc46 as libc::c_uint,
                                        0xfa4fb6c6 as libc::c_uint,
                                        0xe613a0f4 as libc::c_uint,
                                        0xb9934557 as libc::c_uint,
                                        0xe759d6bc as libc::c_uint,
                                        0xab6c9477 as libc::c_uint,
                                        0x94f3b96 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x6afffe9e as libc::c_int as uint32_t,
                                        0x168bb5a0 as libc::c_int as uint32_t,
                                        0xee748c29 as libc::c_uint,
                                        0x950f7ad7 as libc::c_uint,
                                        0xda17203d as libc::c_uint,
                                        0xa4850a2b as libc::c_uint,
                                        0x77289e0f as libc::c_int as uint32_t,
                                        0x62f7a7 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x4b3829fa as libc::c_int as uint32_t,
                                        0x6265d4e9 as libc::c_int as uint32_t,
                                        0xbdfcd386 as libc::c_uint,
                                        0x4f155ada as libc::c_int as uint32_t,
                                        0x475795f6 as libc::c_int as uint32_t,
                                        0x9f38bda4 as libc::c_uint,
                                        0xdece4a4c as libc::c_uint,
                                        0x560ed4b3 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x141e648a as libc::c_int as uint32_t,
                                        0xdad4570a as libc::c_uint,
                                        0x19b965c as libc::c_int as uint32_t,
                                        0x8bbf674c as libc::c_uint,
                                        0xdb08fe30 as libc::c_uint,
                                        0xd7a8d50d as libc::c_uint,
                                        0xa2851109 as libc::c_uint,
                                        0x7efb45d3 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd0c28cda as libc::c_uint,
                                        0x52e818ac as libc::c_int as uint32_t,
                                        0xa321d436 as libc::c_uint,
                                        0x792257dd as libc::c_int as uint32_t,
                                        0x9d71f8b7 as libc::c_uint,
                                        0x867091c6 as libc::c_uint,
                                        0x11a1bf56 as libc::c_int as uint32_t,
                                        0xfe1198b as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x6137ab1 as libc::c_int as uint32_t,
                                        0x4e848339 as libc::c_int as uint32_t,
                                        0x3e6674cc as libc::c_int as uint32_t,
                                        0x5673e864 as libc::c_int as uint32_t,
                                        0x140502b as libc::c_int as uint32_t,
                                        0xad882043 as libc::c_uint,
                                        0x6ea1e46a as libc::c_int as uint32_t,
                                        0x34b5c0cb as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x1d70aa7c as libc::c_int as uint32_t,
                                        0x29786814 as libc::c_int as uint32_t,
                                        0x8cdbb8aa as libc::c_uint,
                                        0x840ae3f9 as libc::c_uint,
                                        0xbd4801fb as libc::c_uint,
                                        0x78b4d622 as libc::c_int as uint32_t,
                                        0xcf18ae9a as libc::c_uint,
                                        0x6cf4e146 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x36297168 as libc::c_int as uint32_t,
                                        0x95c270ad as libc::c_uint,
                                        0x942e7812 as libc::c_uint,
                                        0x2303ce80 as libc::c_int as uint32_t,
                                        0x205cf0e as libc::c_int as uint32_t,
                                        0x71908cc2 as libc::c_int as uint32_t,
                                        0x32bcd754 as libc::c_int as uint32_t,
                                        0xcc15edd as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x2c7ded86 as libc::c_int as uint32_t,
                                        0x1db94364 as libc::c_int as uint32_t,
                                        0xf141b22c as libc::c_uint,
                                        0xc694e39b as libc::c_uint,
                                        0x5e5a9312 as libc::c_int as uint32_t,
                                        0xf22f64ef as libc::c_uint,
                                        0x3c5e6155 as libc::c_int as uint32_t,
                                        0x649b8859 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xb6417945 as libc::c_uint,
                                        0xd5611c6 as libc::c_int as uint32_t,
                                        0xac306c97 as libc::c_uint,
                                        0x9643fdbf as libc::c_uint,
                                        0xdf500ff as libc::c_int as uint32_t,
                                        0xe81faaa4 as libc::c_uint,
                                        0x6f50e615 as libc::c_int as uint32_t,
                                        0x792c79b as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd2af8c8d as libc::c_uint,
                                        0xb45bbc49 as libc::c_uint,
                                        0x84f51bfe as libc::c_uint,
                                        0x16c615ab as libc::c_int as uint32_t,
                                        0xc1d02d32 as libc::c_uint,
                                        0xdc57c526 as libc::c_uint,
                                        0x3c8aaa55 as libc::c_int as uint32_t,
                                        0x5fb9a9a6 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xdee40b98 as libc::c_uint,
                                        0x82faa8db as libc::c_uint,
                                        0x6d520674 as libc::c_int as uint32_t,
                                        0xff8a5208 as libc::c_uint,
                                        0x446ac562 as libc::c_int as uint32_t,
                                        0x1f8c510f as libc::c_int as uint32_t,
                                        0x2cc6b66e as libc::c_int as uint32_t,
                                        0x4676d381 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x2e7429f4 as libc::c_int as uint32_t,
                                        0x8f1aa780 as libc::c_uint,
                                        0x8ed6bdf6 as libc::c_uint,
                                        0x2a95c1bf as libc::c_int as uint32_t,
                                        0x457fa0eb as libc::c_int as uint32_t,
                                        0x51450a0 as libc::c_int as uint32_t,
                                        0x744c57b1 as libc::c_int as uint32_t,
                                        0x7d89e2b7 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x3f95ea15 as libc::c_int as uint32_t,
                                        0xb6bdacd2 as libc::c_uint,
                                        0x2f1a5d69 as libc::c_int as uint32_t,
                                        0xc9a9d1b1 as libc::c_uint,
                                        0xf4d22d72 as libc::c_uint,
                                        0xd4c2f1a9 as libc::c_uint,
                                        0x4dc516b5 as libc::c_int as uint32_t,
                                        0x73ecfdf1 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5391e08 as libc::c_int as uint32_t,
                                        0xa1ce93cd as libc::c_uint,
                                        0x7b8aac17 as libc::c_int as uint32_t,
                                        0x98f1e99e as libc::c_uint,
                                        0xa098cbb3 as libc::c_uint,
                                        0x9ba84f2e as libc::c_uint,
                                        0xf9bdd37a as libc::c_uint,
                                        0x1425aa8b as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x966abfc0 as libc::c_uint,
                                        0x8a385bf4 as libc::c_uint,
                                        0xf081a640 as libc::c_uint,
                                        0x55e5e8bc as libc::c_int as uint32_t,
                                        0xee26f5ff as libc::c_uint,
                                        0x835dff85 as libc::c_uint,
                                        0xe509e1ea as libc::c_uint,
                                        0x4927e622 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x352334b0 as libc::c_int as uint32_t,
                                        0x164c8dbc as libc::c_int as uint32_t,
                                        0xa3fea31f as libc::c_uint,
                                        0xcac1ad63 as libc::c_uint,
                                        0x682fd457 as libc::c_int as uint32_t,
                                        0x9b87a676 as libc::c_uint,
                                        0x1a53145f as libc::c_int as uint32_t,
                                        0x75f382ff as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc3efcb46 as libc::c_uint,
                                        0x16b944f5 as libc::c_int as uint32_t,
                                        0x68cb184c as libc::c_int as uint32_t,
                                        0x1fb55714 as libc::c_int as uint32_t,
                                        0x9ccf2dc8 as libc::c_uint,
                                        0xf1c2b116 as libc::c_uint,
                                        0x808283d8 as libc::c_uint,
                                        0x7417e00f as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x930199ba as libc::c_uint,
                                        0x1ea67a22 as libc::c_int as uint32_t,
                                        0x718990d8 as libc::c_int as uint32_t,
                                        0x9fbaf765 as libc::c_uint,
                                        0x8f3d5d57 as libc::c_uint,
                                        0x231fc664 as libc::c_int as uint32_t,
                                        0xe5853194 as libc::c_uint,
                                        0x38141a19 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x2f81290d as libc::c_int as uint32_t,
                                        0xb9f00390 as libc::c_uint,
                                        0x4a9ca6c as libc::c_int as uint32_t,
                                        0x44877827 as libc::c_int as uint32_t,
                                        0xe1dbdd65 as libc::c_uint,
                                        0x65d7f9b9 as libc::c_int as uint32_t,
                                        0xf7c6698a as libc::c_uint,
                                        0x7133424c as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xa7cd250f as libc::c_uint,
                                        0x604cfb3c as libc::c_int as uint32_t,
                                        0x5acc18f3 as libc::c_int as uint32_t,
                                        0x460c3c4b as libc::c_int as uint32_t,
                                        0xb518e3eb as libc::c_uint,
                                        0xa53e50e0 as libc::c_uint,
                                        0x98a40196 as libc::c_uint,
                                        0x2b4b9267 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc5dbd06c as libc::c_uint,
                                        0x591b0672 as libc::c_int as uint32_t,
                                        0xaa1eeb65 as libc::c_uint,
                                        0x10d43dca as libc::c_int as uint32_t,
                                        0xcd2517af as libc::c_uint,
                                        0x420cdef8 as libc::c_int as uint32_t,
                                        0xb695a8a as libc::c_int as uint32_t,
                                        0x513a307e as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x66503215 as libc::c_int as uint32_t,
                                        0xee9d6a7b as libc::c_uint,
                                        0x88fd9a4 as libc::c_int as uint32_t,
                                        0xdea58720 as libc::c_uint,
                                        0x973afe12 as libc::c_uint,
                                        0x8f3cbbea as libc::c_uint,
                                        0x872f2538 as libc::c_uint,
                                        0x5c2350 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x35af3291 as libc::c_int as uint32_t,
                                        0xe5024b70 as libc::c_uint,
                                        0x4f5e669a as libc::c_int as uint32_t,
                                        0x1d3eec2d as libc::c_int as uint32_t,
                                        0x6e79d539 as libc::c_int as uint32_t,
                                        0xc1f6d766 as libc::c_uint,
                                        0x795b5248 as libc::c_int as uint32_t,
                                        0x34ec043f as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x400960b6 as libc::c_int as uint32_t,
                                        0xb2763511 as libc::c_uint,
                                        0x29e57df0 as libc::c_int as uint32_t,
                                        0xff7a3d84 as libc::c_uint,
                                        0x1666c1f1 as libc::c_int as uint32_t,
                                        0xaeac7792 as libc::c_uint,
                                        0x66084bc0 as libc::c_int as uint32_t,
                                        0x72426e97 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     },
     {
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x44f826ca as libc::c_int as uint32_t,
                                        0x5b1c3199 as libc::c_int as uint32_t,
                                        0x790aa408 as libc::c_int as uint32_t,
                                        0x68b00b73 as libc::c_int as uint32_t,
                                        0x69e9b92b as libc::c_int as uint32_t,
                                        0xaf0984b4 as libc::c_uint,
                                        0x3ffe9093 as libc::c_int as uint32_t,
                                        0x5fe6736f as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xffd49312 as libc::c_uint,
                                        0xd67f2889 as libc::c_uint,
                                        0x5cb9ed21 as libc::c_int as uint32_t,
                                        0x3520d747 as libc::c_int as uint32_t,
                                        0x3c65a606 as libc::c_int as uint32_t,
                                        0x94f893b1 as libc::c_uint,
                                        0x2d65496f as libc::c_int as uint32_t,
                                        0x2fee5e8c as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     }];
/* *
 * @brief	X  = k * G
 *
 * @param K	scalar k
 *
 * Return -1 on error.
 * Return 0 on success.
 */
unsafe extern "C" fn compute_kG_25519(mut X: *mut ac, mut K: *const bn256) {
    let mut Q: [ptc; 1] =
        [ptc{x: [bn256{word: [0; 8],}; 1],
             y: [bn256{word: [0; 8],}; 1],
             z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut i: libc::c_int = 0;
    /* identity element */
    memset(Q.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<ptc>() as libc::c_ulong as size_t);
    (*(*Q.as_mut_ptr()).y.as_mut_ptr()).word[0 as libc::c_int as usize] =
        1 as libc::c_int as uint32_t;
    (*(*Q.as_mut_ptr()).z.as_mut_ptr()).word[0 as libc::c_int as usize] =
        1 as libc::c_int as uint32_t;
    i = 20 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut k0: libc::c_int = 0;
        let mut k1: libc::c_int = 0;
        let mut k2: libc::c_int = 0;
        k0 =
            ((*K).word[0 as libc::c_int as usize] >> i &
                 1 as libc::c_int as libc::c_uint |
                 (if i < 1 as libc::c_int {
                      ((*K).word[1 as libc::c_int as usize] >>
                           30 as libc::c_int) &
                          2 as libc::c_int as libc::c_uint
                  } else {
                      (((*K).word[2 as libc::c_int as usize] >>
                            i - 1 as libc::c_int &
                            1 as libc::c_int as libc::c_uint)) <<
                          1 as libc::c_int
                  }) |
                 (if i < 2 as libc::c_int {
                      ((*K).word[3 as libc::c_int as usize] >>
                           i + 28 as libc::c_int) &
                          4 as libc::c_int as libc::c_uint
                  } else {
                      (((*K).word[4 as libc::c_int as usize] >>
                            i - 2 as libc::c_int &
                            1 as libc::c_int as libc::c_uint)) <<
                          2 as libc::c_int
                  }) |
                 (if i < 3 as libc::c_int {
                      ((*K).word[5 as libc::c_int as usize] >>
                           i + 26 as libc::c_int) &
                          8 as libc::c_int as libc::c_uint
                  } else {
                      (((*K).word[6 as libc::c_int as usize] >>
                            i - 3 as libc::c_int &
                            1 as libc::c_int as libc::c_uint)) <<
                          3 as libc::c_int
                  })) as libc::c_int;
        k1 =
            ((if i < 11 as libc::c_int {
                  ((*K).word[0 as libc::c_int as usize] >>
                       i + 21 as libc::c_int) &
                      1 as libc::c_int as libc::c_uint
              } else {
                  ((*K).word[1 as libc::c_int as usize] >>
                       i - 11 as libc::c_int) &
                      1 as libc::c_int as libc::c_uint
              }) |
                 (if i < 12 as libc::c_int {
                      ((*K).word[2 as libc::c_int as usize] >>
                           i + 19 as libc::c_int) &
                          2 as libc::c_int as libc::c_uint
                  } else {
                      (((*K).word[3 as libc::c_int as usize] >>
                            i - 12 as libc::c_int &
                            1 as libc::c_int as libc::c_uint)) <<
                          1 as libc::c_int
                  }) |
                 (if i < 13 as libc::c_int {
                      ((*K).word[4 as libc::c_int as usize] >>
                           i + 17 as libc::c_int) &
                          4 as libc::c_int as libc::c_uint
                  } else {
                      (((*K).word[5 as libc::c_int as usize] >>
                            i - 13 as libc::c_int &
                            1 as libc::c_int as libc::c_uint)) <<
                          2 as libc::c_int
                  }) |
                 (if i < 14 as libc::c_int {
                      ((*K).word[6 as libc::c_int as usize] >>
                           i + 15 as libc::c_int) &
                          8 as libc::c_int as libc::c_uint
                  } else {
                      (((*K).word[7 as libc::c_int as usize] >>
                            i - 14 as libc::c_int &
                            1 as libc::c_int as libc::c_uint)) <<
                          3 as libc::c_int
                  })) as libc::c_int;
        k2 =
            ((*K).word[1 as libc::c_int as usize] >> i + 10 as libc::c_int &
                 1 as libc::c_int as libc::c_uint |
                 (*K).word[3 as libc::c_int as usize] >> i + 8 as libc::c_int
                     & 2 as libc::c_int as libc::c_uint |
                 (*K).word[5 as libc::c_int as usize] >> i + 6 as libc::c_int
                     & 4 as libc::c_int as libc::c_uint |
                 (*K).word[7 as libc::c_int as usize] >> i + 4 as libc::c_int
                     & 8 as libc::c_int as libc::c_uint) as libc::c_int;
        point_double(Q.as_mut_ptr(), Q.as_mut_ptr());
        point_add(Q.as_mut_ptr(), Q.as_mut_ptr(),
                  &*precomputed_KG.as_ptr().offset(k0 as isize));
        point_add(Q.as_mut_ptr(), Q.as_mut_ptr(),
                  &*precomputed_2E_KG.as_ptr().offset(k1 as isize));
        point_add(Q.as_mut_ptr(), Q.as_mut_ptr(),
                  &*precomputed_4E_KG.as_ptr().offset(k2 as isize));
        i -= 1
    }
    point_ptc_to_ac(X, Q.as_mut_ptr());
}
/* M: The order of the generator G.  */
static mut M: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0x5cf5d3ed as libc::c_int as uint32_t,
                        0x5812631a as libc::c_int as uint32_t,
                        0xa2f79cd6 as libc::c_uint,
                        0x14def9de as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0x10000000 as libc::c_int as uint32_t],};
         init
     }];
unsafe extern "C" fn bnX_mul_C(mut r: *mut uint32_t, mut q: *const uint32_t,
                               mut q_size: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut i_beg: libc::c_int = 0;
    let mut i_end: libc::c_int = 0;
    let mut r0: uint32_t = 0;
    let mut r1: uint32_t = 0;
    let mut r2: uint32_t = 0;
    r2 = 0 as libc::c_int as uint32_t;
    r1 = r2;
    r0 = r1;
    k = 0 as libc::c_int;
    while k <= q_size + 4 as libc::c_int - 2 as libc::c_int {
        if q_size < 4 as libc::c_int {
            if k < q_size {
                i_beg = 0 as libc::c_int;
                i_end = k
            } else {
                i_beg = k - q_size + 1 as libc::c_int;
                i_end = k;
                if i_end > 4 as libc::c_int - 1 as libc::c_int {
                    i_end = 4 as libc::c_int - 1 as libc::c_int
                }
            }
        } else if k < 4 as libc::c_int {
            i_beg = 0 as libc::c_int;
            i_end = k
        } else {
            i_beg = k - 4 as libc::c_int + 1 as libc::c_int;
            i_end = k;
            if i_end > q_size - 1 as libc::c_int {
                i_end = q_size - 1 as libc::c_int
            }
        }
        i = i_beg;
        while i <= i_end {
            let mut uv: uint64_t = 0;
            let mut u: uint32_t = 0;
            let mut v: uint32_t = 0;
            let mut carry: uint32_t = 0;
            j = k - i;
            if q_size < 4 as libc::c_int {
                uv =
                    (*q.offset(j as isize) as
                         uint64_t).wrapping_mul(*(M.as_ptr() as
                                                      *const uint32_t).offset(i
                                                                                  as
                                                                                  isize)
                                                    as uint64_t)
            } else {
                uv =
                    (*q.offset(i as isize) as
                         uint64_t).wrapping_mul(*(M.as_ptr() as
                                                      *const uint32_t).offset(j
                                                                                  as
                                                                                  isize)
                                                    as uint64_t)
            }
            v = uv as uint32_t;
            u = (uv >> 32 as libc::c_int) as uint32_t;
            r0 = (r0 as libc::c_uint).wrapping_add(v) as uint32_t as uint32_t;
            carry = (r0 < v) as libc::c_int as uint32_t;
            r1 =
                (r1 as libc::c_uint).wrapping_add(carry) as uint32_t as
                    uint32_t;
            carry = (r1 < carry) as libc::c_int as uint32_t;
            r1 = (r1 as libc::c_uint).wrapping_add(u) as uint32_t as uint32_t;
            carry =
                (carry as
                     libc::c_uint).wrapping_add((r1 < u) as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            r2 =
                (r2 as libc::c_uint).wrapping_add(carry) as uint32_t as
                    uint32_t;
            i += 1
        }
        *r.offset(k as isize) = r0;
        r0 = r1;
        r1 = r2;
        r2 = 0 as libc::c_int as uint32_t;
        k += 1
    }
    *r.offset(k as isize) = r0;
}
/* *
 * @brief R = A mod M (using M=2^252+C) (Barret reduction)
 *
 * See HAC 14.47.
 */
unsafe extern "C" fn mod_reduce_M(mut R: *mut bn256, mut A: *const bn512) {
    let mut q: [uint32_t; 9] = [0; 9];
    let mut tmp: [uint32_t; 13] = [0; 13];
    let mut r: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut carry: uint32_t = 0;
    let mut next_carry: uint32_t = 0;
    let mut i: libc::c_int = 0;
    q[8 as libc::c_int as usize] =
        (*A).word[15 as libc::c_int as usize] >> 28 as libc::c_int;
    carry =
        (*A).word[15 as libc::c_int as usize] &
            0xfffffff as libc::c_int as libc::c_uint;
    i = 8 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        next_carry =
            (*A).word[(i + 7 as libc::c_int) as usize] &
                0xfffffff as libc::c_int as libc::c_uint;
        q[i as usize] =
            (*A).word[(i + 7 as libc::c_int) as usize] >> 28 as libc::c_int |
                carry << 4 as libc::c_int;
        carry = next_carry;
        i -= 1
    }
    memcpy(R as *mut libc::c_void, A as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong as size_t);
    (*R).word[7 as libc::c_int as usize] &=
        0xfffffff as libc::c_int as libc::c_uint;
    /* Q_size: 9 */
    bnX_mul_C(tmp.as_mut_ptr(), q.as_mut_ptr(),
              9 as libc::c_int); /* TMP = Q*C */
    /* Q = tmp / 2^252 */
    carry =
        tmp[12 as libc::c_int as usize] &
            0xfffffff as libc::c_int as libc::c_uint;
    i = 4 as libc::c_int;
    while i >= 0 as libc::c_int {
        next_carry =
            tmp[(i + 7 as libc::c_int) as usize] &
                0xfffffff as libc::c_int as libc::c_uint;
        q[i as usize] =
            tmp[(i + 7 as libc::c_int) as usize] >> 28 as libc::c_int |
                carry << 4 as libc::c_int;
        carry = next_carry;
        i -= 1
    }
    /* R' = tmp % 2^252 */
    memcpy(r.as_mut_ptr() as *mut libc::c_void,
           tmp.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong as size_t);
    let ref mut fresh0 = (*r.as_mut_ptr()).word[7 as libc::c_int as usize];
    *fresh0 &= 0xfffffff as libc::c_int as libc::c_uint;
    /* R -= R' */
    carry = bn256_sub(R, R, r.as_mut_ptr());
    if carry != 0 {
        bn256_add(R, R, M.as_ptr());
    } else { bn256_add(tmp.as_mut_ptr() as *mut bn256, R, M.as_ptr()); }
    /* Q_size: 5 */
    bnX_mul_C(tmp.as_mut_ptr(), q.as_mut_ptr(),
              5 as libc::c_int); /* TMP = Q*C */
    carry =
        tmp[8 as libc::c_int as usize] &
            0xfffffff as libc::c_int as libc::c_uint;
    q[0 as libc::c_int as usize] =
        tmp[7 as libc::c_int as usize] >> 28 as libc::c_int |
            carry << 4 as libc::c_int;
    /* R' = tmp % 2^252 */
    memcpy(r.as_mut_ptr() as *mut libc::c_void,
           tmp.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong as size_t);
    let ref mut fresh1 = (*r.as_mut_ptr()).word[7 as libc::c_int as usize];
    *fresh1 &= 0xfffffff as libc::c_int as libc::c_uint;
    /* R += R' */
    bn256_add(R, R, r.as_mut_ptr());
    carry = bn256_sub(R, R, M.as_ptr());
    if carry != 0 {
        bn256_add(R, R, M.as_ptr());
    } else { bn256_add(tmp.as_mut_ptr() as *mut bn256, R, M.as_ptr()); }
    /* Q_size: 1 */
    bnX_mul_C(tmp.as_mut_ptr(), q.as_mut_ptr(),
              1 as libc::c_int); /* TMP = Q*C */
    /* R' = tmp % 2^252 */
    memset((r.as_mut_ptr() as
                *mut uint8_t).offset((::std::mem::size_of::<uint32_t>() as
                                          libc::c_ulong).wrapping_mul(5 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                         as isize) as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<uint32_t>() as
                libc::c_ulong).wrapping_mul(3 as libc::c_int as libc::c_ulong)
               as size_t);
    memcpy(r.as_mut_ptr() as *mut libc::c_void,
           tmp.as_mut_ptr() as *const libc::c_void,
           (::std::mem::size_of::<uint32_t>() as
                libc::c_ulong).wrapping_mul(5 as libc::c_int as libc::c_ulong)
               as size_t);
    /* R -= R' */
    carry = bn256_sub(R, R, r.as_mut_ptr()); /* It's upper half of the hash */
    if carry != 0 {
        bn256_add(R, R, M.as_ptr());
    } else { bn256_add(tmp.as_mut_ptr() as *mut bn256, R, M.as_ptr()); };
}
#[no_mangle]
pub unsafe extern "C" fn eddsa_sign_25519(mut input: *const uint8_t,
                                          mut ilen: size_t,
                                          mut out: *mut uint32_t,
                                          mut a: *const bn256,
                                          mut seed: *const uint8_t,
                                          mut pk: *const bn256)
 -> libc::c_int {
    let mut r: *mut bn256 = 0 as *mut bn256;
    let mut s: *mut bn256 = 0 as *mut bn256;
    let mut ctx: sha512_context =
        sha512_context{total: [0; 2], state: [0; 8], wbuf: [0; 16],};
    let mut hash: [uint8_t; 64] = [0; 64];
    let mut tmp: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut R: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut carry: uint32_t = 0;
    let mut borrow_0: uint32_t = 0;
    r = out as *mut bn256;
    s =
        out.offset((32 as libc::c_int / 4 as libc::c_int) as isize) as
            *mut bn256;
    sha512_start(&mut ctx);
    sha512_update(&mut ctx, seed,
                  ::std::mem::size_of::<bn256>() as libc::c_ulong as
                      libc::c_uint);
    sha512_update(&mut ctx, input, ilen);
    sha512_finish(&mut ctx, hash.as_mut_ptr());
    mod_reduce_M(r, hash.as_mut_ptr() as *mut bn512);
    compute_kG_25519(R.as_mut_ptr(), r);
    /* EdDSA encoding.  */
    memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
           (*R.as_mut_ptr()).y.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong as size_t);
    let ref mut fresh2 = (*tmp.as_mut_ptr()).word[7 as libc::c_int as usize];
    *fresh2 ^=
        (mod25519_is_neg((*R.as_mut_ptr()).x.as_mut_ptr()) as
             libc::c_uint).wrapping_mul(0x80000000 as libc::c_uint);
    sha512_start(&mut ctx);
    sha512_update(&mut ctx, tmp.as_mut_ptr() as *mut uint8_t,
                  ::std::mem::size_of::<bn256>() as libc::c_ulong as
                      libc::c_uint);
    sha512_update(&mut ctx, pk as *mut uint8_t,
                  ::std::mem::size_of::<bn256>() as libc::c_ulong as
                      libc::c_uint);
    sha512_update(&mut ctx, input, ilen);
    sha512_finish(&mut ctx, hash.as_mut_ptr());
    mod_reduce_M(s, hash.as_mut_ptr() as *mut bn512);
    bn256_mul(hash.as_mut_ptr() as *mut bn512, s, a);
    mod_reduce_M(s, hash.as_mut_ptr() as *mut bn512);
    carry = bn256_add(s, s, r);
    borrow_0 = bn256_sub(s, s, M.as_ptr());
    memcpy(r as *mut libc::c_void, tmp.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong as size_t);
    if borrow_0 != 0 && carry == 0 {
        bn256_add(s, s, M.as_ptr());
    } else { bn256_add(tmp.as_mut_ptr(), s, M.as_ptr()); }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eddsa_public_key_25519(mut pk: *mut bn256,
                                                mut a: *const bn256) {
    let mut R: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut X: [ptc; 1] =
        [ptc{x: [bn256{word: [0; 8],}; 1],
             y: [bn256{word: [0; 8],}; 1],
             z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut a0: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    bn256_shift(a0.as_mut_ptr(), a, -(3 as libc::c_int));
    compute_kG_25519(R.as_mut_ptr(), a0.as_mut_ptr());
    memcpy(X.as_mut_ptr() as *mut libc::c_void,
           R.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<ac>() as libc::c_ulong as size_t);
    memset((*X.as_mut_ptr()).z.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<bn256>() as libc::c_ulong as size_t);
    (*(*X.as_mut_ptr()).z.as_mut_ptr()).word[0 as libc::c_int as usize] =
        1 as libc::c_int as uint32_t;
    point_double(X.as_mut_ptr(), X.as_mut_ptr());
    point_double(X.as_mut_ptr(), X.as_mut_ptr());
    point_double(X.as_mut_ptr(), X.as_mut_ptr());
    point_ptc_to_ac(R.as_mut_ptr(), X.as_mut_ptr());
    /* EdDSA encoding.  */
    memcpy(pk as *mut libc::c_void,
           (*R.as_mut_ptr()).y.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong as size_t);
    (*pk).word[7 as libc::c_int as usize] ^=
        (mod25519_is_neg((*R.as_mut_ptr()).x.as_mut_ptr()) as
             libc::c_uint).wrapping_mul(0x80000000 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn eddsa_compute_public_25519(mut kd: *const uint8_t)
 -> *mut uint8_t {
    let mut p0: *mut uint8_t = 0 as *mut uint8_t;
    let mut a: *const bn256 = kd as *const bn256;
    p0 =
        gnuk_malloc(::std::mem::size_of::<bn256>() as libc::c_ulong as size_t)
            as *mut uint8_t;
    if p0.is_null() { return 0 as *mut uint8_t }
    eddsa_public_key_25519(p0 as *mut bn256, a);
    return p0;
}
