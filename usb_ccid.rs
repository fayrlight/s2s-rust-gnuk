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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* NOTE: This signature is different to PTHREAD's one.  */
    #[no_mangle]
    fn chopstx_create(flags_and_prio: uint32_t, stack_addr: uintptr_t,
                      stack_size: size_t,
                      thread_entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      _: *mut libc::c_void) -> chopstx_t;
    #[no_mangle]
    fn chopstx_join(_: chopstx_t, _: *mut *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn chopstx_cancel(thd: chopstx_t);
    #[no_mangle]
    fn chopstx_claim_irq(intr: *mut chopstx_intr_t, irq_num: uint8_t);
    #[no_mangle]
    fn chopstx_intr_wait(intr: *mut chopstx_intr_t);
    #[no_mangle]
    fn chopstx_poll(usec_p: *mut uint32_t, n: libc::c_int,
                    pd_array: *const *mut chx_poll_head) -> libc::c_int;
    #[no_mangle]
    fn eventflag_init(ev: *mut eventflag);
    #[no_mangle]
    fn eventflag_signal(ev: *mut eventflag, m: eventmask_t);
    /* For polling */
    #[no_mangle]
    fn eventflag_prepare_poll(ev: *mut eventflag,
                              p: *mut chopstx_poll_cond_t);
    #[no_mangle]
    fn eventflag_get(ev: *mut eventflag) -> eventmask_t;
    #[no_mangle]
    fn led_blink(spec: libc::c_int);
    #[no_mangle]
    fn usb_lld_init(dev: *mut usb_dev, feature: uint8_t);
    #[no_mangle]
    fn usb_lld_event_handler(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_lld_ctrl_ack(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_lld_ctrl_error(dev: *mut usb_dev);
    /* EP_TYPE[1:0] EndPoint TYPE */
    /* EndPoint BULK        */
    /* EndPoint CONTROL     */
    /* EndPoint ISOCHRONOUS */
    /* EndPoint INTERRUPT   */
    #[no_mangle]
    fn usb_lld_tx_enable(ep_num: libc::c_int, len: size_t);
    #[no_mangle]
    fn usb_lld_rx_enable(ep_num: libc::c_int);
    #[no_mangle]
    fn usb_lld_txcpy(src: *const libc::c_void, ep_num: libc::c_int,
                     offset: libc::c_int, len: size_t);
    #[no_mangle]
    fn usb_lld_write(ep_num: uint8_t, buf: *const libc::c_void, len: size_t);
    #[no_mangle]
    fn usb_lld_rxcpy(dst: *mut uint8_t, ep_num: libc::c_int,
                     offset: libc::c_int, len: size_t);
    #[no_mangle]
    fn openpgp_card_thread(arg: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    static mut __process3_stack_base__: [uint8_t; 0];
    #[no_mangle]
    static mut __process3_stack_size__: [uint8_t; 0];
    #[no_mangle]
    static mut bDeviceState: uint32_t;
    #[no_mangle]
    fn usb_device_reset(dev: *mut usb_dev);
    #[no_mangle]
    fn usb_setup(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_ctrl_write_finish(dev: *mut usb_dev);
    #[no_mangle]
    fn usb_set_configuration(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_set_interface(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_get_interface(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_get_status_interface(dev: *mut usb_dev) -> libc::c_int;
    #[no_mangle]
    fn usb_get_descriptor(dev: *mut usb_dev) -> libc::c_int;
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
pub type chopstx_t = uintptr_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_poll_head {
    pub type_0: uint16_t,
    pub ready: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_poll_cond {
    pub type_0: uint16_t,
    pub ready: uint16_t,
    pub cond: *mut chopstx_cond_t,
    pub mutex: *mut chopstx_mutex_t,
    pub check: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                          -> libc::c_int>,
    pub arg: *mut libc::c_void,
}
pub type chopstx_poll_cond_t = chx_poll_cond;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chx_intr {
    pub type_0: uint16_t,
    pub ready: uint16_t,
    pub irq_num: uint8_t,
}
pub type chopstx_intr_t = chx_intr;
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
/* Data structure handled by CCID layer */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccid {
    pub ccid_state: ccid_state,
    pub state: uint8_t,
    pub err: uint8_t,
    pub p: *mut uint8_t,
    pub len: size_t,
    pub ccid_header: ccid_header,
    pub sw1sw2: [uint8_t; 2],
    pub chained_cls_ins_p1_p2: [uint8_t; 4],
    pub epo: *mut ep_out,
    pub epi: *mut ep_in,
    pub ccid_comm: eventflag,
    pub openpgp_comm: eventflag,
    pub application: chopstx_t,
    pub a: *mut apdu,
}
/*
 * There are three layers in USB CCID implementation
 *
 *   +-------------------+
 *   | Application Layer |
 *   +-------------------+
 *      ^ command APDU |
 *      |              v response APDU
 *   +-------------------+
 *   |    CCID Layer     |
 *   +-------------------+
 *    ^ CCID PC_to_RDR  | CCID RDR_to_PC
 *    | Message         v Message
 *   +-------------------+
 *   |    USB Layer      |
 *   +-------------------+
 *    ^ USB             | USB
 *    | Bulk-OUT Packet v Bulk-IN Packet
 *
 */
/*
 * USB layer data structures
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ep_in {
    pub ep_num: uint8_t,
    pub tx_done: uint8_t,
    pub buf: *const uint8_t,
    pub cnt: size_t,
    pub buf_len: size_t,
    pub priv_0: *mut libc::c_void,
    pub next_buf: Option<unsafe extern "C" fn(_: *mut ep_in, _: size_t)
                             -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ep_out {
    pub ep_num: uint8_t,
    pub err: uint8_t,
    pub buf: *mut uint8_t,
    pub cnt: size_t,
    pub buf_len: size_t,
    pub priv_0: *mut libc::c_void,
    pub next_buf: Option<unsafe extern "C" fn(_: *mut ep_out, _: size_t)
                             -> ()>,
    pub end_rx: Option<unsafe extern "C" fn(_: *mut ep_out, _: size_t)
                           -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ccid_header {
    pub msg_type: uint8_t,
    pub data_len: uint32_t,
    pub slot: uint8_t,
    pub seq: uint8_t,
    pub rsvd: uint8_t,
    pub param: uint16_t,
}
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
/* Mask to get request dir  */
/* Mask to get request type */
/* Standard request         */
/* Class request            */
/* Vendor request           */
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
pub type C2RustUnnamed = libc::c_uint;
/* Device addressed.  */
pub const USB_EVENT_DEVICE_ADDRESSED: C2RustUnnamed = 15;
pub const USB_EVENT_CTRL_WRITE_FINISH: C2RustUnnamed = 14;
/* Device Requests (Control READ/WRITE Transfer): Non-Standard */
pub const USB_EVENT_CTRL_REQUEST: C2RustUnnamed = 13;
pub const USB_EVENT_GET_INTERFACE: C2RustUnnamed = 12;
pub const USB_EVENT_GET_DESCRIPTOR: C2RustUnnamed = 11;
/* Device Requests (Control READ Transfer): Standard */
pub const USB_EVENT_GET_STATUS_INTERFACE: C2RustUnnamed = 10;
pub const USB_EVENT_CLEAR_FEATURE_ENDPOINT: C2RustUnnamed = 9;
pub const USB_EVENT_CLEAR_FEATURE_DEVICE: C2RustUnnamed = 8;
pub const USB_EVENT_SET_FEATURE_ENDPOINT: C2RustUnnamed = 7;
pub const USB_EVENT_SET_FEATURE_DEVICE: C2RustUnnamed = 6;
pub const USB_EVENT_SET_INTERFACE: C2RustUnnamed = 5;
/* Device Requests (Control WRITE Transfer): Standard */
pub const USB_EVENT_SET_CONFIGURATION: C2RustUnnamed = 4;
pub const USB_EVENT_DEVICE_WAKEUP: C2RustUnnamed = 3;
pub const USB_EVENT_DEVICE_SUSPEND: C2RustUnnamed = 2;
/* Processed in lower layer.  */
/* Device reset and suspend.  */
pub const USB_EVENT_DEVICE_RESET: C2RustUnnamed = 1;
pub const USB_EVENT_OK: C2RustUnnamed = 0;
/*
 * usb-ccid.c -- USB CCID protocol handling
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
/*
 * USB buffer size of USB-CCID driver
 */
#[no_mangle]
pub static mut apdu: apdu =
    apdu{seq: 0,
         cmd_apdu_head: 0 as *const uint8_t as *mut uint8_t,
         cmd_apdu_data: 0 as *const uint8_t as *mut uint8_t,
         cmd_apdu_data_len: 0,
         expected_res_size: 0,
         sw: 0,
         res_apdu_data_len: 0,
         res_apdu_data: 0 as *const uint8_t as *mut uint8_t,};
unsafe extern "C" fn epi_init(mut epi: *mut ep_in, mut ep_num: libc::c_int,
                              mut priv_0: *mut libc::c_void) {
    (*epi).ep_num = ep_num as uint8_t;
    (*epi).tx_done = 0 as libc::c_int as uint8_t;
    (*epi).buf = 0 as *const uint8_t;
    (*epi).cnt = 0 as libc::c_int as size_t;
    (*epi).buf_len = 0 as libc::c_int as size_t;
    (*epi).priv_0 = priv_0;
    (*epi).next_buf = None;
}
static mut endpoint_out: ep_out =
    ep_out{ep_num: 0,
           err: 0,
           buf: 0 as *const uint8_t as *mut uint8_t,
           cnt: 0,
           buf_len: 0,
           priv_0: 0 as *const libc::c_void as *mut libc::c_void,
           next_buf: None,
           end_rx: None,};
static mut endpoint_in: ep_in =
    ep_in{ep_num: 0,
          tx_done: 0,
          buf: 0 as *const uint8_t,
          cnt: 0,
          buf_len: 0,
          priv_0: 0 as *const libc::c_void as *mut libc::c_void,
          next_buf: None,};
unsafe extern "C" fn epo_init(mut epo: *mut ep_out, mut ep_num: libc::c_int,
                              mut priv_0: *mut libc::c_void) {
    (*epo).ep_num = ep_num as uint8_t;
    (*epo).err = 0 as libc::c_int as uint8_t;
    (*epo).buf = 0 as *mut uint8_t;
    (*epo).cnt = 0 as libc::c_int as size_t;
    (*epo).buf_len = 0 as libc::c_int as size_t;
    (*epo).priv_0 = priv_0;
    (*epo).next_buf = None;
    (*epo).end_rx = None;
}
/*
 * CCID Layer
 */
/*
 * Buffer of USB communication: for both of RX and TX
 *
 * The buffer will be filled by multiple RX packets (Bulk-OUT)
 * or will be used for multiple TX packets (Bulk-IN)
 */
static mut ccid_buffer: [uint8_t; 545] = [0; 545];
unsafe extern "C" fn ccid_reset(mut c: *mut ccid) {
    (*c).err = 0 as libc::c_int as uint8_t; /* will be set by lower layer */
    (*c).state = 0 as libc::c_int as uint8_t; /* will be set by lower layer */
    (*c).p = (*(*c).a).cmd_apdu_data; /* will be set by lower layer */
    (*c).len =
        (24 as libc::c_int + 4 as libc::c_int + 256 as libc::c_int +
             256 as libc::c_int) as size_t; /* will be set by upper layer */
    (*(*c).a).cmd_apdu_data_len =
        0 as libc::c_int as uint16_t; /* will be set by upper layer */
    (*(*c).a).expected_res_size = 0 as libc::c_int as uint16_t;
}
unsafe extern "C" fn ccid_init(mut c: *mut ccid, mut epi: *mut ep_in,
                               mut epo: *mut ep_out, mut a: *mut apdu) {
    (*c).ccid_state = CCID_STATE_START;
    (*c).state = 0 as libc::c_int as uint8_t;
    (*c).p = (*a).cmd_apdu_data;
    (*c).len =
        (24 as libc::c_int + 4 as libc::c_int + 256 as libc::c_int +
             256 as libc::c_int) as size_t;
    (*c).err = 0 as libc::c_int as uint8_t;
    memset(&mut (*c).ccid_header as *mut ccid_header as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<ccid_header>() as libc::c_ulong);
    (*c).sw1sw2[0 as libc::c_int as usize] = 0x90 as libc::c_int as uint8_t;
    (*c).sw1sw2[1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    (*c).application = 0 as libc::c_int as chopstx_t;
    (*c).epi = epi;
    (*c).epo = epo;
    (*c).a = a;
}
unsafe extern "C" fn apdu_init(mut a: *mut apdu) {
    (*a).seq = 0 as libc::c_int as uint8_t;
    (*a).cmd_apdu_head =
        &mut *ccid_buffer.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut uint8_t;
    (*a).cmd_apdu_data =
        &mut *ccid_buffer.as_mut_ptr().offset(5 as libc::c_int as isize) as
            *mut uint8_t;
    (*a).cmd_apdu_data_len = 0 as libc::c_int as uint16_t;
    (*a).expected_res_size = 0 as libc::c_int as uint16_t;
    (*a).sw = 0x9000 as libc::c_int as uint16_t;
    (*a).res_apdu_data =
        &mut *ccid_buffer.as_mut_ptr().offset(5 as libc::c_int as isize) as
            *mut uint8_t;
    (*a).res_apdu_data_len = 0 as libc::c_int as uint16_t;
    /* will be set by upper layer */
}
unsafe extern "C" fn notify_tx(mut epi: *mut ep_in) {
    let mut c: *mut ccid = (*epi).priv_0 as *mut ccid;
    /* The sequence of Bulk-IN transactions finished */
    eventflag_signal(&mut (*c).ccid_comm, 4 as libc::c_int as eventmask_t);
}
unsafe extern "C" fn no_buf(mut epi: *mut ep_in, mut len: size_t) {
    (*epi).buf = 0 as *const uint8_t;
    (*epi).cnt = 0 as libc::c_int as size_t;
    (*epi).buf_len = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn set_sw1sw2(mut c: *mut ccid, mut chunk_len: size_t) {
    if (*(*c).a).expected_res_size as libc::c_ulong >= (*c).len {
        (*c).sw1sw2[0 as libc::c_int as usize] =
            0x90 as libc::c_int as uint8_t;
        (*c).sw1sw2[1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t
    } else {
        (*c).sw1sw2[0 as libc::c_int as usize] =
            0x61 as libc::c_int as uint8_t;
        if (*c).len.wrapping_sub(chunk_len) >=
               256 as libc::c_int as libc::c_ulong {
            (*c).sw1sw2[1 as libc::c_int as usize] =
                0 as libc::c_int as uint8_t
        } else {
            (*c).sw1sw2[1 as libc::c_int as usize] =
                (*c).len.wrapping_sub(chunk_len) as uint8_t
        }
    };
}
unsafe extern "C" fn get_sw1sw2(mut epi: *mut ep_in, mut len: size_t) {
    let mut c: *mut ccid = (*epi).priv_0 as *mut ccid;
    (*epi).buf = (*c).sw1sw2.as_mut_ptr();
    (*epi).cnt = 0 as libc::c_int as size_t;
    (*epi).buf_len = 2 as libc::c_int as size_t;
    (*epi).next_buf =
        Some(no_buf as unsafe extern "C" fn(_: *mut ep_in, _: size_t) -> ());
}
/*
 * Tx done callback
 */
unsafe extern "C" fn EP1_IN_Callback(mut len: uint16_t) {
    let mut epi: *mut ep_in = &mut endpoint_in;
    if (*epi).buf.is_null() {
        if (*epi).tx_done != 0 {
            notify_tx(epi);
        } else {
            (*epi).tx_done = 1 as libc::c_int as uint8_t;
            usb_lld_tx_enable((*epi).ep_num as libc::c_int,
                              0 as libc::c_int as size_t);
            /* send ZLP */
        }
    } else {
        let mut tx_size: libc::c_int = 0 as libc::c_int;
        let mut remain: size_t = 64 as libc::c_int as size_t;
        let mut offset: libc::c_int = 0 as libc::c_int;
        while !(*epi).buf.is_null() {
            if (*epi).buf_len < remain {
                usb_lld_txcpy((*epi).buf as *const libc::c_void,
                              (*epi).ep_num as libc::c_int, offset,
                              (*epi).buf_len);
                offset =
                    (offset as libc::c_ulong).wrapping_add((*epi).buf_len) as
                        libc::c_int as libc::c_int;
                remain =
                    (remain as libc::c_ulong).wrapping_sub((*epi).buf_len) as
                        size_t as size_t;
                tx_size =
                    (tx_size as libc::c_ulong).wrapping_add((*epi).buf_len) as
                        libc::c_int as libc::c_int;
                (*epi).next_buf.expect("non-null function pointer")(epi,
                                                                    remain);
                /* Update epi->buf, cnt, buf_len */
            } else {
                usb_lld_txcpy((*epi).buf as *const libc::c_void,
                              (*epi).ep_num as libc::c_int, offset, remain);
                (*epi).buf = (*epi).buf.offset(remain as isize);
                (*epi).cnt =
                    ((*epi).cnt as libc::c_ulong).wrapping_add(remain) as
                        size_t as size_t;
                (*epi).buf_len =
                    ((*epi).buf_len as libc::c_ulong).wrapping_sub(remain) as
                        size_t as size_t;
                tx_size =
                    (tx_size as libc::c_ulong).wrapping_add(remain) as
                        libc::c_int as libc::c_int;
                break ;
            }
        }
        if tx_size < 64 as libc::c_int {
            (*epi).tx_done = 1 as libc::c_int as uint8_t
        }
        usb_lld_tx_enable((*epi).ep_num as libc::c_int, tx_size as size_t);
    };
}
unsafe extern "C" fn notify_icc(mut epo: *mut ep_out) {
    let mut c: *mut ccid = (*epo).priv_0 as *mut ccid;
    (*c).err = (*epo).err;
    eventflag_signal(&mut (*c).ccid_comm, 1 as libc::c_int as eventmask_t);
}
unsafe extern "C" fn end_ccid_rx(mut epo: *mut ep_out, mut orig_len: size_t)
 -> libc::c_int {
    if (*epo).cnt < ::std::mem::size_of::<ccid_header>() as libc::c_ulong {
        /* short packet, just ignore */
        return 1 as libc::c_int
    }
    /* icc message with no abdata */
    return 0 as libc::c_int;
}
unsafe extern "C" fn end_abdata(mut epo: *mut ep_out, mut orig_len: size_t)
 -> libc::c_int {
    let mut c: *mut ccid = (*epo).priv_0 as *mut ccid;
    let mut len: size_t = (*epo).cnt;
    if orig_len == 64 as libc::c_int as libc::c_ulong &&
           len < (*c).ccid_header.data_len as libc::c_ulong {
        /* more packet comes */
        return 1 as libc::c_int
    }
    if len != (*c).ccid_header.data_len as libc::c_ulong {
        (*epo).err = 1 as libc::c_int as uint8_t
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn end_cmd_apdu_head(mut epo: *mut ep_out,
                                       mut orig_len: size_t) -> libc::c_int {
    let mut c: *mut ccid = (*epo).priv_0 as *mut ccid;
    if (*epo).cnt < 4 as libc::c_int as libc::c_ulong ||
           (*epo).cnt != (*c).ccid_header.data_len as libc::c_ulong {
        (*epo).err = 1 as libc::c_int as uint8_t;
        return 0 as libc::c_int
    }
    if (*c).state as libc::c_int == 1 as libc::c_int &&
           ((*c).chained_cls_ins_p1_p2[0 as libc::c_int as usize] as
                libc::c_int !=
                *(*(*c).a).cmd_apdu_head.offset(0 as libc::c_int as isize) as
                    libc::c_int & !(0x10 as libc::c_int) ||
                (*c).chained_cls_ins_p1_p2[1 as libc::c_int as usize] as
                    libc::c_int !=
                    *(*(*c).a).cmd_apdu_head.offset(1 as libc::c_int as isize)
                        as libc::c_int ||
                (*c).chained_cls_ins_p1_p2[2 as libc::c_int as usize] as
                    libc::c_int !=
                    *(*(*c).a).cmd_apdu_head.offset(2 as libc::c_int as isize)
                        as libc::c_int ||
                (*c).chained_cls_ins_p1_p2[3 as libc::c_int as usize] as
                    libc::c_int !=
                    *(*(*c).a).cmd_apdu_head.offset(3 as libc::c_int as isize)
                        as libc::c_int) {
        /*
     * Handling exceptional request.
     *
     * Host stops sending command APDU using command chaining,
     * and start another command APDU.
     *
     * Discard old one, and start handling new one.
     */
        (*c).state = 0 as libc::c_int as uint8_t;
        (*c).p = (*(*c).a).cmd_apdu_data;
        (*c).len =
            (24 as libc::c_int + 4 as libc::c_int + 256 as libc::c_int +
                 256 as libc::c_int) as size_t
    }
    if (*epo).cnt == 4 as libc::c_int as libc::c_ulong {
        /* No Lc and Le */
        (*(*c).a).expected_res_size = 0 as libc::c_int as uint16_t
    } else if (*epo).cnt == 5 as libc::c_int as libc::c_ulong {
        /* No Lc but Le */
        (*(*c).a).expected_res_size =
            *(*(*c).a).cmd_apdu_head.offset(4 as libc::c_int as isize) as
                uint16_t;
        if (*(*c).a).expected_res_size as libc::c_int == 0 as libc::c_int {
            (*(*c).a).expected_res_size = 256 as libc::c_int as uint16_t
        }
        *(*(*c).a).cmd_apdu_head.offset(4 as libc::c_int as isize) =
            0 as libc::c_int as uint8_t
    }
    (*(*c).a).cmd_apdu_data_len = 0 as libc::c_int as uint16_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn end_nomore_data(mut epo: *mut ep_out,
                                     mut orig_len: size_t) -> libc::c_int {
    if orig_len == 64 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn end_cmd_apdu_data(mut epo: *mut ep_out,
                                       mut orig_len: size_t) -> libc::c_int {
    let mut current_block: u64;
    let mut c: *mut ccid = (*epo).priv_0 as *mut ccid;
    let mut len: size_t = (*epo).cnt;
    if orig_len == 64 as libc::c_int as libc::c_ulong &&
           (5 as libc::c_int as libc::c_ulong).wrapping_add(len) <
               (*c).ccid_header.data_len as libc::c_ulong {
        /* more packet comes */
        return 1 as libc::c_int
    }
    if !((5 as libc::c_int as libc::c_ulong).wrapping_add(len) !=
             (*c).ccid_header.data_len as libc::c_ulong) {
        if len ==
               *(*(*c).a).cmd_apdu_head.offset(4 as libc::c_int as isize) as
                   libc::c_ulong {
            /* No Le field*/
            (*(*c).a).expected_res_size = 0 as libc::c_int as uint16_t;
            current_block = 11050875288958768710;
        } else if len ==
                      (*(*(*c).a).cmd_apdu_head.offset(4 as libc::c_int as
                                                           isize) as
                           size_t).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong) {
            /* it has Le field*/
            (*(*c).a).expected_res_size =
                *(*epo).buf.offset(-(1 as libc::c_int) as isize) as uint16_t;
            if (*(*c).a).expected_res_size as libc::c_int == 0 as libc::c_int
               {
                (*(*c).a).expected_res_size = 256 as libc::c_int as uint16_t
            }
            len = len.wrapping_sub(1);
            current_block = 11050875288958768710;
        } else { current_block = 13431308959347868023; }
        match current_block {
            13431308959347868023 => { }
            _ => {
                (*(*c).a).cmd_apdu_data_len =
                    ((*(*c).a).cmd_apdu_data_len as
                         libc::c_ulong).wrapping_add(len) as uint16_t as
                        uint16_t;
                return 0 as libc::c_int
            }
        }
    }
    (*epo).err = 1 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn nomore_data(mut epo: *mut ep_out, mut len: size_t) {
    (*epo).err = 1 as libc::c_int as uint8_t;
    (*epo).end_rx =
        Some(end_nomore_data as
                 unsafe extern "C" fn(_: *mut ep_out, _: size_t)
                     -> libc::c_int);
    (*epo).buf = 0 as *mut uint8_t;
    (*epo).buf_len = 0 as libc::c_int as size_t;
    (*epo).cnt = 0 as libc::c_int as size_t;
    (*epo).next_buf =
        Some(nomore_data as
                 unsafe extern "C" fn(_: *mut ep_out, _: size_t) -> ());
}
unsafe extern "C" fn ccid_cmd_apdu_data(mut epo: *mut ep_out,
                                        mut len: size_t) {
    let mut c: *mut ccid = (*epo).priv_0 as *mut ccid;
    if (*c).state as libc::c_int == 4 as libc::c_int &&
           *(*(*c).a).cmd_apdu_head.offset(1 as libc::c_int as isize) as
               libc::c_int != 0xc0 as libc::c_int {
        /*
       * Handling exceptional request.
       *
       * Host didn't finish receiving the whole response APDU by GET RESPONSE,
       * but initiates another command.
       */
        (*c).state = 0 as libc::c_int as uint8_t;
        (*c).p = (*(*c).a).cmd_apdu_data;
        (*c).len =
            (24 as libc::c_int + 4 as libc::c_int + 256 as libc::c_int +
                 256 as libc::c_int) as size_t
    } else if (*c).state as libc::c_int == 1 as libc::c_int {
        if (*c).chained_cls_ins_p1_p2[0 as libc::c_int as usize] as
               libc::c_int !=
               *(*(*c).a).cmd_apdu_head.offset(0 as libc::c_int as isize) as
                   libc::c_int & !(0x10 as libc::c_int) ||
               (*c).chained_cls_ins_p1_p2[1 as libc::c_int as usize] as
                   libc::c_int !=
                   *(*(*c).a).cmd_apdu_head.offset(1 as libc::c_int as isize)
                       as libc::c_int ||
               (*c).chained_cls_ins_p1_p2[2 as libc::c_int as usize] as
                   libc::c_int !=
                   *(*(*c).a).cmd_apdu_head.offset(2 as libc::c_int as isize)
                       as libc::c_int ||
               (*c).chained_cls_ins_p1_p2[3 as libc::c_int as usize] as
                   libc::c_int !=
                   *(*(*c).a).cmd_apdu_head.offset(3 as libc::c_int as isize)
                       as libc::c_int {
            /*
	 * Handling exceptional request.
	 *
	 * Host stops sending command APDU using command chaining,
	 * and start another command APDU.
	 *
	 * Discard old one, and start handling new one.
	 */
            (*c).state = 0 as libc::c_int as uint8_t;
            (*c).p = (*(*c).a).cmd_apdu_data;
            (*c).len =
                (24 as libc::c_int + 4 as libc::c_int + 256 as libc::c_int +
                     256 as libc::c_int) as size_t;
            (*(*c).a).cmd_apdu_data_len = 0 as libc::c_int as uint16_t
        }
    }
    (*epo).end_rx =
        Some(end_cmd_apdu_data as
                 unsafe extern "C" fn(_: *mut ep_out, _: size_t)
                     -> libc::c_int);
    (*epo).buf = (*c).p;
    (*epo).buf_len = (*c).len;
    (*epo).cnt = 0 as libc::c_int as size_t;
    (*epo).next_buf =
        Some(nomore_data as
                 unsafe extern "C" fn(_: *mut ep_out, _: size_t) -> ());
}
unsafe extern "C" fn ccid_abdata(mut epo: *mut ep_out, mut len: size_t) {
    let mut c: *mut ccid = (*epo).priv_0 as *mut ccid;
    (*(*c).a).seq = (*c).ccid_header.seq;
    if (*c).ccid_header.msg_type as libc::c_int == 0x6f as libc::c_int {
        (*(*c).a).seq = (*c).ccid_header.seq;
        (*epo).end_rx =
            Some(end_cmd_apdu_head as
                     unsafe extern "C" fn(_: *mut ep_out, _: size_t)
                         -> libc::c_int);
        (*epo).buf = (*(*c).a).cmd_apdu_head;
        (*epo).buf_len = 5 as libc::c_int as size_t;
        (*epo).cnt = 0 as libc::c_int as size_t;
        (*epo).next_buf =
            Some(ccid_cmd_apdu_data as
                     unsafe extern "C" fn(_: *mut ep_out, _: size_t) -> ())
    } else {
        (*epo).end_rx =
            Some(end_abdata as
                     unsafe extern "C" fn(_: *mut ep_out, _: size_t)
                         -> libc::c_int);
        (*epo).buf = (*c).p;
        (*epo).buf_len = (*c).len;
        (*epo).cnt = 0 as libc::c_int as size_t;
        (*epo).next_buf =
            Some(nomore_data as
                     unsafe extern "C" fn(_: *mut ep_out, _: size_t) -> ())
    };
}
unsafe extern "C" fn ccid_prepare_receive(mut c: *mut ccid) {
    (*(*c).epo).err = 0 as libc::c_int as uint8_t;
    (*(*c).epo).buf =
        &mut (*c).ccid_header as *mut ccid_header as *mut uint8_t;
    (*(*c).epo).buf_len =
        ::std::mem::size_of::<ccid_header>() as libc::c_ulong;
    (*(*c).epo).cnt = 0 as libc::c_int as size_t;
    (*(*c).epo).next_buf =
        Some(ccid_abdata as
                 unsafe extern "C" fn(_: *mut ep_out, _: size_t) -> ());
    (*(*c).epo).end_rx =
        Some(end_ccid_rx as
                 unsafe extern "C" fn(_: *mut ep_out, _: size_t)
                     -> libc::c_int);
    usb_lld_rx_enable((*(*c).epo).ep_num as libc::c_int);
}
/*
 * Rx ready callback
 */
unsafe extern "C" fn EP1_OUT_Callback(mut len: uint16_t) {
    let mut epo: *mut ep_out = &mut endpoint_out;
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut cont: libc::c_int = 0;
    let mut orig_len: size_t = len as size_t;
    while (*epo).err as libc::c_int == 0 as libc::c_int {
        if len as libc::c_int == 0 as libc::c_int { break ; }
        if len as libc::c_ulong <= (*epo).buf_len {
            usb_lld_rxcpy((*epo).buf, (*epo).ep_num as libc::c_int, offset,
                          len as size_t);
            (*epo).buf = (*epo).buf.offset(len as libc::c_int as isize);
            (*epo).cnt =
                ((*epo).cnt as
                     libc::c_ulong).wrapping_add(len as libc::c_ulong) as
                    size_t as size_t;
            (*epo).buf_len =
                ((*epo).buf_len as
                     libc::c_ulong).wrapping_sub(len as libc::c_ulong) as
                    size_t as size_t;
            break ;
        } else {
            /* len > buf_len */
            usb_lld_rxcpy((*epo).buf, (*epo).ep_num as libc::c_int, offset,
                          (*epo).buf_len);
            len =
                (len as libc::c_ulong).wrapping_sub((*epo).buf_len) as
                    uint16_t as uint16_t;
            offset =
                (offset as libc::c_ulong).wrapping_add((*epo).buf_len) as
                    libc::c_int as libc::c_int;
            (*epo).next_buf.expect("non-null function pointer")(epo,
                                                                len as
                                                                    size_t);
            /* Update epo->buf, cnt, buf_len */
        }
    }
    /*
   * ORIG_LEN to distingush ZLP and the end of transaction
   *  (ORIG_LEN != USB_LL_BUF_SIZE)
   */
    cont = (*epo).end_rx.expect("non-null function pointer")(epo, orig_len);
    if cont != 0 {
        usb_lld_rx_enable((*epo).ep_num as libc::c_int);
    } else { notify_icc(epo); };
}
unsafe extern "C" fn usb_rx_ready(mut ep_num: uint8_t, mut len: uint16_t) {
    if ep_num as libc::c_int == 1 as libc::c_int as uint8_t as libc::c_int {
        EP1_OUT_Callback(len);
    };
}
unsafe extern "C" fn usb_tx_done(mut ep_num: uint8_t, mut len: uint16_t) {
    if ep_num as libc::c_int == 1 as libc::c_int as uint8_t as libc::c_int {
        EP1_IN_Callback(len);
    } else {
        (ep_num as libc::c_int) == 2 as libc::c_int as uint8_t as libc::c_int;
    };
}
/*
 * ATR (Answer To Reset) string
 *
 * TS = 0x3b: Direct conversion
 * T0 = 0xda: TA1, TC1 and TD1 follow, 10 historical bytes
 * TA1 = 0x11: FI=1, DI=1
 * TC1 = 0xff
 * TD1 = 0x81: TD2 follows, T=1
 * TD2 = 0xb1: TA3, TB3 and TD3 follow, T=1
 * TA3 = 0xFE: IFSC = 254 bytes
 * TB3 = 0x55: BWI = 5, CWI = 5   (BWT timeout 3.2 sec)
 * TD3 = 0x1f: TA4 follows, T=15
 * TA4 = 0x03: 5V or 3.3V
 * Historical bytes: to be explained...
 * XOR check
 *
 * Minimum: 0x3b, 0x8a, 0x80, 0x01, + historical bytes, xor check
 *
 */
static mut ATR: [uint8_t; 21] =
    [0x3b as libc::c_int as uint8_t, 0xda as libc::c_int as uint8_t,
     0x11 as libc::c_int as uint8_t, 0xff as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0xb1 as libc::c_int as uint8_t,
     0xfe as libc::c_int as uint8_t, 0x55 as libc::c_int as uint8_t,
     0x1f as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x31 as libc::c_int as uint8_t,
     0x84 as libc::c_int as uint8_t, 0x73 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x90 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     (0xda as libc::c_int ^ 0x11 as libc::c_int ^ 0xff as libc::c_int ^
          0x81 as libc::c_int ^ 0xb1 as libc::c_int ^ 0xfe as libc::c_int ^
          0x55 as libc::c_int ^ 0x1f as libc::c_int ^ 0x3 as libc::c_int ^
          0 as libc::c_int ^ 0x31 as libc::c_int ^ 0x84 as libc::c_int ^
          0x73 as libc::c_int ^ 0x80 as libc::c_int ^ 0x1 as libc::c_int ^
          0x80 as libc::c_int ^ 0 as libc::c_int ^ 0x90 as libc::c_int ^
          0 as libc::c_int) as uint8_t];
/*
 * Send back error
 */
unsafe extern "C" fn ccid_error(mut c: *mut ccid, mut offset: libc::c_int) {
    let mut ccid_reply: [uint8_t; 10] = [0; 10]; /* Any value should be OK */
    ccid_reply[0 as libc::c_int as usize] =
        0x81 as libc::c_int as uint8_t; /* Slot */
    ccid_reply[1 as libc::c_int as usize] =
        0 as libc::c_int as uint8_t; /* An ICC is present and active */
    ccid_reply[2 as libc::c_int as usize] =
        0 as libc::c_int as uint8_t; /* 2: No ICC present */
    ccid_reply[3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[4 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[6 as libc::c_int as usize] = (*c).ccid_header.seq;
    if (*c).ccid_state as libc::c_uint ==
           CCID_STATE_NOCARD as libc::c_int as libc::c_uint {
        ccid_reply[7 as libc::c_int as usize] = 2 as libc::c_int as uint8_t
    } else if (*c).ccid_state as libc::c_uint ==
                  CCID_STATE_START as libc::c_int as libc::c_uint {
        /* 1: ICC present but not activated */
        ccid_reply[7 as libc::c_int as usize] = 1 as libc::c_int as uint8_t
    } else {
        ccid_reply[7 as libc::c_int as usize] = 0 as libc::c_int as uint8_t
    } /* Failed */
    ccid_reply[7 as libc::c_int as usize] =
        (ccid_reply[7 as libc::c_int as usize] as libc::c_int |
             0x40 as libc::c_int) as uint8_t;
    ccid_reply[8 as libc::c_int as usize] = offset as uint8_t;
    ccid_reply[9 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    /* This is a single packet Bulk-IN transaction */
    (*(*c).epi).buf = 0 as *const uint8_t;
    (*(*c).epi).tx_done = 1 as libc::c_int as uint8_t;
    usb_lld_write((*(*c).epi).ep_num,
                  ccid_reply.as_mut_ptr() as *const libc::c_void,
                  10 as libc::c_int as size_t);
}
/* Send back ATR (Answer To Reset) */
unsafe extern "C" fn ccid_power_on(mut c: *mut ccid) -> ccid_state {
    let mut size_atr: size_t =
        ::std::mem::size_of::<[uint8_t; 21]>() as libc::c_ulong; /* Slot */
    let mut p: [uint8_t; 10] = [0; 10];
    if (*c).application == 0 as libc::c_int as libc::c_ulong {
        (*c).application =
            chopstx_create(1 as libc::c_int as uint32_t,
                           __process3_stack_base__.as_mut_ptr() as uint32_t as
                               uintptr_t,
                           __process3_stack_size__.as_mut_ptr() as uint32_t as
                               size_t,
                           Some(openpgp_card_thread as
                                    unsafe extern "C" fn(_: *mut libc::c_void)
                                        -> *mut libc::c_void),
                           &mut (*c).ccid_comm as *mut eventflag as
                               *mut libc::c_void)
    }
    p[0 as libc::c_int as usize] = 0x80 as libc::c_int as uint8_t;
    p[1 as libc::c_int as usize] = size_atr as uint8_t;
    p[2 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[4 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[6 as libc::c_int as usize] = (*c).ccid_header.seq;
    p[7 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[8 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[9 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    usb_lld_txcpy(p.as_mut_ptr() as *const libc::c_void,
                  (*(*c).epi).ep_num as libc::c_int, 0 as libc::c_int,
                  10 as libc::c_int as size_t);
    usb_lld_txcpy(ATR.as_ptr() as *const libc::c_void,
                  (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                  size_atr);
    /* This is a single packet Bulk-IN transaction */
    (*(*c).epi).buf = 0 as *const uint8_t; /* Slot */
    (*(*c).epi).tx_done =
        1 as libc::c_int as uint8_t; /* An ICC is present and active */
    usb_lld_tx_enable((*(*c).epi).ep_num as libc::c_int,
                      (10 as libc::c_int as
                           libc::c_ulong).wrapping_add(size_atr)); /* 2: No ICC present */
    return CCID_STATE_WAIT;
}
unsafe extern "C" fn ccid_send_status(mut c: *mut ccid) {
    let mut ccid_reply: [uint8_t; 10] = [0; 10];
    ccid_reply[0 as libc::c_int as usize] = 0x81 as libc::c_int as uint8_t;
    ccid_reply[1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[2 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[4 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[6 as libc::c_int as usize] = (*c).ccid_header.seq;
    if (*c).ccid_state as libc::c_uint ==
           CCID_STATE_NOCARD as libc::c_int as libc::c_uint {
        ccid_reply[7 as libc::c_int as usize] = 2 as libc::c_int as uint8_t
    } else if (*c).ccid_state as libc::c_uint ==
                  CCID_STATE_START as libc::c_int as libc::c_uint {
        /* 1: ICC present but not activated */
        ccid_reply[7 as libc::c_int as usize] = 1 as libc::c_int as uint8_t
    } else {
        ccid_reply[7 as libc::c_int as usize] = 0 as libc::c_int as uint8_t
    }
    ccid_reply[8 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    ccid_reply[9 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    /* This is a single packet Bulk-IN transaction */
    (*(*c).epi).buf =
        0 as *const uint8_t; /* This status change should be here */
    (*(*c).epi).tx_done = 1 as libc::c_int as uint8_t; /* Slot */
    usb_lld_write((*(*c).epi).ep_num,
                  ccid_reply.as_mut_ptr() as *const libc::c_void,
                  10 as libc::c_int as size_t); /* Slot */
    led_blink(4 as libc::c_int);
}
unsafe extern "C" fn ccid_power_off(mut c: *mut ccid) -> ccid_state {
    if (*c).application != 0 {
        eventflag_signal(&mut (*c).openpgp_comm,
                         2 as libc::c_int as eventmask_t);
        chopstx_join((*c).application, 0 as *mut *mut libc::c_void);
        (*c).application = 0 as libc::c_int as chopstx_t
    }
    (*c).ccid_state = CCID_STATE_START;
    ccid_send_status(c);
    return CCID_STATE_START;
}
unsafe extern "C" fn ccid_send_data_block_internal(mut c: *mut ccid,
                                                   mut status: uint8_t,
                                                   mut error: uint8_t) {
    let mut tx_size: libc::c_int = 64 as libc::c_int;
    let mut p: [uint8_t; 10] = [0; 10];
    let mut len: size_t = 0;
    if status as libc::c_int == 0 as libc::c_int {
        len =
            ((*(*c).a).res_apdu_data_len as libc::c_int + 2 as libc::c_int) as
                size_t
    } else { len = 0 as libc::c_int as size_t }
    p[0 as libc::c_int as usize] = 0x80 as libc::c_int as uint8_t;
    p[1 as libc::c_int as usize] =
        (len & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    p[2 as libc::c_int as usize] =
        (len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[3 as libc::c_int as usize] =
        (len >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[4 as libc::c_int as usize] =
        (len >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[6 as libc::c_int as usize] = (*(*c).a).seq;
    p[7 as libc::c_int as usize] = status;
    p[8 as libc::c_int as usize] = error;
    p[9 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    usb_lld_txcpy(p.as_mut_ptr() as *const libc::c_void,
                  (*(*c).epi).ep_num as libc::c_int, 0 as libc::c_int,
                  10 as libc::c_int as size_t);
    if len == 0 as libc::c_int as libc::c_ulong {
        usb_lld_tx_enable((*(*c).epi).ep_num as libc::c_int,
                          10 as libc::c_int as size_t);
        return
    }
    if (10 as libc::c_int as libc::c_ulong).wrapping_add(len) <=
           64 as libc::c_int as libc::c_ulong {
        usb_lld_txcpy((*(*c).a).res_apdu_data as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                      (*(*c).a).res_apdu_data_len as size_t);
        usb_lld_txcpy((*c).sw1sw2.as_mut_ptr() as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int,
                      10 as libc::c_int +
                          (*(*c).a).res_apdu_data_len as libc::c_int,
                      2 as libc::c_int as size_t);
        (*(*c).epi).buf = 0 as *const uint8_t;
        if (10 as libc::c_int as libc::c_ulong).wrapping_add(len) <
               64 as libc::c_int as libc::c_ulong {
            (*(*c).epi).tx_done = 1 as libc::c_int as uint8_t
        }
        tx_size =
            (10 as libc::c_int as libc::c_ulong).wrapping_add(len) as
                libc::c_int
    } else if (10 as libc::c_int as
                   libc::c_ulong).wrapping_add(len).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                  == 64 as libc::c_int as libc::c_ulong {
        usb_lld_txcpy((*(*c).a).res_apdu_data as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                      (*(*c).a).res_apdu_data_len as size_t);
        usb_lld_txcpy((*c).sw1sw2.as_mut_ptr() as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int,
                      10 as libc::c_int +
                          (*(*c).a).res_apdu_data_len as libc::c_int,
                      1 as libc::c_int as size_t);
        (*(*c).epi).buf =
            &mut *(*c).sw1sw2.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut uint8_t;
        (*(*c).epi).cnt = 1 as libc::c_int as size_t;
        (*(*c).epi).buf_len = 1 as libc::c_int as size_t;
        (*(*c).epi).next_buf =
            Some(no_buf as
                     unsafe extern "C" fn(_: *mut ep_in, _: size_t) -> ())
    } else if (10 as libc::c_int as
                   libc::c_ulong).wrapping_add(len).wrapping_sub(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                  == 64 as libc::c_int as libc::c_ulong {
        usb_lld_txcpy((*(*c).a).res_apdu_data as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                      (*(*c).a).res_apdu_data_len as size_t);
        (*(*c).epi).buf =
            &mut *(*c).sw1sw2.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut uint8_t;
        (*(*c).epi).cnt = 0 as libc::c_int as size_t;
        (*(*c).epi).buf_len = 2 as libc::c_int as size_t;
        (*(*c).epi).next_buf =
            Some(no_buf as
                     unsafe extern "C" fn(_: *mut ep_in, _: size_t) -> ())
    } else {
        usb_lld_txcpy((*(*c).a).res_apdu_data as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                      (64 as libc::c_int - 10 as libc::c_int) as size_t);
        (*(*c).epi).buf =
            (*(*c).a).res_apdu_data.offset(64 as libc::c_int as
                                               isize).offset(-(10 as
                                                                   libc::c_int
                                                                   as isize));
        (*(*c).epi).cnt = (64 as libc::c_int - 10 as libc::c_int) as size_t;
        (*(*c).epi).buf_len =
            ((*(*c).a).res_apdu_data_len as libc::c_int -
                 (64 as libc::c_int - 10 as libc::c_int)) as size_t;
        (*(*c).epi).next_buf =
            Some(get_sw1sw2 as
                     unsafe extern "C" fn(_: *mut ep_in, _: size_t) -> ())
    }
    usb_lld_tx_enable((*(*c).epi).ep_num as libc::c_int, tx_size as size_t);
}
unsafe extern "C" fn ccid_send_data_block(mut c: *mut ccid) {
    ccid_send_data_block_internal(c, 0 as libc::c_int as uint8_t,
                                  0 as libc::c_int as uint8_t);
}
unsafe extern "C" fn ccid_send_data_block_time_extension(mut c: *mut ccid) {
    ccid_send_data_block_internal(c, 0x80 as libc::c_int as uint8_t,
                                  1 as libc::c_int as uint8_t);
}
unsafe extern "C" fn ccid_send_data_block_0x9000(mut c: *mut ccid) {
    let mut p: [uint8_t; 12] = [0; 12];
    let mut len: size_t = 2 as libc::c_int as size_t;
    p[0 as libc::c_int as usize] = 0x80 as libc::c_int as uint8_t;
    p[1 as libc::c_int as usize] =
        (len & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    p[2 as libc::c_int as usize] =
        (len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[3 as libc::c_int as usize] =
        (len >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[4 as libc::c_int as usize] =
        (len >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[6 as libc::c_int as usize] = (*(*c).a).seq;
    p[7 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[8 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[9 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[(9 as libc::c_int + 1 as libc::c_int) as usize] =
        0x90 as libc::c_int as uint8_t;
    p[(9 as libc::c_int + 2 as libc::c_int) as usize] =
        0 as libc::c_int as uint8_t;
    usb_lld_txcpy(p.as_mut_ptr() as *const libc::c_void,
                  (*(*c).epi).ep_num as libc::c_int, 0 as libc::c_int,
                  (10 as libc::c_int as libc::c_ulong).wrapping_add(len));
    (*(*c).epi).buf = 0 as *const uint8_t;
    (*(*c).epi).tx_done = 1 as libc::c_int as uint8_t;
    usb_lld_tx_enable((*(*c).epi).ep_num as libc::c_int,
                      (10 as libc::c_int as libc::c_ulong).wrapping_add(len));
}
/*
 * Reply to the host for "GET RESPONSE".
 */
unsafe extern "C" fn ccid_send_data_block_gr(mut c: *mut ccid,
                                             mut chunk_len: size_t) {
    let mut tx_size: libc::c_int = 64 as libc::c_int; /* Slot */
    let mut p: [uint8_t; 10] = [0; 10];
    let mut len: size_t =
        chunk_len.wrapping_add(2 as libc::c_int as libc::c_ulong);
    p[0 as libc::c_int as usize] = 0x80 as libc::c_int as uint8_t;
    p[1 as libc::c_int as usize] =
        (len & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    p[2 as libc::c_int as usize] =
        (len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[3 as libc::c_int as usize] =
        (len >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[4 as libc::c_int as usize] =
        (len >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as
            uint8_t;
    p[5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[6 as libc::c_int as usize] = (*(*c).a).seq;
    p[7 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[8 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[9 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    usb_lld_txcpy(p.as_mut_ptr() as *const libc::c_void,
                  (*(*c).epi).ep_num as libc::c_int, 0 as libc::c_int,
                  10 as libc::c_int as size_t);
    set_sw1sw2(c, chunk_len);
    if chunk_len <= (64 as libc::c_int - 10 as libc::c_int) as libc::c_ulong {
        let mut size_for_sw: libc::c_int = 0;
        if chunk_len <=
               (64 as libc::c_int - 10 as libc::c_int - 2 as libc::c_int) as
                   libc::c_ulong {
            size_for_sw = 2 as libc::c_int
        } else if chunk_len ==
                      (64 as libc::c_int - 10 as libc::c_int -
                           1 as libc::c_int) as libc::c_ulong {
            size_for_sw = 1 as libc::c_int
        } else { size_for_sw = 0 as libc::c_int }
        usb_lld_txcpy((*c).p as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                      chunk_len);
        if size_for_sw != 0 {
            usb_lld_txcpy((*c).sw1sw2.as_mut_ptr() as *const libc::c_void,
                          (*(*c).epi).ep_num as libc::c_int,
                          (10 as libc::c_int as
                               libc::c_ulong).wrapping_add(chunk_len) as
                              libc::c_int, size_for_sw as size_t);
        }
        tx_size =
            (10 as libc::c_int as
                 libc::c_ulong).wrapping_add(chunk_len).wrapping_add(size_for_sw
                                                                         as
                                                                         libc::c_ulong)
                as libc::c_int;
        if size_for_sw == 2 as libc::c_int {
            (*(*c).epi).buf = 0 as *const uint8_t;
            if tx_size < 64 as libc::c_int {
                (*(*c).epi).tx_done = 1 as libc::c_int as uint8_t
            }
            /* Don't set epi->tx_done = 1, when it requires ZLP */
        } else {
            (*(*c).epi).buf =
                (*c).sw1sw2.as_mut_ptr().offset(size_for_sw as
                                                    isize); /* Length = 0x00000007 */
            (*(*c).epi).cnt = size_for_sw as size_t; /* Slot */
            (*(*c).epi).buf_len =
                (2 as libc::c_int - size_for_sw) as
                    size_t; /* ProtocolNum: T=1 */
            (*(*c).epi).next_buf =
                Some(no_buf as
                         unsafe extern "C" fn(_: *mut ep_in, _: size_t) -> ())
        }
    } else {
        usb_lld_txcpy((*c).p as *const libc::c_void,
                      (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                      (64 as libc::c_int - 10 as libc::c_int) as size_t);
        (*(*c).epi).buf =
            (*c).p.offset(64 as libc::c_int as
                              isize).offset(-(10 as libc::c_int as isize));
        (*(*c).epi).cnt = 0 as libc::c_int as size_t;
        (*(*c).epi).buf_len =
            chunk_len.wrapping_sub((64 as libc::c_int - 10 as libc::c_int) as
                                       libc::c_ulong);
        (*(*c).epi).next_buf =
            Some(get_sw1sw2 as
                     unsafe extern "C" fn(_: *mut ep_in, _: size_t) -> ())
    }
    (*c).p = (*c).p.offset(chunk_len as isize);
    (*c).len =
        ((*c).len as libc::c_ulong).wrapping_sub(chunk_len) as size_t as
            size_t;
    usb_lld_tx_enable((*(*c).epi).ep_num as libc::c_int, tx_size as size_t);
}
unsafe extern "C" fn ccid_send_params(mut c: *mut ccid) {
    let mut p: [uint8_t; 10] = [0; 10];
    let params: [uint8_t; 7] =
        [0x11 as libc::c_int as uint8_t, 0x11 as libc::c_int as uint8_t,
         0xfe as libc::c_int as uint8_t, 0x55 as libc::c_int as uint8_t,
         0x3 as libc::c_int as uint8_t, 0xfe as libc::c_int as uint8_t,
         0 as libc::c_int as uint8_t];
    p[0 as libc::c_int as usize] = 0x82 as libc::c_int as uint8_t;
    p[1 as libc::c_int as usize] = 0x7 as libc::c_int as uint8_t;
    p[2 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[4 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[6 as libc::c_int as usize] = (*c).ccid_header.seq;
    p[7 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[8 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    p[9 as libc::c_int as usize] = 0x1 as libc::c_int as uint8_t;
    usb_lld_txcpy(p.as_mut_ptr() as *const libc::c_void,
                  (*(*c).epi).ep_num as libc::c_int, 0 as libc::c_int,
                  10 as libc::c_int as size_t);
    usb_lld_txcpy(params.as_ptr() as *const libc::c_void,
                  (*(*c).epi).ep_num as libc::c_int, 10 as libc::c_int,
                  ::std::mem::size_of::<[uint8_t; 7]>() as libc::c_ulong);
    /* This is a single packet Bulk-IN transaction */
    (*(*c).epi).buf = 0 as *const uint8_t;
    (*(*c).epi).tx_done = 1 as libc::c_int as uint8_t;
    usb_lld_tx_enable((*(*c).epi).ep_num as libc::c_int,
                      (10 as libc::c_int as
                           libc::c_ulong).wrapping_add(::std::mem::size_of::<[uint8_t; 7]>()
                                                           as libc::c_ulong));
}
unsafe extern "C" fn ccid_handle_data(mut c: *mut ccid) -> ccid_state {
    let mut next_state: ccid_state = (*c).ccid_state;
    if (*c).err as libc::c_int != 0 as libc::c_int {
        ccid_reset(c);
        ccid_error(c, 1 as libc::c_int);
        return next_state
    }
    match (*c).ccid_state as libc::c_uint {
        0 => {
            if (*c).ccid_header.msg_type as libc::c_int == 0x65 as libc::c_int
               {
                ccid_send_status(c);
            } else { ccid_error(c, 0 as libc::c_int); }
        }
        1 => {
            if (*c).ccid_header.msg_type as libc::c_int == 0x62 as libc::c_int
               {
                ccid_reset(c);
                next_state = ccid_power_on(c)
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x63 as libc::c_int {
                ccid_reset(c);
                next_state = ccid_power_off(c)
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x65 as libc::c_int {
                ccid_send_status(c);
            } else { ccid_error(c, 0 as libc::c_int); }
        }
        2 => {
            if (*c).ccid_header.msg_type as libc::c_int == 0x62 as libc::c_int
               {
                /* Not in the spec., but pcscd/libccid */
                ccid_reset(c);
                next_state = ccid_power_on(c)
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x63 as libc::c_int {
                ccid_reset(c);
                next_state = ccid_power_off(c)
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x65 as libc::c_int {
                ccid_send_status(c);
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x6f as libc::c_int {
                if (*c).ccid_header.param as libc::c_int == 0 as libc::c_int {
                    if *(*(*c).a).cmd_apdu_head.offset(0 as libc::c_int as
                                                           isize) as
                           libc::c_int & 0x10 as libc::c_int ==
                           0 as libc::c_int {
                        if (*c).state as libc::c_int == 1 as libc::c_int {
                            /* command chaining finished */
                            (*c).p =
                                (*c).p.offset(*(*(*c).a).cmd_apdu_head.offset(4
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                                  as libc::c_int as isize);
                            *(*(*c).a).cmd_apdu_head.offset(4 as libc::c_int
                                                                as isize) =
                                0 as libc::c_int as uint8_t
                        }
                        if *(*(*c).a).cmd_apdu_head.offset(1 as libc::c_int as
                                                               isize) as
                               libc::c_int == 0xc0 as libc::c_int &&
                               (*c).state as libc::c_int == 4 as libc::c_int {
                            let mut len: size_t =
                                (*(*c).a).expected_res_size as size_t;
                            if (*c).len <=
                                   (*(*c).a).expected_res_size as
                                       libc::c_ulong {
                                len = (*c).len
                            }
                            ccid_send_data_block_gr(c, len);
                            if (*c).len == 0 as libc::c_int as libc::c_ulong {
                                (*c).state = 3 as libc::c_int as uint8_t
                            }
                            (*c).ccid_state = CCID_STATE_WAIT
                        } else {
                            /* Give this message to GPG thread */
                            (*c).state = 2 as libc::c_int as uint8_t;
                            (*(*c).a).sw = 0x9000 as libc::c_int as uint16_t;
                            (*(*c).a).res_apdu_data_len =
                                0 as libc::c_int as uint16_t;
                            (*(*c).a).res_apdu_data =
                                &mut *ccid_buffer.as_mut_ptr().offset(5 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                    as *mut uint8_t;
                            eventflag_signal(&mut (*c).openpgp_comm,
                                             4 as libc::c_int as eventmask_t);
                            next_state = CCID_STATE_EXECUTE
                        }
                    } else {
                        if (*c).state as libc::c_int == 0 as libc::c_int {
                            /* command chaining is started */
                            let ref mut fresh0 =
                                *(*(*c).a).cmd_apdu_head.offset(0 as
                                                                    libc::c_int
                                                                    as isize);
                            *fresh0 =
                                (*fresh0 as libc::c_int &
                                     !(0x10 as libc::c_int)) as uint8_t;
                            memcpy((*c).chained_cls_ins_p1_p2.as_mut_ptr() as
                                       *mut libc::c_void,
                                   (*(*c).a).cmd_apdu_head as
                                       *const libc::c_void,
                                   4 as libc::c_int as libc::c_ulong);
                            (*c).state = 1 as libc::c_int as uint8_t
                        }
                        (*c).p =
                            (*c).p.offset(*(*(*c).a).cmd_apdu_head.offset(4 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                                              as libc::c_int as isize);
                        (*c).len =
                            ((*c).len as
                                 libc::c_ulong).wrapping_sub(*(*(*c).a).cmd_apdu_head.offset(4
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 isize)
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t;
                        ccid_send_data_block_0x9000(c);
                    }
                } else {
                    /* ICC block chaining is not supported. */
                    ccid_error(c, 8 as libc::c_int);
                }
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x61 as libc::c_int ||
                          (*c).ccid_header.msg_type as libc::c_int ==
                              0x6c as libc::c_int ||
                          (*c).ccid_header.msg_type as libc::c_int ==
                              0x6d as libc::c_int {
                ccid_send_params(c);
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x69 as libc::c_int {
                if (*c).p != (*(*c).a).cmd_apdu_data {
                    /* SECURE received in the middle of command chaining */
                    ccid_reset(c);
                    ccid_error(c, 1 as libc::c_int);
                    return next_state
                }
                if *(*c).p.offset((10 as libc::c_int - 10 as libc::c_int) as
                                      isize) as libc::c_int ==
                       0 as libc::c_int {
                    /* PIN verification */
                    *(*(*c).a).cmd_apdu_head.offset(0 as libc::c_int as isize)
                        =
                        *(*c).p.offset((25 as libc::c_int - 10 as libc::c_int)
                                           as isize);
                    *(*(*c).a).cmd_apdu_head.offset(1 as libc::c_int as isize)
                        =
                        *(*c).p.offset((26 as libc::c_int - 10 as libc::c_int)
                                           as isize);
                    *(*(*c).a).cmd_apdu_head.offset(2 as libc::c_int as isize)
                        =
                        *(*c).p.offset((27 as libc::c_int - 10 as libc::c_int)
                                           as isize);
                    *(*(*c).a).cmd_apdu_head.offset(3 as libc::c_int as isize)
                        =
                        *(*c).p.offset((28 as libc::c_int - 10 as libc::c_int)
                                           as isize);
                    /* */
                    *(*(*c).a).cmd_apdu_data.offset(0 as libc::c_int as isize)
                        = 0 as libc::c_int as uint8_t; /* bConfirmPIN */
                    *(*(*c).a).cmd_apdu_data.offset(1 as libc::c_int as isize)
                        =
                        *(*c).p.offset((17 as libc::c_int - 10 as libc::c_int)
                                           as
                                           isize); /* bEntryValidationCondition */
                    *(*(*c).a).cmd_apdu_data.offset(2 as libc::c_int as isize)
                        =
                        *(*c).p.offset((18 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* bNumberMessage */
                    *(*(*c).a).cmd_apdu_data.offset(3 as libc::c_int as isize)
                        =
                        *(*c).p.offset((19 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* wLangId L */
                    *(*(*c).a).cmd_apdu_data.offset(4 as libc::c_int as isize)
                        =
                        *(*c).p.offset((20 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* wLangId H */
                    *(*(*c).a).cmd_apdu_data.offset(5 as libc::c_int as isize)
                        =
                        *(*c).p.offset((21 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* bMsgIndex */
                    (*(*c).a).cmd_apdu_data_len =
                        6 as libc::c_int as uint16_t;
                    (*(*c).a).expected_res_size =
                        0 as libc::c_int as uint16_t;
                    (*(*c).a).sw = 0x9000 as libc::c_int as uint16_t;
                    (*(*c).a).res_apdu_data_len =
                        0 as libc::c_int as uint16_t;
                    (*(*c).a).res_apdu_data =
                        &mut *(*c).p.offset(5 as libc::c_int as isize) as
                            *mut uint8_t;
                    eventflag_signal(&mut (*c).openpgp_comm,
                                     8 as libc::c_int as eventmask_t);
                    next_state = CCID_STATE_EXECUTE
                } else if *(*c).p.offset((10 as libc::c_int -
                                              10 as libc::c_int) as isize) as
                              libc::c_int == 0x1 as libc::c_int {
                    /* PIN Modification */
                    let mut num_msgs: uint8_t =
                        *(*c).p.offset((21 as libc::c_int - 10 as libc::c_int)
                                           as isize);
                    if num_msgs as libc::c_int == 0 as libc::c_int {
                        num_msgs = 1 as libc::c_int as uint8_t
                    } else if num_msgs as libc::c_int == 0xff as libc::c_int {
                        num_msgs = 3 as libc::c_int as uint8_t
                    }
                    *(*(*c).a).cmd_apdu_head.offset(0 as libc::c_int as isize)
                        =
                        *(*c).p.offset((27 as libc::c_int +
                                            num_msgs as libc::c_int -
                                            10 as libc::c_int) as isize);
                    *(*(*c).a).cmd_apdu_head.offset(1 as libc::c_int as isize)
                        =
                        *(*c).p.offset((28 as libc::c_int +
                                            num_msgs as libc::c_int -
                                            10 as libc::c_int) as isize);
                    *(*(*c).a).cmd_apdu_head.offset(2 as libc::c_int as isize)
                        =
                        *(*c).p.offset((29 as libc::c_int +
                                            num_msgs as libc::c_int -
                                            10 as libc::c_int) as isize);
                    *(*(*c).a).cmd_apdu_head.offset(3 as libc::c_int as isize)
                        =
                        *(*c).p.offset((30 as libc::c_int +
                                            num_msgs as libc::c_int -
                                            10 as libc::c_int) as isize);
                    /* */
                    *(*(*c).a).cmd_apdu_data.offset(0 as libc::c_int as isize)
                        =
                        *(*c).p.offset((19 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* bConfirmPIN */
                    *(*(*c).a).cmd_apdu_data.offset(1 as libc::c_int as isize)
                        =
                        *(*c).p.offset((20 as libc::c_int - 10 as libc::c_int)
                                           as
                                           isize); /* bEntryValidationCondition */
                    *(*(*c).a).cmd_apdu_data.offset(2 as libc::c_int as isize)
                        =
                        *(*c).p.offset((21 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* bNumberMessage */
                    *(*(*c).a).cmd_apdu_data.offset(3 as libc::c_int as isize)
                        =
                        *(*c).p.offset((22 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* wLangId L */
                    *(*(*c).a).cmd_apdu_data.offset(4 as libc::c_int as isize)
                        =
                        *(*c).p.offset((23 as libc::c_int - 10 as libc::c_int)
                                           as isize); /* wLangId H */
                    *(*(*c).a).cmd_apdu_data.offset(5 as libc::c_int as isize)
                        =
                        *(*c).p.offset((24 as libc::c_int - 10 as libc::c_int)
                                           as
                                           isize); /* bMsgIndex, bMsgIndex1 */
                    if num_msgs as libc::c_int >= 2 as libc::c_int {
                        *(*(*c).a).cmd_apdu_data.offset(6 as libc::c_int as
                                                            isize) =
                            *(*c).p.offset((25 as libc::c_int -
                                                10 as libc::c_int) as isize)
                    } /* bMsgIndex2 */
                    if num_msgs as libc::c_int == 3 as libc::c_int {
                        *(*(*c).a).cmd_apdu_data.offset(7 as libc::c_int as
                                                            isize) =
                            *(*c).p.offset((26 as libc::c_int -
                                                10 as libc::c_int) as isize)
                    } /* bMsgIndex3 */
                    (*(*c).a).cmd_apdu_data_len =
                        (5 as libc::c_int + num_msgs as libc::c_int) as
                            uint16_t;
                    (*(*c).a).expected_res_size =
                        0 as libc::c_int as uint16_t;
                    (*(*c).a).sw = 0x9000 as libc::c_int as uint16_t;
                    (*(*c).a).res_apdu_data_len =
                        0 as libc::c_int as uint16_t;
                    (*(*c).a).res_apdu_data =
                        &mut *ccid_buffer.as_mut_ptr().offset(5 as libc::c_int
                                                                  as isize) as
                            *mut uint8_t;
                    eventflag_signal(&mut (*c).openpgp_comm,
                                     16 as libc::c_int as eventmask_t);
                    next_state = CCID_STATE_EXECUTE
                } else { ccid_error(c, 10 as libc::c_int); }
            } else { ccid_error(c, 0 as libc::c_int); }
        }
        3 => {
            if (*c).ccid_header.msg_type as libc::c_int == 0x63 as libc::c_int
               {
                next_state = ccid_power_off(c)
            } else if (*c).ccid_header.msg_type as libc::c_int ==
                          0x65 as libc::c_int {
                ccid_send_status(c);
            } else { ccid_error(c, 0 as libc::c_int); }
        }
        _ => { next_state = CCID_STATE_START }
    }
    return next_state;
}
unsafe extern "C" fn ccid_handle_timeout(mut c: *mut ccid) -> ccid_state {
    let mut next_state: ccid_state = (*c).ccid_state;
    match (*c).ccid_state as libc::c_uint {
        3 => { ccid_send_data_block_time_extension(c); }
        _ => { }
    }
    led_blink(1 as libc::c_int);
    return next_state;
}
static mut ccid: ccid =
    ccid{ccid_state: CCID_STATE_NOCARD,
         state: 0,
         err: 0,
         p: 0 as *const uint8_t as *mut uint8_t,
         len: 0,
         ccid_header:
             ccid_header{msg_type: 0,
                         data_len: 0,
                         slot: 0,
                         seq: 0,
                         rsvd: 0,
                         param: 0,},
         sw1sw2: [0; 2],
         chained_cls_ins_p1_p2: [0; 4],
         epo: 0 as *const ep_out as *mut ep_out,
         epi: 0 as *const ep_in as *mut ep_in,
         ccid_comm:
             eventflag{flags: 0,
                       mask: 0,
                       mutex:
                           chopstx_mutex_t{q:
                                               chx_qh{next:
                                                          0 as *const chx_pq
                                                              as *mut chx_pq,
                                                      prev:
                                                          0 as *const chx_pq
                                                              as
                                                              *mut chx_pq,},
                                           lock: chx_spinlock{},
                                           owner:
                                               0 as *const chx_thread as
                                                   *mut chx_thread,
                                           list:
                                               0 as *const chx_mtx as
                                                   *mut chx_mtx,},
                       cond:
                           chopstx_cond_t{q:
                                              chx_qh{next:
                                                         0 as *const chx_pq as
                                                             *mut chx_pq,
                                                     prev:
                                                         0 as *const chx_pq as
                                                             *mut chx_pq,},
                                          lock: chx_spinlock{},},},
         openpgp_comm:
             eventflag{flags: 0,
                       mask: 0,
                       mutex:
                           chopstx_mutex_t{q:
                                               chx_qh{next:
                                                          0 as *const chx_pq
                                                              as *mut chx_pq,
                                                      prev:
                                                          0 as *const chx_pq
                                                              as
                                                              *mut chx_pq,},
                                           lock: chx_spinlock{},
                                           owner:
                                               0 as *const chx_thread as
                                                   *mut chx_thread,
                                           list:
                                               0 as *const chx_mtx as
                                                   *mut chx_mtx,},
                       cond:
                           chopstx_cond_t{q:
                                              chx_qh{next:
                                                         0 as *const chx_pq as
                                                             *mut chx_pq,
                                                     prev:
                                                         0 as *const chx_pq as
                                                             *mut chx_pq,},
                                          lock: chx_spinlock{},},},
         application: 0,
         a: 0 as *const apdu as *mut apdu,};
// Initialized in run_static_initializers
#[no_mangle]
pub static mut ccid_state_p: *mut ccid_state =
    0 as *const ccid_state as *mut ccid_state;
#[no_mangle]
pub unsafe extern "C" fn ccid_card_change_signal(mut how: libc::c_int) {
    let mut c: *mut ccid = &mut ccid;
    if how == 2 as libc::c_int ||
           (*c).ccid_state as libc::c_uint ==
               CCID_STATE_NOCARD as libc::c_int as libc::c_uint &&
               how == 0 as libc::c_int ||
           (*c).ccid_state as libc::c_uint !=
               CCID_STATE_NOCARD as libc::c_int as libc::c_uint &&
               how == 1 as libc::c_int {
        eventflag_signal(&mut (*c).ccid_comm,
                         8 as libc::c_int as eventmask_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ccid_usb_reset(mut full: libc::c_int) {
    let mut c: *mut ccid = &mut ccid;
    eventflag_signal(&mut (*c).ccid_comm,
                     if full != 0 {
                         32 as libc::c_int
                     } else { 16 as libc::c_int } as eventmask_t);
}
unsafe extern "C" fn ccid_notify_slot_change(mut c: *mut ccid) {
    let mut msg: uint8_t = 0;
    let mut notification: [uint8_t; 2] = [0; 2];
    if (*c).ccid_state as libc::c_uint ==
           CCID_STATE_NOCARD as libc::c_int as libc::c_uint {
        msg = 0x2 as libc::c_int as uint8_t
    } else { msg = 0x3 as libc::c_int as uint8_t }
    notification[0 as libc::c_int as usize] = 0x50 as libc::c_int as uint8_t;
    notification[1 as libc::c_int as usize] = msg;
    usb_lld_write(2 as libc::c_int as uint8_t,
                  notification.as_mut_ptr() as *const libc::c_void,
                  ::std::mem::size_of::<[uint8_t; 2]>() as libc::c_ulong);
    led_blink(2 as libc::c_int);
}
unsafe extern "C" fn usb_event_handle(mut dev: *mut usb_dev) {
    let mut ep_num: uint8_t = 0;
    let mut e: libc::c_int = 0;
    e = usb_lld_event_handler(dev);
    ep_num = (e >> 16 as libc::c_int & 0x7f as libc::c_int) as uint8_t;
    if ep_num as libc::c_int != 0 as libc::c_int {
        if e >> 23 as libc::c_int & 1 as libc::c_int != 0 {
            usb_tx_done(ep_num, (e & 0xffff as libc::c_int) as uint16_t);
        } else {
            usb_rx_ready(ep_num, (e & 0xffff as libc::c_int) as uint16_t);
        }
    } else {
        match e >> 24 as libc::c_int {
            1 => { usb_device_reset(dev); }
            15 => { bDeviceState = 0 as libc::c_int as uint32_t }
            11 => {
                if usb_get_descriptor(dev) < 0 as libc::c_int {
                    usb_lld_ctrl_error(dev);
                }
            }
            4 => {
                if usb_set_configuration(dev) < 0 as libc::c_int {
                    usb_lld_ctrl_error(dev);
                }
            }
            5 => {
                if usb_set_interface(dev) < 0 as libc::c_int {
                    usb_lld_ctrl_error(dev);
                }
            }
            13 => {
                /* Device specific device request.  */
                if usb_setup(dev) < 0 as libc::c_int {
                    usb_lld_ctrl_error(dev);
                }
            }
            10 => {
                if usb_get_status_interface(dev) < 0 as libc::c_int {
                    usb_lld_ctrl_error(dev);
                }
            }
            12 => {
                if usb_get_interface(dev) < 0 as libc::c_int {
                    usb_lld_ctrl_error(dev);
                }
            }
            6 | 7 | 8 | 9 => { usb_lld_ctrl_ack(dev); }
            14 => {
                /* Control WRITE transfer finished.  */
                usb_ctrl_write_finish(dev); /* For old SYS < 3.0 */
            }
            0 | 2 | _ => { }
        }
    };
}
unsafe extern "C" fn poll_event_intr(mut timeout: *mut uint32_t,
                                     mut ev: *mut eventflag,
                                     mut intr: *mut chopstx_intr_t) {
    let mut poll_desc: chopstx_poll_cond_t =
        chopstx_poll_cond_t{type_0: 0,
                            ready: 0,
                            cond: 0 as *mut chopstx_cond_t,
                            mutex: 0 as *mut chopstx_mutex_t,
                            check: None,
                            arg: 0 as *mut libc::c_void,};
    let mut pd_array: [*mut chx_poll_head; 2] =
        [intr as *mut chx_poll_head,
         &mut poll_desc as *mut chopstx_poll_cond_t as *mut chx_poll_head];
    eventflag_prepare_poll(ev, &mut poll_desc);
    chopstx_poll(timeout, 2 as libc::c_int,
                 pd_array.as_mut_ptr() as *const *mut chx_poll_head);
}
#[no_mangle]
pub unsafe extern "C" fn ccid_thread(mut arg: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut m: eventmask_t = 0;
    let mut interrupt: chopstx_intr_t =
        chopstx_intr_t{type_0: 0, ready: 0, irq_num: 0,};
    let mut timeout: uint32_t = 0;
    let mut dev: usb_dev =
        usb_dev{configuration: 0,
                feature: 0,
                state: 0,
                dev_req:
                    device_req{type_0: 0,
                               request: 0,
                               value: 0,
                               index: 0,
                               len: 0,},
                ctrl_data:
                    ctrl_data{addr: 0 as *mut uint8_t,
                              len: 0,
                              require_zlp: 0,},};
    let mut c: *mut ccid = &mut ccid;
    eventflag_init(&mut ccid.ccid_comm);
    eventflag_init(&mut ccid.openpgp_comm);
    usb_lld_init(&mut dev, 0x80 as libc::c_int as uint8_t);
    chopstx_claim_irq(&mut interrupt, 20 as libc::c_int as uint8_t);
    usb_event_handle(&mut dev);
    'c_7082:
        loop  {
            let mut epi: *mut ep_in = &mut endpoint_in;
            let mut epo: *mut ep_out = &mut endpoint_out;
            let mut a: *mut apdu = &mut apdu;
            epi_init(epi, 1 as libc::c_int as uint8_t as libc::c_int,
                     c as *mut libc::c_void);
            epo_init(epo, 1 as libc::c_int as uint8_t as libc::c_int,
                     c as *mut libc::c_void);
            apdu_init(a);
            ccid_init(c, epi, epo, a);
            while bDeviceState != 0 as libc::c_int as libc::c_uint {
                poll_event_intr(0 as *mut uint32_t, &mut (*c).ccid_comm,
                                &mut interrupt);
                if interrupt.ready != 0 { usb_event_handle(&mut dev); }
                eventflag_get(&mut (*c).ccid_comm);
                /* Ignore event while not-configured.  */
            }
            'c_7084:
                loop 
                     /* Upon receival of SET_INTERFACE, the endpoint is reset to RX_NAK.
	 * Thus, we need to prepare receive again.
	 */
                     {
                    timeout =
                        (1950 as libc::c_int * 1000 as libc::c_int) as
                            uint32_t;
                    ccid_prepare_receive(c);
                    ccid_notify_slot_change(c);
                    loop  {
                        m = 0;
                        poll_event_intr(&mut timeout, &mut (*c).ccid_comm,
                                        &mut interrupt);
                        if interrupt.ready != 0 {
                            usb_event_handle(&mut dev);
                        } else {
                            timeout =
                                (1950 as libc::c_int * 1000 as libc::c_int) as
                                    uint32_t;
                            m = eventflag_get(&mut (*c).ccid_comm);
                            if m == 32 as libc::c_int as libc::c_uint {
                                break 'c_7084 ;
                            }
                            if m == 16 as libc::c_int as libc::c_uint {
                                break ;
                            }
                            if m == 8 as libc::c_int as libc::c_uint {
                                if (*c).ccid_state as libc::c_uint ==
                                       CCID_STATE_NOCARD as libc::c_int as
                                           libc::c_uint {
                                    /* Inserted!  */
                                    (*c).ccid_state = CCID_STATE_START
                                } else {
                                    /* Removed!  */
                                    if (*c).application != 0 {
                                        eventflag_signal(&mut (*c).openpgp_comm,
                                                         2 as libc::c_int as
                                                             eventmask_t);
                                        chopstx_join((*c).application,
                                                     0 as
                                                         *mut *mut libc::c_void);
                                        (*c).application =
                                            0 as libc::c_int as chopstx_t
                                    }
                                    (*c).ccid_state = CCID_STATE_NOCARD
                                }
                                ccid_notify_slot_change(c);
                            } else if m == 1 as libc::c_int as libc::c_uint {
                                (*c).ccid_state = ccid_handle_data(c)
                            } else if m == 2 as libc::c_int as libc::c_uint {
                                if !((*c).ccid_state as libc::c_uint ==
                                         CCID_STATE_EXECUTE as libc::c_int as
                                             libc::c_uint) {
                                    continue ;
                                }
                                if (*(*c).a).sw as libc::c_int ==
                                       0xffff as libc::c_int {
                                    break 'c_7082 ;
                                }
                                (*(*c).a).cmd_apdu_data_len =
                                    0 as libc::c_int as uint16_t;
                                (*c).sw1sw2[0 as libc::c_int as usize] =
                                    ((*(*c).a).sw as libc::c_int >>
                                         8 as libc::c_int) as uint8_t;
                                (*c).sw1sw2[1 as libc::c_int as usize] =
                                    ((*(*c).a).sw as libc::c_int &
                                         0xff as libc::c_int) as uint8_t;
                                if (*(*c).a).res_apdu_data_len as libc::c_int
                                       <=
                                       (*(*c).a).expected_res_size as
                                           libc::c_int {
                                    (*c).state = 3 as libc::c_int as uint8_t;
                                    ccid_send_data_block(c);
                                    (*c).ccid_state = CCID_STATE_WAIT
                                } else {
                                    (*c).state = 4 as libc::c_int as uint8_t;
                                    (*c).p = (*(*c).a).res_apdu_data;
                                    (*c).len =
                                        (*(*c).a).res_apdu_data_len as size_t;
                                    ccid_send_data_block_gr(c,
                                                            (*(*c).a).expected_res_size
                                                                as size_t);
                                    (*c).ccid_state = CCID_STATE_WAIT
                                }
                            } else if m == 4 as libc::c_int as libc::c_uint {
                                if (*c).state as libc::c_int ==
                                       3 as libc::c_int {
                                    (*c).state = 0 as libc::c_int as uint8_t;
                                    (*c).p = (*(*c).a).cmd_apdu_data;
                                    (*c).len =
                                        (24 as libc::c_int + 4 as libc::c_int
                                             + 256 as libc::c_int +
                                             256 as libc::c_int) as size_t;
                                    (*c).err = 0 as libc::c_int as uint8_t;
                                    (*(*c).a).cmd_apdu_data_len =
                                        0 as libc::c_int as uint16_t;
                                    (*(*c).a).expected_res_size =
                                        0 as libc::c_int as uint16_t
                                }
                                if (*c).state as libc::c_int ==
                                       0 as libc::c_int ||
                                       (*c).state as libc::c_int ==
                                           1 as libc::c_int ||
                                       (*c).state as libc::c_int ==
                                           4 as libc::c_int {
                                    ccid_prepare_receive(c);
                                }
                            } else {
                                /* Timeout */
                                (*c).ccid_state = ccid_handle_timeout(c)
                            }
                        }
                    }
                }
            if (*c).application != 0 {
                chopstx_cancel((*c).application);
                chopstx_join((*c).application, 0 as *mut *mut libc::c_void);
                (*c).application = 0 as libc::c_int as chopstx_t
            }
        }
    (*c).sw1sw2[0 as libc::c_int as usize] = 0x90 as libc::c_int as uint8_t;
    (*c).sw1sw2[1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    (*c).state = 3 as libc::c_int as uint8_t;
    ccid_send_data_block(c);
    (*c).ccid_state = CCID_STATE_EXITED;
    if (*c).application != 0 {
        chopstx_join((*c).application, 0 as *mut *mut libc::c_void);
        (*c).application = 0 as libc::c_int as chopstx_t
    }
    /* Loading reGNUal.  */
    while bDeviceState != 0 as libc::c_int as libc::c_uint {
        chopstx_intr_wait(&mut interrupt);
        usb_event_handle(&mut dev);
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _write(mut s: *const libc::c_char,
                                mut size: libc::c_int) {
}
unsafe extern "C" fn run_static_initializers() {
    ccid_state_p = &mut ccid.ccid_state
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
