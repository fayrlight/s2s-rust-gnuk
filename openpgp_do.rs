#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn gnuk_malloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn gnuk_free(_: *mut libc::c_void);
    #[no_mangle]
    static mut apdu: apdu;
    #[no_mangle]
    fn ac_check_status(ac_flag: uint8_t) -> libc::c_int;
    #[no_mangle]
    fn ac_reset_pso_cds();
    #[no_mangle]
    fn ac_reset_other();
    #[no_mangle]
    fn ac_reset_admin();
    #[no_mangle]
    fn set_res_sw(sw1: uint8_t, sw2: uint8_t);
    #[no_mangle]
    fn flash_do_release(_: *const uint8_t);
    #[no_mangle]
    fn flash_do_write(nr: uint8_t, data: *const uint8_t, len: libc::c_int)
     -> *const uint8_t;
    #[no_mangle]
    fn flash_key_alloc(_: kind_of_key) -> *mut uint8_t;
    #[no_mangle]
    fn flash_key_release(_: *mut uint8_t, _: libc::c_int);
    #[no_mangle]
    fn flash_key_release_page(_: kind_of_key);
    #[no_mangle]
    fn flash_key_write(key_addr: *mut uint8_t, key_data: *const uint8_t,
                       key_data_len: libc::c_int, pubkey: *const uint8_t,
                       pubkey_len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn flash_set_data_pool_last(p: *const uint8_t);
    #[no_mangle]
    fn s2k(salt: *const libc::c_uchar, slen: size_t,
           input: *const libc::c_uchar, ilen: size_t,
           output: *mut libc::c_uchar);
    #[no_mangle]
    fn gpg_change_keystring(who_old: libc::c_int, old_ks: *const uint8_t,
                            who_new: libc::c_int, new_ks: *const uint8_t)
     -> libc::c_int;
    #[no_mangle]
    fn modulus_calc(_: *const uint8_t, _: libc::c_int) -> *mut uint8_t;
    #[no_mangle]
    fn rsa_genkey(_: libc::c_int) -> *mut uint8_t;
    #[no_mangle]
    fn ecc_compute_public_p256r1(key_data: *const uint8_t) -> *mut uint8_t;
    #[no_mangle]
    fn ecc_check_secret_p256r1(d0: *const uint8_t, d1: *mut uint8_t)
     -> libc::c_int;
    #[no_mangle]
    fn ecc_compute_public_p256k1(key_data: *const uint8_t) -> *mut uint8_t;
    #[no_mangle]
    fn ecc_check_secret_p256k1(d0: *const uint8_t, d1: *mut uint8_t)
     -> libc::c_int;
    #[no_mangle]
    fn eddsa_compute_public_25519(a: *const uint8_t) -> *mut uint8_t;
    #[no_mangle]
    fn ecdh_compute_public_25519(a: *const uint8_t) -> *mut uint8_t;
    #[no_mangle]
    static mut keystring_md_pw3: [uint8_t; 32];
    #[no_mangle]
    static mut admin_authorized: uint8_t;
    #[no_mangle]
    fn flash_bool_clear(addr_p: *mut *const uint8_t);
    #[no_mangle]
    fn flash_bool_write(nr: uint8_t) -> *const uint8_t;
    #[no_mangle]
    fn flash_enum_clear(addr_p: *mut *const uint8_t);
    #[no_mangle]
    fn flash_enum_write(nr: uint8_t, v: uint8_t) -> *const uint8_t;
    #[no_mangle]
    fn flash_cnt123_get_value(p: *const uint8_t) -> libc::c_int;
    #[no_mangle]
    fn flash_cnt123_increment(which: uint8_t, addr_p: *mut *const uint8_t);
    #[no_mangle]
    fn flash_cnt123_clear(addr_p: *mut *const uint8_t);
    #[no_mangle]
    fn flash_put_data(hw: uint16_t);
    #[no_mangle]
    fn flash_warning(msg: *const libc::c_char);
    #[no_mangle]
    fn flash_put_data_internal(p: *const uint8_t, hw: uint16_t);
    #[no_mangle]
    fn flash_bool_write_internal(p: *const uint8_t, nr: libc::c_int);
    #[no_mangle]
    fn flash_enum_write_internal(p: *const uint8_t, nr: libc::c_int,
                                 v: uint8_t);
    #[no_mangle]
    fn flash_cnt123_write_internal(p: *const uint8_t, which: libc::c_int,
                                   v: libc::c_int);
    #[no_mangle]
    fn flash_do_write_internal(p: *const uint8_t, nr: libc::c_int,
                               data: *const uint8_t, len: libc::c_int);
    /* 32-byte random bytes */
    #[no_mangle]
    fn random_bytes_get() -> *const uint8_t;
    #[no_mangle]
    fn random_bytes_free(p: *const uint8_t);
    /* 8-byte salt */
    #[no_mangle]
    fn random_get_salt(p: *mut uint8_t);
    /* *
 * \brief          AES key schedule (encryption)
 *
 * \param ctx      AES context to be initialized
 * \param key      encryption key
 * \param keysize  must be 128, 192 or 256
 *
 * \return         0 if successful, or POLARSSL_ERR_AES_INVALID_KEY_LENGTH
 */
    #[no_mangle]
    fn aes_setkey_enc(ctx: *mut aes_context, key: *const libc::c_uchar,
                      keysize: libc::c_uint) -> libc::c_int;
    /* *
 * \brief          AES key schedule (decryption)
 *
 * \param ctx      AES context to be initialized
 * \param key      decryption key
 * \param keysize  must be 128, 192 or 256
 *
 * \return         0 if successful, or POLARSSL_ERR_AES_INVALID_KEY_LENGTH
 */
    #[no_mangle]
    fn aes_setkey_dec(ctx: *mut aes_context, key: *const libc::c_uchar,
                      keysize: libc::c_uint) -> libc::c_int;
    /* *
 * \brief          AES-ECB block encryption/decryption
 *
 * \param ctx      AES context
 * \param mode     AES_ENCRYPT or AES_DECRYPT
 * \param input    16-byte input block
 * \param output   16-byte output block
 *
 * \return         0 if successful
 */
    #[no_mangle]
    fn aes_crypt_ecb(ctx: *mut aes_context, mode: libc::c_int,
                     input: *const libc::c_uchar, output: *mut libc::c_uchar)
     -> libc::c_int;
    /* *
 * \brief          AES-CFB128 buffer encryption/decryption.
 *
 * Note: Due to the nature of CFB you should use the same key schedule for
 * both encryption and decryption. So a context initialized with
 * aes_setkey_enc() for both AES_ENCRYPT and AES_DECRYPT.
 *
 * both 
 * \param ctx      AES context
 * \param mode     AES_ENCRYPT or AES_DECRYPT
 * \param length   length of the input data
 * \param iv_off   offset in IV (updated after use)
 * \param iv       initialization vector (updated after use)
 * \param input    buffer holding the input data
 * \param output   buffer holding the output data
 *
 * \return         0 if successful
 */
    #[no_mangle]
    fn aes_crypt_cfb128(ctx: *mut aes_context, mode: libc::c_int,
                        length: size_t, iv_off: *mut size_t,
                        iv: *mut libc::c_uchar, input: *const libc::c_uchar,
                        output: *mut libc::c_uchar) -> libc::c_int;
    #[no_mangle]
    fn sha512(input: *const libc::c_uchar, ilen: libc::c_uint,
              output: *mut libc::c_uchar);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct do_table_entry {
    pub tag: uint16_t,
    pub do_type: do_type,
    pub ac_read: uint8_t,
    pub ac_write: uint8_t,
    pub obj: *const libc::c_void,
}
/* RFC4880 */
pub type do_type = libc::c_uint;
pub const DO_PROC_READWRITE: do_type = 5;
pub const DO_PROC_WRITE: do_type = 4;
pub const DO_PROC_READ: do_type = 3;
pub const DO_CMP_READ: do_type = 2;
pub const DO_VAR: do_type = 1;
pub const DO_FIXED: do_type = 0;
pub type kind_of_key = libc::c_uint;
pub const GPG_KEY_FOR_AUTHENTICATION: kind_of_key = 2;
pub const GPG_KEY_FOR_DECRYPTION: kind_of_key = 1;
pub const GPG_KEY_FOR_SIGNING: kind_of_key = 0;
/* decrypted private key data content */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prvkey_data {
    pub iv: [uint8_t; 16],
    pub checksum_encrypted: [uint8_t; 16],
    pub dek_encrypted_1: [uint8_t; 16],
    pub dek_encrypted_2: [uint8_t; 16],
    pub dek_encrypted_3: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_context {
    pub nr: libc::c_int,
    pub rk: *mut uint32_t,
    pub buf: [uint32_t; 68],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_data_internal {
    pub data: [uint32_t; 132],
}
/* Maximum is the case for RSA 4096-bit.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_data {
    pub pubkey: *const uint8_t,
    pub data: [uint8_t; 512],
}
pub type size_of_key = libc::c_uint;
pub const GPG_KEY_PRIVATE: size_of_key = 2;
/* PUBKEY + PRVKEY rounded to 2^N */
pub const GPG_KEY_PUBLIC: size_of_key = 1;
pub const GPG_KEY_STORAGE: size_of_key = 0;
#[inline]
unsafe extern "C" fn unique_device_id() -> *const uint8_t {
    /* STM32F103 has 96-bit unique device identifier */
    let mut addr: *const uint8_t =
        0x1ffff7e8 as libc::c_int as *const uint8_t;
    return addr;
}
/* >= errors, it will be locked */
static mut pw_err_counter_p: [*const uint8_t; 3] = [0 as *const uint8_t; 3];
unsafe extern "C" fn gpg_pw_get_err_counter(mut which: uint8_t)
 -> libc::c_int {
    return flash_cnt123_get_value(pw_err_counter_p[which as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn gpg_pw_get_retry_counter(mut who: libc::c_int)
 -> libc::c_int {
    if who == 0x81 as libc::c_int || who == 0x82 as libc::c_int {
        return 3 as libc::c_int -
                   gpg_pw_get_err_counter(0 as libc::c_int as uint8_t)
    } else if who == 0x83 as libc::c_int {
        return 3 as libc::c_int -
                   gpg_pw_get_err_counter(2 as libc::c_int as uint8_t)
    } else {
        return 3 as libc::c_int -
                   gpg_pw_get_err_counter(1 as libc::c_int as uint8_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_pw_locked(mut which: uint8_t) -> libc::c_int {
    if gpg_pw_get_err_counter(which) >= 3 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_pw_reset_err_counter(mut which: uint8_t) {
    flash_cnt123_clear(&mut *pw_err_counter_p.as_mut_ptr().offset(which as
                                                                      isize));
    if !pw_err_counter_p[which as usize].is_null() {
        set_res_sw(0x65 as libc::c_int as uint8_t,
                   0x81 as libc::c_int as uint8_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_pw_increment_err_counter(mut which: uint8_t) {
    flash_cnt123_increment(which,
                           &mut *pw_err_counter_p.as_mut_ptr().offset(which as
                                                                          isize));
}
#[no_mangle]
pub static mut data_objects_number_of_bytes: uint16_t = 0;
/*
 * Compile time vars:
 *   Historical Bytes (template), Extended Capabilities.
 */
/* Historical Bytes (template) */
static mut historical_bytes: [uint8_t; 11] =
    [10 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x31 as libc::c_int as uint8_t, 0x84 as libc::c_int as uint8_t,
     0x73 as libc::c_int as uint8_t, 0x80 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x80 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x90 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t];
/* Extended Capabilities */
static mut extended_capabilities: [uint8_t; 11] =
    [10 as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     32 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t];
/* It catches 22, finally.  */
static mut algorithm_attr_rsa2k: [uint8_t; 7] =
    [6 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x20 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t];
static mut algorithm_attr_rsa4k: [uint8_t; 7] =
    [6 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x10 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x20 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t];
static mut algorithm_attr_p256r1: [uint8_t; 10] =
    [9 as libc::c_int as uint8_t, 0x13 as libc::c_int as uint8_t,
     0x2a as libc::c_int as uint8_t, 0x86 as libc::c_int as uint8_t,
     0x48 as libc::c_int as uint8_t, 0xce as libc::c_int as uint8_t,
     0x3d as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x7 as libc::c_int as uint8_t];
static mut algorithm_attr_p256k1: [uint8_t; 7] =
    [6 as libc::c_int as uint8_t, 0x13 as libc::c_int as uint8_t,
     0x2b as libc::c_int as uint8_t, 0x81 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0xa as libc::c_int as uint8_t];
static mut algorithm_attr_ed25519: [uint8_t; 11] =
    [10 as libc::c_int as uint8_t, 0x16 as libc::c_int as uint8_t,
     0x2b as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xda as libc::c_int as uint8_t,
     0x47 as libc::c_int as uint8_t, 0xf as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t];
static mut algorithm_attr_cv25519: [uint8_t; 12] =
    [11 as libc::c_int as uint8_t, 0x12 as libc::c_int as uint8_t,
     0x2b as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x97 as libc::c_int as uint8_t,
     0x55 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t];
/*
 * Representation of PW1_LIFETIME:
 *    0: PW1_LIEFTIME_P == NULL : PW1 is valid for single PSO:CDS command
 *    1: PW1_LIEFTIME_P != NULL : PW1 is valid for several PSO:CDS commands
 *
 * The address in the variable PW1_LIEFTIME_P is used when filling zero
 * in flash memory
 */
static mut pw1_lifetime_p: *const uint8_t = 0 as *const uint8_t;
unsafe extern "C" fn gpg_get_pw1_lifetime() -> libc::c_int {
    if pw1_lifetime_p.is_null() {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
/*
 * Representation of algorithm attributes:
 *    0: ALGO_ATTR_<>_P == NULL : RSA-2048
 *    N: ALGO_ATTR_<>_P != NULL :
 *
 */
static mut algo_attr_sig_p: *const uint8_t = 0 as *const uint8_t;
static mut algo_attr_dec_p: *const uint8_t = 0 as *const uint8_t;
static mut algo_attr_aut_p: *const uint8_t = 0 as *const uint8_t;
unsafe extern "C" fn get_algo_attr_pointer(mut kk: kind_of_key)
 -> *mut *const uint8_t {
    if kk as libc::c_uint ==
           GPG_KEY_FOR_SIGNING as libc::c_int as libc::c_uint {
        return &mut algo_attr_sig_p
    } else if kk as libc::c_uint ==
                  GPG_KEY_FOR_DECRYPTION as libc::c_int as libc::c_uint {
        return &mut algo_attr_dec_p
    } else { return &mut algo_attr_aut_p };
}
unsafe extern "C" fn kk_to_nr(mut kk: kind_of_key) -> libc::c_int {
    let mut nr: libc::c_int = 0;
    if kk as libc::c_uint ==
           GPG_KEY_FOR_SIGNING as libc::c_int as libc::c_uint {
        nr = 0xf1 as libc::c_int
    } else if kk as libc::c_uint ==
                  GPG_KEY_FOR_DECRYPTION as libc::c_int as libc::c_uint {
        nr = 0xf2 as libc::c_int
    } else { nr = 0xf3 as libc::c_int }
    return nr;
}
#[no_mangle]
pub unsafe extern "C" fn gpg_get_algo_attr(mut kk: kind_of_key)
 -> libc::c_int {
    let mut algo_attr_p: *const uint8_t = *get_algo_attr_pointer(kk);
    if algo_attr_p.is_null() { return 255 as libc::c_int }
    return *algo_attr_p.offset(1 as libc::c_int as isize) as libc::c_int;
}
unsafe extern "C" fn get_algo_attr_data_object(mut kk: kind_of_key)
 -> *const uint8_t {
    let mut algo_attr_p: *const uint8_t = *get_algo_attr_pointer(kk);
    if algo_attr_p.is_null() { return algorithm_attr_rsa2k.as_ptr() }
    match *algo_attr_p.offset(1 as libc::c_int as isize) as libc::c_int {
        0 => { return algorithm_attr_rsa4k.as_ptr() }
        1 => { return algorithm_attr_p256r1.as_ptr() }
        2 => { return algorithm_attr_p256k1.as_ptr() }
        3 => { return algorithm_attr_ed25519.as_ptr() }
        4 => { return algorithm_attr_cv25519.as_ptr() }
        _ => { return algorithm_attr_rsa2k.as_ptr() }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_get_algo_attr_key_size(mut kk: kind_of_key,
                                                    mut s: size_of_key)
 -> libc::c_int {
    let mut current_block: u64;
    let mut algo_attr_p: *const uint8_t = *get_algo_attr_pointer(kk);
    if !algo_attr_p.is_null() {
        match *algo_attr_p.offset(1 as libc::c_int as isize) as libc::c_int {
            0 => {
                current_block = 14366374458588615391;
                match current_block {
                    5105253204732552581 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    15659505592548260924 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    8987630827386054243 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 32 as libc::c_int
                        } else { return 64 as libc::c_int }
                    }
                    _ => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 1024 as libc::c_int
                        } else { return 512 as libc::c_int }
                    }
                }
            }
            1 | 2 => {
                current_block = 15659505592548260924;
                match current_block {
                    5105253204732552581 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    15659505592548260924 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    8987630827386054243 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 32 as libc::c_int
                        } else { return 64 as libc::c_int }
                    }
                    _ => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 1024 as libc::c_int
                        } else { return 512 as libc::c_int }
                    }
                }
            }
            3 => {
                current_block = 8987630827386054243;
                match current_block {
                    5105253204732552581 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    15659505592548260924 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    8987630827386054243 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 32 as libc::c_int
                        } else { return 64 as libc::c_int }
                    }
                    _ => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 1024 as libc::c_int
                        } else { return 512 as libc::c_int }
                    }
                }
            }
            4 => {
                current_block = 5105253204732552581;
                match current_block {
                    5105253204732552581 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    15659505592548260924 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 64 as libc::c_int
                        } else { return 32 as libc::c_int }
                    }
                    8987630827386054243 => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 128 as libc::c_int
                        } else if s as libc::c_uint ==
                                      GPG_KEY_PUBLIC as libc::c_int as
                                          libc::c_uint {
                            return 32 as libc::c_int
                        } else { return 64 as libc::c_int }
                    }
                    _ => {
                        if s as libc::c_uint ==
                               GPG_KEY_STORAGE as libc::c_int as libc::c_uint
                           {
                            return 1024 as libc::c_int
                        } else { return 512 as libc::c_int }
                    }
                }
            }
            _ => { }
        }
    }
    /* RSA-2048 */
    if s as libc::c_uint == GPG_KEY_STORAGE as libc::c_int as libc::c_uint {
        return 512 as libc::c_int
    } else { return 256 as libc::c_int };
}
static mut digital_signature_counter: uint32_t = 0;
unsafe extern "C" fn gpg_write_digital_signature_counter(mut p:
                                                             *const uint8_t,
                                                         mut dsc: uint32_t)
 -> *const uint8_t {
    let mut hw0: uint16_t = 0;
    let mut hw1: uint16_t = 0;
    if dsc >> 10 as libc::c_int == 0 as libc::c_int as libc::c_uint {
        /* no upper bits */
        hw1 =
            (0xc0 as libc::c_int as libc::c_uint |
                 (dsc & 0x300 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int |
                 (dsc & 0xff as libc::c_int as libc::c_uint) <<
                     8 as libc::c_int) as uint16_t;
        flash_put_data_internal(p, hw1);
        return p.offset(2 as libc::c_int as isize)
    } else {
        hw0 =
            (0x80 as libc::c_int as libc::c_uint |
                 (dsc & 0xfc0000 as libc::c_int as libc::c_uint) >>
                     18 as libc::c_int |
                 (dsc & 0x3fc00 as libc::c_int as libc::c_uint) >>
                     2 as libc::c_int) as uint16_t;
        hw1 = 0xc0 as libc::c_int as uint16_t;
        flash_put_data_internal(p, hw0);
        flash_put_data_internal(p.offset(2 as libc::c_int as isize), hw1);
        return p.offset(4 as libc::c_int as isize)
    };
}
unsafe extern "C" fn gpg_reset_digital_signature_counter() {
    if digital_signature_counter != 0 as libc::c_int as libc::c_uint {
        flash_put_data(0x80 as libc::c_int as uint16_t);
        flash_put_data(0xc0 as libc::c_int as uint16_t);
        digital_signature_counter = 0 as libc::c_int as uint32_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_increment_digital_signature_counter() {
    let mut hw0: uint16_t = 0;
    let mut hw1: uint16_t = 0;
    let mut dsc: uint32_t =
        digital_signature_counter.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) &
            0xffffff as libc::c_int as libc::c_uint;
    if dsc & 0x3ff as libc::c_int as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        /* carry occurs from l10 to h14 */
        hw0 =
            (0x80 as libc::c_int as libc::c_uint |
                 (dsc & 0xfc0000 as libc::c_int as libc::c_uint) >>
                     18 as libc::c_int |
                 (dsc & 0x3fc00 as libc::c_int as libc::c_uint) >>
                     2 as libc::c_int) as uint16_t; /* zero */
        hw1 = 0xc0 as libc::c_int as uint16_t;
        flash_put_data(hw0);
        flash_put_data(hw1);
    } else {
        hw1 =
            (0xc0 as libc::c_int as libc::c_uint |
                 (dsc & 0x300 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int |
                 (dsc & 0xff as libc::c_int as libc::c_uint) <<
                     8 as libc::c_int) as uint16_t;
        flash_put_data(hw1);
    }
    digital_signature_counter = dsc;
    if gpg_get_pw1_lifetime() == 0 as libc::c_int { ac_reset_pso_cds(); };
}
static mut res_p: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut do_ptr: [*const uint8_t; 20] = [0 as *const uint8_t; 20];
unsafe extern "C" fn do_tag_to_nr(mut tag: uint16_t) -> libc::c_int {
    match tag as libc::c_int {
        24373 => { return 0 as libc::c_int }
        199 => { return 0x1 as libc::c_int }
        200 => { return 0x2 as libc::c_int }
        201 => { return 0x3 as libc::c_int }
        202 => { return 0x4 as libc::c_int }
        203 => { return 0x5 as libc::c_int }
        204 => { return 0x6 as libc::c_int }
        206 => { return 0x7 as libc::c_int }
        207 => { return 0x8 as libc::c_int }
        208 => { return 0x9 as libc::c_int }
        94 => { return 0xa as libc::c_int }
        24400 => { return 0xb as libc::c_int }
        91 => { return 0xc as libc::c_int }
        24365 => { return 0xd as libc::c_int }
        _ => { return -(1 as libc::c_int) }
    };
}
unsafe extern "C" fn copy_tag(mut tag: uint16_t) {
    if (tag as libc::c_int) < 0x100 as libc::c_int {
        let fresh0 = res_p;
        res_p = res_p.offset(1);
        *fresh0 = (tag as libc::c_int & 0xff as libc::c_int) as uint8_t
    } else {
        let fresh1 = res_p;
        res_p = res_p.offset(1);
        *fresh1 = (tag as libc::c_int >> 8 as libc::c_int) as uint8_t;
        let fresh2 = res_p;
        res_p = res_p.offset(1);
        *fresh2 = (tag as libc::c_int & 0xff as libc::c_int) as uint8_t
    };
}
unsafe extern "C" fn do_hist_bytes(mut tag: uint16_t,
                                   mut with_tag: libc::c_int) -> libc::c_int {
    /*
   * Currently, we support no life cycle management.  In case of Gnuk,
   * user could flash the MCU with SWD/JTAG, instead.  It is also
   * possible for user to do firmware upgrade through USB.
   *
   * Thus, here, it just returns the template as is.
   *
   * In future (when Gnuk will be on the real smartcard),
   * we can support life cycle management by implementing
   * TERMINATE DF / ACTIVATE FILE and fix code around here.
   */
    copy_do_1(tag, historical_bytes.as_ptr(), with_tag);
    return 1 as libc::c_int;
}
unsafe extern "C" fn do_fp_all(mut tag: uint16_t, mut with_tag: libc::c_int)
 -> libc::c_int {
    let mut data: *const uint8_t = 0 as *const uint8_t;
    if with_tag != 0 {
        copy_tag(tag);
        let fresh3 = res_p;
        res_p = res_p.offset(1);
        *fresh3 = (20 as libc::c_int * 3 as libc::c_int) as uint8_t
    }
    data = gpg_do_read_simple(0x1 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               20 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               20 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(20 as libc::c_int as isize);
    data = gpg_do_read_simple(0x2 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               20 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               20 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(20 as libc::c_int as isize);
    data = gpg_do_read_simple(0x3 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               20 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               20 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(20 as libc::c_int as isize);
    return 1 as libc::c_int;
}
unsafe extern "C" fn do_cafp_all(mut tag: uint16_t, mut with_tag: libc::c_int)
 -> libc::c_int {
    let mut data: *const uint8_t = 0 as *const uint8_t;
    if with_tag != 0 {
        copy_tag(tag);
        let fresh4 = res_p;
        res_p = res_p.offset(1);
        *fresh4 = (20 as libc::c_int * 3 as libc::c_int) as uint8_t
    }
    data = gpg_do_read_simple(0x4 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               20 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               20 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(20 as libc::c_int as isize);
    data = gpg_do_read_simple(0x5 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               20 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               20 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(20 as libc::c_int as isize);
    data = gpg_do_read_simple(0x5 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               20 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               20 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(20 as libc::c_int as isize);
    return 1 as libc::c_int;
}
unsafe extern "C" fn do_kgtime_all(mut tag: uint16_t,
                                   mut with_tag: libc::c_int) -> libc::c_int {
    let mut data: *const uint8_t = 0 as *const uint8_t;
    if with_tag != 0 {
        copy_tag(tag);
        let fresh5 = res_p;
        res_p = res_p.offset(1);
        *fresh5 = (4 as libc::c_int * 3 as libc::c_int) as uint8_t
    }
    data = gpg_do_read_simple(0x7 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               4 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(4 as libc::c_int as isize);
    data = gpg_do_read_simple(0x8 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               4 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(4 as libc::c_int as isize);
    data = gpg_do_read_simple(0x9 as libc::c_int as uint8_t);
    if !data.is_null() {
        memcpy(res_p as *mut libc::c_void, data as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
    } else {
        memset(res_p as *mut libc::c_void, 0 as libc::c_int,
               4 as libc::c_int as libc::c_ulong);
    }
    res_p = res_p.offset(4 as libc::c_int as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut openpgpcard_aid: [uint8_t; 14] =
    [0xd2 as libc::c_int as uint8_t, 0x76 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x24 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t];
unsafe extern "C" fn do_openpgpcard_aid(mut tag: uint16_t,
                                        mut with_tag: libc::c_int)
 -> libc::c_int {
    let mut p: *const uint8_t = openpgpcard_aid.as_ptr();
    let mut vid: uint16_t =
        ((*p.offset(8 as libc::c_int as isize) as libc::c_int) <<
             8 as libc::c_int |
             *p.offset(9 as libc::c_int as isize) as libc::c_int) as uint16_t;
    if with_tag != 0 {
        copy_tag(tag);
        let fresh6 = res_p;
        res_p = res_p.offset(1);
        *fresh6 = 16 as libc::c_int as uint8_t
    }
    if vid as libc::c_int == 0xffff as libc::c_int ||
           vid as libc::c_int == 0 as libc::c_int {
        let mut u: *const uint8_t =
            unique_device_id().offset(8 as libc::c_int as isize);
        memcpy(res_p as *mut libc::c_void,
               openpgpcard_aid.as_ptr() as *const libc::c_void,
               8 as libc::c_int as libc::c_ulong);
        res_p = res_p.offset(8 as libc::c_int as isize);
        /* vid == 0xfffe: serial number is four random bytes */
        let fresh7 = res_p; /* Failure */
        res_p = res_p.offset(1);
        *fresh7 = 0xff as libc::c_int as uint8_t;
        let fresh8 = res_p;
        res_p = res_p.offset(1);
        *fresh8 = 0xfe as libc::c_int as uint8_t;
        let fresh9 = res_p;
        res_p = res_p.offset(1);
        *fresh9 = *u.offset(3 as libc::c_int as isize);
        let fresh10 = res_p;
        res_p = res_p.offset(1);
        *fresh10 = *u.offset(2 as libc::c_int as isize);
        let fresh11 = res_p;
        res_p = res_p.offset(1);
        *fresh11 = *u.offset(1 as libc::c_int as isize);
        let fresh12 = res_p;
        res_p = res_p.offset(1);
        *fresh12 = *u.offset(0 as libc::c_int as isize)
    } else {
        memcpy(res_p as *mut libc::c_void,
               openpgpcard_aid.as_ptr() as *const libc::c_void,
               14 as libc::c_int as libc::c_ulong);
        res_p = res_p.offset(14 as libc::c_int as isize)
    }
    let fresh13 = res_p;
    res_p = res_p.offset(1);
    *fresh13 = 0 as libc::c_int as uint8_t;
    let fresh14 = res_p;
    res_p = res_p.offset(1);
    *fresh14 = 0 as libc::c_int as uint8_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn do_ds_count(mut tag: uint16_t, mut with_tag: libc::c_int)
 -> libc::c_int {
    if with_tag != 0 {
        copy_tag(tag);
        let fresh15 = res_p;
        res_p = res_p.offset(1);
        *fresh15 = 3 as libc::c_int as uint8_t
    }
    let fresh16 = res_p;
    res_p = res_p.offset(1);
    *fresh16 =
        (digital_signature_counter >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as uint8_t;
    let fresh17 = res_p;
    res_p = res_p.offset(1);
    *fresh17 =
        (digital_signature_counter >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as uint8_t;
    let fresh18 = res_p;
    res_p = res_p.offset(1);
    *fresh18 =
        (digital_signature_counter & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn rw_pw_status(mut tag: uint16_t,
                                  mut with_tag: libc::c_int,
                                  mut data: *const uint8_t,
                                  mut len: libc::c_int,
                                  mut is_write: libc::c_int) -> libc::c_int {
    if is_write != 0 {
        if len != 1 as libc::c_int { return 0 as libc::c_int }
        /* Success */
        if *data.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int && !pw1_lifetime_p.is_null() {
            flash_bool_clear(&mut pw1_lifetime_p);
            if !pw1_lifetime_p.is_null() {
                /* The first byte of DATA specifies the lifetime.  */
                /* No change after update */
                return 0 as libc::c_int
            }
        } else if pw1_lifetime_p.is_null() {
            pw1_lifetime_p = flash_bool_write(0xf0 as libc::c_int as uint8_t);
            if pw1_lifetime_p.is_null() {
                /* No change after update */
                return 0 as libc::c_int
            }
        } /* Error.  */
        return 1 as libc::c_int
    } else {
        if with_tag != 0 {
            copy_tag(tag);
            let fresh19 = res_p;
            res_p = res_p.offset(1);
            *fresh19 = 7 as libc::c_int as uint8_t
        }
        let fresh20 = res_p;
        res_p = res_p.offset(1);
        *fresh20 = gpg_get_pw1_lifetime() as uint8_t;
        let fresh21 = res_p;
        res_p = res_p.offset(1);
        *fresh21 = 127 as libc::c_int as uint8_t;
        let fresh22 = res_p;
        res_p = res_p.offset(1);
        *fresh22 = 127 as libc::c_int as uint8_t;
        let fresh23 = res_p;
        res_p = res_p.offset(1);
        *fresh23 = 127 as libc::c_int as uint8_t;
        let fresh24 = res_p;
        res_p = res_p.offset(1);
        *fresh24 =
            (3 as libc::c_int -
                 gpg_pw_get_err_counter(0 as libc::c_int as uint8_t)) as
                uint8_t;
        let fresh25 = res_p;
        res_p = res_p.offset(1);
        *fresh25 =
            (3 as libc::c_int -
                 gpg_pw_get_err_counter(1 as libc::c_int as uint8_t)) as
                uint8_t;
        let fresh26 = res_p;
        res_p = res_p.offset(1);
        *fresh26 =
            (3 as libc::c_int -
                 gpg_pw_get_err_counter(2 as libc::c_int as uint8_t)) as
                uint8_t;
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn rw_algorithm_attr(mut tag: uint16_t,
                                       mut with_tag: libc::c_int,
                                       mut data: *const uint8_t,
                                       mut len: libc::c_int,
                                       mut is_write: libc::c_int)
 -> libc::c_int {
    let mut kk: kind_of_key = GPG_KEY_FOR_SIGNING;
    if tag as libc::c_int == 0xc1 as libc::c_int {
        kk = GPG_KEY_FOR_SIGNING
    } else if tag as libc::c_int == 0xc2 as libc::c_int {
        kk = GPG_KEY_FOR_DECRYPTION
    } else { kk = GPG_KEY_FOR_AUTHENTICATION }
    if is_write != 0 {
        let mut algo: libc::c_int = -(1 as libc::c_int);
        let mut algo_attr_pp: *mut *const uint8_t = get_algo_attr_pointer(kk);
        if len == 6 as libc::c_int {
            if memcmp(data as *const libc::c_void,
                      algorithm_attr_rsa2k.as_ptr().offset(1 as libc::c_int as
                                                               isize) as
                          *const libc::c_void,
                      6 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
                algo = 255 as libc::c_int
            } else if memcmp(data as *const libc::c_void,
                             algorithm_attr_rsa4k.as_ptr().offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                 as *const libc::c_void,
                             6 as libc::c_int as libc::c_ulong) ==
                          0 as libc::c_int {
                algo = 0 as libc::c_int
            } else if tag as libc::c_int != 0xc2 as libc::c_int &&
                          memcmp(data as *const libc::c_void,
                                 algorithm_attr_p256k1.as_ptr().offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                     as *const libc::c_void,
                                 6 as libc::c_int as libc::c_ulong) ==
                              0 as libc::c_int ||
                          tag as libc::c_int == 0xc2 as libc::c_int &&
                              *data.offset(0 as libc::c_int as isize) as
                                  libc::c_int == 0x12 as libc::c_int &&
                              memcmp(data.offset(1 as libc::c_int as isize) as
                                         *const libc::c_void,
                                     algorithm_attr_p256k1.as_ptr().offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                         as *const libc::c_void,
                                     5 as libc::c_int as libc::c_ulong) ==
                                  0 as libc::c_int {
                algo = 2 as libc::c_int
            }
        } else if len == 9 as libc::c_int &&
                      (tag as libc::c_int != 0xc2 as libc::c_int &&
                           memcmp(data as *const libc::c_void,
                                  algorithm_attr_p256r1.as_ptr().offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                      as *const libc::c_void,
                                  9 as libc::c_int as libc::c_ulong) ==
                               0 as libc::c_int ||
                           tag as libc::c_int == 0xc2 as libc::c_int &&
                               *data.offset(0 as libc::c_int as isize) as
                                   libc::c_int == 0x12 as libc::c_int &&
                               memcmp(data.offset(1 as libc::c_int as isize)
                                          as *const libc::c_void,
                                      algorithm_attr_p256r1.as_ptr().offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                          as *const libc::c_void,
                                      8 as libc::c_int as libc::c_ulong) ==
                                   0 as libc::c_int) {
            algo = 1 as libc::c_int
        } else if len == 10 as libc::c_int &&
                      memcmp(data as *const libc::c_void,
                             algorithm_attr_ed25519.as_ptr().offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                 as *const libc::c_void,
                             10 as libc::c_int as libc::c_ulong) ==
                          0 as libc::c_int {
            algo = 3 as libc::c_int
        } else if len == 11 as libc::c_int &&
                      memcmp(data as *const libc::c_void,
                             algorithm_attr_cv25519.as_ptr().offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                 as *const libc::c_void,
                             11 as libc::c_int as libc::c_ulong) ==
                          0 as libc::c_int {
            algo = 4 as libc::c_int
        }
        if algo < 0 as libc::c_int {
            return 0 as libc::c_int
        } else {
            if algo == 255 as libc::c_int && !(*algo_attr_pp).is_null() {
                gpg_do_delete_prvkey(kk, 1 as libc::c_int);
                flash_enum_clear(algo_attr_pp);
                if !(*algo_attr_pp).is_null() { return 0 as libc::c_int }
            } else if algo != 255 as libc::c_int && (*algo_attr_pp).is_null()
                          ||
                          *(*algo_attr_pp).offset(1 as libc::c_int as isize)
                              as libc::c_int != algo {
                gpg_do_delete_prvkey(kk, 1 as libc::c_int);
                *algo_attr_pp =
                    flash_enum_write(kk_to_nr(kk) as uint8_t,
                                     algo as uint8_t);
                if (*algo_attr_pp).is_null() { return 0 as libc::c_int }
            }
        }
        return 1 as libc::c_int
    } else {
        let mut algo_attr_do: *const uint8_t = get_algo_attr_data_object(kk);
        copy_do_1(tag, algo_attr_do, with_tag);
        /* Override the byte when GPG_DO_ALG_DEC.  */
        if tag as libc::c_int == 0xc2 as libc::c_int &&
               *algo_attr_do.offset(1 as libc::c_int as isize) as libc::c_int
                   == 0x13 as libc::c_int {
            *res_p.offset(-(*algo_attr_do.offset(0 as libc::c_int as isize) as
                                libc::c_int as isize)) =
                0x12 as libc::c_int as uint8_t
        }
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn proc_resetting_code(mut data: *const uint8_t,
                                         mut len: libc::c_int)
 -> libc::c_int {
    let mut old_ks: *const uint8_t = keystring_md_pw3.as_mut_ptr();
    let mut new_ks0: [uint8_t; 41] = [0; 41];
    let mut new_ks: *mut uint8_t =
        new_ks0.as_mut_ptr().offset((1 as libc::c_int + 8 as libc::c_int) as
                                        isize);
    let mut newpw: *const uint8_t = 0 as *const uint8_t;
    let mut newpw_len: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut salt: *mut uint8_t =
        new_ks0.as_mut_ptr().offset(1 as libc::c_int as isize);
    newpw_len = len;
    newpw = data;
    new_ks0[0 as libc::c_int as usize] = newpw_len as uint8_t;
    random_get_salt(salt);
    s2k(salt, 8 as libc::c_int as size_t, newpw, newpw_len as size_t, new_ks);
    r =
        gpg_change_keystring(admin_authorized as libc::c_int, old_ks,
                             2 as libc::c_int, new_ks);
    if r <= -(2 as libc::c_int) {
        return 0 as libc::c_int
    } else {
        if r < 0 as libc::c_int {
            return 0 as libc::c_int
        } else {
            if r == 0 as libc::c_int {
                return 0 as libc::c_int
            } else {
                gpg_do_write_simple(0x12 as libc::c_int as uint8_t,
                                    new_ks0.as_mut_ptr(),
                                    1 as libc::c_int + 8 as libc::c_int);
            }
        }
    }
    gpg_pw_reset_err_counter(1 as libc::c_int as uint8_t);
    return 1 as libc::c_int;
}
unsafe extern "C" fn encrypt(mut key: *const uint8_t, mut iv: *const uint8_t,
                             mut data: *mut uint8_t, mut len: libc::c_int) {
    let mut aes: aes_context =
        aes_context{nr: 0, rk: 0 as *mut uint32_t, buf: [0; 68],};
    let mut iv0: [uint8_t; 16] = [0; 16];
    let mut iv_offset: libc::c_uint = 0;
    aes_setkey_enc(&mut aes, key, 128 as libc::c_int as libc::c_uint);
    memcpy(iv0.as_mut_ptr() as *mut libc::c_void, iv as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    iv_offset = 0 as libc::c_int as libc::c_uint;
    aes_crypt_cfb128(&mut aes, 1 as libc::c_int, len as size_t,
                     &mut iv_offset as *mut libc::c_uint as *mut size_t,
                     iv0.as_mut_ptr(), data, data);
}
/* For three keys: Signing, Decryption, and Authentication */
#[no_mangle]
pub static mut kd: [key_data; 3] =
    [key_data{pubkey: 0 as *const uint8_t, data: [0; 512],}; 3];
unsafe extern "C" fn decrypt(mut key: *const uint8_t, mut iv: *const uint8_t,
                             mut data: *mut uint8_t, mut len: libc::c_int) {
    let mut aes: aes_context =
        aes_context{nr: 0,
                    rk: 0 as *mut uint32_t,
                    buf: [0; 68],}; /* This is setkey_enc, because of CFB.  */
    let mut iv0: [uint8_t; 16] = [0; 16];
    let mut iv_offset: libc::c_uint = 0;
    aes_setkey_enc(&mut aes, key, 128 as libc::c_int as libc::c_uint);
    memcpy(iv0.as_mut_ptr() as *mut libc::c_void, iv as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    iv_offset = 0 as libc::c_int as libc::c_uint;
    aes_crypt_cfb128(&mut aes, 0 as libc::c_int, len as size_t,
                     &mut iv_offset as *mut libc::c_uint as *mut size_t,
                     iv0.as_mut_ptr(), data, data);
}
unsafe extern "C" fn encrypt_dek(mut key_string: *const uint8_t,
                                 mut dek: *mut uint8_t) {
    let mut aes: aes_context =
        aes_context{nr: 0, rk: 0 as *mut uint32_t, buf: [0; 68],};
    aes_setkey_enc(&mut aes, key_string, 128 as libc::c_int as libc::c_uint);
    aes_crypt_ecb(&mut aes, 1 as libc::c_int, dek as *const libc::c_uchar,
                  dek);
}
unsafe extern "C" fn decrypt_dek(mut key_string: *const uint8_t,
                                 mut dek: *mut uint8_t) {
    let mut aes: aes_context =
        aes_context{nr: 0, rk: 0 as *mut uint32_t, buf: [0; 68],};
    aes_setkey_dec(&mut aes, key_string, 128 as libc::c_int as libc::c_uint);
    aes_crypt_ecb(&mut aes, 0 as libc::c_int, dek as *const libc::c_uchar,
                  dek);
}
unsafe extern "C" fn get_do_ptr_nr_for_kk(mut kk: kind_of_key) -> uint8_t {
    match kk as libc::c_uint {
        0 => { return 0xe as libc::c_int as uint8_t }
        1 => { return 0xf as libc::c_int as uint8_t }
        2 => { return 0x10 as libc::c_int as uint8_t }
        _ => { }
    }
    return 0xe as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn gpg_do_clear_prvkey(mut kk: kind_of_key) {
    memset(kd[kk as usize].data.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, 512 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn compute_key_data_checksum(mut kdi:
                                                   *mut key_data_internal,
                                               mut prvkey_len: libc::c_int,
                                               mut check_or_calc: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut d: [uint32_t; 4] =
        [0 as libc::c_int as uint32_t, 0 as libc::c_int as uint32_t,
         0 as libc::c_int as uint32_t, 0 as libc::c_int as uint32_t];
    let mut checksum: *mut uint32_t =
        &mut *(*kdi).data.as_mut_ptr().offset((prvkey_len as
                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<uint32_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as isize) as *mut uint32_t;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (prvkey_len as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<uint32_t>()
                                                   as libc::c_ulong) {
        d[(i & 3 as libc::c_int as libc::c_uint) as usize] ^=
            (*kdi).data[i as usize];
        i = i.wrapping_add(1)
    }
    if check_or_calc == 0 as libc::c_int {
        /* store */
        memcpy(checksum as *mut libc::c_void,
               d.as_mut_ptr() as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
        return 0 as libc::c_int
    } else {
        /* check */
        return (memcmp(checksum as *const libc::c_void,
                       d.as_mut_ptr() as *const libc::c_void,
                       16 as libc::c_int as libc::c_ulong) ==
                    0 as libc::c_int) as libc::c_int
    };
}
/*
 * Return  1 on success,
 *         0 if none,
 *        -1 on error,
 */
#[no_mangle]
pub unsafe extern "C" fn gpg_do_load_prvkey(mut kk: kind_of_key,
                                            mut who: libc::c_int,
                                            mut keystring: *const uint8_t)
 -> libc::c_int {
    let mut nr: uint8_t = get_do_ptr_nr_for_kk(kk);
    let mut prvkey_len: libc::c_int =
        gpg_get_algo_attr_key_size(kk, GPG_KEY_PRIVATE);
    let mut do_data: *const uint8_t = do_ptr[nr as usize];
    let mut key_addr: *const uint8_t = 0 as *const uint8_t;
    let mut dek: [uint8_t; 16] = [0; 16];
    let mut iv: *const uint8_t = 0 as *const uint8_t;
    let mut kdi: key_data_internal = key_data_internal{data: [0; 132],};
    if do_data.is_null() { return 0 as libc::c_int }
    key_addr = kd[kk as usize].pubkey.offset(-(prvkey_len as isize));
    memcpy(kdi.data.as_mut_ptr() as *mut libc::c_void,
           key_addr as *const libc::c_void, prvkey_len as libc::c_ulong);
    iv = &*do_data.offset(1 as libc::c_int as isize) as *const uint8_t;
    memcpy(&mut *kdi.data.as_mut_ptr().offset((prvkey_len as
                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<uint32_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as isize) as *mut uint32_t
               as *mut libc::c_void,
           iv.offset(16 as libc::c_int as isize) as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    memcpy(dek.as_mut_ptr() as *mut libc::c_void,
           iv.offset((16 as libc::c_int * (who + 1 as libc::c_int)) as isize)
               as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    decrypt_dek(keystring, dek.as_mut_ptr());
    decrypt(dek.as_mut_ptr(), iv,
            &mut kdi as *mut key_data_internal as *mut uint8_t,
            prvkey_len + 16 as libc::c_int);
    memset(dek.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           16 as libc::c_int as libc::c_ulong);
    if compute_key_data_checksum(&mut kdi, prvkey_len, 1 as libc::c_int) == 0
       {
        return -(1 as libc::c_int)
    }
    memcpy(kd[kk as usize].data.as_mut_ptr() as *mut libc::c_void,
           kdi.data.as_mut_ptr() as *const libc::c_void,
           prvkey_len as libc::c_ulong);
    return 1 as libc::c_int;
}
static mut num_prv_keys: int8_t = 0;
unsafe extern "C" fn gpg_do_delete_prvkey(mut kk: kind_of_key,
                                          mut clean_page_full: libc::c_int) {
    let mut nr: uint8_t = get_do_ptr_nr_for_kk(kk);
    let mut do_data: *const uint8_t = do_ptr[nr as usize];
    let mut key_addr: *mut uint8_t = 0 as *mut uint8_t;
    let mut prvkey_len: libc::c_int =
        gpg_get_algo_attr_key_size(kk, GPG_KEY_PRIVATE);
    let mut key_size: libc::c_int =
        gpg_get_algo_attr_key_size(kk, GPG_KEY_STORAGE);
    if do_data.is_null() {
        if clean_page_full != 0 { flash_key_release_page(kk); }
        return
    }
    do_ptr[nr as usize] = 0 as *const uint8_t;
    flash_do_release(do_data);
    key_addr =
        (kd[kk as usize].pubkey as
             *mut uint8_t).offset(-(prvkey_len as isize));
    kd[kk as usize].pubkey = 0 as *const uint8_t;
    if clean_page_full != 0 {
        flash_key_release_page(kk);
    } else { flash_key_release(key_addr, key_size); }
    if admin_authorized as libc::c_int == 3 as libc::c_int &&
           kk as libc::c_uint ==
               GPG_KEY_FOR_SIGNING as libc::c_int as libc::c_uint {
        /* Recover admin keystring DO.  */
        let mut ks_pw3: *const uint8_t =
            gpg_do_read_simple(0x13 as libc::c_int as uint8_t);
        if !ks_pw3.is_null() {
            let mut ks0: [uint8_t; 41] = [0; 41];
            ks0[0 as libc::c_int as usize] =
                (*ks_pw3.offset(0 as libc::c_int as isize) as libc::c_int |
                     0x80 as libc::c_int) as uint8_t;
            memcpy(ks0.as_mut_ptr().offset(1 as libc::c_int as isize) as
                       *mut libc::c_void,
                   ks_pw3.offset(1 as libc::c_int as isize) as
                       *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            memcpy(ks0.as_mut_ptr().offset((1 as libc::c_int +
                                                8 as libc::c_int) as isize) as
                       *mut libc::c_void,
                   keystring_md_pw3.as_mut_ptr() as *const libc::c_void,
                   32 as libc::c_int as libc::c_ulong);
            gpg_do_write_simple(0x13 as libc::c_int as uint8_t,
                                ks0.as_mut_ptr(),
                                1 as libc::c_int + 8 as libc::c_int +
                                    32 as libc::c_int);
        }
    }
    num_prv_keys -= 1;
    if num_prv_keys as libc::c_int == 0 as libc::c_int {
        /* Delete PW1 and RC if any.  */
        gpg_do_write_simple(0x11 as libc::c_int as uint8_t,
                            0 as *const uint8_t, 0 as libc::c_int);
        gpg_do_write_simple(0x12 as libc::c_int as uint8_t,
                            0 as *const uint8_t, 0 as libc::c_int);
        ac_reset_pso_cds();
        ac_reset_other();
        if admin_authorized as libc::c_int == 1 as libc::c_int {
            ac_reset_admin();
        }
    };
}
unsafe extern "C" fn gpg_do_write_prvkey(mut kk: kind_of_key,
                                         mut key_data: *const uint8_t,
                                         mut prvkey_len: libc::c_int,
                                         mut keystring_admin: *const uint8_t,
                                         mut pubkey: *const uint8_t)
 -> libc::c_int {
    let mut nr: uint8_t = get_do_ptr_nr_for_kk(kk);
    let mut attr: libc::c_int = gpg_get_algo_attr(kk);
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut r: libc::c_int = 0;
    let mut pd: *mut prvkey_data = 0 as *mut prvkey_data;
    let mut key_addr: *mut uint8_t = 0 as *mut uint8_t;
    let mut dek: *const uint8_t = 0 as *const uint8_t;
    let mut iv: *const uint8_t = 0 as *const uint8_t;
    let mut kdi: key_data_internal = key_data_internal{data: [0; 132],};
    let mut pubkey_allocated_here: *mut uint8_t = 0 as *mut uint8_t;
    let mut pubkey_len: libc::c_int = 0;
    let mut ks: [uint8_t; 32] = [0; 32];
    let mut kk0: kind_of_key = GPG_KEY_FOR_SIGNING;
    /* Delete it first, if any.  */
    gpg_do_delete_prvkey(kk, 0 as libc::c_int);
    pd =
        gnuk_malloc(::std::mem::size_of::<prvkey_data>() as libc::c_ulong) as
            *mut prvkey_data;
    if pd.is_null() { return -(1 as libc::c_int) }
    if attr == 1 as libc::c_int || attr == 2 as libc::c_int {
        pubkey_len = prvkey_len * 2 as libc::c_int;
        if prvkey_len != 32 as libc::c_int { return -(1 as libc::c_int) }
    } else if attr == 3 as libc::c_int {
        pubkey_len = prvkey_len / 2 as libc::c_int;
        if prvkey_len != 64 as libc::c_int { return -(1 as libc::c_int) }
    } else if attr == 4 as libc::c_int {
        pubkey_len = prvkey_len;
        if prvkey_len != 32 as libc::c_int { return -(1 as libc::c_int) }
    } else {
        /* RSA */
        let mut key_size: libc::c_int =
            gpg_get_algo_attr_key_size(kk, GPG_KEY_STORAGE);
        pubkey_len = prvkey_len;
        if prvkey_len + pubkey_len != key_size { return -(1 as libc::c_int) }
    }
    if pubkey.is_null() {
        if attr == 2 as libc::c_int {
            pubkey_allocated_here = ecc_compute_public_p256k1(key_data)
        } else if attr == 1 as libc::c_int {
            pubkey_allocated_here = ecc_compute_public_p256r1(key_data)
        } else if attr == 3 as libc::c_int {
            pubkey_allocated_here = eddsa_compute_public_25519(key_data)
        } else if attr == 4 as libc::c_int {
            pubkey_allocated_here = ecdh_compute_public_25519(key_data)
        } else {
            /* RSA */
            pubkey_allocated_here = modulus_calc(key_data, prvkey_len)
        } /* 32-byte random bytes */
        if pubkey_allocated_here.is_null() {
            gnuk_free(pd as *mut libc::c_void);
            return -(1 as libc::c_int)
        }
    }
    key_addr = flash_key_alloc(kk);
    if key_addr.is_null() {
        if !pubkey_allocated_here.is_null() {
            memset(pubkey_allocated_here as *mut libc::c_void,
                   0 as libc::c_int, pubkey_len as libc::c_ulong);
            gnuk_free(pubkey_allocated_here as *mut libc::c_void);
        }
        gnuk_free(pd as *mut libc::c_void);
        return -(1 as libc::c_int)
    }
    kd[kk as usize].pubkey = key_addr.offset(prvkey_len as isize);
    num_prv_keys += 1;
    memcpy(kdi.data.as_mut_ptr() as *mut libc::c_void,
           key_data as *const libc::c_void, prvkey_len as libc::c_ulong);
    memset((kdi.data.as_mut_ptr() as *mut uint8_t).offset(prvkey_len as isize)
               as *mut libc::c_void, 0 as libc::c_int,
           (512 as libc::c_int - prvkey_len) as libc::c_ulong);
    compute_key_data_checksum(&mut kdi, prvkey_len, 0 as libc::c_int);
    dek = random_bytes_get();
    iv = dek.offset(16 as libc::c_int as isize);
    memcpy((*pd).dek_encrypted_1.as_mut_ptr() as *mut libc::c_void,
           dek as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    memcpy((*pd).dek_encrypted_2.as_mut_ptr() as *mut libc::c_void,
           dek as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    memcpy((*pd).dek_encrypted_3.as_mut_ptr() as *mut libc::c_void,
           dek as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    s2k(0 as *const libc::c_uchar, 0 as libc::c_int as size_t,
        b"123456\x00" as *const u8 as *const libc::c_char as *const uint8_t,
        strlen(b"123456\x00" as *const u8 as *const libc::c_char),
        ks.as_mut_ptr());
    /* Handle existing keys and keystring DOs.  */
    gpg_do_write_simple(0x11 as libc::c_int as uint8_t, 0 as *const uint8_t,
                        0 as libc::c_int); /* No private key */
    gpg_do_write_simple(0x12 as libc::c_int as uint8_t, 0 as *const uint8_t,
                        0 as libc::c_int);
    kk0 = GPG_KEY_FOR_SIGNING;
    while kk0 as libc::c_uint <=
              GPG_KEY_FOR_AUTHENTICATION as libc::c_int as libc::c_uint {
        if kk0 as libc::c_uint != kk as libc::c_uint {
            gpg_do_chks_prvkey(kk0, admin_authorized as libc::c_int,
                               keystring_md_pw3.as_mut_ptr(),
                               1 as libc::c_int, ks.as_mut_ptr());
            gpg_do_chks_prvkey(kk0, 2 as libc::c_int, 0 as *const uint8_t,
                               0 as libc::c_int, 0 as *const uint8_t);
        }
        kk0 += 1
    }
    encrypt(dek, iv, &mut kdi as *mut key_data_internal as *mut uint8_t,
            prvkey_len + 16 as libc::c_int);
    r =
        flash_key_write(key_addr, kdi.data.as_mut_ptr() as *const uint8_t,
                        prvkey_len,
                        if !pubkey_allocated_here.is_null() {
                            pubkey_allocated_here
                        } else { pubkey }, pubkey_len);
    if !pubkey_allocated_here.is_null() {
        memset(pubkey_allocated_here as *mut libc::c_void, 0 as libc::c_int,
               pubkey_len as libc::c_ulong);
        gnuk_free(pubkey_allocated_here as *mut libc::c_void);
    }
    if r < 0 as libc::c_int {
        random_bytes_free(dek);
        memset(pd as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<prvkey_data>() as libc::c_ulong);
        gnuk_free(pd as *mut libc::c_void);
        return r
    }
    memcpy((*pd).iv.as_mut_ptr() as *mut libc::c_void,
           iv as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    memcpy((*pd).checksum_encrypted.as_mut_ptr() as *mut libc::c_void,
           &mut *kdi.data.as_mut_ptr().offset((prvkey_len as
                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<uint32_t>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as isize) as *mut uint32_t
               as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    encrypt_dek(ks.as_mut_ptr(), (*pd).dek_encrypted_1.as_mut_ptr());
    memset((*pd).dek_encrypted_2.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
    if !keystring_admin.is_null() {
        encrypt_dek(keystring_admin, (*pd).dek_encrypted_3.as_mut_ptr());
    } else {
        memset((*pd).dek_encrypted_3.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
    }
    p =
        flash_do_write(nr, pd as *const uint8_t,
                       ::std::mem::size_of::<prvkey_data>() as libc::c_ulong
                           as libc::c_int);
    do_ptr[nr as usize] = p;
    random_bytes_free(dek);
    memset(pd as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<prvkey_data>() as libc::c_ulong);
    gnuk_free(pd as *mut libc::c_void);
    if p.is_null() { return -(1 as libc::c_int) }
    if !keystring_admin.is_null() &&
           kk as libc::c_uint ==
               GPG_KEY_FOR_SIGNING as libc::c_int as libc::c_uint {
        let mut ks_admin: *const uint8_t =
            gpg_do_read_simple(0x13 as libc::c_int as uint8_t);
        let mut ks_info: [uint8_t; 9] = [0; 9];
        if !ks_admin.is_null() &&
               *ks_admin.offset(0 as libc::c_int as isize) as libc::c_int &
                   0x80 as libc::c_int != 0 {
            ks_info[0 as libc::c_int as usize] =
                (*ks_admin.offset(0 as libc::c_int as isize) as libc::c_int &
                     0x7f as libc::c_int) as uint8_t;
            memcpy(ks_info.as_mut_ptr().offset(1 as libc::c_int as isize) as
                       *mut libc::c_void,
                   ks_admin.offset(1 as libc::c_int as isize) as
                       *const libc::c_void,
                   8 as libc::c_int as libc::c_ulong);
            gpg_do_write_simple(0x13 as libc::c_int as uint8_t,
                                ks_info.as_mut_ptr(),
                                1 as libc::c_int + 8 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gpg_do_chks_prvkey(mut kk: kind_of_key,
                                            mut who_old: libc::c_int,
                                            mut old_ks: *const uint8_t,
                                            mut who_new: libc::c_int,
                                            mut new_ks: *const uint8_t)
 -> libc::c_int {
    let mut nr: uint8_t = get_do_ptr_nr_for_kk(kk);
    let mut do_data: *const uint8_t = do_ptr[nr as usize];
    let mut dek: [uint8_t; 16] = [0; 16];
    let mut pd: *mut prvkey_data = 0 as *mut prvkey_data;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut dek_p: *mut uint8_t = 0 as *mut uint8_t;
    let mut update_needed: libc::c_int = 0 as libc::c_int;
    if do_data.is_null() { return 0 as libc::c_int }
    pd =
        gnuk_malloc(::std::mem::size_of::<prvkey_data>() as libc::c_ulong) as
            *mut prvkey_data;
    if pd.is_null() { return -(1 as libc::c_int) }
    memcpy(pd as *mut libc::c_void,
           &*do_data.offset(1 as libc::c_int as isize) as *const uint8_t as
               *const libc::c_void,
           ::std::mem::size_of::<prvkey_data>() as libc::c_ulong);
    dek_p =
        (pd as
             *mut uint8_t).offset(16 as libc::c_int as
                                      isize).offset((16 as libc::c_int *
                                                         who_old) as isize);
    memcpy(dek.as_mut_ptr() as *mut libc::c_void,
           dek_p as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    if who_new == 0 as libc::c_int {
        /* Remove */
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if *dek_p.offset(i as isize) as libc::c_int != 0 as libc::c_int {
                update_needed = 1 as libc::c_int;
                *dek_p.offset(i as isize) = 0 as libc::c_int as uint8_t
            }
            i += 1
        }
    } else {
        decrypt_dek(old_ks, dek.as_mut_ptr());
        encrypt_dek(new_ks, dek.as_mut_ptr());
        dek_p =
            dek_p.offset((16 as libc::c_int * (who_new - who_old)) as isize);
        if memcmp(dek_p as *const libc::c_void,
                  dek.as_mut_ptr() as *const libc::c_void,
                  16 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
            memcpy(dek_p as *mut libc::c_void,
                   dek.as_mut_ptr() as *const libc::c_void,
                   16 as libc::c_int as libc::c_ulong);
            update_needed = 1 as libc::c_int
        }
    }
    if update_needed != 0 {
        flash_do_release(do_data);
        do_ptr[nr as usize] = 0 as *const uint8_t;
        p =
            flash_do_write(nr, pd as *const uint8_t,
                           ::std::mem::size_of::<prvkey_data>() as
                               libc::c_ulong as libc::c_int);
        do_ptr[nr as usize] = p
    }
    memset(pd as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<prvkey_data>() as libc::c_ulong);
    gnuk_free(pd as *mut libc::c_void);
    if update_needed != 0 && p.is_null() { return -(1 as libc::c_int) }
    return 1 as libc::c_int;
}
unsafe extern "C" fn kkb_to_kk(mut kk_byte: uint8_t) -> kind_of_key {
    let mut kk: kind_of_key = GPG_KEY_FOR_SIGNING;
    if kk_byte as libc::c_int == 0xb6 as libc::c_int {
        kk = GPG_KEY_FOR_SIGNING
    } else if kk_byte as libc::c_int == 0xb8 as libc::c_int {
        kk = GPG_KEY_FOR_DECRYPTION
    } else {
        /* 0xa4 */
        kk = GPG_KEY_FOR_AUTHENTICATION
    }
    return kk;
}
/*
 * RSA-2048:
 * 4d, xx, xx, xx:    Extended Header List
 *   b6 00 (SIG) / b8 00 (DEC) / a4 00 (AUT)
 *   7f48, xx: cardholder private key template
 *       91 L<E>:        91=tag of E, L<E>: length of E
 *       92 Lh<P> Ll<P>: 92=tag of P, L<P>: length of P
 *       93 Lh<Q> Ll<Q>: 93=tag of Q, L<Q>: length of Q
 *   5f48, xx xx xx: cardholder private key
 *       <E: 4-byte>, <P: 128-byte>, <Q: 128-byte>
 *
 * RSA-4096:
 * 4d, 82, 02, 18:    Extended Header List
 *   b6 00 (SIG) / b8 00 (DEC) / a4 00 (AUT)
 *   7f48, 0a: cardholder private key template
 *       91 L<E>:        91=tag of E, L<E>: length of E
 *       92 82 Lh<P> Ll<P>: 92=tag of P, L<P>: length of P
 *       93 82 Lh<Q> Ll<Q>: 93=tag of Q, L<Q>: length of Q
 *   5f48, 82 02 04: cardholder private key
 *       <E: 4-byte>, <P: 256-byte>, <Q: 256-byte>
 *
 * ECDSA / ECDH / EdDSA:
 * 4d, 2a:    Extended Header List
 *   b6 00 (SIG) / b8 00 (DEC) / a4 00 (AUT)
 *   7f48, 02: cardholder private key template
 *       9x LEN: 9x=tag of private key d,  LEN=length of d
 *   5f48, 20: cardholder private key
 * <d: 32-byte>
 */
unsafe extern "C" fn proc_key_import(mut data: *const uint8_t,
                                     mut len: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = -(1 as libc::c_int);
    let mut kk: kind_of_key = GPG_KEY_FOR_SIGNING;
    let mut keystring_admin: *const uint8_t = 0 as *const uint8_t;
    let mut attr: libc::c_int = 0;
    let mut p: *const uint8_t = data;
    if admin_authorized as libc::c_int == 3 as libc::c_int {
        keystring_admin = keystring_md_pw3.as_mut_ptr()
    } else { keystring_admin = 0 as *const uint8_t }
    let fresh27 = p;
    p = p.offset(1);
    if *fresh27 as libc::c_int != 0x4d as libc::c_int {
        return 0 as libc::c_int
    }
    /* length field */
    if *p as libc::c_int == 0x82 as libc::c_int {
        p = p.offset(3 as libc::c_int as isize)
    } else if *p as libc::c_int == 0x81 as libc::c_int {
        p = p.offset(2 as libc::c_int as isize)
    } else { p = p.offset(1 as libc::c_int as isize) }
    kk = kkb_to_kk(*p);
    if kk as libc::c_uint ==
           GPG_KEY_FOR_SIGNING as libc::c_int as libc::c_uint {
        ac_reset_pso_cds();
        gpg_reset_digital_signature_counter();
    } else { ac_reset_other(); }
    attr = gpg_get_algo_attr(kk);
    if len <= 12 as libc::c_int &&
           (attr == 1 as libc::c_int || attr == 2 as libc::c_int ||
                attr == 3 as libc::c_int || attr == 4 as libc::c_int) ||
           len <= 22 as libc::c_int && attr == 255 as libc::c_int ||
           len <= 24 as libc::c_int && attr == 0 as libc::c_int {
        /* Deletion of the key */
        gpg_do_delete_prvkey(kk, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    if attr == 255 as libc::c_int {
        /* It should starts with 00 01 00 01 (E), skiping E (4-byte) */
        r =
            gpg_do_write_prvkey(kk, &*data.offset(26 as libc::c_int as isize),
                                len - 26 as libc::c_int, keystring_admin,
                                0 as *const uint8_t)
    } else if attr == 0 as libc::c_int {
        /* It should starts with 00 01 00 01 (E), skiping E (4-byte) */
        r =
            gpg_do_write_prvkey(kk, &*data.offset(28 as libc::c_int as isize),
                                len - 28 as libc::c_int, keystring_admin,
                                0 as *const uint8_t)
    } else if attr == 1 as libc::c_int || attr == 2 as libc::c_int {
        r =
            gpg_do_write_prvkey(kk, &*data.offset(12 as libc::c_int as isize),
                                len - 12 as libc::c_int, keystring_admin,
                                0 as *const uint8_t)
    } else if attr == 3 as libc::c_int {
        let mut hash: [uint8_t; 64] = [0; 64]; /* Error.  */
        if len - 12 as libc::c_int != 32 as libc::c_int {
            return 0 as libc::c_int
        } /* Error.  */
        sha512(&*data.offset(12 as libc::c_int as isize),
               32 as libc::c_int as libc::c_uint, hash.as_mut_ptr());
        hash[0 as libc::c_int as usize] =
            (hash[0 as libc::c_int as usize] as libc::c_int &
                 248 as libc::c_int) as uint8_t;
        hash[31 as libc::c_int as usize] =
            (hash[31 as libc::c_int as usize] as libc::c_int &
                 127 as libc::c_int) as uint8_t;
        hash[31 as libc::c_int as usize] =
            (hash[31 as libc::c_int as usize] as libc::c_int |
                 64 as libc::c_int) as uint8_t;
        r =
            gpg_do_write_prvkey(kk, hash.as_mut_ptr(), 64 as libc::c_int,
                                keystring_admin, 0 as *const uint8_t)
    } else if attr == 4 as libc::c_int {
        let mut priv_0: [uint8_t; 32] = [0; 32];
        let mut i: libc::c_int = 0;
        if len - 12 as libc::c_int != 32 as libc::c_int {
            return 0 as libc::c_int
        }
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            priv_0[(31 as libc::c_int - i) as usize] =
                *data.offset((12 as libc::c_int + i) as isize);
            i += 1
        }
        r =
            gpg_do_write_prvkey(kk, priv_0.as_mut_ptr(), 32 as libc::c_int,
                                keystring_admin, 0 as *const uint8_t)
    }
    if r < 0 as libc::c_int {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
static mut cmp_ch_data: [uint16_t; 4] =
    [3 as libc::c_int as uint16_t, 0x5b as libc::c_int as uint16_t,
     0x5f2d as libc::c_int as uint16_t, 0x5f35 as libc::c_int as uint16_t];
static mut cmp_app_data: [uint16_t; 4] =
    [3 as libc::c_int as uint16_t, 0x4f as libc::c_int as uint16_t,
     0x5f52 as libc::c_int as uint16_t, 0x73 as libc::c_int as uint16_t];
static mut cmp_discretionary: [uint16_t; 9] =
    [8 as libc::c_int as uint16_t, 0xc0 as libc::c_int as uint16_t,
     0xc1 as libc::c_int as uint16_t, 0xc2 as libc::c_int as uint16_t,
     0xc3 as libc::c_int as uint16_t, 0xc4 as libc::c_int as uint16_t,
     0xc5 as libc::c_int as uint16_t, 0xc6 as libc::c_int as uint16_t,
     0xcd as libc::c_int as uint16_t];
static mut cmp_ss_temp: [uint16_t; 2] =
    [1 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t];
// Initialized in run_static_initializers
static mut gpg_do_table: [do_table_entry; 31] =
    [do_table_entry{tag: 0,
                    do_type: DO_FIXED,
                    ac_read: 0,
                    ac_write: 0,
                    obj: 0 as *const libc::c_void,}; 31];
/*
 * Reading data from Flash ROM, initialize DO_PTR, PW_ERR_COUNTERS, etc.
 */
#[no_mangle]
pub unsafe extern "C" fn gpg_data_scan(mut p_start: *const uint8_t) {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut i: libc::c_int = 0;
    let mut dsc_h14_p: *const uint8_t = 0 as *const uint8_t;
    let mut dsc_l10_p: *const uint8_t = 0 as *const uint8_t;
    let mut dsc_h14: libc::c_int = 0;
    let mut dsc_l10: libc::c_int = 0;
    dsc_l10_p = 0 as *const uint8_t;
    dsc_h14_p = dsc_l10_p;
    pw1_lifetime_p = 0 as *const uint8_t;
    pw_err_counter_p[0 as libc::c_int as usize] = 0 as *const uint8_t;
    pw_err_counter_p[1 as libc::c_int as usize] = 0 as *const uint8_t;
    pw_err_counter_p[2 as libc::c_int as usize] = 0 as *const uint8_t;
    algo_attr_aut_p = 0 as *const uint8_t;
    algo_attr_dec_p = algo_attr_aut_p;
    algo_attr_sig_p = algo_attr_dec_p;
    /* Traverse DO, counters, etc. in DATA pool */
    p = p_start; /* Skip released word */
    while *p as libc::c_int != 0xff as libc::c_int {
        let fresh28 = p;
        p = p.offset(1);
        let mut nr: uint8_t = *fresh28;
        let mut second_byte: uint8_t = *p;
        if nr as libc::c_int == 0 as libc::c_int &&
               second_byte as libc::c_int == 0 as libc::c_int {
            p = p.offset(1)
        } else if (nr as libc::c_int) < 0x80 as libc::c_int {
            /* It's Data Object */
            do_ptr[nr as usize] = p; /* second_byte has length */
            p =
                p.offset((second_byte as libc::c_int + 1 as libc::c_int) as
                             isize);
            if p as uint32_t & 1 as libc::c_int as libc::c_uint != 0 {
                p = p.offset(1)
            }
        } else if nr as libc::c_int >= 0x80 as libc::c_int &&
                      nr as libc::c_int <= 0xbf as libc::c_int {
            /* Encoded data of Digital Signature Counter: upper 14-bit */
            dsc_h14_p = p.offset(-(1 as libc::c_int as isize));
            p = p.offset(1)
        } else if nr as libc::c_int >= 0xc0 as libc::c_int &&
                      nr as libc::c_int <= 0xc3 as libc::c_int {
            /* Encoded data of Digital Signature Counter: lower 10-bit */
            dsc_l10_p = p.offset(-(1 as libc::c_int as isize));
            p = p.offset(1)
        } else {
            match nr as libc::c_int {
                240 => {
                    pw1_lifetime_p = p.offset(-(1 as libc::c_int as isize));
                    p = p.offset(1)
                }
                241 => {
                    algo_attr_sig_p = p.offset(-(1 as libc::c_int as isize));
                    p = p.offset(1)
                }
                242 => {
                    algo_attr_dec_p = p.offset(-(1 as libc::c_int as isize));
                    p = p.offset(1)
                }
                243 => {
                    algo_attr_aut_p = p.offset(-(1 as libc::c_int as isize));
                    p = p.offset(1)
                }
                254 => {
                    p = p.offset(1);
                    if second_byte as libc::c_int <= 2 as libc::c_int {
                        pw_err_counter_p[second_byte as usize] = p
                    }
                    p = p.offset(2 as libc::c_int as isize)
                }
                _ => {
                    /* Something going wrong.  ignore this word. */
                    p = p.offset(1)
                }
            }
        }
    }
    flash_set_data_pool_last(p);
    num_prv_keys = 0 as libc::c_int as int8_t;
    if !do_ptr[0xe as libc::c_int as usize].is_null() { num_prv_keys += 1 }
    if !do_ptr[0xf as libc::c_int as usize].is_null() { num_prv_keys += 1 }
    if !do_ptr[0x10 as libc::c_int as usize].is_null() { num_prv_keys += 1 }
    data_objects_number_of_bytes = 0 as libc::c_int as uint16_t;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        if !do_ptr[i as usize].is_null() {
            data_objects_number_of_bytes =
                (data_objects_number_of_bytes as libc::c_int +
                     *do_ptr[i as usize] as libc::c_int) as uint16_t
        }
        i += 1
    }
    if dsc_l10_p.is_null() {
        dsc_l10 = 0 as libc::c_int
    } else {
        dsc_l10 =
            (*dsc_l10_p as libc::c_int - 0xc0 as libc::c_int) <<
                8 as libc::c_int |
                *dsc_l10_p.offset(1 as libc::c_int as isize) as libc::c_int
    }
    if dsc_h14_p.is_null() {
        dsc_h14 = 0 as libc::c_int
    } else {
        dsc_h14 =
            (*dsc_h14_p as libc::c_int - 0x80 as libc::c_int) <<
                8 as libc::c_int |
                *dsc_h14_p.offset(1 as libc::c_int as isize) as libc::c_int;
        if !dsc_l10_p.is_null() {
            if dsc_l10_p < dsc_h14_p {
                /* Possibly, power off during writing dsc_l10 */
                dsc_l10 = 0 as libc::c_int
            }
        }
    }
    digital_signature_counter =
        (dsc_h14 << 10 as libc::c_int | dsc_l10) as uint32_t;
}
/*
 * Write all data to newly allocated Flash ROM page (from P_START),
 * updating PW1_LIFETIME_P, PW_ERR_COUNTER_P, and DO_PTR.
 * Called by flash_copying_gc.
 */
#[no_mangle]
pub unsafe extern "C" fn gpg_data_copy(mut p_start: *const uint8_t) {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    p =
        gpg_write_digital_signature_counter(p_start,
                                            digital_signature_counter);
    if !pw1_lifetime_p.is_null() {
        flash_bool_write_internal(p, 0xf0 as libc::c_int);
        pw1_lifetime_p = p;
        p = p.offset(2 as libc::c_int as isize)
    }
    if !algo_attr_sig_p.is_null() {
        flash_enum_write_internal(p, 0xf1 as libc::c_int,
                                  *algo_attr_sig_p.offset(1 as libc::c_int as
                                                              isize));
        algo_attr_sig_p = p;
        p = p.offset(2 as libc::c_int as isize)
    }
    if !algo_attr_dec_p.is_null() {
        flash_enum_write_internal(p, 0xf2 as libc::c_int,
                                  *algo_attr_dec_p.offset(1 as libc::c_int as
                                                              isize));
        algo_attr_dec_p = p;
        p = p.offset(2 as libc::c_int as isize)
    }
    if !algo_attr_aut_p.is_null() {
        flash_enum_write_internal(p, 0xf3 as libc::c_int,
                                  *algo_attr_aut_p.offset(1 as libc::c_int as
                                                              isize));
        algo_attr_aut_p = p;
        p = p.offset(2 as libc::c_int as isize)
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        v = flash_cnt123_get_value(pw_err_counter_p[i as usize]);
        if v != 0 as libc::c_int {
            flash_cnt123_write_internal(p, i, v);
            pw_err_counter_p[i as usize] =
                p.offset(2 as libc::c_int as isize);
            p = p.offset(4 as libc::c_int as isize)
        }
        i += 1
    }
    data_objects_number_of_bytes = 0 as libc::c_int as uint16_t;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        if !do_ptr[i as usize].is_null() {
            let mut do_data: *const uint8_t = do_ptr[i as usize];
            let mut len: libc::c_int =
                *do_data.offset(0 as libc::c_int as isize) as libc::c_int;
            flash_do_write_internal(p, i,
                                    &*do_data.offset(1 as libc::c_int as
                                                         isize), len);
            do_ptr[i as usize] = p.offset(1 as libc::c_int as isize);
            p =
                p.offset((2 as libc::c_int +
                              (len + 1 as libc::c_int & !(1 as libc::c_int)))
                             as isize);
            data_objects_number_of_bytes =
                (data_objects_number_of_bytes as libc::c_int + len) as
                    uint16_t
        }
        i += 1
    }
    flash_set_data_pool_last(p);
}
unsafe extern "C" fn get_do_entry(mut tag: uint16_t)
 -> *const do_table_entry {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[do_table_entry; 31]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<do_table_entry>()
                                                   as libc::c_ulong) as
                  libc::c_int {
        if gpg_do_table[i as usize].tag as libc::c_int == tag as libc::c_int {
            return &*gpg_do_table.as_ptr().offset(i as isize) as
                       *const do_table_entry
        }
        i += 1
    }
    return 0 as *const do_table_entry;
}
unsafe extern "C" fn copy_do_1(mut tag: uint16_t, mut do_data: *const uint8_t,
                               mut with_tag: libc::c_int) {
    let mut len: libc::c_int = 0;
    if with_tag != 0 {
        copy_tag(tag);
        if *do_data.offset(0 as libc::c_int as isize) as libc::c_int >=
               128 as libc::c_int {
            let fresh29 = res_p;
            res_p = res_p.offset(1);
            *fresh29 = 0x81 as libc::c_int as uint8_t
        }
        len =
            *do_data.offset(0 as libc::c_int as isize) as libc::c_int +
                1 as libc::c_int
    } else {
        len = *do_data.offset(0 as libc::c_int as isize) as libc::c_int;
        do_data = do_data.offset(1)
    }
    memcpy(res_p as *mut libc::c_void, do_data as *const libc::c_void,
           len as libc::c_ulong);
    res_p = res_p.offset(len as isize);
}
unsafe extern "C" fn copy_do(mut do_p: *const do_table_entry,
                             mut with_tag: libc::c_int) -> libc::c_int {
    if do_p.is_null() { return 0 as libc::c_int }
    if ac_check_status((*do_p).ac_read) == 0 { return -(1 as libc::c_int) }
    match (*do_p).do_type as libc::c_uint {
        0 => {
            let mut do_data: *const uint8_t = (*do_p).obj as *const uint8_t;
            if do_data.is_null() {
                return 0 as libc::c_int
            } else { copy_do_1((*do_p).tag, do_data, with_tag); }
        }
        1 => {
            let mut do_data_0: *const uint8_t =
                *((*do_p).obj as *mut *const uint8_t);
            if do_data_0.is_null() {
                return 0 as libc::c_int
            } else { copy_do_1((*do_p).tag, do_data_0, with_tag); }
        }
        2 => {
            let mut i: libc::c_int = 0;
            let mut cmp_data: *const uint16_t =
                (*do_p).obj as *const uint16_t;
            let mut num_components: libc::c_int =
                *cmp_data.offset(0 as libc::c_int as isize) as libc::c_int;
            let mut len_p: *mut uint8_t = 0 as *mut uint8_t;
            if with_tag != 0 {
                copy_tag((*do_p).tag);
                /* for now */
                let fresh30 = res_p; /* Assume it's less than 256 */
                res_p = res_p.offset(1);
                *fresh30 = 0x81 as libc::c_int as uint8_t;
                len_p = res_p;
                let fresh31 = res_p;
                res_p = res_p.offset(1);
                *fresh31 = 0 as libc::c_int as uint8_t
            }
            i = 0 as libc::c_int;
            while i < num_components {
                let mut tag0: uint16_t = 0;
                let mut do0_p: *const do_table_entry =
                    0 as *const do_table_entry;
                tag0 = *cmp_data.offset((i + 1 as libc::c_int) as isize);
                do0_p = get_do_entry(tag0);
                if copy_do(do0_p, 1 as libc::c_int) < 0 as libc::c_int {
                    return -(1 as libc::c_int)
                }
                i += 1
            }
            if !len_p.is_null() {
                *len_p =
                    (res_p.wrapping_offset_from(len_p) as libc::c_long -
                         1 as libc::c_int as libc::c_long) as uint8_t
            }
        }
        3 => {
            let mut do_func:
                    Option<unsafe extern "C" fn(_: uint16_t, _: libc::c_int)
                               -> libc::c_int> =
                ::std::mem::transmute::<*const libc::c_void,
                                        Option<unsafe extern "C" fn(_:
                                                                        uint16_t,
                                                                    _:
                                                                        libc::c_int)
                                                   ->
                                                       libc::c_int>>((*do_p).obj);
            return do_func.expect("non-null function pointer")((*do_p).tag,
                                                               with_tag)
        }
        5 => {
            let mut rw_func:
                    Option<unsafe extern "C" fn(_: uint16_t, _: libc::c_int,
                                                _: *const uint8_t,
                                                _: libc::c_int,
                                                _: libc::c_int)
                               -> libc::c_int> =
                ::std::mem::transmute::<*const libc::c_void,
                                        Option<unsafe extern "C" fn(_:
                                                                        uint16_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        *const uint8_t,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        libc::c_int)
                                                   ->
                                                       libc::c_int>>((*do_p).obj);
            return rw_func.expect("non-null function pointer")((*do_p).tag,
                                                               with_tag,
                                                               0 as
                                                                   *const uint8_t,
                                                               0 as
                                                                   libc::c_int,
                                                               0 as
                                                                   libc::c_int)
        }
        4 => { return -(1 as libc::c_int) }
        _ => { }
    }
    return 1 as libc::c_int;
}
/*
 * Process GET_DATA request on Data Object specified by TAG
 *   Call write_res_adpu to fill data returned
 */
#[no_mangle]
pub unsafe extern "C" fn gpg_do_get_data(mut tag: uint16_t,
                                         mut with_tag: libc::c_int) {
    let mut do_p: *const do_table_entry = get_do_entry(tag);
    res_p = apdu.res_apdu_data;
    if !do_p.is_null() {
        if copy_do(do_p, with_tag) < 0 as libc::c_int {
            /* Overwriting partially written result  */
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
        } else {
            apdu.res_apdu_data_len =
                res_p.wrapping_offset_from(apdu.res_apdu_data) as libc::c_long
                    as uint16_t;
            set_res_sw(0x90 as libc::c_int as uint8_t,
                       0 as libc::c_int as uint8_t);
        }
    } else {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x88 as libc::c_int as uint8_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_do_put_data(mut tag: uint16_t,
                                         mut data: *const uint8_t,
                                         mut len: libc::c_int) {
    let mut do_p: *const do_table_entry = get_do_entry(tag);
    if !do_p.is_null() {
        if ac_check_status((*do_p).ac_write) == 0 {
            set_res_sw(0x69 as libc::c_int as uint8_t,
                       0x82 as libc::c_int as uint8_t);
            return
        }
        match (*do_p).do_type as libc::c_uint {
            0 | 2 | 3 => {
                set_res_sw(0x69 as libc::c_int as uint8_t,
                           0x82 as libc::c_int as uint8_t);
            }
            1 => {
                let mut do_data_p: *mut *const uint8_t =
                    (*do_p).obj as *mut *const uint8_t;
                if !(*do_data_p).is_null() { flash_do_release(*do_data_p); }
                if len == 0 as libc::c_int {
                    /* make DO empty */
                    *do_data_p = 0 as *const uint8_t;
                    set_res_sw(0x90 as libc::c_int as uint8_t,
                               0 as libc::c_int as uint8_t);
                } else if len > 255 as libc::c_int {
                    set_res_sw(0x65 as libc::c_int as uint8_t,
                               0x81 as libc::c_int as uint8_t);
                } else {
                    let mut nr: libc::c_int = do_tag_to_nr(tag);
                    if nr < 0 as libc::c_int {
                        set_res_sw(0x65 as libc::c_int as uint8_t,
                                   0x81 as libc::c_int as uint8_t);
                    } else {
                        *do_data_p = 0 as *const uint8_t;
                        *do_data_p = flash_do_write(nr as uint8_t, data, len);
                        if !(*do_data_p).is_null() {
                            set_res_sw(0x90 as libc::c_int as uint8_t,
                                       0 as libc::c_int as uint8_t);
                        } else {
                            set_res_sw(0x65 as libc::c_int as uint8_t,
                                       0x81 as libc::c_int as uint8_t);
                        }
                    }
                }
            }
            5 => {
                let mut rw_func:
                        Option<unsafe extern "C" fn(_: uint16_t,
                                                    _: libc::c_int,
                                                    _: *const uint8_t,
                                                    _: libc::c_int,
                                                    _: libc::c_int)
                                   -> libc::c_int> =
                    ::std::mem::transmute::<*const libc::c_void,
                                            Option<unsafe extern "C" fn(_:
                                                                            uint16_t,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            *const uint8_t,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            libc::c_int)
                                                       ->
                                                           libc::c_int>>((*do_p).obj);
                if rw_func.expect("non-null function pointer")(tag,
                                                               0 as
                                                                   libc::c_int,
                                                               data, len,
                                                               1 as
                                                                   libc::c_int)
                       != 0 {
                    set_res_sw(0x90 as libc::c_int as uint8_t,
                               0 as libc::c_int as uint8_t);
                } else {
                    set_res_sw(0x6f as libc::c_int as uint8_t,
                               0 as libc::c_int as uint8_t);
                }
            }
            4 => {
                let mut proc_func:
                        Option<unsafe extern "C" fn(_: *const uint8_t,
                                                    _: libc::c_int)
                                   -> libc::c_int> =
                    ::std::mem::transmute::<*const libc::c_void,
                                            Option<unsafe extern "C" fn(_:
                                                                            *const uint8_t,
                                                                        _:
                                                                            libc::c_int)
                                                       ->
                                                           libc::c_int>>((*do_p).obj);
                if proc_func.expect("non-null function pointer")(data, len) !=
                       0 {
                    set_res_sw(0x90 as libc::c_int as uint8_t,
                               0 as libc::c_int as uint8_t);
                } else {
                    set_res_sw(0x6f as libc::c_int as uint8_t,
                               0 as libc::c_int as uint8_t);
                }
            }
            _ => { }
        }
    } else {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x88 as libc::c_int as uint8_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_do_public_key(mut kk_byte: uint8_t) {
    let mut kk: kind_of_key = kkb_to_kk(kk_byte);
    let mut attr: libc::c_int = gpg_get_algo_attr(kk);
    let mut pubkey_len: libc::c_int =
        gpg_get_algo_attr_key_size(kk, GPG_KEY_PUBLIC);
    let mut pubkey: *const uint8_t = kd[kk as usize].pubkey;
    if pubkey.is_null() {
        set_res_sw(0x6a as libc::c_int as uint8_t,
                   0x88 as libc::c_int as uint8_t);
        return
    }
    res_p = apdu.res_apdu_data;
    /* TAG */
    let fresh32 = res_p;
    res_p = res_p.offset(1);
    *fresh32 = 0x7f as libc::c_int as uint8_t;
    let fresh33 = res_p;
    res_p = res_p.offset(1);
    *fresh33 = 0x49 as libc::c_int as uint8_t;
    if attr == 1 as libc::c_int || attr == 2 as libc::c_int {
        /* ECDSA or ECDH */
        /* LEN */
        let fresh34 = res_p;
        res_p = res_p.offset(1);
        *fresh34 =
            (2 as libc::c_int + 1 as libc::c_int + 64 as libc::c_int) as
                uint8_t;
        /*TAG*/
        /* LEN = 1+64 */
        let fresh35 = res_p; /* No compression of EC point.  */
        res_p = res_p.offset(1);
        *fresh35 = 0x86 as libc::c_int as uint8_t;
        let fresh36 = res_p;
        res_p = res_p.offset(1);
        *fresh36 = 0x41 as libc::c_int as uint8_t;
        let fresh37 = res_p;
        res_p = res_p.offset(1);
        *fresh37 = 0x4 as libc::c_int as uint8_t;
        /* 64-byte binary (big endian): X || Y */
        memcpy(res_p as *mut libc::c_void, pubkey as *const libc::c_void,
               64 as libc::c_int as libc::c_ulong);
        res_p = res_p.offset(64 as libc::c_int as isize)
    } else if attr == 3 as libc::c_int || attr == 4 as libc::c_int {
        /* EdDSA or ECDH on curve25519 */
        /* LEN */
        let fresh38 = res_p;
        res_p = res_p.offset(1);
        *fresh38 = (2 as libc::c_int + 32 as libc::c_int) as uint8_t;
        /*TAG*/
        /* LEN = 32 */
        let fresh39 = res_p;
        res_p = res_p.offset(1);
        *fresh39 = 0x86 as libc::c_int as uint8_t;
        let fresh40 = res_p;
        res_p = res_p.offset(1);
        *fresh40 = 0x20 as libc::c_int as uint8_t;
        /* 32-byte binary (little endian): Y with parity or X*/
        memcpy(res_p as *mut libc::c_void, pubkey as *const libc::c_void,
               32 as libc::c_int as libc::c_ulong);
        res_p = res_p.offset(32 as libc::c_int as isize)
    } else {
        /* RSA */
        /* LEN = 9+256or512 */
        let fresh41 = res_p;
        res_p = res_p.offset(1);
        *fresh41 = 0x82 as libc::c_int as uint8_t;
        let fresh42 = res_p;
        res_p = res_p.offset(1);
        *fresh42 =
            if pubkey_len > 256 as libc::c_int {
                0x2 as libc::c_int
            } else { 0x1 as libc::c_int } as uint8_t;
        let fresh43 = res_p;
        res_p = res_p.offset(1);
        *fresh43 = 0x9 as libc::c_int as uint8_t;
        /*TAG*/
        /* LEN = 256or512 */
        let fresh44 = res_p;
        res_p = res_p.offset(1);
        *fresh44 = 0x81 as libc::c_int as uint8_t;
        let fresh45 = res_p;
        res_p = res_p.offset(1);
        *fresh45 = 0x82 as libc::c_int as uint8_t;
        let fresh46 = res_p;
        res_p = res_p.offset(1);
        *fresh46 =
            if pubkey_len > 256 as libc::c_int {
                0x2 as libc::c_int
            } else { 0x1 as libc::c_int } as uint8_t;
        let fresh47 = res_p;
        res_p = res_p.offset(1);
        *fresh47 = 0 as libc::c_int as uint8_t;
        /* PUBKEY_LEN-byte binary (big endian) */
        memcpy(res_p as *mut libc::c_void, pubkey as *const libc::c_void,
               pubkey_len as libc::c_ulong);
        res_p = res_p.offset(pubkey_len as isize);
        /*TAG*/
        /* LEN= 3 */
        let fresh48 = res_p;
        res_p = res_p.offset(1);
        *fresh48 = 0x82 as libc::c_int as uint8_t;
        let fresh49 = res_p;
        res_p = res_p.offset(1);
        *fresh49 = 3 as libc::c_int as uint8_t;
        /* 3-byte E=0x10001 (big endian) */
        let fresh50 = res_p;
        res_p = res_p.offset(1);
        *fresh50 = 0x1 as libc::c_int as uint8_t;
        let fresh51 = res_p;
        res_p = res_p.offset(1);
        *fresh51 = 0 as libc::c_int as uint8_t;
        let fresh52 = res_p;
        res_p = res_p.offset(1);
        *fresh52 = 0x1 as libc::c_int as uint8_t
    }
    /* Success */
    apdu.res_apdu_data_len =
        res_p.wrapping_offset_from(apdu.res_apdu_data) as libc::c_long as
            uint16_t;
    set_res_sw(0x90 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn gpg_do_read_simple(mut nr: uint8_t)
 -> *const uint8_t {
    let mut do_data: *const uint8_t = 0 as *const uint8_t;
    do_data = do_ptr[nr as usize];
    if do_data.is_null() { return 0 as *const uint8_t }
    return do_data.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gpg_do_write_simple(mut nr: uint8_t,
                                             mut data: *const uint8_t,
                                             mut size: libc::c_int) {
    let mut do_data_p: *mut *const uint8_t = 0 as *mut *const uint8_t;
    do_data_p =
        &mut *do_ptr.as_mut_ptr().offset(nr as isize) as *mut *const uint8_t;
    if !(*do_data_p).is_null() { flash_do_release(*do_data_p); }
    if !data.is_null() {
        *do_data_p = 0 as *const uint8_t;
        *do_data_p = flash_do_write(nr, data, size);
        if (*do_data_p).is_null() {
            flash_warning(b"DO WRITE ERROR\x00" as *const u8 as
                              *const libc::c_char);
        }
    } else { *do_data_p = 0 as *const uint8_t };
}
#[no_mangle]
pub unsafe extern "C" fn gpg_do_keygen(mut kk_byte: uint8_t) {
    let mut kk: kind_of_key = kkb_to_kk(kk_byte);
    let mut attr: libc::c_int = gpg_get_algo_attr(kk);
    let mut prvkey_len: libc::c_int =
        gpg_get_algo_attr_key_size(kk, GPG_KEY_PRIVATE);
    let mut keystring_admin: *const uint8_t = 0 as *const uint8_t;
    let mut p_q_modulus: *mut uint8_t = 0 as *mut uint8_t;
    let mut d: [uint8_t; 64] = [0; 64];
    let mut rnd: *const uint8_t = 0 as *const uint8_t;
    let mut prv: *const uint8_t = 0 as *const uint8_t;
    let mut pubkey: *const uint8_t = 0 as *const uint8_t;
    let mut r: libc::c_int = 0;
    if admin_authorized as libc::c_int == 3 as libc::c_int {
        keystring_admin = keystring_md_pw3.as_mut_ptr()
    } else { keystring_admin = 0 as *const uint8_t }
    if attr == 255 as libc::c_int || attr == 0 as libc::c_int {
        p_q_modulus = rsa_genkey(prvkey_len);
        if p_q_modulus.is_null() {
            set_res_sw(0x65 as libc::c_int as uint8_t,
                       0x81 as libc::c_int as uint8_t);
            return
        }
        prv = p_q_modulus;
        pubkey = p_q_modulus.offset(prvkey_len as isize)
    } else if attr == 1 as libc::c_int || attr == 2 as libc::c_int {
        let mut d1: [uint8_t; 32] = [0; 32];
        let mut p: *const uint8_t = 0 as *const uint8_t;
        let mut i: libc::c_int = 0;
        let mut r_0: libc::c_int = 0;
        rnd = 0 as *const uint8_t;
        loop  {
            if !rnd.is_null() { random_bytes_free(rnd); }
            rnd = random_bytes_get();
            if attr == 1 as libc::c_int {
                r_0 = ecc_check_secret_p256r1(rnd, d1.as_mut_ptr())
            } else { r_0 = ecc_check_secret_p256k1(rnd, d1.as_mut_ptr()) }
            if !(r_0 == 0 as libc::c_int) { break ; }
        }
        /* Convert it to big endian */
        if r_0 < 0 as libc::c_int {
            p = d1.as_mut_ptr() as *const uint8_t
        } else { p = rnd }
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            d[(32 as libc::c_int - i - 1 as libc::c_int) as usize] =
                *p.offset(i as isize);
            i += 1
        }
        random_bytes_free(rnd);
        prv = d.as_mut_ptr();
        pubkey = 0 as *const uint8_t
    } else if attr == 3 as libc::c_int {
        rnd = random_bytes_get();
        sha512(rnd, 32 as libc::c_int as libc::c_uint, d.as_mut_ptr());
        random_bytes_free(rnd);
        d[0 as libc::c_int as usize] =
            (d[0 as libc::c_int as usize] as libc::c_int & 248 as libc::c_int)
                as uint8_t;
        d[31 as libc::c_int as usize] =
            (d[31 as libc::c_int as usize] as libc::c_int &
                 127 as libc::c_int) as uint8_t;
        d[31 as libc::c_int as usize] =
            (d[31 as libc::c_int as usize] as libc::c_int | 64 as libc::c_int)
                as uint8_t;
        prv = d.as_mut_ptr();
        pubkey = 0 as *const uint8_t
    } else if attr == 4 as libc::c_int {
        rnd = random_bytes_get();
        memcpy(d.as_mut_ptr() as *mut libc::c_void,
               rnd as *const libc::c_void,
               32 as libc::c_int as libc::c_ulong);
        random_bytes_free(rnd);
        d[0 as libc::c_int as usize] =
            (d[0 as libc::c_int as usize] as libc::c_int & 248 as libc::c_int)
                as uint8_t;
        d[31 as libc::c_int as usize] =
            (d[31 as libc::c_int as usize] as libc::c_int &
                 127 as libc::c_int) as uint8_t;
        d[31 as libc::c_int as usize] =
            (d[31 as libc::c_int as usize] as libc::c_int | 64 as libc::c_int)
                as uint8_t;
        prv = d.as_mut_ptr();
        pubkey = 0 as *const uint8_t
    } else {
        set_res_sw(0x69 as libc::c_int as uint8_t,
                   0x85 as libc::c_int as uint8_t);
        return
    }
    r = gpg_do_write_prvkey(kk, prv, prvkey_len, keystring_admin, pubkey);
    if !p_q_modulus.is_null() {
        memset(p_q_modulus as *mut libc::c_void, 0 as libc::c_int,
               (prvkey_len * 2 as libc::c_int) as libc::c_ulong);
        gnuk_free(p_q_modulus as *mut libc::c_void);
    }
    if r < 0 as libc::c_int {
        set_res_sw(0x6f as libc::c_int as uint8_t,
                   0 as libc::c_int as uint8_t);
        return
    }
    if kk as libc::c_uint ==
           GPG_KEY_FOR_SIGNING as libc::c_int as libc::c_uint {
        let mut pw: *const uint8_t =
            b"123456\x00" as *const u8 as *const libc::c_char as
                *const uint8_t;
        let mut keystring: [uint8_t; 32] = [0; 32];
        /* GnuPG expects it's ready for signing. */
      /* Don't call ac_reset_pso_cds here, but load the private key */
        gpg_reset_digital_signature_counter();
        s2k(0 as *const libc::c_uchar, 0 as libc::c_int as size_t, pw,
            strlen(b"123456\x00" as *const u8 as *const libc::c_char),
            keystring.as_mut_ptr());
        gpg_do_load_prvkey(GPG_KEY_FOR_SIGNING, 1 as libc::c_int,
                           keystring.as_mut_ptr());
    } else { ac_reset_other(); }
    gpg_do_public_key(kk_byte);
}
unsafe extern "C" fn run_static_initializers() {
    gpg_do_table =
        [{
             let mut init =
                 do_table_entry{tag: 0x5f35 as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc7 as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc8 as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(2 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc9 as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(3 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xca as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(4 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xcb as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(5 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xcc as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(6 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xce as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(7 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xcf as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(8 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xd0 as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(9 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x5e as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(10 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x5f50 as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(11 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x5b as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(12 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x5f2d as libc::c_int as uint16_t,
                                do_type: DO_VAR,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    &mut *do_ptr.as_mut_ptr().offset(13 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                        as *mut *const uint8_t as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x5f52 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(do_hist_bytes
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc5 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(do_fp_all
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc6 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(do_cafp_all
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xcd as libc::c_int as uint16_t,
                                do_type: DO_PROC_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(do_kgtime_all
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x93 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(do_ds_count
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x4f as libc::c_int as uint16_t,
                                do_type: DO_PROC_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(do_openpgpcard_aid
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc4 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READWRITE,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            *const uint8_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(rw_pw_status
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *const uint8_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc1 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READWRITE,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            *const uint8_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(rw_algorithm_attr
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *const uint8_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc2 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READWRITE,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            *const uint8_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(rw_algorithm_attr
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *const uint8_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc3 as libc::c_int as uint16_t,
                                do_type: DO_PROC_READWRITE,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            uint16_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            *const uint8_t,
                                                                                        _:
                                                                                            libc::c_int,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(rw_algorithm_attr
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   uint16_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *const uint8_t,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xc0 as libc::c_int as uint16_t,
                                do_type: DO_FIXED,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    extended_capabilities.as_ptr() as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x65 as libc::c_int as uint16_t,
                                do_type: DO_CMP_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    cmp_ch_data.as_ptr() as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x6e as libc::c_int as uint16_t,
                                do_type: DO_CMP_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    cmp_app_data.as_ptr() as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x73 as libc::c_int as uint16_t,
                                do_type: DO_CMP_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    cmp_discretionary.as_ptr() as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x7a as libc::c_int as uint16_t,
                                do_type: DO_CMP_READ,
                                ac_read: 0xff as libc::c_int as uint8_t,
                                ac_write: 0x80 as libc::c_int as uint8_t,
                                obj:
                                    cmp_ss_temp.as_ptr() as
                                        *const libc::c_void,};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0xd3 as libc::c_int as uint16_t,
                                do_type: DO_PROC_WRITE,
                                ac_read: 0x80 as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            *const uint8_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(proc_resetting_code
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   *const uint8_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         },
         {
             let mut init =
                 do_table_entry{tag: 0x3fff as libc::c_int as uint16_t,
                                do_type: DO_PROC_WRITE,
                                ac_read: 0x80 as libc::c_int as uint8_t,
                                ac_write: 0x4 as libc::c_int as uint8_t,
                                obj:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            *const uint8_t,
                                                                                        _:
                                                                                            libc::c_int)
                                                                       ->
                                                                           libc::c_int>,
                                                            *const libc::c_void>(Some(proc_key_import
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   *const uint8_t,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)),};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
