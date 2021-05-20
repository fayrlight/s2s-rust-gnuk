#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    #[no_mangle]
    fn usb_lld_ctrl_recv(dev: *mut usb_dev, p: *mut libc::c_void, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn usb_lld_ctrl_ack(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_lld_reset(dev: *mut usb_dev, feature: uint8_t);
    #[no_mangle]
    fn usb_lld_set_configuration(dev: *mut usb_dev, config: uint8_t);
    #[no_mangle]
    fn usb_lld_current_configuration(dev: *mut usb_dev) -> uint8_t;
    #[no_mangle]
    fn usb_lld_prepare_shutdown();
    #[no_mangle]
    fn usb_lld_setup_endpoint(ep_num: libc::c_int, ep_type: libc::c_int,
                              ep_kind: libc::c_int, ep_rx_addr: libc::c_int,
                              ep_tx_addr: libc::c_int,
                              ep_rx_memory_size: libc::c_int);
    #[no_mangle]
    fn usb_lld_stall_tx(ep_num: libc::c_int);
    #[no_mangle]
    fn usb_lld_stall_rx(ep_num: libc::c_int);
    #[no_mangle]
    fn ccid_card_change_signal(how: libc::c_int);
    #[no_mangle]
    fn ccid_usb_reset(_: libc::c_int);
    #[no_mangle]
    static ccid_state_p: *mut ccid_state;
    #[no_mangle]
    fn led_blink(spec: libc::c_int);
    #[no_mangle]
    static mut _regnual_start: uint8_t;
    #[no_mangle]
    static mut __heap_end__: [uint8_t; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type RECIPIENT_TYPE = libc::c_uint;
/* Recipient endpoint  */
pub const OTHER_RECIPIENT: RECIPIENT_TYPE = 3;
/* Recipient interface */
pub const ENDPOINT_RECIPIENT: RECIPIENT_TYPE = 2;
/* Recipient device    */
pub const INTERFACE_RECIPIENT: RECIPIENT_TYPE = 1;
pub const DEVICE_RECIPIENT: RECIPIENT_TYPE = 0;
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
/* CCID thread */
/* USB Rx data available  */
/* OpenPGP Execution finished */
/* CCID Tx finished  */
/* OpenPGPcard thread */
/* Maximum cmd apdu data is key import 24+4+256+256 (proc_key_import) */
/* without header */
/* Maximum res apdu data is public key 5+9+512 (gpg_do_public_key) */
/* without trailer */
/* USB buffer size of LL (Low-level): size of single Bulk transaction */
pub type ccid_state = libc::c_uint;
/* Exec requested */
/* ICC Thread Terminated */
pub const CCID_STATE_EXEC_REQUESTED: ccid_state = 7;
/* APDU Sent Partially */
pub const CCID_STATE_EXITED: ccid_state = 6;
/* APDU Received Partially */
pub const CCID_STATE_SEND: ccid_state = 5;
/* Busy4 */
pub const CCID_STATE_RECEIVE: ccid_state = 4;
/* Waiting APDU */
/* Busy1, Busy2, Busy3, Busy5 */
pub const CCID_STATE_EXECUTE: ccid_state = 3;
/* Initial */
pub const CCID_STATE_WAIT: ccid_state = 2;
/* No card available */
pub const CCID_STATE_START: ccid_state = 1;
pub const CCID_STATE_NOCARD: ccid_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RCC {
    pub CR: uint32_t,
    pub CFGR: uint32_t,
    pub CIR: uint32_t,
    pub APB2RSTR: uint32_t,
    pub APB1RSTR: uint32_t,
    pub AHBENR: uint32_t,
    pub APB2ENR: uint32_t,
    pub APB1ENR: uint32_t,
    pub BDCR: uint32_t,
    pub CSR: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CRC {
    pub DR: uint32_t,
    pub IDR: uint8_t,
    pub RESERVED0: uint8_t,
    pub RESERVED1: uint16_t,
    pub CR: uint32_t,
}
static mut RCC: *mut RCC =
    unsafe {
        (0x40000000 as libc::c_int + 0x20000 as libc::c_int +
             0x1000 as libc::c_int) as *mut RCC
    };
static mut CRC: *mut CRC =
    unsafe {
        (0x40000000 as libc::c_int + 0x20000 as libc::c_int +
             0x3000 as libc::c_int) as *mut CRC
    };
#[no_mangle]
pub static mut bDeviceState: uint32_t = 0 as libc::c_int as uint32_t;
/* USB device status */
/* NumLock=1, CapsLock=2, ScrollLock=4 */
unsafe extern "C" fn gnuk_setup_endpoints_for_interface(mut interface:
                                                            uint16_t,
                                                        mut stop:
                                                            libc::c_int) {
    if interface as libc::c_int == 0 as libc::c_int {
        if stop == 0 {
            usb_lld_setup_endpoint(1 as libc::c_int as uint8_t as libc::c_int,
                                   0 as libc::c_int, 0 as libc::c_int,
                                   0x100 as libc::c_int, 0xc0 as libc::c_int,
                                   64 as libc::c_int);
            usb_lld_setup_endpoint(2 as libc::c_int as uint8_t as libc::c_int,
                                   0x600 as libc::c_int, 0 as libc::c_int,
                                   0 as libc::c_int, 0x140 as libc::c_int,
                                   0 as libc::c_int);
        } else {
            usb_lld_stall_rx(1 as libc::c_int as uint8_t as libc::c_int);
            usb_lld_stall_tx(1 as libc::c_int as uint8_t as libc::c_int);
            usb_lld_stall_tx(2 as libc::c_int as uint8_t as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn usb_device_reset(mut dev: *mut usb_dev) {
    let mut i: libc::c_int = 0;
    usb_lld_reset(dev, 0x80 as libc::c_int as uint8_t);
    /* Initialize Endpoint 0 */
    usb_lld_setup_endpoint(0 as libc::c_int as uint8_t as libc::c_int,
                           0x200 as libc::c_int, 0 as libc::c_int,
                           0x40 as libc::c_int, 0x80 as libc::c_int,
                           64 as libc::c_int);
    /* Stop the interface */
    i = 0 as libc::c_int;
    while i <
              1 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int +
                  0 as libc::c_int {
        gnuk_setup_endpoints_for_interface(i as uint16_t, 1 as libc::c_int);
        i += 1
    }
    bDeviceState = 0 as libc::c_int as uint32_t;
    ccid_usb_reset(1 as libc::c_int);
}
static mut freq_table: [uint8_t; 4] =
    [0xa0 as libc::c_int as uint8_t, 0xf as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t];
/* dwDefaultClock */
static mut data_rate_table: [uint8_t; 4] =
    [0x80 as libc::c_int as uint8_t, 0x25 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t];
/* dwDataRate */
static mut mem_info: [*const uint8_t; 2] =
    unsafe {
        [&_regnual_start as *const uint8_t as *mut uint8_t as *const uint8_t,
         __heap_end__.as_ptr() as *mut _ as *const uint8_t]
    };
unsafe extern "C" fn rbit(mut v: uint32_t) -> uint32_t {
    let mut r: uint32_t = 0;
    asm!("rbit\t$0, $1" : "=r" (r) : "r" (v) :);
    return r;
}
/* After calling this function, CRC module remain enabled.  */
unsafe extern "C" fn download_check_crc32(mut dev: *mut usb_dev,
                                          mut end_p: *const uint32_t)
 -> libc::c_int {
    let mut crc32: uint32_t = *end_p;
    let mut p: *const uint32_t = 0 as *const uint32_t;
    ::std::ptr::write_volatile(&mut (*RCC).AHBENR as *mut uint32_t,
                               (::std::ptr::read_volatile::<uint32_t>(&(*RCC).AHBENR
                                                                          as
                                                                          *const uint32_t)
                                    as libc::c_uint |
                                    0x40 as libc::c_int as libc::c_uint) as
                                   uint32_t as uint32_t);
    ::std::ptr::write_volatile(&mut (*CRC).CR as *mut uint32_t,
                               0x1 as libc::c_int as uint32_t);
    p = &mut _regnual_start as *mut uint8_t as *const uint32_t;
    while p < end_p {
        ::std::ptr::write_volatile(&mut (*CRC).DR as *mut uint32_t, rbit(*p));
        p = p.offset(1)
    }
    if rbit((*CRC).DR) ^ crc32 == 0xffffffff as libc::c_uint {
        return usb_lld_ctrl_ack(dev)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn usb_setup(mut dev: *mut usb_dev) -> libc::c_int {
    let mut arg: *mut device_req = &mut (*dev).dev_req;
    let mut type_rcp: uint8_t =
        ((*arg).type_0 as libc::c_int &
             (0x60 as libc::c_int | 0x1f as libc::c_int)) as uint8_t;
    if type_rcp as libc::c_int ==
           0x40 as libc::c_int | DEVICE_RECIPIENT as libc::c_int {
        if (*arg).type_0 as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int {
            if (*arg).request as libc::c_int == 0 as libc::c_int {
                return usb_lld_ctrl_send(dev,
                                         mem_info.as_ptr() as
                                             *const libc::c_void,
                                         ::std::mem::size_of::<[*const uint8_t; 2]>()
                                             as libc::c_ulong)
            }
        } else {
            /* SETUP_SET */
            let mut addr: *mut uint8_t =
                (0x20000000 as libc::c_int +
                     (*arg).value as libc::c_int * 0x100 as libc::c_int +
                     (*arg).index as libc::c_int) as *mut uint8_t;
            if (*arg).request as libc::c_int == 1 as libc::c_int {
                if *ccid_state_p as libc::c_uint !=
                       CCID_STATE_EXITED as libc::c_int as libc::c_uint {
                    return -(1 as libc::c_int)
                }
                if addr < &mut _regnual_start as *mut uint8_t ||
                       addr.offset((*arg).len as libc::c_int as isize) >
                           __heap_end__.as_mut_ptr() {
                    return -(1 as libc::c_int)
                }
                if ((*arg).index as libc::c_int + (*arg).len as libc::c_int) <
                       256 as libc::c_int {
                    memset(addr.offset((*arg).index as libc::c_int as
                                           isize).offset((*arg).len as
                                                             libc::c_int as
                                                             isize) as
                               *mut libc::c_void, 0 as libc::c_int,
                           (256 as libc::c_int -
                                ((*arg).index as libc::c_int +
                                     (*arg).len as libc::c_int)) as
                               libc::c_ulong);
                }
                return usb_lld_ctrl_recv(dev, addr as *mut libc::c_void,
                                         (*arg).len as size_t)
            } else {
                if (*arg).request as libc::c_int == 2 as libc::c_int &&
                       (*arg).len as libc::c_int == 0 as libc::c_int {
                    if *ccid_state_p as libc::c_uint !=
                           CCID_STATE_EXITED as libc::c_int as libc::c_uint {
                        return -(1 as libc::c_int)
                    }
                    if addr as uint32_t & 0x3 as libc::c_int as libc::c_uint
                           != 0 {
                        return -(1 as libc::c_int)
                    }
                    return download_check_crc32(dev, addr as *mut uint32_t)
                } else {
                    if (*arg).request as libc::c_int == 3 as libc::c_int &&
                           (*arg).len as libc::c_int == 0 as libc::c_int {
                        if (*arg).value as libc::c_int != 0 as libc::c_int &&
                               (*arg).value as libc::c_int != 1 as libc::c_int
                               &&
                               (*arg).value as libc::c_int != 2 as libc::c_int
                           {
                            return -(1 as libc::c_int)
                        }
                        ccid_card_change_signal((*arg).value as libc::c_int);
                        return usb_lld_ctrl_ack(dev)
                    }
                }
            }
        }
    } else if type_rcp as libc::c_int ==
                  0x20 as libc::c_int | INTERFACE_RECIPIENT as libc::c_int {
        if (*arg).index as libc::c_int == 0 as libc::c_int {
            if (*arg).type_0 as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int {
                if (*arg).request as libc::c_int == 0x2 as libc::c_int {
                    return usb_lld_ctrl_send(dev,
                                             freq_table.as_ptr() as
                                                 *const libc::c_void,
                                             ::std::mem::size_of::<[uint8_t; 4]>()
                                                 as libc::c_ulong)
                } else {
                    if (*arg).request as libc::c_int == 0x3 as libc::c_int {
                        return usb_lld_ctrl_send(dev,
                                                 data_rate_table.as_ptr() as
                                                     *const libc::c_void,
                                                 ::std::mem::size_of::<[uint8_t; 4]>()
                                                     as libc::c_ulong)
                    }
                }
            } else if (*arg).request as libc::c_int == 0x1 as libc::c_int {
                /* wValue: bSeq, bSlot */
		/* Abortion is not supported in Gnuk */
                return -(1 as libc::c_int)
            }
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn usb_ctrl_write_finish(mut dev: *mut usb_dev) {
    let mut arg: *mut device_req = &mut (*dev).dev_req;
    let mut type_rcp: uint8_t =
        ((*arg).type_0 as libc::c_int &
             (0x60 as libc::c_int | 0x1f as libc::c_int)) as uint8_t;
    if type_rcp as libc::c_int ==
           0x40 as libc::c_int | DEVICE_RECIPIENT as libc::c_int {
        if (*arg).type_0 as libc::c_int & 0x80 as libc::c_int ==
               0 as libc::c_int &&
               (*arg).request as libc::c_int == 2 as libc::c_int {
            if *ccid_state_p as libc::c_uint !=
                   CCID_STATE_EXITED as libc::c_int as libc::c_uint {
                return
            }
            bDeviceState = 0 as libc::c_int as uint32_t;
            /* Notify the main.  */
            usb_lld_prepare_shutdown(); /* No further USB communication */
            led_blink(32 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn usb_set_configuration(mut dev: *mut usb_dev)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut current_conf: uint8_t = 0;
    current_conf = usb_lld_current_configuration(dev);
    if current_conf as libc::c_int == 0 as libc::c_int {
        if (*dev).dev_req.value as libc::c_int != 1 as libc::c_int {
            return -(1 as libc::c_int)
        }
        usb_lld_set_configuration(dev, 1 as libc::c_int as uint8_t);
        i = 0 as libc::c_int;
        while i <
                  1 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int +
                      0 as libc::c_int {
            gnuk_setup_endpoints_for_interface(i as uint16_t,
                                               0 as libc::c_int);
            i += 1
        }
        bDeviceState = 0 as libc::c_int as uint32_t
    } else if current_conf as libc::c_int !=
                  (*dev).dev_req.value as libc::c_int {
        if (*dev).dev_req.value as libc::c_int != 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        usb_lld_set_configuration(dev, 0 as libc::c_int as uint8_t);
        i = 0 as libc::c_int;
        while i <
                  1 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int +
                      0 as libc::c_int {
            gnuk_setup_endpoints_for_interface(i as uint16_t,
                                               1 as libc::c_int);
            i += 1
        }
        bDeviceState = 0 as libc::c_int as uint32_t;
        ccid_usb_reset(1 as libc::c_int);
    }
    /* Do nothing when current_conf == value */
    return usb_lld_ctrl_ack(dev);
}
#[no_mangle]
pub unsafe extern "C" fn usb_set_interface(mut dev: *mut usb_dev)
 -> libc::c_int {
    let mut interface: uint16_t = (*dev).dev_req.index;
    let mut alt: uint16_t = (*dev).dev_req.value;
    if interface as libc::c_int >=
           1 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int +
               0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if alt as libc::c_int != 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        gnuk_setup_endpoints_for_interface(interface, 0 as libc::c_int);
        ccid_usb_reset(0 as libc::c_int);
        return usb_lld_ctrl_ack(dev)
    };
}
#[no_mangle]
pub unsafe extern "C" fn usb_get_interface(mut dev: *mut usb_dev)
 -> libc::c_int {
    let zero: uint8_t = 0 as libc::c_int as uint8_t;
    let mut interface: uint16_t = (*dev).dev_req.index;
    if interface as libc::c_int >=
           1 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int +
               0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return usb_lld_ctrl_send(dev,
                             &zero as *const uint8_t as *const libc::c_void,
                             1 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn usb_get_status_interface(mut dev: *mut usb_dev)
 -> libc::c_int {
    let status_info: uint16_t = 0 as libc::c_int as uint16_t;
    let mut interface: uint16_t = (*dev).dev_req.index;
    if interface as libc::c_int >=
           1 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int +
               0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return usb_lld_ctrl_send(dev,
                             &status_info as *const uint16_t as
                                 *const libc::c_void,
                             2 as libc::c_int as size_t);
}
