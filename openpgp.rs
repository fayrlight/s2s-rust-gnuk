#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type chx_pq;
    pub type chx_thread;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_setcancelstate(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn eventflag_wait(ev: *mut eventflag) -> eventmask_t;
    #[no_mangle]
    fn eventflag_signal(ev: *mut eventflag, m: eventmask_t);
    #[no_mangle]
    static mut apdu: apdu;
    #[no_mangle]
    fn gpg_pw_get_retry_counter(who: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn gpg_pw_locked(which: uint8_t) -> libc::c_int;
    #[no_mangle]
    fn gpg_pw_reset_err_counter(which: uint8_t);
    #[no_mangle]
    fn gpg_pw_increment_err_counter(which: uint8_t);
    #[no_mangle]
    fn ac_check_status(ac_flag: uint8_t) -> libc::c_int;
    #[no_mangle]
    fn verify_pso_cds(pw: *const uint8_t, pw_len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn verify_other(pw: *const uint8_t, pw_len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn verify_user_0(access: uint8_t, pw: *const uint8_t,
                     buf_len: libc::c_int, pw_len_known: libc::c_int,
                     ks_pw1: *const uint8_t, saveks: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn verify_admin(pw: *const uint8_t, pw_len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn verify_admin_0(pw: *const uint8_t, buf_len: libc::c_int,
                      pw_len_known: libc::c_int, ks_pw3: *const uint8_t,
                      saveks: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ac_reset_pso_cds();
    #[no_mangle]
    fn ac_reset_other();
    #[no_mangle]
    fn ac_reset_admin();
    #[no_mangle]
    fn ac_fini();
    #[no_mangle]
    static mut data_objects_number_of_bytes: uint16_t;
    #[no_mangle]
    fn gpg_data_scan(p: *const uint8_t);
    #[no_mangle]
    fn gpg_do_get_data(tag: uint16_t, with_tag: libc::c_int);
    #[no_mangle]
    fn gpg_do_put_data(tag: uint16_t, data: *const uint8_t, len: libc::c_int);
    #[no_mangle]
    fn gpg_do_public_key(kk_byte: uint8_t);
    #[no_mangle]
    fn gpg_do_keygen(kk_byte: uint8_t);
    #[no_mangle]
    fn gpg_get_algo_attr(kk: kind_of_key) -> libc::c_int;
    #[no_mangle]
    fn gpg_get_algo_attr_key_size(kk: kind_of_key, s: size_of_key)
     -> libc::c_int;
    #[no_mangle]
    fn flash_init() -> *const uint8_t;
    #[no_mangle]
    fn flash_init_keys();
    #[no_mangle]
    fn flash_erase_binary(file_id: uint8_t) -> libc::c_int;
    #[no_mangle]
    fn flash_write_binary(file_id: uint8_t, data: *const uint8_t,
                          len: uint16_t, offset: uint16_t) -> libc::c_int;
    #[no_mangle]
    fn gpg_do_load_prvkey(kk: kind_of_key, who: libc::c_int,
                          keystring: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn gpg_do_chks_prvkey(kk: kind_of_key, who_old: libc::c_int,
                          old_ks: *const uint8_t, who_new: libc::c_int,
                          new_ks: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    static mut kd: [key_data; 3];
    #[no_mangle]
    fn rsa_sign(_: *const uint8_t, _: *mut uint8_t, _: libc::c_int,
                _: *mut key_data, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn rsa_decrypt(_: *const uint8_t, _: *mut uint8_t, _: libc::c_int,
                   _: *mut key_data, _: *mut libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn rsa_verify(_: *const uint8_t, _: libc::c_int, _: *const uint8_t,
                  _: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn ecdsa_sign_p256r1(hash: *const uint8_t, output: *mut uint8_t,
                         key_data: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn ecdh_decrypt_p256r1(input: *const uint8_t, output: *mut uint8_t,
                           key_data: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn ecdsa_sign_p256k1(hash: *const uint8_t, output: *mut uint8_t,
                         key_data: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn ecdh_decrypt_p256k1(input: *const uint8_t, output: *mut uint8_t,
                           key_data: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn eddsa_sign_25519(input: *const uint8_t, ilen: size_t,
                        output: *mut uint32_t, sk_a: *const uint8_t,
                        seed: *const uint8_t, pk: *const uint8_t)
     -> libc::c_int;
    #[no_mangle]
    fn ecdh_decrypt_curve25519(input: *const uint8_t, output: *mut uint8_t,
                               key_data: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn gpg_do_read_simple(_: uint8_t) -> *const uint8_t;
    #[no_mangle]
    fn gpg_do_write_simple(_: uint8_t, _: *const uint8_t, _: libc::c_int);
    #[no_mangle]
    fn gpg_increment_digital_signature_counter();
    #[no_mangle]
    static mut keystring_md_pw3: [uint8_t; 32];
    #[no_mangle]
    static mut admin_authorized: uint8_t;
    #[no_mangle]
    static openpgpcard_aid: [uint8_t; 14];
    #[no_mangle]
    fn led_blink(spec: libc::c_int);
    #[no_mangle]
    static mut vector: [handler; 16];
    #[no_mangle]
    fn sha256_start(ctx: *mut sha256_context);
    #[no_mangle]
    fn sha256_finish(ctx: *mut sha256_context, output: *mut libc::c_uchar);
    #[no_mangle]
    fn sha256_update(ctx: *mut sha256_context, input: *const libc::c_uchar,
                     ilen: libc::c_uint);
    /* 32-byte random bytes */
    #[no_mangle]
    fn random_bytes_get() -> *const uint8_t;
    #[no_mangle]
    fn random_bytes_free(p: *const uint8_t);
    /* 8-byte salt */
    #[no_mangle]
    fn random_get_salt(p: *mut uint8_t);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
/*
 * chopstx.h - Threads and only threads.
 *
 * Copyright (C) 2013, 2016, 2017, 2018  Flying Stone Technology
 * Author: NIIBE Yutaka <gniibe@fsij.org>
 *
 * This file is a part of Chopstx, a thread library for embedded.
 *
 * Chopstx is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Chopstx is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * As additional permission under GNU GPL version 3 section 7, you may
 * distribute non-source form of the Program without the copy of the
 * GNU GPL normally required by section 4, provided you inform the
 * receipents of GNU GPL by a written offer.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_qh {
    pub next: *mut chx_pq,
    pub prev: *mut chx_pq,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_spinlock {
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_mtx {
    pub q: chx_qh,
    pub lock: chx_spinlock,
    pub owner: *mut chx_thread,
    pub list: *mut chx_mtx,
}
/* nothing for uniprocessor.  */
pub type chopstx_mutex_t = chx_mtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_cond {
    pub q: chx_qh,
    pub lock: chx_spinlock,
}
pub type chopstx_cond_t = chx_cond;
pub type eventmask_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eventflag {
    pub flags: eventmask_t,
    pub mask: eventmask_t,
    pub mutex: chopstx_mutex_t,
    pub cond: chopstx_cond_t,
}
/*
 * Application layer <-> CCID layer data structure
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct apdu {
    pub seq: uint8_t,
    pub cmd_apdu_head: *mut uint8_t,
    pub cmd_apdu_data: *mut uint8_t,
    pub cmd_apdu_data_len: uint16_t,
    pub expected_res_size: uint16_t,
    pub sw: uint16_t,
    pub res_apdu_data_len: uint16_t,
    pub res_apdu_data: *mut uint8_t,
}
pub type kind_of_key = libc::c_uint;
pub const GPG_KEY_FOR_AUTHENTICATION: kind_of_key = 2;
pub const GPG_KEY_FOR_DECRYPTION: kind_of_key = 1;
pub const GPG_KEY_FOR_SIGNING: kind_of_key = 0;
pub type size_of_key = libc::c_uint;
pub const GPG_KEY_PRIVATE: size_of_key = 2;
/* PUBKEY + PRVKEY rounded to 2^N */
pub const GPG_KEY_PUBLIC: size_of_key = 1;
pub const GPG_KEY_STORAGE: size_of_key = 0;
/* RSA-2048 (p and q) */
/* Maximum is the case for RSA 4096-bit.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_data {
    pub pubkey: *const uint8_t,
    pub data: [uint8_t; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_context {
    pub total: [uint32_t; 2],
    pub state: [uint32_t; 8],
    pub wbuf: [uint32_t; 16],
}
pub type handler = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
    pub command: uint8_t,
    pub cmd_handler: Option<unsafe extern "C" fn() -> ()>,
}
#[inline]
unsafe extern "C" fn unique_device_id() -> *const uint8_t {
    /* STM32F103 has 96-bit unique device identifier */
    let mut addr: *const uint8_t =
        0x1ffff7e8 as libc::c_int as *const uint8_t;
    return addr;
}
#[inline]
unsafe extern "C" fn flash_erase_page(mut addr: uintptr_t) -> libc::c_int {
    let mut func: Option<unsafe extern "C" fn(_: uintptr_t) -> libc::c_int> =
        ::std::mem::transmute::<handler,
                                Option<unsafe extern "C" fn(_: uintptr_t)
                                           ->
                                               libc::c_int>>(vector[5 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(addr);
}
/*
 * openpgp.c -- OpenPGP card protocol support
 *
 * Copyright (C) 2010, 2011, 2012, 2013, 2014, 2015, 2016
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
static mut openpgp_comm: *mut eventflag =
    0 as *const eventflag as *mut eventflag;
/* For key import */
static mut challenge: *const uint8_t = 0 as *const uint8_t;
/* Random bytes */
static mut select_file_TOP_result: [uint8_t; 20] =
    [0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x3f as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x38 as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0x44 as libc::c_int as uint8_t,
     0x44 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t];
#[no_mangle]
pub unsafe extern "C" fn set_res_sw(mut sw1: uint8_t, mut sw2: uint8_t) {
    apdu.sw =
        ((sw1 as libc::c_int) << 8 as libc::c_int | sw2 as libc::c_int) as
            uint16_t;
}
static mut file_selection: uint8_t = 0;
unsafe extern "C" fn gpg_init() {
    let mut flash_data_start: *const uint8_t = 0 as *const uint8_t;
    file_selection = 0 as libc::c_int as uint8_t;
    flash_data_start = flash_init();
    gpg_data_scan(flash_data_start);
    flash_init_keys();
}
unsafe extern "C" fn gpg_fini() { ac_fini(); }
unsafe extern "C" fn cmd_verify() {
    let mut len: libc::c_int = 0;
    let mut p1: uint8_t =
        *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize);
    let mut p2: uint8_t =
        *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize);
    let mut r: libc::c_int = 0;
    let mut pw: *const uint8_t = 0 as *const uint8_t;
    len = apdu.cmd_apdu_data_len as libc::c_int;
    pw = apdu.cmd_apdu_data;
    if len == 0 as libc::c_int {
        if p1 as libc::c_int == 0 as libc::c_int {
            /* This is to examine status.  */
            if p2 as libc::c_int == 0x81 as libc::c_int {
                r = ac_check_status(0x1 as libc::c_int as uint8_t)
            } else if p2 as libc::c_int == 0x82 as libc::c_int {
                r = ac_check_status(0x2 as libc::c_int as uint8_t)
            } else {
                r = ac_check_status(0x4 as libc::c_int as uint8_t)
            } /* If authentication done already, return success.  */
            if r != 0 {
                set_res_sw(0x90 as libc::c_int as uint8_t,
                           0 as libc::c_int as uint8_t);
            } else {
                /* If not, return retry counter, encoded.  */
                r = gpg_pw_get_retry_counter(p2 as libc::c_int);
                set_res_sw(0x63 as libc::c_int as uint8_t,
                           (0xc0 as libc::c_int | r & 0xf as libc::c_int) as
                               uint8_t);
            }
        } else if p1 as libc::c_int == 0xff as libc::c_int {
            /* Reset the status.  */
            if p2 as libc::c_int == 0x81 as libc::c_int {
                ac_reset_pso_cds();
            } else if p2 as libc::c_int == 0x82 as libc::c_int {
                ac_reset_other();
            } else { ac_reset_admin(); }
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        } else {
            set_res_sw(0x6b as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        }
        return
    }
    /* This is real authentication.  */
    if p2 as libc::c_int == 0x81 as libc::c_int {
        r = verify_pso_cds(pw, len)
    } else if p2 as libc::c_int == 0x82 as libc::c_int {
        r = verify_other(pw, len)
    } else {
        r = verify_admin(pw, len)
    } /* 0: change (old+new), 1: exchange (new) */
    if r < 0 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
    } else if r == 0 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x83 as libc::c_int as uint8_t);
    } else {
        set_res_sw(0x90 as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_change_keystring(mut who_old: libc::c_int,
                                              mut old_ks: *const uint8_t,
                                              mut who_new: libc::c_int,
                                              mut new_ks: *const uint8_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut prv_keys_exist: libc::c_int = 0 as libc::c_int;
    r = gpg_do_load_prvkey(GPG_KEY_FOR_SIGNING, who_old, old_ks);
    if r < 0 as libc::c_int { return r }
    if r > 0 as libc::c_int { prv_keys_exist += 1 }
    r =
        gpg_do_chks_prvkey(GPG_KEY_FOR_SIGNING, who_old, old_ks, who_new,
                           new_ks);
    if r < 0 as libc::c_int { return -(2 as libc::c_int) }
    r = gpg_do_load_prvkey(GPG_KEY_FOR_DECRYPTION, who_old, old_ks);
    if r < 0 as libc::c_int { return r }
    if r > 0 as libc::c_int { prv_keys_exist += 1 }
    r =
        gpg_do_chks_prvkey(GPG_KEY_FOR_DECRYPTION, who_old, old_ks, who_new,
                           new_ks);
    if r < 0 as libc::c_int { return -(2 as libc::c_int) }
    r = gpg_do_load_prvkey(GPG_KEY_FOR_AUTHENTICATION, who_old, old_ks);
    if r < 0 as libc::c_int { return r }
    if r > 0 as libc::c_int { prv_keys_exist += 1 }
    r =
        gpg_do_chks_prvkey(GPG_KEY_FOR_AUTHENTICATION, who_old, old_ks,
                           who_new, new_ks);
    if r < 0 as libc::c_int { return -(2 as libc::c_int) }
    if prv_keys_exist != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn cmd_change_password() {
    let mut old_ks: [uint8_t; 32] = [0; 32];
    let mut new_ks0: [uint8_t; 41] = [0; 41];
    let mut new_salt: *mut uint8_t =
        new_ks0.as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut newsalt_len: libc::c_int = 8 as libc::c_int;
    let mut new_ks: *mut uint8_t =
        new_ks0.as_mut_ptr().offset((1 as libc::c_int + 8 as libc::c_int) as
                                        isize);
    let mut p1: uint8_t =
        *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize);
    let mut p2: uint8_t =
        *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize);
    let mut len: libc::c_int = 0;
    let mut pw: *mut uint8_t = 0 as *mut uint8_t;
    let mut newpw: *mut uint8_t = 0 as *mut uint8_t;
    let mut pw_len: libc::c_int = 0;
    let mut newpw_len: libc::c_int = 0;
    let mut who: libc::c_int = p2 as libc::c_int - 0x80 as libc::c_int;
    let mut who_old: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut pw3_null: libc::c_int = 0 as libc::c_int;
    let mut salt: *const uint8_t = 0 as *const uint8_t;
    let mut salt_len: libc::c_int = 0;
    let mut ks_pw3: *const uint8_t = 0 as *const uint8_t;
    len = apdu.cmd_apdu_data_len as libc::c_int;
    pw = apdu.cmd_apdu_data;
    if p1 as libc::c_int != 0 as libc::c_int {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x81 as libc::c_int as uint8_t);
        return
    }
    if who == 1 as libc::c_int {
        /* PW1 */
        let mut ks_pw1: *const uint8_t =
            gpg_do_read_simple(0x11 as libc::c_int as uint8_t);
        who_old = who;
        pw_len =
            verify_user_0(0x1 as libc::c_int as uint8_t, pw, len,
                          -(1 as libc::c_int), ks_pw1, 0 as libc::c_int);
        if ks_pw1.is_null() {
            salt = 0 as *const uint8_t;
            salt_len = 0 as libc::c_int
        } else {
            salt = ks_pw1.offset(1 as libc::c_int as isize);
            salt_len = 8 as libc::c_int
        }
        if pw_len < 0 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        } else {
            if pw_len == 0 as libc::c_int {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x83 as libc::c_int as uint8_t);
                return
            } else {
                newpw = pw.offset(pw_len as isize);
                newpw_len = len - pw_len;
                ks_pw3 = gpg_do_read_simple(0x13 as libc::c_int as uint8_t);
                /* Check length of password for admin-less mode.  */
                if ks_pw3.is_null() && newpw_len < 8 as libc::c_int {
                    set_res_sw(0x69 as libc::c_int as uint8_t,
                               0x85 as libc::c_int as uint8_t);
                    return
                }
            }
        }
    } else {
        /* PW3 (0x83) */
        ks_pw3 = gpg_do_read_simple(0x13 as libc::c_int as uint8_t);
        pw_len =
            verify_admin_0(pw, len, -(1 as libc::c_int), ks_pw3,
                           0 as libc::c_int);
        if ks_pw3.is_null() {
            salt = 0 as *const uint8_t;
            salt_len = 0 as libc::c_int
        } else {
            salt = ks_pw3.offset(1 as libc::c_int as isize);
            salt_len = 8 as libc::c_int
        }
        if pw_len < 0 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        } else {
            if pw_len == 0 as libc::c_int {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x83 as libc::c_int as uint8_t);
                return
            } else {
                newpw = pw.offset(pw_len as isize);
                newpw_len = len - pw_len;
                if newpw_len == 0 as libc::c_int &&
                       admin_authorized as libc::c_int == 3 as libc::c_int {
                    newpw_len =
                        strlen(b"12345678\x00" as *const u8 as
                                   *const libc::c_char) as libc::c_int;
                    memcpy(newpw as *mut libc::c_void,
                           b"12345678\x00" as *const u8 as *const libc::c_char
                               as *const libc::c_void,
                           newpw_len as libc::c_ulong);
                    newsalt_len = 0 as libc::c_int;
                    pw3_null = 1 as libc::c_int
                }
                who_old = admin_authorized as libc::c_int
            }
        }
    }
    if newsalt_len != 0 as libc::c_int { random_get_salt(new_salt); }
    s2k(salt, salt_len as size_t, pw, pw_len as size_t, old_ks.as_mut_ptr());
    s2k(new_salt, newsalt_len as size_t, newpw, newpw_len as size_t, new_ks);
    new_ks0[0 as libc::c_int as usize] = newpw_len as uint8_t;
    r = gpg_change_keystring(who_old, old_ks.as_mut_ptr(), who, new_ks);
    if r <= -(2 as libc::c_int) {
        set_res_sw(0x65 as libc::c_int as uint8_t,
                   0x81 as libc::c_int as uint8_t);
    } else if r < 0 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
    } else if r == 0 as libc::c_int && who == 1 as libc::c_int {
        /* no prvkey */
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x85 as libc::c_int as uint8_t);
    } else if r > 0 as libc::c_int && who == 1 as libc::c_int {
        gpg_do_write_simple(0x11 as libc::c_int as uint8_t,
                            new_ks0.as_mut_ptr(),
                            1 as libc::c_int + 8 as libc::c_int);
        ac_reset_pso_cds();
        ac_reset_other();
        if admin_authorized as libc::c_int == 1 as libc::c_int {
            ac_reset_admin();
        }
        set_res_sw(0x90 as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
    } else if r > 0 as libc::c_int && who == 3 as libc::c_int {
        if pw3_null != 0 {
            gpg_do_write_simple(0x13 as libc::c_int as uint8_t,
                                0 as *const uint8_t, 0 as libc::c_int);
        } else {
            gpg_do_write_simple(0x13 as libc::c_int as uint8_t,
                                new_ks0.as_mut_ptr(),
                                1 as libc::c_int + 8 as libc::c_int);
        }
        ac_reset_admin();
        set_res_sw(0x90 as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
    } else {
        /* r == 0 && who == BY_ADMIN */	/* no prvkey */
        if pw3_null != 0 {
            gpg_do_write_simple(0x13 as libc::c_int as uint8_t,
                                0 as *const uint8_t, 0 as libc::c_int);
        } else {
            new_ks0[0 as libc::c_int as usize] =
                (new_ks0[0 as libc::c_int as usize] as libc::c_int |
                     0x80 as libc::c_int) as uint8_t;
            gpg_do_write_simple(0x13 as libc::c_int as uint8_t,
                                new_ks0.as_mut_ptr(),
                                1 as libc::c_int + 8 as libc::c_int +
                                    32 as libc::c_int);
        }
        ac_reset_admin();
        set_res_sw(0x90 as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn s2k(mut salt: *const libc::c_uchar, mut slen: size_t,
                             mut input: *const libc::c_uchar,
                             mut ilen: size_t,
                             mut output: *mut libc::c_uchar) {
    let mut ctx: sha256_context =
        sha256_context{total: [0; 2], state: [0; 8], wbuf: [0; 16],};
    let mut count: size_t = 192 as libc::c_int as size_t;
    let mut unique: *const uint8_t = unique_device_id();
    sha256_start(&mut ctx);
    sha256_update(&mut ctx, unique, 12 as libc::c_int as libc::c_uint);
    while count > slen.wrapping_add(ilen) {
        if slen != 0 { sha256_update(&mut ctx, salt, slen as libc::c_uint); }
        sha256_update(&mut ctx, input, ilen as libc::c_uint);
        count =
            (count as libc::c_ulong).wrapping_sub(slen.wrapping_add(ilen)) as
                size_t as size_t
    }
    if count <= slen {
        sha256_update(&mut ctx, salt, count as libc::c_uint);
    } else {
        if slen != 0 {
            sha256_update(&mut ctx, salt, slen as libc::c_uint);
            count =
                (count as libc::c_ulong).wrapping_sub(slen) as size_t as
                    size_t
        }
        sha256_update(&mut ctx, input, count as libc::c_uint);
    }
    sha256_finish(&mut ctx, output);
}
unsafe extern "C" fn cmd_reset_user_password() {
    let mut p1: uint8_t =
        *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize);
    let mut len: libc::c_int = 0;
    let mut pw: *const uint8_t = 0 as *const uint8_t;
    let mut newpw: *const uint8_t = 0 as *const uint8_t;
    let mut pw_len: libc::c_int = 0;
    let mut newpw_len: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut new_ks0: [uint8_t; 41] = [0; 41];
    let mut new_ks: *mut uint8_t =
        new_ks0.as_mut_ptr().offset((1 as libc::c_int + 8 as libc::c_int) as
                                        isize);
    let mut new_salt: *mut uint8_t =
        new_ks0.as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut salt: *const uint8_t = 0 as *const uint8_t;
    let mut salt_len: libc::c_int = 0;
    len = apdu.cmd_apdu_data_len as libc::c_int;
    pw = apdu.cmd_apdu_data;
    if p1 as libc::c_int == 0 as libc::c_int {
        /* by User with Reseting Code */
        let mut ks_rc: *const uint8_t =
            gpg_do_read_simple(0x12 as libc::c_int as uint8_t);
        let mut old_ks: [uint8_t; 32] = [0; 32];
        if gpg_pw_locked(1 as libc::c_int as uint8_t) != 0 {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x83 as libc::c_int as uint8_t);
            return
        }
        if ks_rc.is_null() {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        }
        pw_len =
            *ks_rc.offset(0 as libc::c_int as isize) as libc::c_int &
                0x7f as libc::c_int;
        salt = ks_rc.offset(1 as libc::c_int as isize);
        salt_len = 8 as libc::c_int;
        newpw = pw.offset(pw_len as isize);
        newpw_len = len - pw_len;
        random_get_salt(new_salt);
        s2k(salt, salt_len as size_t, pw, pw_len as size_t,
            old_ks.as_mut_ptr());
        s2k(new_salt, 8 as libc::c_int as size_t, newpw, newpw_len as size_t,
            new_ks);
        new_ks0[0 as libc::c_int as usize] = newpw_len as uint8_t;
        r =
            gpg_change_keystring(2 as libc::c_int, old_ks.as_mut_ptr(),
                                 1 as libc::c_int, new_ks);
        if r <= -(2 as libc::c_int) {
            set_res_sw(0x65 as libc::c_int as uint8_t,
                       0x81 as libc::c_int as uint8_t);
        } else if r < 0 as libc::c_int {
            gpg_pw_increment_err_counter(1 as libc::c_int as uint8_t);
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
        } else if r == 0 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x85 as libc::c_int as uint8_t);
        } else {
            gpg_do_write_simple(0x11 as libc::c_int as uint8_t,
                                new_ks0.as_mut_ptr(),
                                1 as libc::c_int + 8 as libc::c_int);
            ac_reset_pso_cds();
            ac_reset_other();
            if admin_authorized as libc::c_int == 1 as libc::c_int {
                ac_reset_admin();
            }
            gpg_pw_reset_err_counter(1 as libc::c_int as uint8_t);
            gpg_pw_reset_err_counter(0 as libc::c_int as uint8_t);
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        }
    } else {
        /* by Admin (p1 == 0x02) */
        let mut old_ks_0: *const uint8_t = keystring_md_pw3.as_mut_ptr();
        if ac_check_status(0x4 as libc::c_int as uint8_t) == 0 {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        }
        newpw_len = len;
        newpw = pw;
        random_get_salt(new_salt);
        s2k(new_salt, 8 as libc::c_int as size_t, newpw, newpw_len as size_t,
            new_ks);
        new_ks0[0 as libc::c_int as usize] = newpw_len as uint8_t;
        r =
            gpg_change_keystring(admin_authorized as libc::c_int, old_ks_0,
                                 1 as libc::c_int, new_ks);
        if r <= -(2 as libc::c_int) {
            set_res_sw(0x65 as libc::c_int as uint8_t,
                       0x81 as libc::c_int as uint8_t);
        } else if r < 0 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
        } else if r == 0 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x85 as libc::c_int as uint8_t);
        } else {
            gpg_do_write_simple(0x11 as libc::c_int as uint8_t,
                                new_ks0.as_mut_ptr(),
                                1 as libc::c_int + 8 as libc::c_int);
            ac_reset_pso_cds();
            ac_reset_other();
            if admin_authorized as libc::c_int == 1 as libc::c_int {
                ac_reset_admin();
            }
            gpg_pw_reset_err_counter(0 as libc::c_int as uint8_t);
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        }
    };
}
unsafe extern "C" fn cmd_put_data() {
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut tag: uint16_t = 0;
    let mut len: libc::c_int = 0;
    if file_selection as libc::c_int != 1 as libc::c_int {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x88 as libc::c_int as uint8_t);
    }
    tag =
        ((*apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as
              libc::c_int) << 8 as libc::c_int |
             *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
                 libc::c_int) as uint16_t;
    len = apdu.cmd_apdu_data_len as libc::c_int;
    data = apdu.cmd_apdu_data;
    gpg_do_put_data(tag, data, len);
}
unsafe extern "C" fn cmd_pgp_gakp() {
    if *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as libc::c_int ==
           0x81 as libc::c_int {
        /* Get public key */
        gpg_do_public_key(*apdu.cmd_apdu_data.offset(0 as libc::c_int as
                                                         isize)); /* Get AID... */
    } else {
        if ac_check_status(0x4 as libc::c_int as uint8_t) == 0 {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
        }
        gpg_do_keygen(*apdu.cmd_apdu_data.offset(0 as libc::c_int as isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_get_firmware_update_key(mut keyno: uint8_t)
 -> *const uint8_t {
    extern "C" {
        #[no_mangle]
        static mut _updatekey_store: uint8_t;
    }
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p =
        (&mut _updatekey_store as
             *mut uint8_t).offset((keyno as libc::c_int * 256 as libc::c_int)
                                      as isize);
    return p;
}
unsafe extern "C" fn cmd_read_binary() {
    let mut is_short_EF: libc::c_int =
        (*apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as libc::c_int
             & 0x80 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    let mut file_id: uint8_t = 0;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut offset: uint16_t = 0;
    if is_short_EF != 0 {
        file_id =
            (*apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as
                 libc::c_int & 0x1f as libc::c_int) as uint8_t
    } else {
        file_id =
            (file_selection as libc::c_int - 4 as libc::c_int +
                 0 as libc::c_int) as uint8_t
    }
    if 0 as libc::c_int == 0 && file_id as libc::c_int == 5 as libc::c_int ||
           file_id as libc::c_int > 5 as libc::c_int {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
        return
    }
    if is_short_EF != 0 {
        file_selection =
            (file_id as libc::c_int - 0 as libc::c_int + 4 as libc::c_int) as
                uint8_t;
        offset =
            *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as uint16_t
    } else {
        offset =
            ((*apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as
                  libc::c_int) << 8 as libc::c_int |
                 *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
                     libc::c_int) as uint16_t
    }
    if file_id as libc::c_int == 0 as libc::c_int {
        if offset as libc::c_int != 0 as libc::c_int {
            set_res_sw(0x6b as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        } else {
            gpg_do_get_data(0x4f as libc::c_int as uint16_t,
                            1 as libc::c_int);
            *apdu.res_apdu_data.offset(0 as libc::c_int as isize) =
                0x5a as libc::c_int as uint8_t
            /* ... and overwrite the first byte of data. */
        }
        return
    }
    if file_id as libc::c_int >= 1 as libc::c_int &&
           file_id as libc::c_int <= 4 as libc::c_int {
        if offset as libc::c_int != 0 as libc::c_int {
            set_res_sw(0x65 as libc::c_int as uint8_t,
                       0x81 as libc::c_int as uint8_t);
        } else {
            p =
                gpg_get_firmware_update_key((file_id as libc::c_int -
                                                 1 as libc::c_int) as
                                                uint8_t);
            apdu.res_apdu_data_len = 256 as libc::c_int as uint16_t;
            memcpy(apdu.res_apdu_data as *mut libc::c_void,
                   p as *const libc::c_void,
                   256 as libc::c_int as libc::c_ulong);
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        }
    };
}
unsafe extern "C" fn cmd_select_file() {
    if *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as libc::c_int ==
           4 as libc::c_int {
        /* Selection by DF name */
        /* name = D2 76 00 01 24 01 */
        if apdu.cmd_apdu_data_len as libc::c_int != 6 as libc::c_int ||
               memcmp(openpgpcard_aid.as_ptr() as *const libc::c_void,
                      apdu.cmd_apdu_data as *const libc::c_void,
                      6 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
            set_res_sw(0x6a as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        }
        file_selection = 1 as libc::c_int as uint8_t;
        if *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
               libc::c_int & 0xc as libc::c_int == 0xc as libc::c_int {
            /* No FCI */
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t); /* AID */
        } else {
            gpg_do_get_data(0x4f as libc::c_int as uint16_t,
                            1 as libc::c_int); /* overwrite: DF name */
            memmove(apdu.res_apdu_data.offset(2 as libc::c_int as isize) as
                        *mut libc::c_void,
                    apdu.res_apdu_data as *const libc::c_void,
                    apdu.res_apdu_data_len as libc::c_ulong);
            *apdu.res_apdu_data.offset(0 as libc::c_int as isize) =
                0x6f as libc::c_int as uint8_t;
            *apdu.res_apdu_data.offset(1 as libc::c_int as isize) =
                0x12 as libc::c_int as uint8_t;
            *apdu.res_apdu_data.offset(2 as libc::c_int as isize) =
                0x84 as libc::c_int as uint8_t;
            apdu.res_apdu_data_len =
                (apdu.res_apdu_data_len as libc::c_int + 2 as libc::c_int) as
                    uint16_t;
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        }
    } else if apdu.cmd_apdu_data_len as libc::c_int == 2 as libc::c_int &&
                  *apdu.cmd_apdu_data.offset(0 as libc::c_int as isize) as
                      libc::c_int == 0x2f as libc::c_int &&
                  *apdu.cmd_apdu_data.offset(1 as libc::c_int as isize) as
                      libc::c_int == 0x2 as libc::c_int {
        /*
       * MF.EF-GDO -- Serial number of the card and name of the owner
       */
        set_res_sw(0x90 as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
        file_selection = 4 as libc::c_int as uint8_t
    } else if apdu.cmd_apdu_data_len as libc::c_int == 2 as libc::c_int &&
                  *apdu.cmd_apdu_data.offset(0 as libc::c_int as isize) as
                      libc::c_int == 0x3f as libc::c_int &&
                  *apdu.cmd_apdu_data.offset(1 as libc::c_int as isize) as
                      libc::c_int == 0 as libc::c_int {
        if *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
               libc::c_int == 0xc as libc::c_int {
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        } else {
            let mut len: libc::c_int =
                ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong as
                    libc::c_int;
            apdu.res_apdu_data_len = len as uint16_t;
            memcpy(apdu.res_apdu_data as *mut libc::c_void,
                   select_file_TOP_result.as_ptr() as *const libc::c_void,
                   len as libc::c_ulong);
            *apdu.res_apdu_data.offset(2 as libc::c_int as isize) =
                (data_objects_number_of_bytes as libc::c_int &
                     0xff as libc::c_int) as uint8_t;
            *apdu.res_apdu_data.offset(3 as libc::c_int as isize) =
                (data_objects_number_of_bytes as libc::c_int >>
                     8 as libc::c_int) as uint8_t;
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        }
        file_selection = 2 as libc::c_int as uint8_t;
        ac_fini();
        /* Reset authentication */
    } else {
        file_selection = 0 as libc::c_int as uint8_t;
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
    };
}
unsafe extern "C" fn cmd_get_data() {
    let mut tag: uint16_t =
        ((*apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as
              libc::c_int) << 8 as libc::c_int |
             *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
                 libc::c_int) as uint16_t;
    if file_selection as libc::c_int != 1 as libc::c_int {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x88 as libc::c_int as uint8_t);
    }
    gpg_do_get_data(tag, 0 as libc::c_int);
}
unsafe extern "C" fn cmd_pso() {
    let mut len: libc::c_int = apdu.cmd_apdu_data_len as libc::c_int;
    let mut r: libc::c_int = -(1 as libc::c_int);
    let mut attr: libc::c_int = 0;
    let mut pubkey_len: libc::c_int = 0;
    let mut result_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cs: libc::c_int = 0;
    if *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as libc::c_int ==
           0x9e as libc::c_int &&
           *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
               libc::c_int == 0x9a as libc::c_int {
        attr = gpg_get_algo_attr(GPG_KEY_FOR_SIGNING);
        pubkey_len =
            gpg_get_algo_attr_key_size(GPG_KEY_FOR_SIGNING, GPG_KEY_PUBLIC);
        if ac_check_status(0x1 as libc::c_int as uint8_t) == 0 {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        }
        if attr == 255 as libc::c_int || attr == 0 as libc::c_int {
            /* Check size of digestInfo */
            if len != 34 as libc::c_int && len != 35 as libc::c_int &&
                   len != 47 as libc::c_int && len != 51 as libc::c_int &&
                   len != 67 as libc::c_int && len != 83 as libc::c_int {
                /* SHA512 */
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x85 as libc::c_int as uint8_t);
                return
            }
            result_len = pubkey_len as libc::c_uint;
            r =
                rsa_sign(apdu.cmd_apdu_data, apdu.res_apdu_data, len,
                         &mut *kd.as_mut_ptr().offset(GPG_KEY_FOR_SIGNING as
                                                          libc::c_int as
                                                          isize), pubkey_len)
        } else if attr == 1 as libc::c_int || attr == 2 as libc::c_int {
            /* ECDSA with p256r1/p256k1 for signature */
            if len != 32 as libc::c_int {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x85 as libc::c_int as uint8_t);
                return
            }
            cs = chopstx_setcancelstate(0 as libc::c_int);
            result_len = 64 as libc::c_int as libc::c_uint;
            if attr == 1 as libc::c_int {
                r =
                    ecdsa_sign_p256r1(apdu.cmd_apdu_data, apdu.res_apdu_data,
                                      kd[GPG_KEY_FOR_SIGNING as libc::c_int as
                                             usize].data.as_mut_ptr())
            } else {
                /* ALGO_SECP256K1 */
                r =
                    ecdsa_sign_p256k1(apdu.cmd_apdu_data, apdu.res_apdu_data,
                                      kd[GPG_KEY_FOR_SIGNING as libc::c_int as
                                             usize].data.as_mut_ptr())
            } /* Require 4-byte alignment. */
            chopstx_setcancelstate(cs);
        } else if attr == 3 as libc::c_int {
            let mut output: [uint32_t; 16] = [0; 16];
            if len > 256 as libc::c_int {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x85 as libc::c_int as uint8_t);
                return
            }
            cs = chopstx_setcancelstate(0 as libc::c_int);
            result_len = 64 as libc::c_int as libc::c_uint;
            r =
                eddsa_sign_25519(apdu.cmd_apdu_data, len as size_t,
                                 output.as_mut_ptr(),
                                 kd[GPG_KEY_FOR_SIGNING as libc::c_int as
                                        usize].data.as_mut_ptr(),
                                 kd[GPG_KEY_FOR_SIGNING as libc::c_int as
                                        usize].data.as_mut_ptr().offset(32 as
                                                                            libc::c_int
                                                                            as
                                                                            isize),
                                 kd[GPG_KEY_FOR_SIGNING as libc::c_int as
                                        usize].pubkey);
            chopstx_setcancelstate(cs);
            memcpy(apdu.res_apdu_data as *mut libc::c_void,
                   output.as_mut_ptr() as *const libc::c_void,
                   64 as libc::c_int as libc::c_ulong);
        } else {
            set_res_sw(0x6a as libc::c_int as uint8_t,
                       0x81 as libc::c_int as uint8_t);
            return
        }
        if r == 0 as libc::c_int {
            apdu.res_apdu_data_len = result_len as uint16_t;
            gpg_increment_digital_signature_counter();
        } else {
            /* Failure */
            ac_reset_pso_cds();
        }
    } else if *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as
                  libc::c_int == 0x80 as libc::c_int &&
                  *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
                      libc::c_int == 0x86 as libc::c_int {
        attr = gpg_get_algo_attr(GPG_KEY_FOR_DECRYPTION);
        pubkey_len =
            gpg_get_algo_attr_key_size(GPG_KEY_FOR_DECRYPTION,
                                       GPG_KEY_PUBLIC);
        if ac_check_status(0x2 as libc::c_int as uint8_t) == 0 {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        }
        if attr == 255 as libc::c_int || attr == 0 as libc::c_int {
            /* Skip padding 0x00 */
            len -= 1;
            if len != pubkey_len {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x85 as libc::c_int as uint8_t);
                return
            }
            r =
                rsa_decrypt(apdu.cmd_apdu_data.offset(1 as libc::c_int as
                                                          isize),
                            apdu.res_apdu_data, len,
                            &mut *kd.as_mut_ptr().offset(GPG_KEY_FOR_DECRYPTION
                                                             as libc::c_int as
                                                             isize),
                            &mut result_len)
        } else if attr == 1 as libc::c_int || attr == 2 as libc::c_int {
            let mut header: libc::c_int = 7 as libc::c_int;
            /* Format is in big endian MPI: 04 || x || y */
            if len != 65 as libc::c_int + 7 as libc::c_int ||
                   *apdu.cmd_apdu_data.offset(header as isize) as libc::c_int
                       != 0x4 as libc::c_int {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x85 as libc::c_int as uint8_t);
                return
            }
            cs = chopstx_setcancelstate(0 as libc::c_int);
            result_len = 65 as libc::c_int as libc::c_uint;
            if attr == 1 as libc::c_int {
                r =
                    ecdh_decrypt_p256r1(apdu.cmd_apdu_data.offset(header as
                                                                      isize),
                                        apdu.res_apdu_data,
                                        kd[GPG_KEY_FOR_DECRYPTION as
                                               libc::c_int as
                                               usize].data.as_mut_ptr())
            } else {
                r =
                    ecdh_decrypt_p256k1(apdu.cmd_apdu_data.offset(header as
                                                                      isize),
                                        apdu.res_apdu_data,
                                        kd[GPG_KEY_FOR_DECRYPTION as
                                               libc::c_int as
                                               usize].data.as_mut_ptr())
            }
            chopstx_setcancelstate(cs);
        } else if attr == 4 as libc::c_int {
            let mut header_0: libc::c_int = 7 as libc::c_int;
            if len != 32 as libc::c_int + 7 as libc::c_int {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x85 as libc::c_int as uint8_t);
                return
            }
            cs = chopstx_setcancelstate(0 as libc::c_int);
            result_len = 32 as libc::c_int as libc::c_uint;
            r =
                ecdh_decrypt_curve25519(apdu.cmd_apdu_data.offset(header_0 as
                                                                      isize),
                                        apdu.res_apdu_data,
                                        kd[GPG_KEY_FOR_DECRYPTION as
                                               libc::c_int as
                                               usize].data.as_mut_ptr());
            chopstx_setcancelstate(cs);
        } else {
            set_res_sw(0x6a as libc::c_int as uint8_t,
                       0x81 as libc::c_int as uint8_t);
            return
        }
        if r == 0 as libc::c_int {
            apdu.res_apdu_data_len = result_len as uint16_t
        }
    }
    if r < 0 as libc::c_int {
        set_res_sw(0x6f as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
    };
}
/* 40% */
unsafe extern "C" fn cmd_internal_authenticate() {
    let mut attr: libc::c_int =
        gpg_get_algo_attr(GPG_KEY_FOR_AUTHENTICATION); /* Require 4-byte alignment. */
    let mut pubkey_len: libc::c_int =
        gpg_get_algo_attr_key_size(GPG_KEY_FOR_AUTHENTICATION,
                                   GPG_KEY_PUBLIC);
    let mut len: libc::c_int = apdu.cmd_apdu_data_len as libc::c_int;
    let mut r: libc::c_int = -(1 as libc::c_int);
    let mut result_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cs: libc::c_int = 0;
    if *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize) as libc::c_int !=
           0 as libc::c_int ||
           *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize) as
               libc::c_int != 0 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x85 as libc::c_int as uint8_t);
        return
    }
    if ac_check_status(0x2 as libc::c_int as uint8_t) == 0 {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
        return
    }
    if attr == 255 as libc::c_int || attr == 0 as libc::c_int {
        if len > 102 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x85 as libc::c_int as uint8_t);
            return
        }
        result_len = pubkey_len as libc::c_uint;
        r =
            rsa_sign(apdu.cmd_apdu_data, apdu.res_apdu_data, len,
                     &mut *kd.as_mut_ptr().offset(GPG_KEY_FOR_AUTHENTICATION
                                                      as libc::c_int as
                                                      isize), pubkey_len)
    } else if attr == 1 as libc::c_int {
        if len != 32 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x85 as libc::c_int as uint8_t);
            return
        }
        cs = chopstx_setcancelstate(0 as libc::c_int);
        result_len = 64 as libc::c_int as libc::c_uint;
        r =
            ecdsa_sign_p256r1(apdu.cmd_apdu_data, apdu.res_apdu_data,
                              kd[GPG_KEY_FOR_AUTHENTICATION as libc::c_int as
                                     usize].data.as_mut_ptr());
        chopstx_setcancelstate(cs);
    } else if attr == 2 as libc::c_int {
        if len != 32 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x85 as libc::c_int as uint8_t);
            return
        }
        cs = chopstx_setcancelstate(0 as libc::c_int);
        result_len = 64 as libc::c_int as libc::c_uint;
        r =
            ecdsa_sign_p256k1(apdu.cmd_apdu_data, apdu.res_apdu_data,
                              kd[GPG_KEY_FOR_AUTHENTICATION as libc::c_int as
                                     usize].data.as_mut_ptr());
        chopstx_setcancelstate(cs);
    } else if attr == 3 as libc::c_int {
        let mut output: [uint32_t; 16] = [0; 16];
        if len > 256 as libc::c_int {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x85 as libc::c_int as uint8_t);
            return
        }
        cs = chopstx_setcancelstate(0 as libc::c_int);
        result_len = 64 as libc::c_int as libc::c_uint;
        r =
            eddsa_sign_25519(apdu.cmd_apdu_data, len as size_t,
                             output.as_mut_ptr(),
                             kd[GPG_KEY_FOR_AUTHENTICATION as libc::c_int as
                                    usize].data.as_mut_ptr(),
                             kd[GPG_KEY_FOR_AUTHENTICATION as libc::c_int as
                                    usize].data.as_mut_ptr().offset(32 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                             kd[GPG_KEY_FOR_AUTHENTICATION as libc::c_int as
                                    usize].pubkey);
        chopstx_setcancelstate(cs);
        memcpy(apdu.res_apdu_data as *mut libc::c_void,
               output.as_mut_ptr() as *const libc::c_void,
               64 as libc::c_int as libc::c_ulong);
    }
    if r == 0 as libc::c_int {
        apdu.res_apdu_data_len = result_len as uint16_t
    } else {
        set_res_sw(0x6f as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
    };
}
unsafe extern "C" fn modify_binary(mut op: uint8_t, mut p1: uint8_t,
                                   mut p2: uint8_t, mut len: libc::c_int) {
    let mut file_id: uint8_t = 0;
    let mut offset: uint16_t = 0;
    let mut is_short_EF: libc::c_int =
        (p1 as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int) as
            libc::c_int;
    let mut r: libc::c_int = 0;
    if ac_check_status(0x4 as libc::c_int as uint8_t) == 0 {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
        return
    }
    if is_short_EF != 0 {
        file_id = (p1 as libc::c_int & 0x1f as libc::c_int) as uint8_t
    } else {
        file_id =
            (file_selection as libc::c_int - 4 as libc::c_int +
                 0 as libc::c_int) as uint8_t
    }
    if 0 as libc::c_int == 0 && file_id as libc::c_int == 5 as libc::c_int {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
        return
    }
    if op as libc::c_int == 1 as libc::c_int &&
           file_id as libc::c_int != 5 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x85 as libc::c_int as uint8_t);
        return
    }
    if file_id as libc::c_int > 5 as libc::c_int {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
        return
    }
    if is_short_EF != 0 {
        file_selection =
            (file_id as libc::c_int - 0 as libc::c_int + 4 as libc::c_int) as
                uint8_t;
        offset = p2 as uint16_t;
        if op as libc::c_int == 1 as libc::c_int {
            r = flash_erase_binary(file_id);
            if r < 0 as libc::c_int {
                set_res_sw(0x65 as libc::c_int as uint8_t,
                           0x81 as libc::c_int as uint8_t);
                return
            }
        }
    } else {
        offset =
            ((p1 as libc::c_int) << 8 as libc::c_int | p2 as libc::c_int) as
                uint16_t
    }
    if file_id as libc::c_int == 5 as libc::c_int &&
           len & 1 as libc::c_int != 0 {
        /* It's OK the size of last write is odd.  */
        let fresh0 = len;
        len = len + 1;
        *apdu.cmd_apdu_data.offset(fresh0 as isize) =
            0xff as libc::c_int as uint8_t
    }
    r =
        flash_write_binary(file_id, apdu.cmd_apdu_data, len as uint16_t,
                           offset);
    if r < 0 as libc::c_int {
        set_res_sw(0x65 as libc::c_int as uint8_t,
                   0x81 as libc::c_int as uint8_t);
        return
    }
    if file_id as libc::c_int >= 1 as libc::c_int &&
           file_id as libc::c_int <= 4 as libc::c_int &&
           len == 0 as libc::c_int &&
           offset as libc::c_int == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut p: *const uint8_t = 0 as *const uint8_t;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            p = gpg_get_firmware_update_key(i as uint8_t);
            if *p.offset(0 as libc::c_int as isize) as libc::c_int !=
                   0 as libc::c_int ||
                   *p.offset(1 as libc::c_int as isize) as libc::c_int !=
                       0 as libc::c_int {
                break ;
            }
            i += 1
        }
        if i == 4 as libc::c_int {
            /* all update keys are removed */
            p = gpg_get_firmware_update_key(0 as libc::c_int as uint8_t);
            flash_erase_page(p as uint32_t as uintptr_t);
        }
    }
    set_res_sw(0x90 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t);
}
unsafe extern "C" fn cmd_write_binary() {
    let mut len: libc::c_int = apdu.cmd_apdu_data_len as libc::c_int;
    modify_binary(0 as libc::c_int as uint8_t,
                  *apdu.cmd_apdu_head.offset(2 as libc::c_int as isize),
                  *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize), len);
}
unsafe extern "C" fn cmd_external_authenticate() {
    let mut pubkey: *const uint8_t = 0 as *const uint8_t;
    let mut signature: *const uint8_t = apdu.cmd_apdu_data;
    let mut len: libc::c_int = apdu.cmd_apdu_data_len as libc::c_int;
    let mut keyno: uint8_t =
        *apdu.cmd_apdu_head.offset(3 as libc::c_int as isize);
    let mut r: libc::c_int = 0;
    if keyno as libc::c_int >= 4 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x85 as libc::c_int as uint8_t);
        return
    }
    pubkey = gpg_get_firmware_update_key(keyno);
    if len != 256 as libc::c_int ||
           *pubkey.offset(0 as libc::c_int as isize) as libc::c_int ==
               0xff as libc::c_int &&
               *pubkey.offset(1 as libc::c_int as isize) as libc::c_int ==
                   0xff as libc::c_int ||
           *pubkey.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int &&
               *pubkey.offset(1 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int {
        /* removed */
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x85 as libc::c_int as uint8_t); /* signal to self.  */
        return
    }
    r = rsa_verify(pubkey, 256 as libc::c_int, challenge, signature);
    random_bytes_free(challenge);
    challenge = 0 as *const uint8_t;
    if r < 0 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x82 as libc::c_int as uint8_t);
        return
    }
    eventflag_signal(openpgp_comm, 2 as libc::c_int as eventmask_t);
    set_res_sw(0xff as libc::c_int as uint8_t,
               0xff as libc::c_int as uint8_t);
}
unsafe extern "C" fn cmd_get_challenge() {
    let mut len: libc::c_int = apdu.expected_res_size as libc::c_int;
    if len > 32 as libc::c_int {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x85 as libc::c_int as uint8_t);
        return
    } else {
        if len == 0 as libc::c_int {
            /* Le is not specified.  Return full-sized challenge by GET_RESPONSE.  */
            len = 32 as libc::c_int
        }
    }
    if !challenge.is_null() { random_bytes_free(challenge); }
    challenge = random_bytes_get();
    memcpy(apdu.res_apdu_data as *mut libc::c_void,
           challenge as *const libc::c_void, len as libc::c_ulong);
    apdu.res_apdu_data_len = len as uint16_t;
    set_res_sw(0x90 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t);
}
#[no_mangle]
pub static mut cmds: [command; 14] =
    unsafe {
        [{
             let mut init =
                 command{command: 0x20 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_verify as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0x24 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_change_password as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0x2a as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_pso as unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0x2c as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_reset_user_password as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0x47 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_pgp_gakp as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0x82 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_external_authenticate as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0x84 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_get_challenge as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0x88 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_internal_authenticate as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0xa4 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_select_file as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0xb0 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_read_binary as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0xca as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_get_data as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0xd0 as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_write_binary as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0xda as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_put_data as
                                      unsafe extern "C" fn() -> ()),};
             init
         },
         {
             let mut init =
                 command{command: 0xdb as libc::c_int as uint8_t,
                         cmd_handler:
                             Some(cmd_put_data as
                                      unsafe extern "C" fn() -> ()),};
             init
         }]
    };
unsafe extern "C" fn process_command_apdu() {
    let mut i: libc::c_int = 0;
    let mut cmd: uint8_t =
        *apdu.cmd_apdu_head.offset(1 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[command; 14]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<command>()
                                                   as libc::c_ulong) as
                  libc::c_int {
        if cmds[i as usize].command as libc::c_int == cmd as libc::c_int {
            break ;
        }
        i += 1
    }
    if i <
           (::std::mem::size_of::<[command; 14]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<command>()
                                                as libc::c_ulong) as
               libc::c_int {
        chopstx_setcancelstate(1 as libc::c_int);
        cmds[i as usize].cmd_handler.expect("non-null function pointer")();
        chopstx_setcancelstate(0 as libc::c_int);
    } else {
        set_res_sw(0x6d as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn openpgp_card_thread(mut arg: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut ccid_comm: *mut eventflag = arg as *mut eventflag;
    openpgp_comm = ccid_comm.offset(1 as libc::c_int as isize);
    gpg_init();
    loop  {
        let mut m: eventmask_t = eventflag_wait(openpgp_comm);
        if m == 8 as libc::c_int as libc::c_uint {
            set_res_sw(0x6f as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        } else if m == 16 as libc::c_int as libc::c_uint {
            set_res_sw(0x6f as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        } else {
            if m == 2 as libc::c_int as libc::c_uint { break ; }
            led_blink(64 as libc::c_int);
            process_command_apdu();
            led_blink(128 as libc::c_int);
        }
        eventflag_signal(ccid_comm, 2 as libc::c_int as eventmask_t);
    }
    gpg_fini();
    return 0 as *mut libc::c_void;
}
