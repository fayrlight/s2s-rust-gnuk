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
    fn modp256k1_add(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn modp256k1_mul(X: *mut bn256, A: *const bn256, B: *const bn256);
    #[no_mangle]
    fn modp256k1_sqr(X: *mut bn256, A: *const bn256);
    #[no_mangle]
    fn jpc_double_p256k1(X: *mut jpc, A: *const jpc);
    #[no_mangle]
    fn jpc_add_ac_p256k1(X: *mut jpc, A: *const jpc, B: *const ac);
    #[no_mangle]
    fn jpc_add_ac_signed_p256k1(X: *mut jpc, A: *const jpc, B: *const ac,
                                minus: libc::c_int);
    #[no_mangle]
    fn jpc_to_ac_p256k1(X: *mut ac, A: *const jpc) -> libc::c_int;
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
static mut coefficient_b: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0x7 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t],};
         init
     }];
static mut precomputed_KG: [ac; 15] =
    [{
         let mut init =
             ac{x:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x16f81798 as libc::c_int as uint32_t,
                                        0x59f2815b as libc::c_int as uint32_t,
                                        0x2dce28d9 as libc::c_int as uint32_t,
                                        0x29bfcdb as libc::c_int as uint32_t,
                                        0xce870b07 as libc::c_uint,
                                        0x55a06295 as libc::c_int as uint32_t,
                                        0xf9dcbbac as libc::c_uint,
                                        0x79be667e as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xfb10d4b8 as libc::c_uint,
                                        0x9c47d08f as libc::c_uint,
                                        0xa6855419 as libc::c_uint,
                                        0xfd17b448 as libc::c_uint,
                                        0xe1108a8 as libc::c_int as uint32_t,
                                        0x5da4fbfc as libc::c_int as uint32_t,
                                        0x26a3c465 as libc::c_int as uint32_t,
                                        0x483ada77 as libc::c_int as
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
                                       [0x42d0e6bd as libc::c_int as uint32_t,
                                        0x13b7e0e7 as libc::c_int as uint32_t,
                                        0xdb0f5e53 as libc::c_uint,
                                        0xf774d163 as libc::c_uint,
                                        0x104d6ecb as libc::c_int as uint32_t,
                                        0x82a2147c as libc::c_uint,
                                        0x243c4e25 as libc::c_int as uint32_t,
                                        0x3322d401 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x6c28b2a0 as libc::c_int as uint32_t,
                                        0x24f3a2e9 as libc::c_int as uint32_t,
                                        0xa2873af6 as libc::c_uint,
                                        0x2805f63e as libc::c_int as uint32_t,
                                        0x4ddaf9b7 as libc::c_int as uint32_t,
                                        0xbfb019bc as libc::c_uint,
                                        0xe9664ef5 as libc::c_uint,
                                        0x56e70797 as libc::c_int as
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
                                       [0x829d122a as libc::c_uint,
                                        0xdca81127 as libc::c_uint,
                                        0x67e99549 as libc::c_int as uint32_t,
                                        0x8f17f314 as libc::c_uint,
                                        0x6a8a9e73 as libc::c_int as uint32_t,
                                        0x9b889085 as libc::c_uint,
                                        0x846dd99d as libc::c_uint,
                                        0x583fdfd9 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x63c4eac4 as libc::c_int as uint32_t,
                                        0xf3c7719e as libc::c_uint,
                                        0xb734b37a as libc::c_uint,
                                        0xb44685a3 as libc::c_uint,
                                        0x572a47a6 as libc::c_int as uint32_t,
                                        0x9f92d2d6 as libc::c_uint,
                                        0x2ff57d81 as libc::c_int as uint32_t,
                                        0xabc6232f as libc::c_uint],};
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
                                       [0x9ec4c0da as libc::c_uint,
                                        0x1b7b444c as libc::c_int as uint32_t,
                                        0x723ea335 as libc::c_int as uint32_t,
                                        0xe88c5678 as libc::c_uint,
                                        0x981f162e as libc::c_uint,
                                        0x9239c1ad as libc::c_uint,
                                        0xf63b5f33 as libc::c_uint,
                                        0x8f68b9d2 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x501fff82 as libc::c_int as uint32_t,
                                        0xf23cbf79 as libc::c_uint,
                                        0x95510bfd as libc::c_uint,
                                        0xbbea2cfe as libc::c_uint,
                                        0xb6be215d as libc::c_uint,
                                        0xde1d90c2 as libc::c_uint,
                                        0xba063986 as libc::c_uint,
                                        0x662a9f2d as libc::c_int as
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
                                       [0x114cbf09 as libc::c_int as uint32_t,
                                        0x63c5e885 as libc::c_int as uint32_t,
                                        0x7be77e3e as libc::c_int as uint32_t,
                                        0x2f27ce93 as libc::c_int as uint32_t,
                                        0xf54a3e33 as libc::c_uint,
                                        0xdaa6d12d as libc::c_uint,
                                        0x3eff872c as libc::c_int as uint32_t,
                                        0x8b300e51 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xb3b10a39 as libc::c_uint,
                                        0x26c6ff28 as libc::c_int as uint32_t,
                                        0x9aaf7169 as libc::c_uint,
                                        0x8f6a7aa as libc::c_int as uint32_t,
                                        0x6b8238ea as libc::c_int as uint32_t,
                                        0x446f0d46 as libc::c_int as uint32_t,
                                        0x7f43c0cc as libc::c_int as uint32_t,
                                        0x1cec3067 as libc::c_int as
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
                                       [0x75e9070 as libc::c_int as uint32_t,
                                        0xba16ce6a as libc::c_uint,
                                        0x9b5cfe37 as libc::c_uint,
                                        0xbc26893d as libc::c_uint,
                                        0x9c510774 as libc::c_uint,
                                        0xe1ddadfe as libc::c_uint,
                                        0xfe3ae2f4 as libc::c_uint,
                                        0x90922d88 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x5c08824a as libc::c_int as uint32_t,
                                        0x653943cc as libc::c_int as uint32_t,
                                        0xfce8f4bc as libc::c_uint,
                                        0x6d74475 as libc::c_int as uint32_t,
                                        0x533c615d as libc::c_int as uint32_t,
                                        0x8d101fa7 as libc::c_uint,
                                        0x742108a9 as libc::c_int as uint32_t,
                                        0x7b1903f6 as libc::c_int as
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
                                       [0x6ebdc96c as libc::c_int as uint32_t,
                                        0x1bcfa45c as libc::c_int as uint32_t,
                                        0x1c7584ba as libc::c_int as uint32_t,
                                        0xe400bc04 as libc::c_uint,
                                        0x74cf531f as libc::c_int as uint32_t,
                                        0x6395e20e as libc::c_int as uint32_t,
                                        0xc5131b30 as libc::c_uint,
                                        0x1edd0bb1 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe358cf9e as libc::c_uint,
                                        0xa117161b as libc::c_uint,
                                        0x2724d11c as libc::c_int as uint32_t,
                                        0xe490d6f0 as libc::c_uint,
                                        0xee6dd8c9 as libc::c_uint,
                                        0xf75062f6 as libc::c_uint,
                                        0xfba373e4 as libc::c_uint,
                                        0x31e03b2b as libc::c_int as
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
                                       [0x2120e2b3 as libc::c_int as uint32_t,
                                        0x7f3b58fa as libc::c_int as uint32_t,
                                        0x7f47f9aa as libc::c_int as uint32_t,
                                        0x7a58fdce as libc::c_int as uint32_t,
                                        0x4ce6e521 as libc::c_int as uint32_t,
                                        0xe7be4ae3 as libc::c_uint,
                                        0x1f51bdba as libc::c_int as uint32_t,
                                        0xeaa649f2 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xba5ad93d as libc::c_uint,
                                        0xd47a5305 as libc::c_uint,
                                        0xf13f7e59 as libc::c_uint,
                                        0x1a6b965 as libc::c_int as uint32_t,
                                        0x9879aa5a as libc::c_uint,
                                        0xc69a80f8 as libc::c_uint,
                                        0x5bbbb03a as libc::c_int as uint32_t,
                                        0xbe3279ed as libc::c_uint],};
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
                                       [0x27bb4d71 as libc::c_int as uint32_t,
                                        0xcf291a33 as libc::c_uint,
                                        0x33524832 as libc::c_int as uint32_t,
                                        0x6caf7d6b as libc::c_int as uint32_t,
                                        0x766584ee as libc::c_int as uint32_t,
                                        0x6e0ee131 as libc::c_int as uint32_t,
                                        0xd064c589 as libc::c_uint,
                                        0x160cb0f6 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x17136e8d as libc::c_int as uint32_t,
                                        0x9d5de554 as libc::c_uint,
                                        0x1aab720e as libc::c_int as uint32_t,
                                        0xe3f2d468 as libc::c_uint,
                                        0xccf75cc2 as libc::c_uint,
                                        0xd1378b49 as libc::c_uint,
                                        0xc4ff16e1 as libc::c_uint,
                                        0x6920c375 as libc::c_int as
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
                                       [0x1a9ee611 as libc::c_int as uint32_t,
                                        0x3eef9e96 as libc::c_int as uint32_t,
                                        0x9cc37faf as libc::c_uint,
                                        0xfe4d7bf3 as libc::c_uint,
                                        0xb321d965 as libc::c_uint,
                                        0x462aa9b3 as libc::c_int as uint32_t,
                                        0x208736c5 as libc::c_int as uint32_t,
                                        0x1702da3e as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x3a545ceb as libc::c_int as uint32_t,
                                        0xfba57bbf as libc::c_uint,
                                        0x7ea858f5 as libc::c_int as uint32_t,
                                        0x6dbcd766 as libc::c_int as uint32_t,
                                        0x680d92f1 as libc::c_int as uint32_t,
                                        0x88e897c as libc::c_int as uint32_t,
                                        0xbc626c80 as libc::c_uint,
                                        0x468c1fd8 as libc::c_int as
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
                                       [0xb188660a as libc::c_uint,
                                        0xb40f85c7 as libc::c_uint,
                                        0x99bc3c36 as libc::c_uint,
                                        0xc5873c19 as libc::c_uint,
                                        0x7f33b54c as libc::c_int as uint32_t,
                                        0x3c7b4541 as libc::c_int as uint32_t,
                                        0x1f8c9bf8 as libc::c_int as uint32_t,
                                        0x4cd3a93c as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x33099cb0 as libc::c_int as uint32_t,
                                        0xf8dce380 as libc::c_uint,
                                        0x2edd2f33 as libc::c_int as uint32_t,
                                        0x7a167dd6 as libc::c_int as uint32_t,
                                        0xffe35b7 as libc::c_int as uint32_t,
                                        0x576d8987 as libc::c_int as uint32_t,
                                        0xc68ace5c as libc::c_uint,
                                        0xd2de0386 as libc::c_uint],};
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
                                       [0x6658bb08 as libc::c_int as uint32_t,
                                        0x9a9e0a72 as libc::c_uint,
                                        0xc589607b as libc::c_uint,
                                        0xe23c5f2a as libc::c_uint,
                                        0xf2bfb4c8 as libc::c_uint,
                                        0xa048ca14 as libc::c_uint,
                                        0xc62c2291 as libc::c_uint,
                                        0x4d9a0f89 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xf827294 as libc::c_int as uint32_t,
                                        0x427b5f31 as libc::c_int as uint32_t,
                                        0x9f2c35cd as libc::c_uint,
                                        0x1ea7a8b5 as libc::c_int as uint32_t,
                                        0x85a3c00f as libc::c_uint,
                                        0x95442e56 as libc::c_uint,
                                        0x9b57975a as libc::c_uint,
                                        0x8cb83121 as libc::c_uint],};
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
                                       [0x51f5cf67 as libc::c_int as uint32_t,
                                        0x4333f0da as libc::c_int as uint32_t,
                                        0xf4f0d3cb as libc::c_uint,
                                        0x6d3ea47c as libc::c_int as uint32_t,
                                        0xa05a831f as libc::c_uint,
                                        0x442fda14 as libc::c_int as uint32_t,
                                        0x16d3e81 as libc::c_int as uint32_t,
                                        0x6a496013 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xe52e0f48 as libc::c_uint,
                                        0xf647318c as libc::c_uint,
                                        0x4a0d5ff1 as libc::c_int as uint32_t,
                                        0x5ff3a66e as libc::c_int as uint32_t,
                                        0x61199ba8 as libc::c_int as uint32_t,
                                        0x46ed81a as libc::c_int as uint32_t,
                                        0x3e79c23a as libc::c_int as uint32_t,
                                        0x578edf08 as libc::c_int as
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
                                       [0x3ea01ea7 as libc::c_int as uint32_t,
                                        0xb8f996f8 as libc::c_uint,
                                        0x7497bb15 as libc::c_int as uint32_t,
                                        0xc0045d33 as libc::c_uint,
                                        0x6205647c as libc::c_int as uint32_t,
                                        0xc4749dc9 as libc::c_uint,
                                        0xefd22c9 as libc::c_int as uint32_t,
                                        0xd8946054 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x12774ad5 as libc::c_int as uint32_t,
                                        0x62dcb09 as libc::c_int as uint32_t,
                                        0x8be06e3a as libc::c_uint,
                                        0xcb13f310 as libc::c_uint,
                                        0x235de1a9 as libc::c_int as uint32_t,
                                        0xca281d35 as libc::c_uint,
                                        0x69c3645c as libc::c_int as uint32_t,
                                        0xaf8a7412 as libc::c_uint],};
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
                                       [0xbeb8b1e2 as libc::c_uint,
                                        0x8808ca5f as libc::c_uint,
                                        0xea0dda76 as libc::c_uint,
                                        0x262b204 as libc::c_int as uint32_t,
                                        0xddeb356b as libc::c_uint,
                                        0xb6fffffc as libc::c_uint,
                                        0xfbb83870 as libc::c_uint,
                                        0x52de253a as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x8f8d21ea as libc::c_uint,
                                        0x961f40c0 as libc::c_uint,
                                        0x2f03ed as libc::c_int as uint32_t,
                                        0x89686278 as libc::c_uint,
                                        0x38e421ea as libc::c_int as uint32_t,
                                        0xff834d7 as libc::c_int as uint32_t,
                                        0xd36fb8db as libc::c_uint,
                                        0x3a270d6f as libc::c_int as
                                            uint32_t],};
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
                                       [0x39a48db0 as libc::c_int as uint32_t,
                                        0xefd7835b as libc::c_uint,
                                        0x9b3c03bf as libc::c_uint,
                                        0x9f1215a2 as libc::c_uint,
                                        0x9b7bde45 as libc::c_uint,
                                        0x2791d0a0 as libc::c_int as uint32_t,
                                        0x696e7167 as libc::c_int as uint32_t,
                                        0x100f44da as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x2bc65a09 as libc::c_int as uint32_t,
                                        0xfbd5cd6 as libc::c_int as uint32_t,
                                        0xff5195ac as libc::c_uint,
                                        0xb7ff4a18 as libc::c_uint,
                                        0xc090666 as libc::c_int as uint32_t,
                                        0x2ec8f330 as libc::c_int as uint32_t,
                                        0x92a00b77 as libc::c_uint,
                                        0xcdd9e131 as libc::c_uint],};
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
                                       [0x40fb27b6 as libc::c_int as uint32_t,
                                        0x32427e28 as libc::c_int as uint32_t,
                                        0xbe430576 as libc::c_uint,
                                        0xc76e3db2 as libc::c_uint,
                                        0x61686aa5 as libc::c_int as uint32_t,
                                        0x10f238ad as libc::c_int as uint32_t,
                                        0xbe778b1b as libc::c_uint,
                                        0xfea74e3d as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xf23cb96f as libc::c_uint,
                                        0x701d3db7 as libc::c_int as uint32_t,
                                        0x973f7b77 as libc::c_uint,
                                        0x126b596b as libc::c_int as uint32_t,
                                        0xccb6af93 as libc::c_uint,
                                        0x7cf674de as libc::c_int as uint32_t,
                                        0x9b0b1329 as libc::c_uint,
                                        0x6e0568db as libc::c_int as
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
                                       [0x2c8118bc as libc::c_int as uint32_t,
                                        0x6cac5154 as libc::c_int as uint32_t,
                                        0x399ddd98 as libc::c_int as uint32_t,
                                        0x19bd4b34 as libc::c_int as uint32_t,
                                        0x2e9c8949 as libc::c_int as uint32_t,
                                        0x47248a8d as libc::c_int as uint32_t,
                                        0x2cefa3b1 as libc::c_int as uint32_t,
                                        0x734cb6a8 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x1e410fd5 as libc::c_int as uint32_t,
                                        0xf1b340ad as libc::c_uint,
                                        0xc4873539 as libc::c_uint,
                                        0xa2982bee as libc::c_uint,
                                        0xd4de4530 as libc::c_uint,
                                        0x7b5a3ea4 as libc::c_int as uint32_t,
                                        0x42202574 as libc::c_int as uint32_t,
                                        0xae46e10e as libc::c_uint],};
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
                                       [0xac1f98cd as libc::c_uint,
                                        0xcbfc99c8 as libc::c_uint,
                                        0x4d7f0308 as libc::c_int as uint32_t,
                                        0x52348905 as libc::c_int as uint32_t,
                                        0x1cc66021 as libc::c_int as uint32_t,
                                        0xfaed8a9c as libc::c_uint,
                                        0x4a474870 as libc::c_int as uint32_t,
                                        0x9c3919a8 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd4fc599d as libc::c_uint,
                                        0xbe7e5e03 as libc::c_uint,
                                        0x6c64c8e6 as libc::c_int as uint32_t,
                                        0x905326f7 as libc::c_uint,
                                        0xf260e641 as libc::c_uint,
                                        0x584f044b as libc::c_int as uint32_t,
                                        0x4a4ddd57 as libc::c_int as uint32_t,
                                        0xddb84f0f as libc::c_uint],};
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
                                       [0xed7cebed as libc::c_uint,
                                        0xc4aacaa8 as libc::c_uint,
                                        0x4fae424e as libc::c_int as uint32_t,
                                        0xb75d2dce as libc::c_uint,
                                        0xba20735e as libc::c_uint,
                                        0xa01585a2 as libc::c_uint,
                                        0xba122399 as libc::c_uint,
                                        0x3d75f24b as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xd5570dce as libc::c_uint,
                                        0xcbe4606f as libc::c_uint,
                                        0x2da192c2 as libc::c_int as uint32_t,
                                        0x9d00bfd7 as libc::c_uint,
                                        0xa57b7265 as libc::c_uint,
                                        0x9c3ce86b as libc::c_uint,
                                        0xec4edf5e as libc::c_uint,
                                        0x987a22f1 as libc::c_uint],};
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
                                       [0x73ea0665 as libc::c_int as uint32_t,
                                        0x211b9715 as libc::c_int as uint32_t,
                                        0xf3a1abbb as libc::c_uint,
                                        0x86f485d4 as libc::c_uint,
                                        0xcd076f0e as libc::c_uint,
                                        0xabd242d8 as libc::c_uint,
                                        0xba5dc88 as libc::c_int as uint32_t,
                                        0x862332ab as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x7b784911 as libc::c_int as uint32_t,
                                        0x9af505c as libc::c_int as uint32_t,
                                        0xcaf4fae7 as libc::c_uint,
                                        0xc89544e8 as libc::c_uint,
                                        0xae9a32eb as libc::c_uint,
                                        0x256625f6 as libc::c_int as uint32_t,
                                        0x606d1a3f as libc::c_int as uint32_t,
                                        0xe2532b72 as libc::c_uint],};
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
                                       [0xdeaf885 as libc::c_int as uint32_t,
                                        0x79e9f313 as libc::c_int as uint32_t,
                                        0x46df21c9 as libc::c_int as uint32_t,
                                        0x938ff76e as libc::c_uint,
                                        0xa953bb2c as libc::c_uint,
                                        0x1968f5fb as libc::c_int as uint32_t,
                                        0x29155f27 as libc::c_int as uint32_t,
                                        0xdff538bf as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x31d5d020 as libc::c_int as uint32_t,
                                        0xf7bae0b1 as libc::c_uint,
                                        0x1a676a8d as libc::c_int as uint32_t,
                                        0x5afdc787 as libc::c_int as uint32_t,
                                        0xfa9d53ff as libc::c_uint,
                                        0x11b4f032 as libc::c_int as uint32_t,
                                        0xc5959167 as libc::c_uint,
                                        0x86ba433e as libc::c_uint],};
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
                                       [0x9475b7ba as libc::c_uint,
                                        0x884fdff0 as libc::c_uint,
                                        0xe4918b3d as libc::c_uint,
                                        0xe039e730 as libc::c_uint,
                                        0xf5018cdb as libc::c_uint,
                                        0x3d3e57ed as libc::c_int as uint32_t,
                                        0x1943785c as libc::c_int as uint32_t,
                                        0x95939698 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x7524f2fd as libc::c_int as uint32_t,
                                        0xe9b8abf8 as libc::c_uint,
                                        0xc8709385 as libc::c_uint,
                                        0x9c653f64 as libc::c_uint,
                                        0x4b9cd684 as libc::c_int as uint32_t,
                                        0x8ba0386a as libc::c_uint,
                                        0x88c331dd as libc::c_uint,
                                        0x2e7e5528 as libc::c_int as
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
                                       [0xeefe79e5 as libc::c_uint,
                                        0x940bef53 as libc::c_uint,
                                        0xbe9b87f3 as libc::c_uint,
                                        0xc518d286 as libc::c_uint,
                                        0x7833042c as libc::c_int as uint32_t,
                                        0x9e0c7c76 as libc::c_uint,
                                        0x11fbe152 as libc::c_int as uint32_t,
                                        0x104e2cb5 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x50bbec83 as libc::c_int as uint32_t,
                                        0xc0d35e0f as libc::c_uint,
                                        0x4acd0fcc as libc::c_int as uint32_t,
                                        0xee4879be as libc::c_uint,
                                        0x6085ee as libc::c_int as uint32_t,
                                        0xc8d80f5d as libc::c_uint,
                                        0x72fe1ac1 as libc::c_int as uint32_t,
                                        0x3c51bc1c as libc::c_int as
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
                                       [0xb2de976e as libc::c_uint,
                                        0x6187f61 as libc::c_int as uint32_t,
                                        0xf5e4b4b6 as libc::c_uint,
                                        0x52869e18 as libc::c_int as uint32_t,
                                        0x38d332ca as libc::c_int as uint32_t,
                                        0x74d4facd as libc::c_int as uint32_t,
                                        0xb3a2f8d9 as libc::c_uint,
                                        0x5c1c90b4 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0xdaa37893 as libc::c_uint,
                                        0x98644d09 as libc::c_uint,
                                        0xabe39818 as libc::c_uint,
                                        0x682435a8 as libc::c_int as uint32_t,
                                        0x469c53a0 as libc::c_int as uint32_t,
                                        0x17e46617 as libc::c_int as uint32_t,
                                        0x77dc2e64 as libc::c_int as uint32_t,
                                        0x642f9632 as libc::c_int as
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
                                       [0x222f6c54 as libc::c_int as uint32_t,
                                        0xad2101c5 as libc::c_uint,
                                        0xfa74785e as libc::c_uint,
                                        0xb05c7a58 as libc::c_uint,
                                        0x489bcdaf as libc::c_int as uint32_t,
                                        0xce55fa79 as libc::c_uint,
                                        0xffe88d54 as libc::c_uint,
                                        0xc1f920fd as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x9065e490 as libc::c_uint,
                                        0x32553ab0 as libc::c_int as uint32_t,
                                        0x35329f74 as libc::c_int as uint32_t,
                                        0x7611b9af as libc::c_int as uint32_t,
                                        0xab7b24c0 as libc::c_uint,
                                        0x57df19ef as libc::c_int as uint32_t,
                                        0x6181c447 as libc::c_int as uint32_t,
                                        0xb9a78749 as libc::c_uint],};
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
                                       [0xa80b7ea8 as libc::c_uint,
                                        0x392f156f as libc::c_int as uint32_t,
                                        0x8ae4a8bf as libc::c_uint,
                                        0x57ab7ca0 as libc::c_int as uint32_t,
                                        0x50c4b178 as libc::c_int as uint32_t,
                                        0xac320747 as libc::c_uint,
                                        0xe781feb as libc::c_int as uint32_t,
                                        0x146041b9 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x845279b2 as libc::c_uint,
                                        0xd343f075 as libc::c_uint,
                                        0x7387afa5 as libc::c_int as uint32_t,
                                        0x2d4fe757 as libc::c_int as uint32_t,
                                        0xa72f3c39 as libc::c_uint,
                                        0x151e0948 as libc::c_int as uint32_t,
                                        0x550da168 as libc::c_int as uint32_t,
                                        0x41a6d54e as libc::c_int as
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
                                       [0x75a0010 as libc::c_int as uint32_t,
                                        0xb3134ed3 as libc::c_uint,
                                        0x7ae93e23 as libc::c_int as uint32_t,
                                        0x9fa76f4b as libc::c_uint,
                                        0x7bb4daaa as libc::c_int as uint32_t,
                                        0xc0db256f as libc::c_uint,
                                        0x464dd8a3 as libc::c_int as uint32_t,
                                        0x7668dc27 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x9f5da977 as libc::c_uint,
                                        0x150063f5 as libc::c_int as uint32_t,
                                        0x5efce00 as libc::c_int as uint32_t,
                                        0x3acac5c8 as libc::c_int as uint32_t,
                                        0x884493fe as libc::c_uint,
                                        0xc8e12ffc as libc::c_uint,
                                        0x88f06bd2 as libc::c_uint,
                                        0x4ab936d8 as libc::c_int as
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
                                       [0x5d09ea98 as libc::c_int as uint32_t,
                                        0x996fde77 as libc::c_uint,
                                        0x4145da58 as libc::c_int as uint32_t,
                                        0x16ddf512 as libc::c_int as uint32_t,
                                        0xdc2fb225 as libc::c_uint,
                                        0xa97a6ca8 as libc::c_uint,
                                        0xfbdcdf5a as libc::c_uint,
                                        0xc7331f30 as libc::c_uint],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x86a86e52 as libc::c_uint,
                                        0x838f99e0 as libc::c_uint,
                                        0x77795edd as libc::c_int as uint32_t,
                                        0x68d39b29 as libc::c_int as uint32_t,
                                        0x9f412aaa as libc::c_uint,
                                        0xe4e4f97e as libc::c_uint,
                                        0x30d25352 as libc::c_int as uint32_t,
                                        0xe5cc2c0a as libc::c_uint],};
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
                                       [0x9c21ff71 as libc::c_uint,
                                        0xb3d68650 as libc::c_uint,
                                        0xddbe3884 as libc::c_uint,
                                        0x11e7589d as libc::c_int as uint32_t,
                                        0x423bac67 as libc::c_int as uint32_t,
                                        0x7efd4055 as libc::c_int as uint32_t,
                                        0x46957425 as libc::c_int as uint32_t,
                                        0x587a7293 as libc::c_int as
                                            uint32_t],};
                         init
                     }],
                y:
                    [{
                         let mut init =
                             bn256{word:
                                       [0x8f5a8fc6 as libc::c_uint,
                                        0x360adc2e as libc::c_int as uint32_t,
                                        0xbd69f12e as libc::c_uint,
                                        0x6f8bbafb as libc::c_int as uint32_t,
                                        0xa3f3b4d as libc::c_int as uint32_t,
                                        0xf671f423 as libc::c_uint,
                                        0x59942dc3 as libc::c_int as uint32_t,
                                        0xb49acb47 as libc::c_uint],};
                         init
                     }],};
         init
     }];
/*
 * N: order of G
 *    0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
 */
static mut N: [bn256; 1] =
    [{
         let mut init =
             bn256{word:
                       [0xd0364141 as libc::c_uint,
                        0xbfd25e8c as libc::c_uint,
                        0xaf48a03b as libc::c_uint,
                        0xbaaedce6 as libc::c_uint,
                        0xfffffffe as libc::c_uint,
                        0xffffffff as libc::c_uint,
                        0xffffffff as libc::c_uint,
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
                       [0x2fc9bec0 as libc::c_int as uint32_t,
                        0x402da173 as libc::c_int as uint32_t,
                        0x50b75fc4 as libc::c_int as uint32_t,
                        0x45512319 as libc::c_int as uint32_t,
                        0x1 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
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
pub unsafe extern "C" fn compute_kG_p256k1(mut X: *mut ac,
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
        jpc_double_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_add_ac_signed_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr(),
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
        jpc_add_ac_signed_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr(),
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
    jpc_add_ac_p256k1(dst, Q.as_mut_ptr(),
                      &*precomputed_KG.as_ptr().offset(0 as libc::c_int as
                                                           isize));
    return jpc_to_ac_p256k1(X, Q.as_mut_ptr());
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
    modp256k1_sqr(s.as_mut_ptr(), (*P).x.as_ptr());
    modp256k1_mul(s.as_mut_ptr(), s.as_mut_ptr(), (*P).x.as_ptr());
    modp256k1_add(s.as_mut_ptr(), s.as_mut_ptr(), coefficient_b.as_ptr());
    modp256k1_sqr(t.as_mut_ptr(), (*P).y.as_ptr());
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
pub unsafe extern "C" fn compute_kP_p256k1(mut X: *mut ac,
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
    jpc_double_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr());
    jpc_add_ac_p256k1(Q1.as_mut_ptr(), Q.as_mut_ptr(), P);
    if jpc_to_ac_p256k1(P3.as_mut_ptr(), Q1.as_mut_ptr()) < 0 as libc::c_int {
        /* Never occurs, except coding errors.  */
        return -(1 as libc::c_int)
    }
    jpc_double_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr());
    jpc_add_ac_p256k1(Q1.as_mut_ptr(), Q.as_mut_ptr(), P);
    if jpc_to_ac_p256k1(P5.as_mut_ptr(), Q1.as_mut_ptr()) < 0 as libc::c_int {
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
    jpc_double_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr());
    jpc_add_ac_p256k1(Q1.as_mut_ptr(), Q.as_mut_ptr(), P);
    if jpc_to_ac_p256k1(P7.as_mut_ptr(), Q1.as_mut_ptr()) < 0 as libc::c_int {
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
        jpc_double_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_double_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_double_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr());
        jpc_add_ac_signed_p256k1(Q.as_mut_ptr(), Q.as_mut_ptr(),
                                 p_Pi[(index[i as usize] as libc::c_int &
                                           0x3 as libc::c_int) as usize],
                                 index[i as usize] as libc::c_int >>
                                     7 as libc::c_int);
        i -= 1
    }
    dst = if k_is_even != 0 { Q.as_mut_ptr() } else { tmp.as_mut_ptr() };
    jpc_add_ac_p256k1(dst, Q.as_mut_ptr(), P);
    return jpc_to_ac_p256k1(X, Q.as_mut_ptr());
}
/* *
 * @brief Compute signature (r,s) of hash string z with secret key d
 */
#[no_mangle]
pub unsafe extern "C" fn ecdsa_p256k1(mut r: *mut bn256, mut s: *mut bn256,
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
                    compute_kG_p256k1(KG.as_mut_ptr(), k.as_mut_ptr());
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
pub unsafe extern "C" fn check_secret_p256k1(mut d0: *const bn256,
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
    compute_kG_p256k1(Q0.as_mut_ptr(), d0);
    compute_kG_p256k1(Q1.as_mut_ptr(), d1);
    /*
   * Jivsov compliant key check
   */
    return bn256_cmp(Q1[0 as libc::c_int as usize].y.as_mut_ptr(),
                     Q0[0 as libc::c_int as usize].y.as_mut_ptr());
}
