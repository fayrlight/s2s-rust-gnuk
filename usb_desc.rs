#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static sys_version: [uint8_t; 8];
    /*
 * Control Endpoint ENDP0 does device requests handling.
 * In response to an event of
 *     USB_EVENT_SET_CONFIGURATION
 *     USB_EVENT_SET_INTERFACE
 *     USB_EVENT_SET_FEATURE_DEVICE
 *     USB_EVENT_SET_FEATURE_ENDPOINT
 *     USB_EVENT_CLEAR_FEATURE_DEVICE
 *     USB_EVENT_CLEAR_FEATURE_ENDPOINT
 *     USB_EVENT_GET_STATUS_INTERFACE
 *     USB_EVENT_GET_DESCRIPTOR
 *     USB_EVENT_GET_INTERFACE
 *     USB_EVENT_CTRL_REQUEST
 *  a single action should be done, which is SEND, RECV, or,
 *  ACKNOWLEDGE (no data to be sent, or to be received).
 *  Otherwise, it's an error.
 */
    #[no_mangle]
    fn usb_lld_ctrl_send(dev: *mut usb_dev, buf: *const libc::c_void,
                         buflen: size_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
/* endpoints enumeration */
pub type RECIPIENT_TYPE = libc::c_uint;
/* Recipient endpoint  */
pub const OTHER_RECIPIENT: RECIPIENT_TYPE = 3;
/* Recipient interface */
pub const ENDPOINT_RECIPIENT: RECIPIENT_TYPE = 2;
/* Recipient device    */
pub const INTERFACE_RECIPIENT: RECIPIENT_TYPE = 1;
pub const DEVICE_RECIPIENT: RECIPIENT_TYPE = 0;
pub type DESCRIPTOR_TYPE = libc::c_uint;
pub const ENDPOINT_DESCRIPTOR: DESCRIPTOR_TYPE = 5;
pub const INTERFACE_DESCRIPTOR: DESCRIPTOR_TYPE = 4;
pub const STRING_DESCRIPTOR: DESCRIPTOR_TYPE = 3;
pub const CONFIG_DESCRIPTOR: DESCRIPTOR_TYPE = 2;
pub const DEVICE_DESCRIPTOR: DESCRIPTOR_TYPE = 1;
/* Mask to get recipient    */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device_req {
    pub type_0: uint8_t,
    pub request: uint8_t,
    pub value: uint16_t,
    pub index: uint16_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctrl_data {
    pub addr: *mut uint8_t,
    pub len: uint16_t,
    pub require_zlp: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usb_dev {
    pub configuration: uint8_t,
    pub feature: uint8_t,
    pub state: uint8_t,
    pub dev_req: device_req,
    pub ctrl_data: ctrl_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct desc {
    pub desc: *const uint8_t,
    pub size: uint16_t,
}
/* USB Standard Device Descriptor */
static mut device_desc: [uint8_t; 18] =
    [18 as libc::c_int as uint8_t,
     DEVICE_DESCRIPTOR as libc::c_int as uint8_t,
     0x10 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x4b as libc::c_int as uint8_t, 0x23 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     3 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t];
/* Configuation Descriptor */
static mut config_desc: [uint8_t; 93] =
    [9 as libc::c_int as uint8_t, CONFIG_DESCRIPTOR as libc::c_int as uint8_t,
     (9 as libc::c_int + 9 as libc::c_int + 54 as libc::c_int +
          7 as libc::c_int + 7 as libc::c_int + 7 as libc::c_int +
          0 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int) as uint8_t,
     0 as libc::c_int as uint8_t,
     (1 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int +
          0 as libc::c_int) as uint8_t, 0x1 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x80 as libc::c_int as uint8_t,
     50 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     INTERFACE_DESCRIPTOR as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     3 as libc::c_int as uint8_t, 0xb as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 54 as libc::c_int as uint8_t,
     0x21 as libc::c_int as uint8_t, 0x10 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xa0 as libc::c_int as uint8_t,
     0xf as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xa0 as libc::c_int as uint8_t,
     0xf as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x25 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x25 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xfe as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x7a as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xf as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     ENDPOINT_DESCRIPTOR as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     64 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     ENDPOINT_DESCRIPTOR as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     64 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     ENDPOINT_DESCRIPTOR as libc::c_int as uint8_t,
     0x82 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t];
/* USB String Descriptors */
static mut gnuk_string_lang_id: [uint8_t; 4] =
    [4 as libc::c_int as uint8_t, STRING_DESCRIPTOR as libc::c_int as uint8_t,
     0x9 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t];
static mut gnuk_string_vendor: [uint8_t; 68] =
    [(33 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as uint8_t,
     STRING_DESCRIPTOR as libc::c_int as uint8_t, 'F' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'r' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ' ' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'S' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'f' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 't' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'w' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'a' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'r' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ' ' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'I' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'i' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 't' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'i' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'a' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 't' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'i' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'v' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ' ' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'f' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ' ' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'J' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'a' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'p' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'a' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t];
static mut gnuk_string_product: [uint8_t; 22] =
    [(10 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as uint8_t,
     STRING_DESCRIPTOR as libc::c_int as uint8_t, 'G' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'u' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'k' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ' ' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'T' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'k' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t];
#[no_mangle]
pub static mut gnuk_string_serial: [uint8_t; 40] =
    [(11 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int +
          16 as libc::c_int) as uint8_t,
     STRING_DESCRIPTOR as libc::c_int as uint8_t, 'F' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'S' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'I' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'J' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '-' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '1' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '.' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '2' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '.' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '1' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '-' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t];
static mut gnuk_revision_detail: [uint8_t; 28] =
    [(13 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as uint8_t,
     STRING_DESCRIPTOR as libc::c_int as uint8_t, 'r' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'l' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'a' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 's' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '/' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '1' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '.' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '2' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '.' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '1' as i32 as uint8_t,
     0 as libc::c_int as uint8_t];
static mut gnuk_config_options: [uint8_t; 86] =
    [(42 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as uint8_t,
     STRING_DESCRIPTOR as libc::c_int as uint8_t, 'F' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'S' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'T' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '_' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '0' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '1' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ':' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'd' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'f' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'u' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '=' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ':' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'd' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'b' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'u' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'g' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '=' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ':' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'p' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'i' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'p' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'a' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'd' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '=' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, ':' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'c' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'e' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'r' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 't' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'd' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, '=' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'n' as i32 as uint8_t,
     0 as libc::c_int as uint8_t, 'o' as i32 as uint8_t,
     0 as libc::c_int as uint8_t];
static mut string_descriptors: [desc; 7] =
    unsafe {
        [{
             let mut init =
                 desc{desc: gnuk_string_lang_id.as_ptr(),
                      size:
                          ::std::mem::size_of::<[uint8_t; 4]>() as
                              libc::c_ulong as uint16_t,};
             init
         },
         {
             let mut init =
                 desc{desc: gnuk_string_vendor.as_ptr(),
                      size:
                          ::std::mem::size_of::<[uint8_t; 68]>() as
                              libc::c_ulong as uint16_t,};
             init
         },
         {
             let mut init =
                 desc{desc: gnuk_string_product.as_ptr(),
                      size:
                          ::std::mem::size_of::<[uint8_t; 22]>() as
                              libc::c_ulong as uint16_t,};
             init
         },
         {
             let mut init =
                 desc{desc: gnuk_string_serial.as_ptr(),
                      size:
                          ::std::mem::size_of::<[uint8_t; 40]>() as
                              libc::c_ulong as uint16_t,};
             init
         },
         {
             let mut init =
                 desc{desc: gnuk_revision_detail.as_ptr(),
                      size:
                          ::std::mem::size_of::<[uint8_t; 28]>() as
                              libc::c_ulong as uint16_t,};
             init
         },
         {
             let mut init =
                 desc{desc: gnuk_config_options.as_ptr(),
                      size:
                          ::std::mem::size_of::<[uint8_t; 86]>() as
                              libc::c_ulong as uint16_t,};
             init
         },
         {
             let mut init =
                 desc{desc: sys_version.as_ptr(),
                      size:
                          ::std::mem::size_of::<[uint8_t; 8]>() as
                              libc::c_ulong as uint16_t,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn usb_get_descriptor(mut dev: *mut usb_dev)
 -> libc::c_int {
    let mut arg: *mut device_req = &mut (*dev).dev_req;
    let mut rcp: uint8_t =
        ((*arg).type_0 as libc::c_int & 0x1f as libc::c_int) as uint8_t;
    let mut desc_type: uint8_t =
        ((*arg).value as libc::c_int >> 8 as libc::c_int) as uint8_t;
    let mut desc_index: uint8_t =
        ((*arg).value as libc::c_int & 0xff as libc::c_int) as uint8_t;
    if rcp as libc::c_int == DEVICE_RECIPIENT as libc::c_int {
        if desc_type as libc::c_int == DEVICE_DESCRIPTOR as libc::c_int {
            return usb_lld_ctrl_send(dev,
                                     device_desc.as_ptr() as
                                         *const libc::c_void,
                                     ::std::mem::size_of::<[uint8_t; 18]>() as
                                         libc::c_ulong)
        } else {
            if desc_type as libc::c_int == CONFIG_DESCRIPTOR as libc::c_int {
                return usb_lld_ctrl_send(dev,
                                         config_desc.as_ptr() as
                                             *const libc::c_void,
                                         ::std::mem::size_of::<[uint8_t; 93]>()
                                             as libc::c_ulong)
            } else {
                if desc_type as libc::c_int ==
                       STRING_DESCRIPTOR as libc::c_int {
                    if (desc_index as libc::c_ulong) <
                           (::std::mem::size_of::<[desc; 7]>() as
                                libc::c_ulong).wrapping_div(::std::mem::size_of::<desc>()
                                                                as
                                                                libc::c_ulong)
                       {
                        return usb_lld_ctrl_send(dev,
                                                 string_descriptors[desc_index
                                                                        as
                                                                        usize].desc
                                                     as *const libc::c_void,
                                                 string_descriptors[desc_index
                                                                        as
                                                                        usize].size
                                                     as size_t)
                    }
                }
            }
        }
    }
    return -(1 as libc::c_int);
}
