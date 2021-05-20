#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    static mut vector: [handler; 16];
    #[no_mangle]
    fn gpg_data_copy(p: *const uint8_t);
    #[no_mangle]
    fn gpg_get_firmware_update_key(keyno: uint8_t) -> *const uint8_t;
    #[no_mangle]
    fn gpg_get_algo_attr_key_size(kk: kind_of_key, s: size_of_key)
     -> libc::c_int;
    #[no_mangle]
    static mut kd: [key_data; 3];
    #[no_mangle]
    fn fatal(code: uint8_t) -> !;
    /* SIG, DEC and AUT */
    #[no_mangle]
    static openpgpcard_aid: [uint8_t; 14];
    #[no_mangle]
    static mut _keystore_pool: uint8_t;
    /* Linker set this symbol */
    #[no_mangle]
    static mut _data_pool: uint8_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type handler = Option<unsafe extern "C" fn() -> ()>;
/* Constants: algo+size */
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
#[inline]
unsafe extern "C" fn flash_program_halfword(mut addr: uintptr_t,
                                            mut data: uint16_t)
 -> libc::c_int {
    let mut func:
            Option<unsafe extern "C" fn(_: uintptr_t, _: uint16_t)
                       -> libc::c_int> =
        ::std::mem::transmute::<handler,
                                Option<unsafe extern "C" fn(_: uintptr_t,
                                                            _: uint16_t)
                                           ->
                                               libc::c_int>>(vector[4 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(addr,
                                                                                              data);
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
#[inline]
unsafe extern "C" fn flash_check_blank(mut p_start: *const uint8_t,
                                       mut size: size_t) -> libc::c_int {
    let mut func:
            Option<unsafe extern "C" fn(_: *const uint8_t, _: libc::c_int)
                       -> libc::c_int> =
        ::std::mem::transmute::<handler,
                                Option<unsafe extern "C" fn(_: *const uint8_t,
                                                            _: libc::c_int)
                                           ->
                                               libc::c_int>>(vector[6 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    return Some(func.expect("non-null function pointer")).expect("non-null function pointer")(p_start,
                                                                                              size
                                                                                                  as
                                                                                                  libc::c_int);
}
static mut flash_page_size: uint16_t = 0;
static mut data_pool: *const uint8_t = 0 as *const uint8_t;
static mut last_p: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
/* The first halfword is generation for the data page (little endian) */
#[no_mangle]
#[link_section = ".gnuk_data"]
pub static mut flash_data: [uint8_t; 4] =
    [0x1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t];
unsafe extern "C" fn key_available_at(mut k: *const uint8_t,
                                      mut key_size: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < key_size { if *k.offset(i as isize) != 0 { break ; } i += 1 }
    if i == key_size {
        /* It's ZERO.  Released key.  */
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < key_size {
        if *k.offset(i as isize) as libc::c_int != 0xff as libc::c_int {
            break ;
        }
        i += 1
    }
    if i == key_size {
        /* It's FULL.  Unused key.  */
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn flash_init() -> *const uint8_t {
    let mut gen0: uint16_t = 0;
    let mut gen1: uint16_t = 0;
    let mut gen0_p: *mut uint16_t =
        &mut _data_pool as *mut uint8_t as *mut uint16_t;
    let mut gen1_p: *mut uint16_t = 0 as *mut uint16_t;
    flash_page_size = 1024 as libc::c_int as uint16_t;
    if *(0xe0042000 as libc::c_uint as *mut uint32_t) &
           0xfff as libc::c_int as libc::c_uint ==
           0x414 as libc::c_int as libc::c_uint {
        flash_page_size = 2048 as libc::c_int as uint16_t
    }
    gen1_p =
        (&mut _data_pool as
             *mut uint8_t).offset(flash_page_size as libc::c_int as isize) as
            *mut uint16_t;
    /* Check data pool generation and choose the page */
    gen0 = *gen0_p;
    gen1 = *gen1_p;
    if gen0 as libc::c_int == 0xffff as libc::c_int {
        data_pool =
            (&mut _data_pool as
                 *mut uint8_t).offset(flash_page_size as libc::c_int as isize)
    } else if gen1 as libc::c_int == 0xffff as libc::c_int {
        data_pool = &mut _data_pool
    } else if gen1 as libc::c_int > gen0 as libc::c_int {
        data_pool =
            (&mut _data_pool as
                 *mut uint8_t).offset(flash_page_size as libc::c_int as isize)
    } else { data_pool = &mut _data_pool }
    return data_pool.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn flash_init_keys() {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut i: libc::c_int = 0;
    /* For each key, find its address.  */
    p = &mut _keystore_pool;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut k: *const uint8_t = 0 as *const uint8_t;
        let mut key_size: libc::c_int =
            gpg_get_algo_attr_key_size(i as kind_of_key, GPG_KEY_STORAGE);
        kd[i as usize].pubkey = 0 as *const uint8_t;
        k = p;
        while k < p.offset(flash_page_size as libc::c_int as isize) {
            if key_available_at(k, key_size) != 0 {
                let mut prv_len: libc::c_int =
                    gpg_get_algo_attr_key_size(i as kind_of_key,
                                               GPG_KEY_PRIVATE);
                kd[i as usize].pubkey = k.offset(prv_len as isize);
                break ;
            } else { k = k.offset(key_size as isize) }
        }
        p = p.offset(flash_page_size as libc::c_int as isize);
        i += 1
    };
}
/*
 * Flash data pool managenent
 *
 * Flash data pool consists of two parts:
 *   2-byte header
 *   contents
 *
 * Flash data pool objects:
 *   Data Object (DO) (of smart card)
 *   Internal objects:
 *     NONE (0x0000)
 *     123-counter
 *     14-bit counter
 *     bool object
 *     small enum
 *
 * Format of a Data Object:
 *    NR:   8-bit tag_number
 *    LEN:  8-bit length
 *    DATA: data * LEN
 *    PAD:  optional byte for 16-bit alignment
 */
#[no_mangle]
pub unsafe extern "C" fn flash_set_data_pool_last(mut p: *const uint8_t) {
    last_p = p as *mut uint8_t;
}
/*
 * We use two pages
 */
unsafe extern "C" fn flash_copying_gc() -> libc::c_int {
    let mut src: *mut uint8_t =
        0 as *mut uint8_t; /* allocation unit is 1-halfword (2-byte) */
    let mut dst: *mut uint8_t = 0 as *mut uint8_t;
    let mut generation: uint16_t = 0;
    if data_pool == &mut _data_pool as *mut uint8_t {
        src = &mut _data_pool;
        dst =
            (&mut _data_pool as
                 *mut uint8_t).offset(flash_page_size as libc::c_int as isize)
    } else {
        src =
            (&mut _data_pool as
                 *mut uint8_t).offset(flash_page_size as libc::c_int as
                                          isize);
        dst = &mut _data_pool
    }
    generation = *(src as *mut uint16_t);
    data_pool = dst;
    gpg_data_copy(data_pool.offset(2 as libc::c_int as isize));
    flash_erase_page(src as uint32_t as uintptr_t);
    flash_program_halfword(dst as uint32_t as uintptr_t,
                           (generation as libc::c_int + 1 as libc::c_int) as
                               uint16_t);
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_data_pool_full(mut size: size_t) -> libc::c_int {
    return (last_p.offset(size as isize) >
                data_pool.offset(flash_page_size as libc::c_int as isize) as
                    *mut uint8_t) as libc::c_int;
}
unsafe extern "C" fn flash_data_pool_allocate(mut size: size_t)
 -> *mut uint8_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    size =
        size.wrapping_add(1 as libc::c_int as libc::c_ulong) &
            !(1 as libc::c_int) as libc::c_ulong;
    if is_data_pool_full(size) != 0 {
        if flash_copying_gc() < 0 as libc::c_int ||
               is_data_pool_full(size) != 0 {
            fatal(1 as libc::c_int as uint8_t);
        }
    }
    p = last_p;
    last_p = last_p.offset(size as isize);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn flash_do_write_internal(mut p: *const uint8_t,
                                                 mut nr: libc::c_int,
                                                 mut data: *const uint8_t,
                                                 mut len: libc::c_int) {
    let mut hw: uint16_t = 0;
    let mut addr: uint32_t = 0;
    let mut i: libc::c_int = 0;
    addr = p as uint32_t;
    hw = (nr | len << 8 as libc::c_int) as uint16_t;
    if flash_program_halfword(addr as uintptr_t, hw) != 0 as libc::c_int {
        flash_warning(b"DO WRITE ERROR\x00" as *const u8 as
                          *const libc::c_char);
    }
    addr =
        (addr as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    i = 0 as libc::c_int;
    while i < len / 2 as libc::c_int {
        hw =
            (*data.offset((i * 2 as libc::c_int) as isize) as libc::c_int |
                 (*data.offset((i * 2 as libc::c_int + 1 as libc::c_int) as
                                   isize) as libc::c_int) << 8 as libc::c_int)
                as uint16_t;
        if flash_program_halfword(addr as uintptr_t, hw) != 0 as libc::c_int {
            flash_warning(b"DO WRITE ERROR\x00" as *const u8 as
                              *const libc::c_char);
        }
        addr =
            (addr as
                 libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        i += 1
    }
    if len & 1 as libc::c_int != 0 {
        hw =
            (*data.offset((i * 2 as libc::c_int) as isize) as libc::c_int |
                 0xff00 as libc::c_int) as uint16_t;
        if flash_program_halfword(addr as uintptr_t, hw) != 0 as libc::c_int {
            flash_warning(b"DO WRITE ERROR\x00" as *const u8 as
                              *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn flash_do_write(mut nr: uint8_t,
                                        mut data: *const uint8_t,
                                        mut len: libc::c_int)
 -> *const uint8_t {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p = flash_data_pool_allocate((2 as libc::c_int + len) as size_t);
    if p.is_null() { return 0 as *const uint8_t }
    flash_do_write_internal(p, nr as libc::c_int, data, len);
    return p.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn flash_warning(mut msg: *const libc::c_char) { }
#[no_mangle]
pub unsafe extern "C" fn flash_do_release(mut do_data: *const uint8_t) {
    let mut addr: uint32_t =
        (do_data as uint32_t).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut addr_tag: uint32_t = addr;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int =
        *do_data.offset(0 as libc::c_int as isize) as libc::c_int;
    /* Don't filling zero for data in code (such as ds_count_initial_value) */
    if do_data < &mut _data_pool as *mut uint8_t ||
           do_data >
               (&mut _data_pool as
                    *mut uint8_t).offset((flash_page_size as libc::c_int *
                                              2 as libc::c_int) as isize) {
        return
    }
    addr =
        (addr as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    /* Fill zero for content and pad */
    i = 0 as libc::c_int;
    while i < len / 2 as libc::c_int {
        if flash_program_halfword(addr as uintptr_t,
                                  0 as libc::c_int as uint16_t) !=
               0 as libc::c_int {
            flash_warning(b"fill-zero failure\x00" as *const u8 as
                              *const libc::c_char);
        }
        addr =
            (addr as
                 libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        i += 1
    }
    if len & 1 as libc::c_int != 0 {
        if flash_program_halfword(addr as uintptr_t,
                                  0 as libc::c_int as uint16_t) !=
               0 as libc::c_int {
            flash_warning(b"fill-zero pad failure\x00" as *const u8 as
                              *const libc::c_char);
        }
    }
    /* Fill 0x0000 for "tag_number and length" word */
    if flash_program_halfword(addr_tag as uintptr_t,
                              0 as libc::c_int as uint16_t) !=
           0 as libc::c_int {
        flash_warning(b"fill-zero tag_nr failure\x00" as *const u8 as
                          *const libc::c_char);
    };
}
unsafe extern "C" fn flash_key_getpage(mut kk: kind_of_key) -> *mut uint8_t {
    /* There is a page for each KK.  */
    return (&mut _keystore_pool as
                *mut uint8_t).offset((flash_page_size as
                                          libc::c_uint).wrapping_mul(kk as
                                                                         libc::c_uint)
                                         as isize);
}
#[no_mangle]
pub unsafe extern "C" fn flash_key_alloc(mut kk: kind_of_key)
 -> *mut uint8_t {
    let mut k: *mut uint8_t = 0 as *mut uint8_t;
    let mut k0: *mut uint8_t = flash_key_getpage(kk);
    let mut i: libc::c_int = 0;
    let mut key_size: libc::c_int =
        gpg_get_algo_attr_key_size(kk, GPG_KEY_STORAGE);
    /* Seek free space in the page.  */
    k = k0;
    while k < k0.offset(flash_page_size as libc::c_int as isize) {
        let mut p: *const uint32_t = k as *const uint32_t;
        i = 0 as libc::c_int;
        while i < key_size / 4 as libc::c_int {
            if *p.offset(i as isize) != 0xffffffff as libc::c_uint { break ; }
            i += 1
        }
        if i == key_size / 4 as libc::c_int {
            /* Yes, it's empty.  */
            return k
        }
        k = k.offset(key_size as isize)
    }
    /* Should not happen as we have enough free space all time, but just
     in case.  */
    return 0 as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn flash_key_write(mut key_addr: *mut uint8_t,
                                         mut key_data: *const uint8_t,
                                         mut key_data_len: libc::c_int,
                                         mut pubkey: *const uint8_t,
                                         mut pubkey_len: libc::c_int)
 -> libc::c_int {
    let mut hw: uint16_t = 0;
    let mut addr: uint32_t = 0;
    let mut i: libc::c_int = 0;
    addr = key_addr as uint32_t;
    i = 0 as libc::c_int;
    while i < key_data_len / 2 as libc::c_int {
        hw =
            (*key_data.offset((i * 2 as libc::c_int) as isize) as libc::c_int
                 |
                 (*key_data.offset((i * 2 as libc::c_int + 1 as libc::c_int)
                                       as isize) as libc::c_int) <<
                     8 as libc::c_int) as uint16_t;
        if flash_program_halfword(addr as uintptr_t, hw) != 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        addr =
            (addr as
                 libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < pubkey_len / 2 as libc::c_int {
        hw =
            (*pubkey.offset((i * 2 as libc::c_int) as isize) as libc::c_int |
                 (*pubkey.offset((i * 2 as libc::c_int + 1 as libc::c_int) as
                                     isize) as libc::c_int) <<
                     8 as libc::c_int) as uint16_t;
        if flash_program_halfword(addr as uintptr_t, hw) != 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        addr =
            (addr as
                 libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn flash_check_all_other_keys_released(mut key_addr:
                                                             *const uint8_t,
                                                         mut key_size:
                                                             libc::c_int)
 -> libc::c_int {
    let mut start: uint32_t =
        key_addr as uint32_t &
            !(flash_page_size as libc::c_int - 1 as libc::c_int) as
                libc::c_uint;
    let mut p: *const uint32_t = start as *const uint32_t;
    while p <
              start.wrapping_add(flash_page_size as libc::c_uint) as
                  *const uint32_t {
        if p == key_addr as *const uint32_t {
            p = p.offset((key_size / 4 as libc::c_int) as isize)
        } else if *p != 0 { return 0 as libc::c_int } else { p = p.offset(1) }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn flash_key_fill_zero_as_released(mut key_addr:
                                                         *mut uint8_t,
                                                     mut key_size:
                                                         libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut addr: uint32_t = key_addr as uint32_t;
    i = 0 as libc::c_int;
    while i < key_size / 2 as libc::c_int {
        flash_program_halfword(addr.wrapping_add((i * 2 as libc::c_int) as
                                                     libc::c_uint) as
                                   uintptr_t, 0 as libc::c_int as uint16_t);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn flash_key_release(mut key_addr: *mut uint8_t,
                                           mut key_size: libc::c_int) {
    if flash_check_all_other_keys_released(key_addr, key_size) != 0 {
        flash_erase_page((key_addr as uint32_t &
                              !(flash_page_size as libc::c_int -
                                    1 as libc::c_int) as libc::c_uint) as
                             uintptr_t);
    } else { flash_key_fill_zero_as_released(key_addr, key_size); };
}
#[no_mangle]
pub unsafe extern "C" fn flash_key_release_page(mut kk: kind_of_key) {
    flash_erase_page(flash_key_getpage(kk) as uint32_t as uintptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn flash_clear_halfword(mut addr: uint32_t) {
    flash_program_halfword(addr as uintptr_t, 0 as libc::c_int as uint16_t);
}
#[no_mangle]
pub unsafe extern "C" fn flash_put_data_internal(mut p: *const uint8_t,
                                                 mut hw: uint16_t) {
    flash_program_halfword(p as uint32_t as uintptr_t, hw);
}
#[no_mangle]
pub unsafe extern "C" fn flash_put_data(mut hw: uint16_t) {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    p = flash_data_pool_allocate(2 as libc::c_int as size_t);
    p.is_null();
    flash_program_halfword(p as uint32_t as uintptr_t, hw);
}
#[no_mangle]
pub unsafe extern "C" fn flash_bool_clear(mut addr_p: *mut *const uint8_t) {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p = *addr_p;
    if p.is_null() { return }
    flash_program_halfword(p as uint32_t as uintptr_t,
                           0 as libc::c_int as uint16_t);
    *addr_p = 0 as *const uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn flash_bool_write_internal(mut p: *const uint8_t,
                                                   mut nr: libc::c_int) {
    flash_program_halfword(p as uint32_t as uintptr_t, nr as uint16_t);
}
#[no_mangle]
pub unsafe extern "C" fn flash_bool_write(mut nr: uint8_t) -> *const uint8_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut hw: uint16_t = nr as uint16_t;
    p = flash_data_pool_allocate(2 as libc::c_int as size_t);
    if p.is_null() { return 0 as *const uint8_t }
    flash_program_halfword(p as uint32_t as uintptr_t, hw);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn flash_enum_clear(mut addr_p: *mut *const uint8_t) {
    flash_bool_clear(addr_p);
}
#[no_mangle]
pub unsafe extern "C" fn flash_enum_write_internal(mut p: *const uint8_t,
                                                   mut nr: libc::c_int,
                                                   mut v: uint8_t) {
    let mut hw: uint16_t =
        (nr | (v as libc::c_int) << 8 as libc::c_int) as uint16_t;
    flash_program_halfword(p as uint32_t as uintptr_t, hw);
}
#[no_mangle]
pub unsafe extern "C" fn flash_enum_write(mut nr: uint8_t, mut v: uint8_t)
 -> *const uint8_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut hw: uint16_t =
        (nr as libc::c_int | (v as libc::c_int) << 8 as libc::c_int) as
            uint16_t;
    p = flash_data_pool_allocate(2 as libc::c_int as size_t);
    if p.is_null() { return 0 as *const uint8_t }
    flash_program_halfword(p as uint32_t as uintptr_t, hw);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn flash_cnt123_get_value(mut p: *const uint8_t)
 -> libc::c_int {
    if p.is_null() {
        return 0 as libc::c_int
    } else {
        let mut v: uint8_t = *p;
        /*
       * After erase, a halfword in flash memory becomes 0xffff.
       * The halfword can be programmed to any value.
       * Then, the halfword can be programmed to zero.
       *
       * Thus, we can represent value 1, 2, and 3.
       */
        if v as libc::c_int == 0xff as libc::c_int {
            return 1 as libc::c_int
        } else if v as libc::c_int == 0 as libc::c_int {
            return 3 as libc::c_int
        } else { return 2 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn flash_cnt123_write_internal(mut p: *const uint8_t,
                                                     mut which: libc::c_int,
                                                     mut v: libc::c_int) {
    let mut hw: uint16_t = 0;
    hw = (0xfe as libc::c_int | which << 8 as libc::c_int) as uint16_t;
    flash_program_halfword(p as uint32_t as uintptr_t, hw);
    if v == 1 as libc::c_int {
        return
    } else {
        if v == 2 as libc::c_int {
            flash_program_halfword((p as
                                        uint32_t).wrapping_add(2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                       as uintptr_t,
                                   0xc3c3 as libc::c_int as uint16_t);
        } else {
            /* v == 3 */
            flash_program_halfword((p as
                                        uint32_t).wrapping_add(2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                       as uintptr_t,
                                   0 as libc::c_int as uint16_t);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn flash_cnt123_increment(mut which: uint8_t,
                                                mut addr_p:
                                                    *mut *const uint8_t) {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut hw: uint16_t = 0;
    p = *addr_p;
    if p.is_null() {
        p = flash_data_pool_allocate(4 as libc::c_int as size_t);
        if p.is_null() { return }
        hw =
            (0xfe as libc::c_int | (which as libc::c_int) << 8 as libc::c_int)
                as uint16_t;
        flash_program_halfword(p as uint32_t as uintptr_t, hw);
        *addr_p = p.offset(2 as libc::c_int as isize)
    } else {
        let mut v: uint8_t = *p;
        if v as libc::c_int == 0 as libc::c_int { return }
        if v as libc::c_int == 0xff as libc::c_int {
            hw = 0xc3c3 as libc::c_int as uint16_t
        } else { hw = 0 as libc::c_int as uint16_t }
        flash_program_halfword(p as uint32_t as uintptr_t, hw);
    };
}
#[no_mangle]
pub unsafe extern "C" fn flash_cnt123_clear(mut addr_p: *mut *const uint8_t) {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p = *addr_p;
    if p.is_null() { return }
    flash_program_halfword(p as uint32_t as uintptr_t,
                           0 as libc::c_int as uint16_t);
    p = p.offset(-(2 as libc::c_int as isize));
    flash_program_halfword(p as uint32_t as uintptr_t,
                           0 as libc::c_int as uint16_t);
    *addr_p = 0 as *const uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn flash_write_binary(mut file_id: uint8_t,
                                            mut data: *const uint8_t,
                                            mut len: uint16_t,
                                            mut offset: uint16_t)
 -> libc::c_int {
    let mut maxsize: uint16_t = 0;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    if file_id as libc::c_int == 0 as libc::c_int {
        maxsize = 6 as libc::c_int as uint16_t;
        p =
            &*openpgpcard_aid.as_ptr().offset(8 as libc::c_int as isize) as
                *const uint8_t
    } else if file_id as libc::c_int >= 1 as libc::c_int &&
                  file_id as libc::c_int <= 4 as libc::c_int {
        maxsize = 256 as libc::c_int as uint16_t;
        p =
            gpg_get_firmware_update_key((file_id as libc::c_int -
                                             1 as libc::c_int) as uint8_t);
        if len as libc::c_int == 0 as libc::c_int &&
               offset as libc::c_int == 0 as libc::c_int {
            /* This means removal of update key.  */
            if flash_program_halfword(p as uint32_t as uintptr_t,
                                      0 as libc::c_int as uint16_t) !=
                   0 as libc::c_int {
                flash_warning(b"DO WRITE ERROR\x00" as *const u8 as
                                  *const libc::c_char);
            }
            return 0 as libc::c_int
        }
    } else { return -(1 as libc::c_int) }
    if offset as libc::c_int + len as libc::c_int > maxsize as libc::c_int ||
           offset as libc::c_int & 1 as libc::c_int != 0 ||
           len as libc::c_int & 1 as libc::c_int != 0 {
        return -(1 as libc::c_int)
    } else {
        let mut hw: uint16_t = 0;
        let mut addr: uint32_t = 0;
        let mut i: libc::c_int = 0;
        if flash_check_blank(p.offset(offset as libc::c_int as isize),
                             len as size_t) == 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        addr = (p as uint32_t).wrapping_add(offset as libc::c_uint);
        i = 0 as libc::c_int;
        while i < len as libc::c_int / 2 as libc::c_int {
            hw =
                (*data.offset((i * 2 as libc::c_int) as isize) as libc::c_int
                     |
                     (*data.offset((i * 2 as libc::c_int + 1 as libc::c_int)
                                       as isize) as libc::c_int) <<
                         8 as libc::c_int) as uint16_t;
            if flash_program_halfword(addr as uintptr_t, hw) !=
                   0 as libc::c_int {
                flash_warning(b"DO WRITE ERROR\x00" as *const u8 as
                                  *const libc::c_char);
            }
            addr =
                (addr as
                     libc::c_uint).wrapping_add(2 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            i += 1
        }
        return 0 as libc::c_int
    };
}
