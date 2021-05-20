#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn gpg_pw_locked(which: uint8_t) -> libc::c_int;
    #[no_mangle]
    fn gpg_pw_reset_err_counter(which: uint8_t);
    #[no_mangle]
    fn gpg_pw_increment_err_counter(which: uint8_t);
    #[no_mangle]
    fn s2k(salt: *const libc::c_uchar, slen: size_t,
           input: *const libc::c_uchar, ilen: size_t,
           output: *mut libc::c_uchar);
    #[no_mangle]
    fn gpg_do_clear_prvkey(kk: kind_of_key);
    #[no_mangle]
    fn gpg_do_load_prvkey(kk: kind_of_key, who: libc::c_int,
                          keystring: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn gpg_do_read_simple(_: uint8_t) -> *const uint8_t;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
/* Constants: algo+size */
pub type kind_of_key = libc::c_uint;
pub const GPG_KEY_FOR_AUTHENTICATION: kind_of_key = 2;
pub const GPG_KEY_FOR_DECRYPTION: kind_of_key = 1;
pub const GPG_KEY_FOR_SIGNING: kind_of_key = 0;
/*
 * ac.c -- Check access condition
 *
 * Copyright (C) 2010, 2012, 2013 Free Software Initiative of Japan
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
pub static mut auth_status: uint8_t = 0;
/* Initialized to AC_NONE_AUTHORIZED */
#[no_mangle]
pub unsafe extern "C" fn ac_check_status(mut ac_flag: uint8_t)
 -> libc::c_int {
    if ac_flag as libc::c_int == 0xff as libc::c_int {
        return 1 as libc::c_int
    } else if ac_flag as libc::c_int == 0x80 as libc::c_int {
        return 0 as libc::c_int
    } else {
        return if ac_flag as libc::c_int & auth_status as libc::c_int != 0 {
                   1 as libc::c_int
               } else { 0 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ac_reset_pso_cds() {
    gpg_do_clear_prvkey(GPG_KEY_FOR_SIGNING);
    ::std::ptr::write_volatile(&mut auth_status as *mut uint8_t,
                               (::std::ptr::read_volatile::<uint8_t>(&auth_status
                                                                         as
                                                                         *const uint8_t)
                                    as libc::c_int & !(0x1 as libc::c_int)) as
                                   uint8_t as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn ac_reset_other() {
    gpg_do_clear_prvkey(GPG_KEY_FOR_DECRYPTION);
    gpg_do_clear_prvkey(GPG_KEY_FOR_AUTHENTICATION);
    ::std::ptr::write_volatile(&mut auth_status as *mut uint8_t,
                               (::std::ptr::read_volatile::<uint8_t>(&auth_status
                                                                         as
                                                                         *const uint8_t)
                                    as libc::c_int & !(0x2 as libc::c_int)) as
                                   uint8_t as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn verify_user_0(mut access: uint8_t,
                                       mut pw: *const uint8_t,
                                       mut buf_len: libc::c_int,
                                       mut pw_len_known: libc::c_int,
                                       mut ks_pw1: *const uint8_t,
                                       mut save_ks: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut pw_len: libc::c_int = 0;
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut keystring: [uint8_t; 32] = [0; 32];
    let mut salt: *const uint8_t = 0 as *const uint8_t;
    let mut salt_len: libc::c_int = 0;
    if gpg_pw_locked(0 as libc::c_int as uint8_t) != 0 {
        return 0 as libc::c_int
    }
    if ks_pw1.is_null() {
        pw_len =
            strlen(b"123456\x00" as *const u8 as *const libc::c_char) as
                libc::c_int;
        salt = 0 as *const uint8_t;
        salt_len = 0 as libc::c_int;
        if pw_len_known >= 0 as libc::c_int && pw_len_known != pw_len ||
               buf_len < pw_len ||
               strncmp(pw as *const libc::c_char,
                       b"123456\x00" as *const u8 as *const libc::c_char,
                       pw_len as libc::c_ulong) != 0 {
            current_block = 11748768052136803632;
        } else { current_block = 3512920355445576850; }
    } else {
        pw_len =
            *ks_pw1.offset(0 as libc::c_int as isize) as libc::c_int &
                0x7f as libc::c_int;
        salt = ks_pw1.offset(1 as libc::c_int as isize);
        salt_len = 8 as libc::c_int;
        if pw_len_known >= 0 as libc::c_int && pw_len_known != pw_len ||
               buf_len < pw_len {
            current_block = 11748768052136803632;
        } else { current_block = 3512920355445576850; }
    }
    match current_block {
        3512920355445576850 => {
            s2k(salt, salt_len as size_t, pw, pw_len as size_t,
                keystring.as_mut_ptr());
            if save_ks != 0 {
                memcpy(keystring_md_pw3.as_mut_ptr() as *mut libc::c_void,
                       keystring.as_mut_ptr() as *const libc::c_void,
                       32 as libc::c_int as libc::c_ulong);
            }
            if access as libc::c_int == 0x1 as libc::c_int {
                r1 =
                    gpg_do_load_prvkey(GPG_KEY_FOR_SIGNING, 1 as libc::c_int,
                                       keystring.as_mut_ptr());
                r2 = 0 as libc::c_int
            } else {
                r1 =
                    gpg_do_load_prvkey(GPG_KEY_FOR_DECRYPTION,
                                       1 as libc::c_int,
                                       keystring.as_mut_ptr());
                r2 =
                    gpg_do_load_prvkey(GPG_KEY_FOR_AUTHENTICATION,
                                       1 as libc::c_int,
                                       keystring.as_mut_ptr())
            }
            if !(r1 < 0 as libc::c_int || r2 < 0 as libc::c_int ||
                     r1 == 0 as libc::c_int && r2 == 0 as libc::c_int &&
                         !ks_pw1.is_null() &&
                         (*ks_pw1.offset(0 as libc::c_int as isize) as
                              libc::c_int & 0x80 as libc::c_int ==
                              0 as libc::c_int ||
                              memcmp(ks_pw1.offset((1 as libc::c_int +
                                                        8 as libc::c_int) as
                                                       isize) as
                                         *const libc::c_void,
                                     keystring.as_mut_ptr() as
                                         *const libc::c_void,
                                     32 as libc::c_int as libc::c_ulong) !=
                                  0 as libc::c_int)) {
                gpg_pw_reset_err_counter(0 as libc::c_int as uint8_t);
                return pw_len
            }
        }
        _ => { }
    }
    gpg_pw_increment_err_counter(0 as libc::c_int as uint8_t);
    return -(1 as libc::c_int);
}
/*
 * Verify for "Perform Security Operation : Compute Digital Signature"
 */
#[no_mangle]
pub unsafe extern "C" fn verify_pso_cds(mut pw: *const uint8_t,
                                        mut pw_len: libc::c_int)
 -> libc::c_int {
    let mut ks_pw1: *const uint8_t =
        gpg_do_read_simple(0x11 as libc::c_int as uint8_t);
    let mut r: libc::c_int = 0;
    r =
        verify_user_0(0x1 as libc::c_int as uint8_t, pw, pw_len, pw_len,
                      ks_pw1, 0 as libc::c_int);
    if r > 0 as libc::c_int {
        ::std::ptr::write_volatile(&mut auth_status as *mut uint8_t,
                                   (::std::ptr::read_volatile::<uint8_t>(&auth_status
                                                                             as
                                                                             *const uint8_t)
                                        as libc::c_int | 0x1 as libc::c_int)
                                       as uint8_t as uint8_t)
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn verify_other(mut pw: *const uint8_t,
                                      mut pw_len: libc::c_int)
 -> libc::c_int {
    let mut ks_pw1: *const uint8_t =
        gpg_do_read_simple(0x11 as libc::c_int as uint8_t);
    let mut r: libc::c_int = 0;
    r =
        verify_user_0(0x2 as libc::c_int as uint8_t, pw, pw_len, pw_len,
                      ks_pw1, 0 as libc::c_int);
    if r > 0 as libc::c_int {
        ::std::ptr::write_volatile(&mut auth_status as *mut uint8_t,
                                   (::std::ptr::read_volatile::<uint8_t>(&auth_status
                                                                             as
                                                                             *const uint8_t)
                                        as libc::c_int | 0x2 as libc::c_int)
                                       as uint8_t as uint8_t)
    }
    return r;
}
unsafe extern "C" fn verify_admin_00(mut pw: *const uint8_t,
                                     mut buf_len: libc::c_int,
                                     mut pw_len_known: libc::c_int,
                                     mut ks: *const uint8_t,
                                     mut save_ks: libc::c_int)
 -> libc::c_int {
    let mut pw_len: libc::c_int = 0;
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut keystring: [uint8_t; 32] = [0; 32];
    let mut salt: *const uint8_t = 0 as *const uint8_t;
    let mut salt_len: libc::c_int = 0;
    pw_len =
        *ks.offset(0 as libc::c_int as isize) as libc::c_int &
            0x7f as libc::c_int;
    salt = ks.offset(1 as libc::c_int as isize);
    salt_len = 8 as libc::c_int;
    if pw_len_known >= 0 as libc::c_int && pw_len_known != pw_len ||
           buf_len < pw_len {
        return -(1 as libc::c_int)
    }
    s2k(salt, salt_len as size_t, pw, pw_len as size_t,
        keystring.as_mut_ptr());
    if save_ks != 0 {
        memcpy(keystring_md_pw3.as_mut_ptr() as *mut libc::c_void,
               keystring.as_mut_ptr() as *const libc::c_void,
               32 as libc::c_int as libc::c_ulong);
    }
    r1 =
        gpg_do_load_prvkey(GPG_KEY_FOR_SIGNING, 3 as libc::c_int,
                           keystring.as_mut_ptr());
    r2 = 0 as libc::c_int;
    if r1 < 0 as libc::c_int || r2 < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if r1 == 0 as libc::c_int && r2 == 0 as libc::c_int {
            if *ks.offset(0 as libc::c_int as isize) as libc::c_int &
                   0x80 as libc::c_int == 0 as libc::c_int ||
                   memcmp(ks.offset((1 as libc::c_int + 8 as libc::c_int) as
                                        isize) as *const libc::c_void,
                          keystring.as_mut_ptr() as *const libc::c_void,
                          32 as libc::c_int as libc::c_ulong) !=
                       0 as libc::c_int {
                return -(1 as libc::c_int)
            }
        }
    }
    return pw_len;
}
#[no_mangle]
pub static mut keystring_md_pw3: [uint8_t; 32] = [0; 32];
#[no_mangle]
pub static mut admin_authorized: uint8_t = 0;
#[no_mangle]
pub unsafe extern "C" fn verify_admin_0(mut pw: *const uint8_t,
                                        mut buf_len: libc::c_int,
                                        mut pw_len_known: libc::c_int,
                                        mut pw3_keystring: *const uint8_t,
                                        mut save_ks: libc::c_int)
 -> libc::c_int {
    let mut pw_len: libc::c_int = 0;
    let mut current_block_18: u64;
    if !pw3_keystring.is_null() {
        if gpg_pw_locked(2 as libc::c_int as uint8_t) != 0 {
            return 0 as libc::c_int
        }
        pw_len =
            verify_admin_00(pw, buf_len, pw_len_known, pw3_keystring,
                            save_ks);
        if pw_len < 0 as libc::c_int {
            current_block_18 = 13079727743812876798;
        } else {
            admin_authorized = 3 as libc::c_int as uint8_t;
            current_block_18 = 4453295867175542882;
        }
    } else {
        let mut ks_pw1: *const uint8_t =
            gpg_do_read_simple(0x11 as libc::c_int as uint8_t);
        if !ks_pw1.is_null() {
            /* empty PW3, but PW1 exists */
            let mut r: libc::c_int =
                verify_user_0(0x1 as libc::c_int as uint8_t, pw, buf_len,
                              pw_len_known, ks_pw1, save_ks);
            if r > 0 as libc::c_int {
                admin_authorized = 1 as libc::c_int as uint8_t
            }
            return r
        }
        if gpg_pw_locked(2 as libc::c_int as uint8_t) != 0 {
            return 0 as libc::c_int
        }
        /*
       * For the case of empty PW3 (with empty PW1), pass phrase
       * should be OPENPGP_CARD_INITIAL_PW3
       */
        pw_len =
            strlen(b"12345678\x00" as *const u8 as *const libc::c_char) as
                libc::c_int;
        if pw_len_known >= 0 as libc::c_int && pw_len_known != pw_len ||
               buf_len < pw_len ||
               strncmp(pw as *const libc::c_char,
                       b"12345678\x00" as *const u8 as *const libc::c_char,
                       pw_len as libc::c_ulong) != 0 {
            current_block_18 = 13079727743812876798;
        } else {
            admin_authorized = 3 as libc::c_int as uint8_t;
            if save_ks != 0 {
                s2k(0 as *const libc::c_uchar, 0 as libc::c_int as size_t, pw,
                    pw_len as size_t, keystring_md_pw3.as_mut_ptr());
            }
            current_block_18 = 4453295867175542882;
        }
    }
    match current_block_18 {
        13079727743812876798 => {
            gpg_pw_increment_err_counter(2 as libc::c_int as uint8_t);
            return -(1 as libc::c_int)
        }
        _ => {
            /* OK, the admin is now authenticated.  */
            gpg_pw_reset_err_counter(2 as libc::c_int as uint8_t);
            return pw_len
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn verify_admin(mut pw: *const uint8_t,
                                      mut pw_len: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut pw3_keystring: *const uint8_t = 0 as *const uint8_t;
    pw3_keystring = gpg_do_read_simple(0x13 as libc::c_int as uint8_t);
    r = verify_admin_0(pw, pw_len, pw_len, pw3_keystring, 1 as libc::c_int);
    if r <= 0 as libc::c_int { return r }
    ::std::ptr::write_volatile(&mut auth_status as *mut uint8_t,
                               (::std::ptr::read_volatile::<uint8_t>(&auth_status
                                                                         as
                                                                         *const uint8_t)
                                    as libc::c_int | 0x4 as libc::c_int) as
                                   uint8_t as uint8_t);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ac_reset_admin() {
    memset(keystring_md_pw3.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, 32 as libc::c_int as libc::c_ulong);
    ::std::ptr::write_volatile(&mut auth_status as *mut uint8_t,
                               (::std::ptr::read_volatile::<uint8_t>(&auth_status
                                                                         as
                                                                         *const uint8_t)
                                    as libc::c_int & !(0x4 as libc::c_int)) as
                                   uint8_t as uint8_t);
    admin_authorized = 0 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn ac_fini() {
    memset(keystring_md_pw3.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, 32 as libc::c_int as libc::c_ulong);
    gpg_do_clear_prvkey(GPG_KEY_FOR_SIGNING);
    gpg_do_clear_prvkey(GPG_KEY_FOR_DECRYPTION);
    gpg_do_clear_prvkey(GPG_KEY_FOR_AUTHENTICATION);
    ::std::ptr::write_volatile(&mut auth_status as *mut uint8_t,
                               0 as libc::c_int as uint8_t);
    admin_authorized = 0 as libc::c_int as uint8_t;
}
