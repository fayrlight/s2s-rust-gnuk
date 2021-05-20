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
    fn bn256_add_uint(X: *mut bn256, A: *const bn256, w: uint32_t)
     -> uint32_t;
    #[no_mangle]
    fn bn256_sub_uint(X: *mut bn256, A: *const bn256, w: uint32_t)
     -> uint32_t;
    #[no_mangle]
    fn bn256_mul(X: *mut bn512, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn bn256_is_zero(X: *const bn256) -> libc::c_int;
    #[no_mangle]
    fn bn256_is_even(X: *const bn256) -> libc::c_int;
    #[no_mangle]
    fn bn256_cmp(A: *const bn256, B: *const bn256) -> libc::c_int;
    #[no_mangle]
    fn bn256_random(X: *mut bn256);
    #[no_mangle]
    fn modp256r1_add(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn modp256r1_mul(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn modp256r1_sqr(X: *mut bn256, A: *const bn256);
    #[no_mangle]
    fn jpc_double_p256r1(X: *mut jpc, A: *const jpc);
    #[no_mangle]
    fn jpc_add_ac_p256r1(X: *mut jpc, A: *const jpc, B: *const ac);
    #[no_mangle]
    fn jpc_add_ac_signed_p256r1(X: *mut jpc, A: *const jpc, B: *const ac,
                                minus: libc::c_int);
    #[no_mangle]
    fn jpc_to_ac_p256r1(X: *mut ac, A: *const jpc) -> libc::c_int;
    #[no_mangle]
    fn mod_reduce(X: *mut bn256, A: *const bn512, B: *const bn256,
                  MU_lower_0: *const bn256);
    #[no_mangle]
    fn mod_inv(X: *mut bn256, A: *const bn256, N_0: *const bn256);
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
/*                                                    -*- coding: utf-8 -*-
 * ec_p256r1.c - Elliptic curve over GF(p256r1)
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
 * a = -3 mod p256r1
 */
static mut coefficient_a: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0xfffffffc as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0x1 as libc::c_int as uint32_t,
                        0xffffffff as libc::c_uint],};
         init
     }];
static mut coefficient_b: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0x27d2604b as libc::c_int as uint32_t,
                        0x3bce3c3e as libc::c_int as uint32_t,
                        0xcc53b0f6 as libc::c_uint,
                        0x651d06b0 as libc::c_int as uint32_t,
                        0x769886bc as libc::c_int as uint32_t,
                        0xb3ebbd55 as libc::c_uint,
                        0xaa3a93e7 as libc::c_uint,
                        0x5ac635d8 as libc::c_int as uint32_t],};
         init
     }];
static mut precomputed_KG: [ac; 15] =
    [{
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd898c296 as libc::c_uint,
                                        0xf4a13945 as libc::c_uint,
                                        0x2deb33a0 as libc::c_int as uint32_t,
                                        0x77037d81 as libc::c_int as uint32_t,
                                        0x63a440f2 as libc::c_int as uint32_t,
                                        0xf8bce6e5 as libc::c_uint,
                                        0xe12c4247 as libc::c_uint,
                                        0x6b17d1f2 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x37bf51f5 as libc::c_int as uint32_t,
                                        0xcbb64068 as libc::c_uint,
                                        0x6b315ece as libc::c_int as uint32_t,
                                        0x2bce3357 as libc::c_int as uint32_t,
                                        0x7c0f9e16 as libc::c_int as uint32_t,
                                        0x8ee7eb4a as libc::c_uint,
                                        0xfe1a7f9b as libc::c_uint,
                                        0x4fe342e2 as libc::c_int as
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
                                       [0x8e14db63 as libc::c_uint,
                                        0x90e75cb4 as libc::c_uint,
                                        0xad651f7e as libc::c_uint,
                                        0x29493baa as libc::c_int as uint32_t,
                                        0x326e25de as libc::c_int as uint32_t,
                                        0x8492592e as libc::c_uint,
                                        0x2811aaa5 as libc::c_int as uint32_t,
                                        0xfa822bc as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5f462ee7 as libc::c_int as uint32_t,
                                        0xe4112454 as libc::c_uint,
                                        0x50fe82f5 as libc::c_int as uint32_t,
                                        0x34b1a650 as libc::c_int as uint32_t,
                                        0xb3df188b as libc::c_uint,
                                        0x6f4ad4bc as libc::c_int as uint32_t,
                                        0xf5dba80d as libc::c_uint,
                                        0xbff44ae8 as libc::c_uint],};
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
                                       [0x97992af as libc::c_int as uint32_t,
                                        0x93391ce2 as libc::c_uint,
                                        0xd35f1fa as libc::c_int as uint32_t,
                                        0xe96c98fd as libc::c_uint,
                                        0x95e02789 as libc::c_uint,
                                        0xb257c0de as libc::c_uint,
                                        0x89d6726f as libc::c_uint,
                                        0x300a4bbc as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc08127a0 as libc::c_uint,
                                        0xaa54a291 as libc::c_uint,
                                        0xa9d806a5 as libc::c_uint,
                                        0x5bb1eead as libc::c_int as uint32_t,
                                        0xff1e3c6f as libc::c_uint,
                                        0x7f1ddb25 as libc::c_int as uint32_t,
                                        0xd09b4644 as libc::c_uint,
                                        0x72aac7e0 as libc::c_int as
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
                                       [0xd789bd85 as libc::c_uint,
                                        0x57c84fc9 as libc::c_int as uint32_t,
                                        0xc297eac3 as libc::c_uint,
                                        0xfc35ff7d as libc::c_uint,
                                        0x88c6766e as libc::c_uint,
                                        0xfb982fd5 as libc::c_uint,
                                        0xeedb5e67 as libc::c_uint,
                                        0x447d739b as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x72e25b32 as libc::c_int as uint32_t,
                                        0xc7e33c9 as libc::c_int as uint32_t,
                                        0xa7fae500 as libc::c_uint,
                                        0x3d349b95 as libc::c_int as uint32_t,
                                        0x3a4aaff7 as libc::c_int as uint32_t,
                                        0xe12e9d95 as libc::c_uint,
                                        0x834131ee as libc::c_uint,
                                        0x2d4825ab as libc::c_int as
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
                                       [0x2a1d367f as libc::c_int as uint32_t,
                                        0x13949c93 as libc::c_int as uint32_t,
                                        0x1a0a11b7 as libc::c_int as uint32_t,
                                        0xef7fbd2b as libc::c_uint,
                                        0xb91dfc60 as libc::c_uint,
                                        0xddc6068b as libc::c_uint,
                                        0x8a9c72ff as libc::c_uint,
                                        0xef951932 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x7376d8a8 as libc::c_int as uint32_t,
                                        0x196035a7 as libc::c_int as uint32_t,
                                        0x95ca1740 as libc::c_uint,
                                        0x23183b08 as libc::c_int as uint32_t,
                                        0x22c219c as libc::c_int as uint32_t,
                                        0xc1ee9807 as libc::c_uint,
                                        0x7dbb2c9b as libc::c_int as uint32_t,
                                        0x611e9fc3 as libc::c_int as
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
                                       [0xb57f4bc as libc::c_int as uint32_t,
                                        0xcae2b192 as libc::c_uint,
                                        0xc6c9bc36 as libc::c_uint,
                                        0x2936df5e as libc::c_int as uint32_t,
                                        0xe11238bf as libc::c_uint,
                                        0x7dea6482 as libc::c_int as uint32_t,
                                        0x7b51f5d8 as libc::c_int as uint32_t,
                                        0x55066379 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x348a964c as libc::c_int as uint32_t,
                                        0x44ffe216 as libc::c_int as uint32_t,
                                        0xdbdefbe1 as libc::c_uint,
                                        0x9fb3d576 as libc::c_uint,
                                        0x8d9d50e5 as libc::c_uint,
                                        0xafa4001 as libc::c_int as uint32_t,
                                        0x8aecb851 as libc::c_uint,
                                        0x15716484 as libc::c_int as
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
                                       [0xfc5cde01 as libc::c_uint,
                                        0xe48ecaff as libc::c_uint,
                                        0xd715f26 as libc::c_int as uint32_t,
                                        0x7ccd84e7 as libc::c_int as uint32_t,
                                        0xf43e4391 as libc::c_uint,
                                        0xa2e8f483 as libc::c_uint,
                                        0xb21141ea as libc::c_uint,
                                        0xeb5d7745 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x731a3479 as libc::c_int as uint32_t,
                                        0xcac917e2 as libc::c_uint,
                                        0x2844b645 as libc::c_int as uint32_t,
                                        0x85f22cfe as libc::c_uint,
                                        0x58006cee as libc::c_int as uint32_t,
                                        0x990e6a1 as libc::c_int as uint32_t,
                                        0xdbecc17b as libc::c_uint,
                                        0xeafd72eb as libc::c_uint],};
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
                                       [0x313728be as libc::c_int as uint32_t,
                                        0x6cf20ffb as libc::c_int as uint32_t,
                                        0xa3c6b94a as libc::c_uint,
                                        0x96439591 as libc::c_uint,
                                        0x44315fc5 as libc::c_int as uint32_t,
                                        0x2736ff83 as libc::c_int as uint32_t,
                                        0xa7849276 as libc::c_uint,
                                        0xa6d39677 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc357f5f4 as libc::c_uint,
                                        0xf2bab833 as libc::c_uint,
                                        0x2284059b as libc::c_int as uint32_t,
                                        0x824a920c as libc::c_uint,
                                        0x2d27ecdf as libc::c_int as uint32_t,
                                        0x66b8babd as libc::c_int as uint32_t,
                                        0x9b0b8816 as libc::c_uint,
                                        0x674f8474 as libc::c_int as
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
                                       [0x677c8a3e as libc::c_int as uint32_t,
                                        0x2df48c04 as libc::c_int as uint32_t,
                                        0x203a56b as libc::c_int as uint32_t,
                                        0x74e02f08 as libc::c_int as uint32_t,
                                        0xb8c7fedb as libc::c_uint,
                                        0x31855f7d as libc::c_int as uint32_t,
                                        0x72c9ddad as libc::c_int as uint32_t,
                                        0x4e769e76 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xb824bbb0 as libc::c_uint,
                                        0xa4c36165 as libc::c_uint,
                                        0x3b9122a5 as libc::c_int as uint32_t,
                                        0xfb9ae16f as libc::c_uint,
                                        0x6947281 as libc::c_int as uint32_t,
                                        0x1ec00572 as libc::c_int as uint32_t,
                                        0xde830663 as libc::c_uint,
                                        0x42b99082 as libc::c_int as
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
                                       [0xdda868b9 as libc::c_uint,
                                        0x6ef95150 as libc::c_int as uint32_t,
                                        0x9c0ce131 as libc::c_uint,
                                        0xd1f89e79 as libc::c_uint,
                                        0x8a1c478 as libc::c_int as uint32_t,
                                        0x7fdc1ca0 as libc::c_int as uint32_t,
                                        0x1c6ce04d as libc::c_int as uint32_t,
                                        0x78878ef6 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x1fe0d976 as libc::c_int as uint32_t,
                                        0x9c62b912 as libc::c_uint,
                                        0xbde08d4f as libc::c_uint,
                                        0x6ace570e as libc::c_int as uint32_t,
                                        0x12309def as libc::c_int as uint32_t,
                                        0xde53142c as libc::c_uint,
                                        0x7b72c321 as libc::c_int as uint32_t,
                                        0xb6cb3f5d as libc::c_uint],};
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
                                       [0xc31a3573 as libc::c_uint,
                                        0x7f991ed2 as libc::c_int as uint32_t,
                                        0xd54fb496 as libc::c_uint,
                                        0x5b82dd5b as libc::c_int as uint32_t,
                                        0x812ffcae as libc::c_uint,
                                        0x595c5220 as libc::c_int as uint32_t,
                                        0x716b1287 as libc::c_int as uint32_t,
                                        0xc88bc4d as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5f48aca8 as libc::c_int as uint32_t,
                                        0x3a57bf63 as libc::c_int as uint32_t,
                                        0xdf2564f3 as libc::c_uint,
                                        0x7c8181f4 as libc::c_int as uint32_t,
                                        0x9c04e6aa as libc::c_uint,
                                        0x18d1b5b3 as libc::c_int as uint32_t,
                                        0xf3901dc6 as libc::c_uint,
                                        0xdd5ddea3 as libc::c_uint],};
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
                                       [0x3e72ad0c as libc::c_int as uint32_t,
                                        0xe96a79fb as libc::c_uint,
                                        0x42ba792f as libc::c_int as uint32_t,
                                        0x43a0a28c as libc::c_int as uint32_t,
                                        0x83e49f3 as libc::c_int as uint32_t,
                                        0xefe0a423 as libc::c_uint,
                                        0x6b317466 as libc::c_int as uint32_t,
                                        0x68f344af as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x3fb24d4a as libc::c_int as uint32_t,
                                        0xcdfe17db as libc::c_uint,
                                        0x71f5c626 as libc::c_int as uint32_t,
                                        0x668bfc22 as libc::c_int as uint32_t,
                                        0x24d67ff3 as libc::c_int as uint32_t,
                                        0x604ed93c as libc::c_int as uint32_t,
                                        0xf8540a20 as libc::c_uint,
                                        0x31b9c405 as libc::c_int as
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
                                       [0xa2582e7f as libc::c_uint,
                                        0xd36b4789 as libc::c_uint,
                                        0x4ec39c28 as libc::c_int as uint32_t,
                                        0xd1a1014 as libc::c_int as uint32_t,
                                        0xedbad7a0 as libc::c_uint,
                                        0x663c62c3 as libc::c_int as uint32_t,
                                        0x6f461db9 as libc::c_int as uint32_t,
                                        0x4052bf4b as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x188d25eb as libc::c_int as uint32_t,
                                        0x235a27c3 as libc::c_int as uint32_t,
                                        0x99bfcc5b as libc::c_uint,
                                        0xe724f339 as libc::c_uint,
                                        0x71d70cc8 as libc::c_int as uint32_t,
                                        0x862be6bd as libc::c_uint,
                                        0x90b0fc61 as libc::c_uint,
                                        0xfecf4d51 as libc::c_uint],};
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
                                       [0xa1d4cfac as libc::c_uint,
                                        0x74346c10 as libc::c_int as uint32_t,
                                        0x8526a7a4 as libc::c_uint,
                                        0xafdf5cc0 as libc::c_uint,
                                        0xf62bff7a as libc::c_uint,
                                        0x123202a8 as libc::c_int as uint32_t,
                                        0xc802e41a as libc::c_uint,
                                        0x1eddbae2 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd603f844 as libc::c_uint,
                                        0x8fa0af2d as libc::c_uint,
                                        0x4c701917 as libc::c_int as uint32_t,
                                        0x36e06b7e as libc::c_int as uint32_t,
                                        0x73db33a0 as libc::c_int as uint32_t,
                                        0xc45f452 as libc::c_int as uint32_t,
                                        0x560ebcfc as libc::c_int as uint32_t,
                                        0x43104d86 as libc::c_int as
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
                                       [0xd1d78e5 as libc::c_int as uint32_t,
                                        0x9615b511 as libc::c_uint,
                                        0x25c4744b as libc::c_int as uint32_t,
                                        0x66b0de32 as libc::c_int as uint32_t,
                                        0x6aaf363a as libc::c_int as uint32_t,
                                        0xa4a46fb as libc::c_int as uint32_t,
                                        0x84f7a21c as libc::c_uint,
                                        0xb48e26b4 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x21a01b2d as libc::c_int as uint32_t,
                                        0x6ebb0f6 as libc::c_int as uint32_t,
                                        0x8b7b0f98 as libc::c_uint,
                                        0xc004e404 as libc::c_uint,
                                        0xfed6f668 as libc::c_uint,
                                        0x64131bcd as libc::c_int as uint32_t,
                                        0x4d4d3dab as libc::c_int as uint32_t,
                                        0xfac01540 as libc::c_uint],};
                         init
                     }],};
         init
     }];
static mut precomputed_2E_KG: [ac; 15] =
    [{
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x185a5943 as libc::c_int as uint32_t,
                                        0x3a5a9e22 as libc::c_int as uint32_t,
                                        0x5c65dfb6 as libc::c_int as uint32_t,
                                        0x1ab91936 as libc::c_int as uint32_t,
                                        0x262c71da as libc::c_int as uint32_t,
                                        0x21656b32 as libc::c_int as uint32_t,
                                        0xaf22af89 as libc::c_uint,
                                        0x7fe36b40 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x699ca101 as libc::c_int as uint32_t,
                                        0xd50d152c as libc::c_uint,
                                        0x7b8af212 as libc::c_int as uint32_t,
                                        0x74b3d586 as libc::c_int as uint32_t,
                                        0x7dca6f1 as libc::c_int as uint32_t,
                                        0x9f09f404 as libc::c_uint,
                                        0x25b63624 as libc::c_int as uint32_t,
                                        0xe697d458 as libc::c_uint],};
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
                                       [0x7512218e as libc::c_int as uint32_t,
                                        0xa84aa939 as libc::c_uint,
                                        0x74ca0141 as libc::c_int as uint32_t,
                                        0xe9a521b0 as libc::c_uint,
                                        0x18a2e902 as libc::c_int as uint32_t,
                                        0x57880b3a as libc::c_int as uint32_t,
                                        0x12a677a6 as libc::c_int as uint32_t,
                                        0x4a5b5066 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x4c4f3840 as libc::c_int as uint32_t,
                                        0xbeada7a as libc::c_int as uint32_t,
                                        0x19e26d9d as libc::c_int as uint32_t,
                                        0x626db154 as libc::c_int as uint32_t,
                                        0xe1627d40 as libc::c_uint,
                                        0xc42604fb as libc::c_uint,
                                        0xeac089f1 as libc::c_uint,
                                        0xeb13461c as libc::c_uint],};
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
                                       [0x27a43281 as libc::c_int as uint32_t,
                                        0xf9faed09 as libc::c_uint,
                                        0x4103ecbc as libc::c_int as uint32_t,
                                        0x5e52c414 as libc::c_int as uint32_t,
                                        0xa815c857 as libc::c_uint,
                                        0xc342967a as libc::c_uint,
                                        0x1c6a220a as libc::c_int as uint32_t,
                                        0x781b829 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xeac55f80 as libc::c_uint,
                                        0x5a8343ce as libc::c_int as uint32_t,
                                        0xe54a05e3 as libc::c_uint,
                                        0x88f80eee as libc::c_uint,
                                        0x12916434 as libc::c_int as uint32_t,
                                        0x97b2a14f as libc::c_uint,
                                        0xf0151593 as libc::c_uint,
                                        0x690cde8d as libc::c_int as
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
                                       [0xf7f82f2a as libc::c_uint,
                                        0xaee9c75d as libc::c_uint,
                                        0x4afdf43a as libc::c_int as uint32_t,
                                        0x9e4c3587 as libc::c_uint,
                                        0x37371326 as libc::c_int as uint32_t,
                                        0xf5622df4 as libc::c_uint,
                                        0x6ec73617 as libc::c_int as uint32_t,
                                        0x8a535f56 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x223094b7 as libc::c_int as uint32_t,
                                        0xc5f9a0ac as libc::c_uint,
                                        0x4c8c7669 as libc::c_int as uint32_t,
                                        0xcde53386 as libc::c_uint,
                                        0x85a92bf as libc::c_int as uint32_t,
                                        0x37e02819 as libc::c_int as uint32_t,
                                        0x68b08bd7 as libc::c_int as uint32_t,
                                        0x455c084 as libc::c_int as
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
                                       [0x9477b5d9 as libc::c_uint,
                                        0xc0a6e2c as libc::c_int as uint32_t,
                                        0x876dc444 as libc::c_uint,
                                        0xf9a4bf62 as libc::c_uint,
                                        0xb6cdc279 as libc::c_uint,
                                        0x5050a949 as libc::c_int as uint32_t,
                                        0xb77f8276 as libc::c_uint,
                                        0x6bada7a as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xea48dac9 as libc::c_uint,
                                        0xc8b4aed1 as libc::c_uint,
                                        0x7ea1070f as libc::c_int as uint32_t,
                                        0xdebd8a4b as libc::c_uint,
                                        0x1366eb70 as libc::c_int as uint32_t,
                                        0x427d4910 as libc::c_int as uint32_t,
                                        0xe6cb18a as libc::c_int as uint32_t,
                                        0x5b476dfd as libc::c_int as
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
                                       [0x278c340a as libc::c_int as uint32_t,
                                        0x7c5c3e44 as libc::c_int as uint32_t,
                                        0x12d66f3b as libc::c_int as uint32_t,
                                        0x4d546068 as libc::c_int as uint32_t,
                                        0xae23c5d8 as libc::c_uint,
                                        0x29a751b1 as libc::c_int as uint32_t,
                                        0x8a2ec908 as libc::c_uint,
                                        0x3e29864e as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x26dbb850 as libc::c_int as uint32_t,
                                        0x142d2a66 as libc::c_int as uint32_t,
                                        0x765bd780 as libc::c_int as uint32_t,
                                        0xad1744c4 as libc::c_uint,
                                        0xe322d1ed as libc::c_uint,
                                        0x1f150e68 as libc::c_int as uint32_t,
                                        0x3dc31e7e as libc::c_int as uint32_t,
                                        0x239b90ea as libc::c_int as
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
                                       [0x7a53322a as libc::c_int as uint32_t,
                                        0x78c41652 as libc::c_int as uint32_t,
                                        0x9776f8e as libc::c_int as uint32_t,
                                        0x305dde67 as libc::c_int as uint32_t,
                                        0xf8862ed4 as libc::c_uint,
                                        0xdbcab759 as libc::c_uint,
                                        0x49f72ff7 as libc::c_int as uint32_t,
                                        0x820f4dd9 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x2b5debd4 as libc::c_int as uint32_t,
                                        0x6cc544a6 as libc::c_int as uint32_t,
                                        0x7b4e8cc4 as libc::c_int as uint32_t,
                                        0x75be5d93 as libc::c_int as uint32_t,
                                        0x215c14d3 as libc::c_int as uint32_t,
                                        0x1b481b1b as libc::c_int as uint32_t,
                                        0x783a05ec as libc::c_int as uint32_t,
                                        0x140406ec as libc::c_int as
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
                                       [0xe895df07 as libc::c_uint,
                                        0x6a703f10 as libc::c_int as uint32_t,
                                        0x1876bd8 as libc::c_int as uint32_t,
                                        0xfd75f3fa as libc::c_uint,
                                        0xce08ffe as libc::c_int as uint32_t,
                                        0xeb5b06e7 as libc::c_uint,
                                        0x2783dfee as libc::c_int as uint32_t,
                                        0x68f6b854 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x78712655 as libc::c_int as uint32_t,
                                        0x90c76f8a as libc::c_uint,
                                        0xf310bf7f as libc::c_uint,
                                        0xcf5293d2 as libc::c_uint,
                                        0xfda45028 as libc::c_uint,
                                        0xfbc8044d as libc::c_uint,
                                        0x92e40ce6 as libc::c_uint,
                                        0xcbe1feba as libc::c_uint],};
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
                                       [0x4396e4c1 as libc::c_int as uint32_t,
                                        0xe998ceea as libc::c_uint,
                                        0x6acea274 as libc::c_int as uint32_t,
                                        0xfc82ef0b as libc::c_uint,
                                        0x2250e927 as libc::c_int as uint32_t,
                                        0x230f729f as libc::c_int as uint32_t,
                                        0x2f420109 as libc::c_int as uint32_t,
                                        0xd0b2f94d as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xb38d4966 as libc::c_uint,
                                        0x4305addd as libc::c_int as uint32_t,
                                        0x624c3b45 as libc::c_int as uint32_t,
                                        0x10b838f8 as libc::c_int as uint32_t,
                                        0x58954e7a as libc::c_int as uint32_t,
                                        0x7db26366 as libc::c_int as uint32_t,
                                        0x8b0719e5 as libc::c_uint,
                                        0x97145982 as libc::c_uint],};
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
                                       [0x23369fc9 as libc::c_int as uint32_t,
                                        0x4bd6b726 as libc::c_int as uint32_t,
                                        0x53d0b876 as libc::c_int as uint32_t,
                                        0x57f2929e as libc::c_int as uint32_t,
                                        0xf2340687 as libc::c_uint,
                                        0xc2d5cba4 as libc::c_uint,
                                        0x4a866aba as libc::c_int as uint32_t,
                                        0x96161000 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x2e407a5e as libc::c_int as uint32_t,
                                        0x49997bcd as libc::c_int as uint32_t,
                                        0x92ddcb24 as libc::c_uint,
                                        0x69ab197d as libc::c_int as uint32_t,
                                        0x8fe5131c as libc::c_uint,
                                        0x2cf1f243 as libc::c_int as uint32_t,
                                        0xcee75e44 as libc::c_uint,
                                        0x7acb9fad as libc::c_int as
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
                                       [0x23d2d4c0 as libc::c_int as uint32_t,
                                        0x254e8394 as libc::c_int as uint32_t,
                                        0x7aea685b as libc::c_int as uint32_t,
                                        0xf57f0c91 as libc::c_uint,
                                        0x6f75aaea as libc::c_int as uint32_t,
                                        0xa60d880f as libc::c_uint,
                                        0xa333bf5b as libc::c_uint,
                                        0x24eb9acc as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x1cda5dea as libc::c_int as uint32_t,
                                        0xe3de4ccb as libc::c_uint,
                                        0xc51a6b4f as libc::c_uint,
                                        0xfeef9341 as libc::c_uint,
                                        0x8bac4c4d as libc::c_uint,
                                        0x743125f8 as libc::c_int as uint32_t,
                                        0xacd079cc as libc::c_uint,
                                        0x69f891c5 as libc::c_int as
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
                                       [0x702476b5 as libc::c_int as uint32_t,
                                        0xeee44b35 as libc::c_uint,
                                        0xe45c2258 as libc::c_uint,
                                        0x7ed031a0 as libc::c_int as uint32_t,
                                        0xbd6f8514 as libc::c_uint,
                                        0xb422d1e7 as libc::c_uint,
                                        0x5972a107 as libc::c_int as uint32_t,
                                        0xe51f547c as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xc9cf343d as libc::c_uint,
                                        0xa25bcd6f as libc::c_uint,
                                        0x97c184e as libc::c_int as uint32_t,
                                        0x8ca922ee as libc::c_uint,
                                        0xa9fe9a06 as libc::c_uint,
                                        0xa62f98b3 as libc::c_uint,
                                        0x25bb1387 as libc::c_int as uint32_t,
                                        0x1c309a2b as libc::c_int as
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
                                       [0x1967c459 as libc::c_int as uint32_t,
                                        0x9295dbeb as libc::c_uint,
                                        0x3472c98e as libc::c_int as uint32_t,
                                        0xb0014883 as libc::c_uint,
                                        0x8011828 as libc::c_int as uint32_t,
                                        0xc5049777 as libc::c_uint,
                                        0xa2c4e503 as libc::c_uint,
                                        0x20b87b8a as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe057c277 as libc::c_uint,
                                        0x3063175d as libc::c_int as uint32_t,
                                        0x8fe582dd as libc::c_uint,
                                        0x1bd53933 as libc::c_int as uint32_t,
                                        0x5f69a044 as libc::c_int as uint32_t,
                                        0xd11adef as libc::c_int as uint32_t,
                                        0x919776be as libc::c_uint,
                                        0xf5c6fa49 as libc::c_uint],};
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
                                       [0xfd59e11 as libc::c_int as uint32_t,
                                        0x8c944e76 as libc::c_uint,
                                        0x102fad5f as libc::c_int as uint32_t,
                                        0x3876cba1 as libc::c_int as uint32_t,
                                        0xd83faa56 as libc::c_uint,
                                        0xa454c3fa as libc::c_uint,
                                        0x332010b9 as libc::c_int as uint32_t,
                                        0x1ed7d1b9 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x24b889 as libc::c_int as uint32_t,
                                        0xa1011a27 as libc::c_uint,
                                        0xac0cd344 as libc::c_uint,
                                        0x5e4d0dc as libc::c_int as uint32_t,
                                        0xeb6a2a24 as libc::c_uint,
                                        0x52b520f0 as libc::c_int as uint32_t,
                                        0x3217257a as libc::c_int as uint32_t,
                                        0x3a2b03f0 as libc::c_int as
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
                                       [0xdf1d043d as libc::c_uint,
                                        0xf20fc2af as libc::c_uint,
                                        0xb58d5a62 as libc::c_uint,
                                        0xf330240d as libc::c_uint,
                                        0xa0058c3b as libc::c_uint,
                                        0xfc7d229c as libc::c_uint,
                                        0xc78dd9f6 as libc::c_uint,
                                        0x15fee545 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5bc98cda as libc::c_int as uint32_t,
                                        0x501e8288 as libc::c_int as uint32_t,
                                        0xd046ac04 as libc::c_uint,
                                        0x41ef80e5 as libc::c_int as uint32_t,
                                        0x461210fb as libc::c_int as uint32_t,
                                        0x557d9f49 as libc::c_int as uint32_t,
                                        0xb8753f81 as libc::c_uint,
                                        0x4ab5b6b2 as libc::c_int as
                                            uint32_t],};
                         init
                     }],};
         init
     }];
/*
 * N: order of G
 */
static mut N: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0xfc632551 as libc::c_uint,
                        0xf3b9cac2 as libc::c_uint,
                        0xa7179e84 as libc::c_uint,
                        0xbce6faad as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0 as libc::c_int as uint32_t,
                        0xffffffff as libc::c_uint],};
         init
     }];
/*
 * MU = 2^512 / N
 * MU = ( (1 << 256) | MU_lower )
 */
static mut MU_lower: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0xeedf9bfe as libc::c_uint,
                        0x12ffd85 as libc::c_int as uint32_t,
                        0xdf1a6c21 as libc::c_uint,
                        0x43190552 as libc::c_int as uint32_t,
                        0xffffffff as libc::c_uint,
                        0xfffffffe as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0 as libc::c_int as uint32_t],};
         init
     }];
/*                                                    -*- coding: utf-8 -*-
 * ecc.c - Elliptic curve over GF(prime)
 *
 * Copyright (C) 2011, 2013, 2014, 2015
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
 * References:
 *
 * [1] Suite B Implementer's Guide to FIPS 186-3 (ECDSA), February 3, 2010.
 *
 * [2] Michael Brown, Darrel Hankerson, Julio López, and Alfred Menezes,
 *     Software Implementation of the NIST Elliptic Curves Over Prime Fields,
 *     Proceedings of the 2001 Conference on Topics in Cryptology: The
 *     Cryptographer's Track at RSA
 *     Pages 250-265, Springer-Verlag London, UK, 2001
 *     ISBN:3-540-41898-9
 *
 * [3] Mustapha Hedabou, Pierre Pinel, Lucien Bénéteau,
 *     A comb method to render ECC resistant against Side Channel Attacks,
 *     2004
 */
/*
 * Coefficients
 */
/*
 * static const bn256 *coefficient_a;
 * static const bn256 *coefficient_b;
 */
/*
 * N: order of G
 */
/*
 * static const bn256 N[1];
 */
/*
 * MU = 2^512 / N
 * MU = ( (1 << 256) | MU_lower )
 */
/*
 * static const bn256 MU_lower[1];
 */
/*
 * w = 4
 * m = 256
 * d = 64
 * e = 32
 */
/*
 * static const ac precomputed_KG[15];
 * static const ac precomputed_2E_KG[15];
 */
unsafe extern "C" fn get_vk(mut K: *const bn256, mut i: libc::c_int)
 -> libc::c_int {
    let mut w0: uint32_t = 0;
    let mut w1: uint32_t = 0;
    let mut w2: uint32_t = 0;
    let mut w3: uint32_t = 0;
    if i < 32 as libc::c_int {
        w3 = (*K).word[6 as libc::c_int as usize];
        w2 = (*K).word[4 as libc::c_int as usize];
        w1 = (*K).word[2 as libc::c_int as usize];
        w0 = (*K).word[0 as libc::c_int as usize]
    } else {
        w3 = (*K).word[7 as libc::c_int as usize];
        w2 = (*K).word[5 as libc::c_int as usize];
        w1 = (*K).word[3 as libc::c_int as usize];
        w0 = (*K).word[1 as libc::c_int as usize];
        i -= 32 as libc::c_int
    }
    w3 >>= i;
    w2 >>= i;
    w1 >>= i;
    w0 >>= i;
    return ((w3 & 1 as libc::c_int as libc::c_uint) << 3 as libc::c_int |
                (w2 & 1 as libc::c_int as libc::c_uint) << 2 as libc::c_int |
                (w1 & 1 as libc::c_int as libc::c_uint) << 1 as libc::c_int |
                w0 & 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
/* *
 * @brief	X  = k * G
 *
 * @param K	scalar k
 *
 * Return -1 on error.
 * Return 0 on success.
 */
#[no_mangle]
pub unsafe extern "C" fn compute_kG_p256r1(mut X: *mut ac,
                                           mut K: *const bn256)
 -> libc::c_int {
    let mut index: [uint8_t; 64] =
        [0;
            64]; /* Lower 4-bit for index absolute value, msb is
			for sign (encoded as: 0 means 1, 1 means -1).  */
    let mut K_dash: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut Q: [jpc; 1] =
        [jpc{x: [bn256{word: [0; 8],}; 1],
             y: [bn256{word: [0; 8],}; 1],
             z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut tmp: [jpc; 1] =
        [jpc{x: [bn256{word: [0; 8],}; 1],
             y: [bn256{word: [0; 8],}; 1],
             z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut dst: *mut jpc = 0 as *mut jpc;
    let mut i: libc::c_int = 0;
    let mut vk: libc::c_int = 0;
    let mut k_is_even: uint32_t = bn256_is_even(K) as uint32_t;
    bn256_sub_uint(K_dash.as_mut_ptr(), K, k_is_even);
    /* It keeps the condition: 1 <= K' <= N - 2, and K' is odd.  */
    /* Fill index.  */
    vk = get_vk(K_dash.as_mut_ptr(), 0 as libc::c_int); /* infinity */
    i = 1 as libc::c_int;
    while i < 64 as libc::c_int {
        let mut vk_next: libc::c_int = 0;
        let mut is_zero: libc::c_int = 0;
        vk_next = get_vk(K_dash.as_mut_ptr(), i);
        is_zero = (vk_next == 0 as libc::c_int) as libc::c_int;
        index[(i - 1 as libc::c_int) as usize] =
            (vk - 1 as libc::c_int | is_zero << 7 as libc::c_int) as uint8_t;
        vk = if is_zero != 0 { vk } else { vk_next };
        i += 1
    }
    index[63 as libc::c_int as usize] = (vk - 1 as libc::c_int) as uint8_t;
    memset((*Q.as_mut_ptr()).z.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<bn256>() as libc::c_ulong);
    i = 31 as libc::c_int;
    while i >= 0 as libc::c_int {
        jpc_double_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_add_ac_signed_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr(),
                                 &*precomputed_2E_KG.as_ptr().offset((*index.as_mut_ptr().offset((i
                                                                                                      +
                                                                                                      32
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     as
                                                                                                     isize)
                                                                          as
                                                                          libc::c_int
                                                                          &
                                                                          0xf
                                                                              as
                                                                              libc::c_int)
                                                                         as
                                                                         isize),
                                 index[(i + 32 as libc::c_int) as usize] as
                                     libc::c_int >> 7 as libc::c_int);
        jpc_add_ac_signed_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr(),
                                 &*precomputed_KG.as_ptr().offset((*index.as_mut_ptr().offset(i
                                                                                                  as
                                                                                                  isize)
                                                                       as
                                                                       libc::c_int
                                                                       &
                                                                       0xf as
                                                                           libc::c_int)
                                                                      as
                                                                      isize),
                                 index[i as usize] as libc::c_int >>
                                     7 as libc::c_int);
        i -= 1
    }
    dst = if k_is_even != 0 { Q.as_mut_ptr() } else { tmp.as_mut_ptr() };
    jpc_add_ac_p256r1(dst, Q.as_mut_ptr(),
                      &*precomputed_KG.as_ptr().offset(0 as libc::c_int as
                                                           isize));
    return jpc_to_ac_p256r1(X, Q.as_mut_ptr());
}
/* *
 * check if P is on the curve.
 *
 * Return -1 on error.
 * Return 0 on success.
 */
unsafe extern "C" fn point_is_on_the_curve(mut P: *const ac) -> libc::c_int {
    let mut s: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut t: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    /* Elliptic curve: y^2 = x^3 + a*x + b */
    modp256r1_sqr(s.as_mut_ptr(), (*P).x.as_ptr());
    modp256r1_mul(s.as_mut_ptr(), s.as_mut_ptr(), (*P).x.as_ptr());
    modp256r1_mul(t.as_mut_ptr(), coefficient_a.as_ptr(), (*P).x.as_ptr());
    modp256r1_add(s.as_mut_ptr(), s.as_mut_ptr(), t.as_mut_ptr());
    modp256r1_add(s.as_mut_ptr(), s.as_mut_ptr(), coefficient_b.as_ptr());
    modp256r1_sqr(t.as_mut_ptr(), (*P).y.as_ptr());
    if bn256_cmp(s.as_mut_ptr(), t.as_mut_ptr()) == 0 as libc::c_int {
        return 0 as libc::c_int
    } else { return -(1 as libc::c_int) };
}
unsafe extern "C" fn get_vk_kP(mut K: *const bn256, mut i: libc::c_int)
 -> libc::c_int {
    let mut w: uint32_t = 0;
    let mut blk: uint8_t = (i / 32 as libc::c_int) as uint8_t;
    let mut pos: uint8_t = (i % 32 as libc::c_int) as uint8_t;
    let mut col: uint8_t =
        (3 as libc::c_int * (pos as libc::c_int % 11 as libc::c_int) +
             (pos as libc::c_int >= 11 as libc::c_int) as libc::c_int +
             (pos as libc::c_int >= 22 as libc::c_int) as libc::c_int) as
            uint8_t;
    let mut word_index: uint8_t =
        (blk as libc::c_int * 3 as libc::c_int +
             pos as libc::c_int / 11 as libc::c_int) as uint8_t;
    w =
        (*K).word[word_index as usize] >> col as libc::c_int &
            7 as libc::c_int as libc::c_uint;
    if (word_index as libc::c_int) < 7 as libc::c_int &&
           (pos as libc::c_int == 10 as libc::c_int ||
                pos as libc::c_int == 21 as libc::c_int) {
        let mut mask: uint8_t = 0;
        let mut shift: uint8_t = 0;
        word_index = word_index.wrapping_add(1);
        if pos as libc::c_int == 10 as libc::c_int {
            shift = 2 as libc::c_int as uint8_t;
            mask = 4 as libc::c_int as uint8_t
        } else {
            shift = 1 as libc::c_int as uint8_t;
            mask = 6 as libc::c_int as uint8_t
        }
        w |=
            (*K).word[word_index as usize] << shift as libc::c_int &
                mask as libc::c_uint
    }
    return w as libc::c_int;
}
/* *
 * @brief	X  = k * P
 *
 * @param K	scalar k
 * @param P	P in affine coordiate
 *
 * Return -1 on error.
 * Return 0 on success.
 *
 * For the curve (cofactor is 1 and n is prime), possible error cases are:
 *
 *     P is not on the curve.
 *     P = G, k = n
 *     Something wrong in the code.
 *
 * Mathmatically, k=1 and P=O is another possible case, but O cannot be
 * represented by affine coordinate.
 */
#[no_mangle]
pub unsafe extern "C" fn compute_kP_p256r1(mut X: *mut ac,
                                           mut K: *const bn256,
                                           mut P: *const ac) -> libc::c_int {
    let mut index: [uint8_t; 86] =
        [0;
            86]; /* Lower 2-bit for index absolute value, msb is
			for sign (encoded as: 0 means 1, 1 means -1).  */
    let mut K_dash: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut k_is_even: uint32_t = bn256_is_even(K) as uint32_t;
    let mut Q: [jpc; 1] =
        [jpc{x: [bn256{word: [0; 8],}; 1],
             y: [bn256{word: [0; 8],}; 1],
             z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut tmp: [jpc; 1] =
        [jpc{x: [bn256{word: [0; 8],}; 1],
             y: [bn256{word: [0; 8],}; 1],
             z: [bn256{word: [0; 8],}; 1],}; 1];
    let mut dst: *mut jpc = 0 as *mut jpc;
    let mut i: libc::c_int = 0;
    let mut vk: libc::c_int = 0;
    let mut P3: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut P5: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut P7: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut p_Pi: [*const ac; 4] = [0 as *const ac; 4];
    if point_is_on_the_curve(P) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if bn256_sub(K_dash.as_mut_ptr(), K, N.as_ptr()) ==
           0 as libc::c_int as libc::c_uint {
        /* >= N, it's too big.  */
        return -(1 as libc::c_int)
    }
    bn256_sub_uint(K_dash.as_mut_ptr(), K, k_is_even);
    /* It keeps the condition: 1 <= K' <= N - 2, and K' is odd.  */
    p_Pi[0 as libc::c_int as usize] = P;
    p_Pi[1 as libc::c_int as usize] = P3.as_mut_ptr();
    p_Pi[2 as libc::c_int as usize] = P5.as_mut_ptr();
    p_Pi[3 as libc::c_int as usize] = P7.as_mut_ptr();
    let mut Q1: [jpc; 1] =
        [jpc{x: [bn256{word: [0; 8],}; 1],
             y: [bn256{word: [0; 8],}; 1],
             z: [bn256{word: [0; 8],}; 1],}; 1];
    memcpy((*Q.as_mut_ptr()).x.as_mut_ptr() as *mut libc::c_void,
           (*P).x.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memcpy((*Q.as_mut_ptr()).y.as_mut_ptr() as *mut libc::c_void,
           (*P).y.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memset((*Q.as_mut_ptr()).z.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*(*Q.as_mut_ptr()).z.as_mut_ptr()).word[0 as libc::c_int as usize] =
        1 as libc::c_int as uint32_t;
    jpc_double_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr());
    jpc_add_ac_p256r1(Q1.as_mut_ptr(), Q.as_mut_ptr(), P);
    if jpc_to_ac_p256r1(P3.as_mut_ptr(), Q1.as_mut_ptr()) < 0 as libc::c_int {
        /* Never occurs, except coding errors.  */
        return -(1 as libc::c_int)
    }
    jpc_double_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr());
    jpc_add_ac_p256r1(Q1.as_mut_ptr(), Q.as_mut_ptr(), P);
    if jpc_to_ac_p256r1(P5.as_mut_ptr(), Q1.as_mut_ptr()) < 0 as libc::c_int {
        /* Never occurs, except coding errors.  */
        return -(1 as libc::c_int)
    }
    memcpy((*Q.as_mut_ptr()).x.as_mut_ptr() as *mut libc::c_void,
           (*P3.as_mut_ptr()).x.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memcpy((*Q.as_mut_ptr()).y.as_mut_ptr() as *mut libc::c_void,
           (*P3.as_mut_ptr()).y.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<bn256>() as libc::c_ulong);
    memset((*Q.as_mut_ptr()).z.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<bn256>() as libc::c_ulong);
    (*(*Q.as_mut_ptr()).z.as_mut_ptr()).word[0 as libc::c_int as usize] =
        1 as libc::c_int as uint32_t;
    jpc_double_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr());
    jpc_add_ac_p256r1(Q1.as_mut_ptr(), Q.as_mut_ptr(), P);
    if jpc_to_ac_p256r1(P7.as_mut_ptr(), Q1.as_mut_ptr()) < 0 as libc::c_int {
        /* Never occurs, except coding errors.  */
        return -(1 as libc::c_int)
    }
    /* Fill index.  */
    vk = get_vk_kP(K_dash.as_mut_ptr(), 0 as libc::c_int); /* infinity */
    i = 1 as libc::c_int;
    while i < 86 as libc::c_int {
        let mut vk_next: libc::c_int = 0;
        let mut is_even: libc::c_int = 0;
        vk_next = get_vk_kP(K_dash.as_mut_ptr(), i);
        is_even =
            (vk_next & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        index[(i - 1 as libc::c_int) as usize] =
            (is_even << 7 as libc::c_int |
                 (if is_even != 0 {
                      (7 as libc::c_int) - vk
                  } else { (vk) - 1 as libc::c_int }) >> 1 as libc::c_int) as
                uint8_t;
        vk = vk_next + is_even;
        i += 1
    }
    index[85 as libc::c_int as usize] =
        (vk - 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    memset((*Q.as_mut_ptr()).z.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<bn256>() as libc::c_ulong);
    i = 85 as libc::c_int;
    while i >= 0 as libc::c_int {
        jpc_double_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_double_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_double_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_add_ac_signed_p256r1(Q.as_mut_ptr(), Q.as_mut_ptr(),
                                 p_Pi[(index[i as usize] as libc::c_int &
                                           0x3 as libc::c_int) as usize],
                                 index[i as usize] as libc::c_int >>
                                     7 as libc::c_int);
        i -= 1
    }
    dst = if k_is_even != 0 { Q.as_mut_ptr() } else { tmp.as_mut_ptr() };
    jpc_add_ac_p256r1(dst, Q.as_mut_ptr(), P);
    return jpc_to_ac_p256r1(X, Q.as_mut_ptr());
}
/* *
 * @brief Compute signature (r,s) of hash string z with secret key d
 */
#[no_mangle]
pub unsafe extern "C" fn ecdsa_p256r1(mut r: *mut bn256, mut s: *mut bn256,
                                      mut z: *const bn256,
                                      mut d: *const bn256) {
    let mut k: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut KG: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut tmp: [bn512; 1] = [bn512{word: [0; 16],}; 1];
    let mut k_inv: [bn256; 1] = [bn256{word: [0; 8],}; 1];
    let mut carry: uint32_t = 0;
    loop  {
        loop  {
            bn256_random(k.as_mut_ptr());
            if !(bn256_add_uint(k.as_mut_ptr(), k.as_mut_ptr(),
                                1 as libc::c_int as uint32_t) != 0) {
                if !(bn256_sub(k_inv.as_mut_ptr(), k.as_mut_ptr(), N.as_ptr())
                         == 0 as libc::c_int as libc::c_uint) {
                    /* 1 <= k <= N - 1 */
                    compute_kG_p256r1(KG.as_mut_ptr(), k.as_mut_ptr());
                    carry =
                        bn256_sub(r, (*KG.as_mut_ptr()).x.as_mut_ptr(),
                                  N.as_ptr());
                    if carry != 0 {
                        memcpy(r as *mut libc::c_void,
                               (*KG.as_mut_ptr()).x.as_mut_ptr() as
                                   *const libc::c_void,
                               ::std::mem::size_of::<bn256>() as
                                   libc::c_ulong);
                    } else {
                        memcpy((*KG.as_mut_ptr()).x.as_mut_ptr() as
                                   *mut libc::c_void,
                               r as *const libc::c_void,
                               ::std::mem::size_of::<bn256>() as
                                   libc::c_ulong);
                    }
                }
            }
            /* >= N, it's too big.  */
            if !(bn256_is_zero(r) != 0) { break ; }
        }
        mod_inv(k_inv.as_mut_ptr(), k.as_mut_ptr(), N.as_ptr());
        bn256_mul(tmp.as_mut_ptr(), r, d);
        mod_reduce(s, tmp.as_mut_ptr(), N.as_ptr(), MU_lower.as_ptr());
        carry = bn256_add(s, s, z);
        if carry != 0 {
            bn256_sub(s, s, N.as_ptr());
        } else { bn256_sub(tmp.as_mut_ptr() as *mut bn256, s, N.as_ptr()); }
        bn256_mul(tmp.as_mut_ptr(), s, k_inv.as_mut_ptr());
        mod_reduce(s, tmp.as_mut_ptr(), N.as_ptr(), MU_lower.as_ptr());
        if !(bn256_is_zero(s) != 0) { break ; }
    };
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
pub unsafe extern "C" fn check_secret_p256r1(mut d0: *const bn256,
                                             mut d1: *mut bn256)
 -> libc::c_int {
    let mut Q0: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    let mut Q1: [ac; 1] =
        [ac{x: [bn256{word: [0; 8],}; 1], y: [bn256{word: [0; 8],}; 1],}; 1];
    if bn256_is_zero(d0) != 0 ||
           bn256_sub(d1, N.as_ptr(), d0) <= 0 as libc::c_int as libc::c_uint {
        /* == 0 or >= N, it's not valid.  */
        return 0 as libc::c_int
    }
    compute_kG_p256r1(Q0.as_mut_ptr(), d0);
    compute_kG_p256r1(Q1.as_mut_ptr(), d1);
    /*
   * Jivsov compliant key check
   */
    return bn256_cmp(Q1[0 as libc::c_int as usize].y.as_mut_ptr(),
                     Q0[0 as libc::c_int as usize].y.as_mut_ptr());
}
