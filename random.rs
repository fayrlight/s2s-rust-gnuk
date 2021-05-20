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
    fn neug_init(buf: *mut uint32_t, size: uint8_t);
    #[no_mangle]
    fn neug_get(kick: libc::c_int) -> uint32_t;
    #[no_mangle]
    fn neug_flush();
    #[no_mangle]
    fn neug_wait_full();
    #[no_mangle]
    fn neug_fini();
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
static mut random_word: [uint32_t; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn random_init() {
    let mut i: libc::c_int = 0;
    neug_init(random_word.as_mut_ptr(),
              (32 as libc::c_int as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<uint32_t>()
                                                   as libc::c_ulong) as
                  uint8_t);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int { neug_get(1 as libc::c_int); i += 1 };
}
#[no_mangle]
pub unsafe extern "C" fn random_fini() { neug_fini(); }
/*
 * Return pointer to random 32-byte
 */
#[no_mangle]
pub unsafe extern "C" fn random_bytes_get() -> *const uint8_t {
    neug_wait_full();
    return random_word.as_mut_ptr() as *const uint8_t;
}
/*
 * Free pointer to random 32-byte
 */
#[no_mangle]
pub unsafe extern "C" fn random_bytes_free(mut p: *const uint8_t) {
    memset(random_word.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           32 as libc::c_int as libc::c_ulong);
    neug_flush();
}
/*
 * Return 4-byte salt
 */
#[no_mangle]
pub unsafe extern "C" fn random_get_salt(mut p: *mut uint8_t) {
    let mut rnd: uint32_t = 0;
    rnd = neug_get(1 as libc::c_int);
    memcpy(p as *mut libc::c_void,
           &mut rnd as *mut uint32_t as *const libc::c_void,
           ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    rnd = neug_get(1 as libc::c_int);
    memcpy(p.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                        isize) as *mut libc::c_void,
           &mut rnd as *mut uint32_t as *const libc::c_void,
           ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
}
/*
 * Random byte iterator
 */
#[no_mangle]
pub unsafe extern "C" fn random_gen(mut arg: *mut libc::c_void,
                                    mut out: *mut libc::c_uchar,
                                    mut out_len: size_t) -> libc::c_int {
    let mut index_p: *mut uint8_t = arg as *mut uint8_t;
    let mut index: uint8_t = *index_p;
    let mut n: size_t = 0;
    while out_len != 0 {
        neug_wait_full();
        n = (32 as libc::c_int - index as libc::c_int) as size_t;
        if n > out_len { n = out_len }
        memcpy(out as *mut libc::c_void,
               (random_word.as_mut_ptr() as
                    *mut libc::c_uchar).offset(index as libc::c_int as isize)
                   as *const libc::c_void, n);
        out = out.offset(n as isize);
        out_len =
            (out_len as libc::c_ulong).wrapping_sub(n) as size_t as size_t;
        index =
            (index as libc::c_ulong).wrapping_add(n) as uint8_t as uint8_t;
        if index as libc::c_int >= 32 as libc::c_int {
            index = 0 as libc::c_int as uint8_t;
            neug_flush();
        }
    }
    *index_p = index;
    return 0 as libc::c_int;
}
