// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_uppercase_statics)]
#![allow(non_camel_case_types)]

use libc::*;
use libc::types::os::arch::c95::wchar_t;

pub type XID = c_ulong;

pub type Mask = c_ulong;

pub type Atom = c_ulong;

pub type VisualID = c_ulong;

pub type Time = c_ulong;

pub type Window = XID;

pub type Drawable = XID;

pub type Font = XID;

pub type Pixmap = XID;

pub type Cursor = XID;

pub type Colormap = XID;

pub type GContext = XID;

pub type KeySym = XID;

pub type KeyCode = c_uchar;

pub type XPointer = *const c_char;

pub struct struct__XExtData {
    pub number: c_int,
    pub next: *const c_void /* struct__XExtData */,
    pub free_private: *const u8,
    pub private_data: XPointer,
}

pub type XExtData = struct__XExtData;

pub struct XExtCodes {
    pub extension: c_int,
    pub major_opcode: c_int,
    pub first_event: c_int,
    pub first_error: c_int,
}

pub struct XPixmapFormatValues {
    pub depth: c_int,
    pub bits_per_pixel: c_int,
    pub scanline_pad: c_int,
}

pub struct XGCValues {
    pub function: c_int,
    pub plane_mask: c_ulong,
    pub foreground: c_ulong,
    pub background: c_ulong,
    pub line_width: c_int,
    pub line_style: c_int,
    pub cap_style: c_int,
    pub join_style: c_int,
    pub fill_style: c_int,
    pub fill_rule: c_int,
    pub arc_mode: c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: c_int,
    pub ts_y_origin: c_int,
    pub font: Font,
    pub subwindow_mode: c_int,
    pub graphics_exposures: c_int,
    pub clip_x_origin: c_int,
    pub clip_y_origin: c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: c_int,
    pub dashes: c_char,
}

pub type struct__XGC = c_void;

pub type GC = *const struct__XGC;

pub struct Visual {
    pub ext_data: *const XExtData,
    pub visualid: VisualID,
    pub _class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub bits_per_rgb: c_int,
    pub map_entries: c_int,
}

pub struct Depth {
    pub depth: c_int,
    pub nvisuals: c_int,
    pub visuals: *const Visual,
}

pub type struct__XDisplay = c_void;

pub struct Screen {
    pub ext_data: *const XExtData,
    pub display: *const c_void /* struct__XDisplay */,
    pub root: Window,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
    pub ndepths: c_int,
    pub depths: *const Depth,
    pub root_depth: c_int,
    pub root_visual: *const Visual,
    pub default_gc: *const c_void /*const  GC */,
    pub cmap: Colormap,
    pub white_pixel: c_ulong,
    pub black_pixel: c_ulong,
    pub max_maps: c_int,
    pub min_maps: c_int,
    pub backing_store: c_int,
    pub save_unders: c_int,
    pub root_input_mask: c_long,
}

pub struct ScreenFormat {
    pub ext_data: *const XExtData,
    pub depth: c_int,
    pub bits_per_pixel: c_int,
    pub scanline_pad: c_int,
}

pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: c_int,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

pub struct XWindowAttributes {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub depth: c_int,
    pub visual: *const Visual,
    pub root: Window,
    pub _class: c_int,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: c_int,
    pub colormap: Colormap,
    pub map_installed: c_int,
    pub map_state: c_int,
    pub all_event_masks: c_long,
    pub your_event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: c_int,
    pub screen: *const Screen,
}

pub struct XHostAddress {
    pub family: c_int,
    pub length: c_int,
    pub address: *const c_char,
}

pub struct XServerInterpretedAddress {
    pub typelength: c_int,
    pub valuelength: c_int,
    pub _type: *const c_char,
    pub value: *const c_char,
}

pub struct struct__XImage {
    pub width: c_int,
    pub height: c_int,
    pub xoffset: c_int,
    pub format: c_int,
    pub data: *const c_char,
    pub byte_order: c_int,
    pub bitmap_unit: c_int,
    pub bitmap_bit_order: c_int,
    pub bitmap_pad: c_int,
    pub depth: c_int,
    pub bytes_per_line: c_int,
    pub bits_per_pixel: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub obdata: XPointer,
    pub f: struct_funcs,
}

pub struct struct_funcs {
    pub create_image: *const u8,
    pub destroy_image: *const u8,
    pub get_pixel: *const u8,
    pub put_pixel: *const u8,
    pub sub_image: *const u8,
    pub add_pixel: *const u8,
}

pub type XImage = struct__XImage;

pub struct XWindowChanges {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub sibling: Window,
    pub stack_mode: c_int,
}

pub struct XColor {
    pub pixel: c_ulong,
    pub red: c_ushort,
    pub green: c_ushort,
    pub blue: c_ushort,
    pub flags: c_char,
    pub pad: c_char,
}

pub struct XSegment {
    pub x1: c_short,
    pub y1: c_short,
    pub x2: c_short,
    pub y2: c_short,
}

pub struct XPoint {
    pub x: c_short,
    pub y: c_short,
}

pub struct XRectangle {
    pub x: c_short,
    pub y: c_short,
    pub width: c_ushort,
    pub height: c_ushort,
}

pub struct XArc {
    pub x: c_short,
    pub y: c_short,
    pub width: c_ushort,
    pub height: c_ushort,
    pub angle1: c_short,
    pub angle2: c_short,
}

pub struct XKeyboardControl {
    pub key_click_percent: c_int,
    pub bell_percent: c_int,
    pub bell_pitch: c_int,
    pub bell_duration: c_int,
    pub led: c_int,
    pub led_mode: c_int,
    pub key: c_int,
    pub auto_repeat_mode: c_int,
}

pub struct XKeyboardState {
    pub key_click_percent: c_int,
    pub bell_percent: c_int,
    pub bell_pitch: c_uint,
    pub bell_duration: c_uint,
    pub led_mask: c_ulong,
    pub global_auto_repeat: c_int,
    pub auto_repeats: (c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char),
}

pub struct XTimeCoord {
    pub time: Time,
    pub x: c_short,
    pub y: c_short,
}

pub struct XModifierKeymap {
    pub max_keypermod: c_int,
    pub modifiermap: *const KeyCode,
}

pub type Display = struct__XDisplay;

pub type struct__XPrivate = c_void;

pub type struct__XrmHashBucketRec = c_void;

pub type _XPrivDisplay = *const struct_unnamed1;

pub struct XKeyEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub keycode: c_uint,
    pub same_screen: c_int,
}

pub type XKeyPressedEvent = XKeyEvent;

pub type XKeyReleasedEvent = XKeyEvent;

pub struct XButtonEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub button: c_uint,
    pub same_screen: c_int,
}

pub type XButtonPressedEvent = XButtonEvent;

pub type XButtonReleasedEvent = XButtonEvent;

pub struct XMotionEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub is_hint: c_char,
    pub same_screen: c_int,
}

pub type XPointerMovedEvent = XMotionEvent;

pub struct XCrossingEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub mode: c_int,
    pub detail: c_int,
    pub same_screen: c_int,
    pub focus: c_int,
    pub state: c_uint,
}

pub type XEnterWindowEvent = XCrossingEvent;

pub type XLeaveWindowEvent = XCrossingEvent;

pub struct XFocusChangeEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub mode: c_int,
    pub detail: c_int,
}

pub type XFocusInEvent = XFocusChangeEvent;

pub type XFocusOutEvent = XFocusChangeEvent;

pub struct XKeymapEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub key_vector: (c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char,c_char),
}

pub struct XExposeEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
}

pub struct XGraphicsExposeEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub drawable: Drawable,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
    pub major_code: c_int,
    pub minor_code: c_int,
}

pub struct XNoExposeEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub drawable: Drawable,
    pub major_code: c_int,
    pub minor_code: c_int,
}

pub struct XVisibilityEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub state: c_int,
}

pub struct XCreateWindowEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub override_redirect: c_int,
}

pub struct XDestroyWindowEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
}

pub struct XUnmapEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: c_int,
}

pub struct XMapEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: c_int,
}

pub struct XMapRequestEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
}

pub struct XReparentEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: c_int,
    pub y: c_int,
    pub override_redirect: c_int,
}

pub struct XConfigureEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub override_redirect: c_int,
}

pub struct XGravityEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
}

pub struct XResizeRequestEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub width: c_int,
    pub height: c_int,
}

pub struct XConfigureRequestEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub detail: c_int,
    pub value_mask: c_ulong,
}

pub struct XCirculateEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub place: c_int,
}

pub struct XCirculateRequestEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
    pub place: c_int,
}

pub struct XPropertyEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: c_int,
}

pub struct XSelectionClearEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

pub struct XSelectionRequestEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

pub struct XSelectionEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

pub struct XColormapEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub colormap: Colormap,
    pub _new: c_int,
    pub state: c_int,
}

pub struct XClientMessageEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: c_int,
    pub data: union_unnamed2,
}

pub struct XMappingEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
    pub request: c_int,
    pub first_keycode: c_int,
    pub count: c_int,
}

pub struct XErrorEvent {
    pub _type: c_int,
    pub display: *const Display,
    pub resourceid: XID,
    pub serial: c_ulong,
    pub error_code: c_uchar,
    pub request_code: c_uchar,
    pub minor_code: c_uchar,
}

pub struct XAnyEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub window: Window,
}

pub struct XGenericEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub extension: c_int,
    pub evtype: c_int,
}

pub struct XGenericEventCookie {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *const Display,
    pub extension: c_int,
    pub evtype: c_int,
    pub cookie: c_uint,
    pub data: *const c_void,
}

pub type union__XEvent = c_void /* FIXME: union type */;

pub type XEvent = union__XEvent;

pub struct XCharStruct {
    pub lbearing: c_short,
    pub rbearing: c_short,
    pub width: c_short,
    pub ascent: c_short,
    pub descent: c_short,
    pub attributes: c_ushort,
}

pub struct XFontProp {
    pub name: Atom,
    pub card32: c_ulong,
}

pub struct XFontStruct {
    pub ext_data: *const XExtData,
    pub fid: Font,
    pub direction: c_uint,
    pub min_char_or_byte2: c_uint,
    pub max_char_or_byte2: c_uint,
    pub min_byte1: c_uint,
    pub max_byte1: c_uint,
    pub all_chars_exist: c_int,
    pub default_char: c_uint,
    pub n_properties: c_int,
    pub properties: *const XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *const XCharStruct,
    pub ascent: c_int,
    pub descent: c_int,
}

pub struct XTextItem {
    pub chars: *const c_char,
    pub nchars: c_int,
    pub delta: c_int,
    pub font: Font,
}

pub struct XChar2b {
    pub byte1: c_uchar,
    pub byte2: c_uchar,
}

pub struct XTextItem16 {
    pub chars: *const XChar2b,
    pub nchars: c_int,
    pub delta: c_int,
    pub font: Font,
}

pub type XEDataObject = c_void /* FIXME: union type */;

pub struct XFontSetExtents {
    pub max_ink_extent: XRectangle,
    pub max_logical_extent: XRectangle,
}

pub type struct__XOM = c_void;

pub type XOM = *const struct__XOM;

pub type struct__XOC = c_void;

pub type XOC = *const struct__XOC;

pub type XFontSet = *const struct__XOC;

pub struct XmbTextItem {
    pub chars: *const c_char,
    pub nchars: c_int,
    pub delta: c_int,
    pub font_set: *const c_void /*const  XFontSet */,
}

pub struct XwcTextItem {
    pub chars: *const wchar_t,
    pub nchars: c_int,
    pub delta: c_int,
    pub font_set: *const c_void /*const  XFontSet */,
}

pub struct XOMCharSetList {
    pub charset_count: c_int,
    pub charset_list: *const *const c_char,
}


pub type XOrientation = c_uint;
pub static XOMOrientation_LTR_TTB: u32 = 0_u32;
pub static XOMOrientation_RTL_TTB: u32 = 1_u32;
pub static XOMOrientation_TTB_LTR: u32 = 2_u32;
pub static XOMOrientation_TTB_RTL: u32 = 3_u32;
pub static XOMOrientation_Context: u32 = 4_u32;

pub struct XOMOrientation {
    pub num_orientation: c_int,
    pub orientation: *const XOrientation,
}

pub struct XOMFontInfo {
    pub num_font: c_int,
    pub font_struct_list: *const *const XFontStruct,
    pub font_name_list: *const *const c_char,
}

pub type struct__XIM = c_void;

pub type XIM = *const struct__XIM;

pub type struct__XIC = c_void;

pub type XIC = *const struct__XIC;

pub type XIMProc = *const u8;

pub type XICProc = *const u8;

pub type XIDProc = *const u8;

pub type XIMStyle = c_ulong;

pub struct XIMStyles {
    pub count_styles: c_ushort,
    pub supported_styles: *const XIMStyle,
}

pub type XVaNestedList = *const c_void;

pub struct XIMCallback {
    pub client_data: XPointer,
    pub callback: XIMProc,
}

pub struct XICCallback {
    pub client_data: XPointer,
    pub callback: XICProc,
}

pub type XIMFeedback = c_ulong;

pub struct struct__XIMText {
    pub length: c_ushort,
    pub feedback: *const XIMFeedback,
    pub encoding_is_wchar: c_int,
    pub string: union_unnamed3,
}

pub type XIMText = struct__XIMText;

pub type XIMPreeditState = c_ulong;

pub struct struct__XIMPreeditStateNotifyCallbackStruct {
    pub state: XIMPreeditState,
}

pub type XIMPreeditStateNotifyCallbackStruct = struct__XIMPreeditStateNotifyCallbackStruct;

pub type XIMResetState = c_ulong;

pub type XIMStringConversionFeedback = c_ulong;

pub struct struct__XIMStringConversionText {
    pub length: c_ushort,
    pub feedback: *const XIMStringConversionFeedback,
    pub encoding_is_wchar: c_int,
    pub string: union_unnamed4,
}

pub type XIMStringConversionText = struct__XIMStringConversionText;

pub type XIMStringConversionPosition = c_ushort;

pub type XIMStringConversionType = c_ushort;

pub type XIMStringConversionOperation = c_ushort;


pub type XIMCaretDirection = c_uint;
pub static XIMForwardChar: u32 = 0_u32;
pub static XIMBackwardChar: u32 = 1_u32;
pub static XIMForwardWord: u32 = 2_u32;
pub static XIMBackwardWord: u32 = 3_u32;
pub static XIMCaretUp: u32 = 4_u32;
pub static XIMCaretDown: u32 = 5_u32;
pub static XIMNextLine: u32 = 6_u32;
pub static XIMPreviousLine: u32 = 7_u32;
pub static XIMLineStart: u32 = 8_u32;
pub static XIMLineEnd: u32 = 9_u32;
pub static XIMAbsolutePosition: u32 = 10_u32;
pub static XIMDontChange: u32 = 11_u32;

pub struct struct__XIMStringConversionCallbackStruct {
    pub position: XIMStringConversionPosition,
    pub direction: XIMCaretDirection,
    pub operation: XIMStringConversionOperation,
    pub factor: c_ushort,
    pub text: *const XIMStringConversionText,
}

pub type XIMStringConversionCallbackStruct = struct__XIMStringConversionCallbackStruct;

pub struct struct__XIMPreeditDrawCallbackStruct {
    pub caret: c_int,
    pub chg_first: c_int,
    pub chg_length: c_int,
    pub text: *const XIMText,
}

pub type XIMPreeditDrawCallbackStruct = struct__XIMPreeditDrawCallbackStruct;


pub type XIMCaretStyle = c_uint;
pub static XIMIsInvisible: u32 = 0_u32;
pub static XIMIsPrimary: u32 = 1_u32;
pub static XIMIsSecondary: u32 = 2_u32;

pub struct struct__XIMPreeditCaretCallbackStruct {
    pub position: c_int,
    pub direction: XIMCaretDirection,
    pub style: XIMCaretStyle,
}

pub type XIMPreeditCaretCallbackStruct = struct__XIMPreeditCaretCallbackStruct;


pub type XIMStatusDataType = c_uint;
pub static XIMTextType: u32 = 0_u32;
pub static XIMBitmapType: u32 = 1_u32;

pub struct struct__XIMStatusDrawCallbackStruct {
    pub _type: XIMStatusDataType,
    pub data: union_unnamed5,
}

pub type XIMStatusDrawCallbackStruct = struct__XIMStatusDrawCallbackStruct;

pub struct struct__XIMHotKeyTrigger {
    pub keysym: KeySym,
    pub modifier: c_int,
    pub modifier_mask: c_int,
}

pub type XIMHotKeyTrigger = struct__XIMHotKeyTrigger;

pub struct struct__XIMHotKeyTriggers {
    pub num_hot_key: c_int,
    pub key: *const XIMHotKeyTrigger,
}

pub type XIMHotKeyTriggers = struct__XIMHotKeyTriggers;

pub type XIMHotKeyState = c_ulong;

pub struct XIMValuesList {
    pub count_values: c_ushort,
    pub supported_values: *const *const c_char,
}

/* FIXME: global variable _Xdebug */

pub type XErrorHandler = *const u8;

pub type XIOErrorHandler = *const u8;

pub type XConnectionWatchProc = *const u8;

pub type union_unnamed3 = c_void /* FIXME: union type */;

pub type union_unnamed5 = c_void /* FIXME: union type */;

pub type union_unnamed2 = c_void /* FIXME: union type */;

pub type union_unnamed4 = c_void /* FIXME: union type */;

pub struct struct_unnamed1 {
    pub ext_data: *const XExtData,
    pub private1: *const c_void /* struct__XPrivate */,
    pub fd: c_int,
    pub private2: c_int,
    pub proto_major_version: c_int,
    pub proto_minor_version: c_int,
    pub vendor: *const c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: c_int,
    pub resource_alloc: *const u8,
    pub byte_order: c_int,
    pub bitmap_unit: c_int,
    pub bitmap_pad: c_int,
    pub bitmap_bit_order: c_int,
    pub nformats: c_int,
    pub pixmap_format: *const ScreenFormat,
    pub private8: c_int,
    pub release: c_int,
    pub private9: *const c_void /* struct__XPrivate */,
    pub private10: *const c_void /* struct__XPrivate */,
    pub qlen: c_int,
    pub last_request_read: c_ulong,
    pub request: c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: c_uint,
    pub db: *const c_void /* struct__XrmHashBucketRec */,
    pub private15: *const u8,
    pub display_name: *const c_char,
    pub default_screen: c_int,
    pub nscreens: c_int,
    pub screens: *const Screen,
    pub motion_buffer: c_ulong,
    pub private16: c_ulong,
    pub min_keycode: c_int,
    pub max_keycode: c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: c_int,
    pub xdefaults: *const c_char,
}

// Additions --pcwalton

pub struct XVisualInfo {
    pub visual: *const Visual,
    pub visualid: VisualID,
    pub screen: c_int,
    pub depth: c_int,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub colormap_size: c_int,
    pub bits_per_rgb: c_int,
}

pub static ZPixmap: c_int = 2;  // depth == drawable depth

#[link(name="X11")]
extern {

    pub fn _Xmblen(arg0: *const c_char, arg1: c_int) -> c_int;

    pub fn XLoadQueryFont(arg0: *const Display, arg1: *const c_char) -> *const XFontStruct;

    pub fn XQueryFont(arg0: *const Display, arg1: XID) -> *const XFontStruct;

    pub fn XGetMotionEvents(arg0: *const Display, arg1: Window, arg2: Time, arg3: Time, arg4: *const c_int) -> *const XTimeCoord;

    pub fn XDeleteModifiermapEntry(arg0: *const XModifierKeymap, arg1: KeyCode, arg2: c_int) -> *const XModifierKeymap;

    pub fn XGetModifierMapping(arg0: *const Display) -> *const XModifierKeymap;

    pub fn XInsertModifiermapEntry(arg0: *const XModifierKeymap, arg1: KeyCode, arg2: c_int) -> *const XModifierKeymap;

    pub fn XNewModifiermap(arg0: c_int) -> *const XModifierKeymap;

    pub fn XCreateImage(arg0: *const Display, arg1: *const Visual, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: *const c_char, arg6: c_uint, arg7: c_uint, arg8: c_int, arg9: c_int) -> *const XImage;

    pub fn XInitImage(arg0: *const XImage) -> c_int;

    pub fn XGetImage(arg0: *const Display, arg1: Drawable, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_ulong, arg7: c_int) -> *const XImage;

    pub fn XGetSubImage(arg0: *const Display, arg1: Drawable, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_ulong, arg7: c_int, arg8: *const XImage, arg9: c_int, arg10: c_int) -> *const XImage;

    pub fn XOpenDisplay(arg0: *const c_char) -> *const Display;

    pub fn XrmInitialize();

    pub fn XFetchBytes(arg0: *const Display, arg1: *const c_int) -> *const c_char;

    pub fn XFetchBuffer(arg0: *const Display, arg1: *const c_int, arg2: c_int) -> *const c_char;

    pub fn XGetAtomName(arg0: *const Display, arg1: Atom) -> *const c_char;

    pub fn XGetAtomNames(arg0: *const Display, arg1: *const Atom, arg2: c_int, arg3: *const *const c_char) -> c_int;

    pub fn XGetDefault(arg0: *const Display, arg1: *const c_char, arg2: *const c_char) -> *const c_char;

    pub fn XDisplayName(arg0: *const c_char) -> *const c_char;

    pub fn XKeysymToString(arg0: KeySym) -> *const c_char;

    pub fn XSynchronize(arg0: *const Display, arg1: c_int) -> *const u8;

    pub fn XSetAfterFunction(arg0: *const Display, arg1: *const u8) -> *const u8;

    pub fn XInternAtom(arg0: *const Display, arg1: *const c_char, arg2: c_int) -> Atom;

    pub fn XInternAtoms(arg0: *const Display, arg1: *const *const c_char, arg2: c_int, arg3: c_int, arg4: *const Atom) -> c_int;

    pub fn XCopyColormapAndFree(arg0: *const Display, arg1: Colormap) -> Colormap;

    pub fn XCreateColormap(arg0: *const Display, arg1: Window, arg2: *const Visual, arg3: c_int) -> Colormap;

    pub fn XCreatePixmapCursor(arg0: *const Display, arg1: Pixmap, arg2: Pixmap, arg3: *const XColor, arg4: *const XColor, arg5: c_uint, arg6: c_uint) -> Cursor;

    pub fn XCreateGlyphCursor(arg0: *const Display, arg1: Font, arg2: Font, arg3: c_uint, arg4: c_uint, arg5: *const XColor, arg6: *const XColor) -> Cursor;

    pub fn XCreateFontCursor(arg0: *const Display, arg1: c_uint) -> Cursor;

    pub fn XLoadFont(arg0: *const Display, arg1: *const c_char) -> Font;

    pub fn XCreateGC(arg0: *const Display, arg1: Drawable, arg2: c_ulong, arg3: *const XGCValues) -> GC;

    pub fn XGContextFromGC(arg0: GC) -> GContext;

    pub fn XFlushGC(arg0: *const Display, arg1: GC);

    pub fn XCreatePixmap(arg0: *const Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: c_uint) -> Pixmap;

    pub fn XCreateBitmapFromData(arg0: *const Display, arg1: Drawable, arg2: *const c_char, arg3: c_uint, arg4: c_uint) -> Pixmap;

    pub fn XCreatePixmapFromBitmapData(arg0: *const Display, arg1: Drawable, arg2: *const c_char, arg3: c_uint, arg4: c_uint, arg5: c_ulong, arg6: c_ulong, arg7: c_uint) -> Pixmap;

    pub fn XCreateSimpleWindow(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_uint, arg7: c_ulong, arg8: c_ulong) -> Window;

    pub fn XGetSelectionOwner(arg0: *const Display, arg1: Atom) -> Window;

    pub fn XCreateWindow(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_uint, arg9: *const Visual, arg10: c_ulong, arg11: *const XSetWindowAttributes) -> Window;

    pub fn XListInstalledColormaps(arg0: *const Display, arg1: Window, arg2: *const c_int) -> *const Colormap;

    pub fn XListFonts(arg0: *const Display, arg1: *const c_char, arg2: c_int, arg3: *const c_int) -> *const *const c_char;

    pub fn XListFontsWithInfo(arg0: *const Display, arg1: *const c_char, arg2: c_int, arg3: *const c_int, arg4: *const *const XFontStruct) -> *const *const c_char;

    pub fn XGetFontPath(arg0: *const Display, arg1: *const c_int) -> *const *const c_char;

    pub fn XListExtensions(arg0: *const Display, arg1: *const c_int) -> *const *const c_char;

    pub fn XListProperties(arg0: *const Display, arg1: Window, arg2: *const c_int) -> *const Atom;

    pub fn XListHosts(arg0: *const Display, arg1: *const c_int, arg2: *const c_int) -> *const XHostAddress;

    pub fn XKeycodeToKeysym(arg0: *const Display, arg1: KeyCode, arg2: c_int) -> KeySym;

    pub fn XLookupKeysym(arg0: *const XKeyEvent, arg1: c_int) -> KeySym;

    pub fn XGetKeyboardMapping(arg0: *const Display, arg1: KeyCode, arg2: c_int, arg3: *const c_int) -> *const KeySym;

    pub fn XStringToKeysym(arg0: *const c_char) -> KeySym;

    pub fn XMaxRequestSize(arg0: *const Display) -> c_long;

    pub fn XExtendedMaxRequestSize(arg0: *const Display) -> c_long;

    pub fn XResourceManagerString(arg0: *const Display) -> *const c_char;

    pub fn XScreenResourceString(arg0: *const Screen) -> *const c_char;

    pub fn XDisplayMotionBufferSize(arg0: *const Display) -> c_ulong;

    pub fn XVisualIDFromVisual(arg0: *const Visual) -> VisualID;

    pub fn XInitThreads() -> c_int;

    pub fn XLockDisplay(arg0: *const Display);

    pub fn XUnlockDisplay(arg0: *const Display);

    pub fn XInitExtension(arg0: *const Display, arg1: *const c_char) -> *const XExtCodes;

    pub fn XAddExtension(arg0: *const Display) -> *const XExtCodes;

    pub fn XFindOnExtensionList(arg0: *const *const XExtData, arg1: c_int) -> *const XExtData;

    pub fn XEHeadOfExtensionList(arg0: XEDataObject) -> *const *const XExtData;

    pub fn XRootWindow(arg0: *const Display, arg1: c_int) -> Window;

    pub fn XDefaultRootWindow(arg0: *const Display) -> Window;

    pub fn XRootWindowOfScreen(arg0: *const Screen) -> Window;

    pub fn XDefaultVisual(arg0: *const Display, arg1: c_int) -> *const Visual;

    pub fn XDefaultVisualOfScreen(arg0: *const Screen) -> *const Visual;

    pub fn XDefaultGC(arg0: *const Display, arg1: c_int) -> GC;

    pub fn XDefaultGCOfScreen(arg0: *const Screen) -> GC;

    pub fn XBlackPixel(arg0: *const Display, arg1: c_int) -> c_ulong;

    pub fn XWhitePixel(arg0: *const Display, arg1: c_int) -> c_ulong;

    pub fn XAllPlanes() -> c_ulong;

    pub fn XBlackPixelOfScreen(arg0: *const Screen) -> c_ulong;

    pub fn XWhitePixelOfScreen(arg0: *const Screen) -> c_ulong;

    pub fn XNextRequest(arg0: *const Display) -> c_ulong;

    pub fn XLastKnownRequestProcessed(arg0: *const Display) -> c_ulong;

    pub fn XServerVendor(arg0: *const Display) -> *const c_char;

    pub fn XDisplayString(arg0: *const Display) -> *const c_char;

    pub fn XDefaultColormap(arg0: *const Display, arg1: c_int) -> Colormap;

    pub fn XDefaultColormapOfScreen(arg0: *const Screen) -> Colormap;

    pub fn XDisplayOfScreen(arg0: *const Screen) -> *const Display;

    pub fn XScreenOfDisplay(arg0: *const Display, arg1: c_int) -> *const Screen;

    pub fn XDefaultScreenOfDisplay(arg0: *const Display) -> *const Screen;

    pub fn XEventMaskOfScreen(arg0: *const Screen) -> c_long;

    pub fn XScreenNumberOfScreen(arg0: *const Screen) -> c_int;

    pub fn XSetErrorHandler(arg0: XErrorHandler) -> XErrorHandler;

    pub fn XSetIOErrorHandler(arg0: XIOErrorHandler) -> XIOErrorHandler;

    pub fn XListPixmapFormats(arg0: *const Display, arg1: *const c_int) -> *const XPixmapFormatValues;

    pub fn XListDepths(arg0: *const Display, arg1: c_int, arg2: *const c_int) -> *const c_int;

    pub fn XReconfigureWMWindow(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_uint, arg4: *const XWindowChanges) -> c_int;

    pub fn XGetWMProtocols(arg0: *const Display, arg1: Window, arg2: *const *const Atom, arg3: *const c_int) -> c_int;

    pub fn XSetWMProtocols(arg0: *const Display, arg1: Window, arg2: *const Atom, arg3: c_int) -> c_int;

    pub fn XIconifyWindow(arg0: *const Display, arg1: Window, arg2: c_int) -> c_int;

    pub fn XWithdrawWindow(arg0: *const Display, arg1: Window, arg2: c_int) -> c_int;

    pub fn XGetCommand(arg0: *const Display, arg1: Window, arg2: *const *const *const c_char, arg3: *const c_int) -> c_int;

    pub fn XGetWMColormapWindows(arg0: *const Display, arg1: Window, arg2: *const *const Window, arg3: *const c_int) -> c_int;

    pub fn XSetWMColormapWindows(arg0: *const Display, arg1: Window, arg2: *const Window, arg3: c_int) -> c_int;

    pub fn XFreeStringList(arg0: *const *const c_char);

    pub fn XSetTransientForHint(arg0: *const Display, arg1: Window, arg2: Window) -> c_int;

    pub fn XActivateScreenSaver(arg0: *const Display) -> c_int;

    pub fn XAddHost(arg0: *const Display, arg1: *const XHostAddress) -> c_int;

    pub fn XAddHosts(arg0: *const Display, arg1: *const XHostAddress, arg2: c_int) -> c_int;

    pub fn XAddToExtensionList(arg0: *const *const struct__XExtData, arg1: *const XExtData) -> c_int;

    pub fn XAddToSaveSet(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XAllocColor(arg0: *const Display, arg1: Colormap, arg2: *const XColor) -> c_int;

    pub fn XAllocColorCells(arg0: *const Display, arg1: Colormap, arg2: c_int, arg3: *const c_ulong, arg4: c_uint, arg5: *const c_ulong, arg6: c_uint) -> c_int;

    pub fn XAllocColorPlanes(arg0: *const Display, arg1: Colormap, arg2: c_int, arg3: *const c_ulong, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: *const c_ulong, arg9: *const c_ulong, arg10: *const c_ulong) -> c_int;

    pub fn XAllocNamedColor(arg0: *const Display, arg1: Colormap, arg2: *const c_char, arg3: *const XColor, arg4: *const XColor) -> c_int;

    pub fn XAllowEvents(arg0: *const Display, arg1: c_int, arg2: Time) -> c_int;

    pub fn XAutoRepeatOff(arg0: *const Display) -> c_int;

    pub fn XAutoRepeatOn(arg0: *const Display) -> c_int;

    pub fn XBell(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XBitmapBitOrder(arg0: *const Display) -> c_int;

    pub fn XBitmapPad(arg0: *const Display) -> c_int;

    pub fn XBitmapUnit(arg0: *const Display) -> c_int;

    pub fn XCellsOfScreen(arg0: *const Screen) -> c_int;

    pub fn XChangeActivePointerGrab(arg0: *const Display, arg1: c_uint, arg2: Cursor, arg3: Time) -> c_int;

    pub fn XChangeGC(arg0: *const Display, arg1: GC, arg2: c_ulong, arg3: *const XGCValues) -> c_int;

    pub fn XChangeKeyboardControl(arg0: *const Display, arg1: c_ulong, arg2: *const XKeyboardControl) -> c_int;

    pub fn XChangeKeyboardMapping(arg0: *const Display, arg1: c_int, arg2: c_int, arg3: *const KeySym, arg4: c_int) -> c_int;

    pub fn XChangePointerControl(arg0: *const Display, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;

    pub fn XChangeProperty(arg0: *const Display, arg1: Window, arg2: Atom, arg3: Atom, arg4: c_int, arg5: c_int, arg6: *const c_uchar, arg7: c_int) -> c_int;

    pub fn XChangeSaveSet(arg0: *const Display, arg1: Window, arg2: c_int) -> c_int;

    pub fn XChangeWindowAttributes(arg0: *const Display, arg1: Window, arg2: c_ulong, arg3: *const XSetWindowAttributes) -> c_int;

    pub fn XCheckIfEvent(arg0: *const Display, arg1: *const XEvent, arg2: *const u8, arg3: XPointer) -> c_int;

    pub fn XCheckMaskEvent(arg0: *const Display, arg1: c_long, arg2: *const XEvent) -> c_int;

    pub fn XCheckTypedEvent(arg0: *const Display, arg1: c_int, arg2: *const XEvent) -> c_int;

    pub fn XCheckTypedWindowEvent(arg0: *const Display, arg1: Window, arg2: c_int, arg3: *const XEvent) -> c_int;

    pub fn XCheckWindowEvent(arg0: *const Display, arg1: Window, arg2: c_long, arg3: *const XEvent) -> c_int;

    pub fn XCirculateSubwindows(arg0: *const Display, arg1: Window, arg2: c_int) -> c_int;

    pub fn XCirculateSubwindowsDown(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XCirculateSubwindowsUp(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XClearArea(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint, arg6: c_int) -> c_int;

    pub fn XClearWindow(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XCloseDisplay(arg0: *const Display) -> c_int;

    pub fn XConfigureWindow(arg0: *const Display, arg1: Window, arg2: c_uint, arg3: *const XWindowChanges) -> c_int;

    pub fn XConnectionNumber(arg0: *const Display) -> c_int;

    pub fn XConvertSelection(arg0: *const Display, arg1: Atom, arg2: Atom, arg3: Atom, arg4: Window, arg5: Time) -> c_int;

    pub fn XCopyArea(arg0: *const Display, arg1: Drawable, arg2: Drawable, arg3: GC, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: c_int, arg9: c_int) -> c_int;

    pub fn XCopyGC(arg0: *const Display, arg1: GC, arg2: c_ulong, arg3: GC) -> c_int;

    pub fn XCopyPlane(arg0: *const Display, arg1: Drawable, arg2: Drawable, arg3: GC, arg4: c_int, arg5: c_int, arg6: c_uint, arg7: c_uint, arg8: c_int, arg9: c_int, arg10: c_ulong) -> c_int;

    pub fn XDefaultDepth(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XDefaultDepthOfScreen(arg0: *const Screen) -> c_int;

    pub fn XDefaultScreen(arg0: *const Display) -> c_int;

    pub fn XDefineCursor(arg0: *const Display, arg1: Window, arg2: Cursor) -> c_int;

    pub fn XDeleteProperty(arg0: *const Display, arg1: Window, arg2: Atom) -> c_int;

    pub fn XDestroyWindow(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XDestroySubwindows(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XDoesBackingStore(arg0: *const Screen) -> c_int;

    pub fn XDoesSaveUnders(arg0: *const Screen) -> c_int;

    pub fn XDisableAccessControl(arg0: *const Display) -> c_int;

    pub fn XDisplayCells(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XDisplayHeight(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XDisplayHeightMM(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XDisplayKeycodes(arg0: *const Display, arg1: *const c_int, arg2: *const c_int) -> c_int;

    pub fn XDisplayPlanes(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XDisplayWidth(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XDisplayWidthMM(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XDrawArc(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int) -> c_int;

    pub fn XDrawArcs(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XArc, arg4: c_int) -> c_int;

    pub fn XDrawImageString(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const c_char, arg6: c_int) -> c_int;

    pub fn XDrawImageString16(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const XChar2b, arg6: c_int) -> c_int;

    pub fn XDrawLine(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int) -> c_int;

    pub fn XDrawLines(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XPoint, arg4: c_int, arg5: c_int) -> c_int;

    pub fn XDrawPoint(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int) -> c_int;

    pub fn XDrawPoints(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XPoint, arg4: c_int, arg5: c_int) -> c_int;

    pub fn XDrawRectangle(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint) -> c_int;

    pub fn XDrawRectangles(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XRectangle, arg4: c_int) -> c_int;

    pub fn XDrawSegments(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XSegment, arg4: c_int) -> c_int;

    pub fn XDrawString(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const c_char, arg6: c_int) -> c_int;

    pub fn XDrawString16(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const XChar2b, arg6: c_int) -> c_int;

    pub fn XDrawText(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const XTextItem, arg6: c_int) -> c_int;

    pub fn XDrawText16(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const XTextItem16, arg6: c_int) -> c_int;

    pub fn XEnableAccessControl(arg0: *const Display) -> c_int;

    pub fn XEventsQueued(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XFetchName(arg0: *const Display, arg1: Window, arg2: *const *const c_char) -> c_int;

    pub fn XFillArc(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int) -> c_int;

    pub fn XFillArcs(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XArc, arg4: c_int) -> c_int;

    pub fn XFillPolygon(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XPoint, arg4: c_int, arg5: c_int, arg6: c_int) -> c_int;

    pub fn XFillRectangle(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint) -> c_int;

    pub fn XFillRectangles(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XRectangle, arg4: c_int) -> c_int;

    pub fn XFlush(arg0: *const Display) -> c_int;

    pub fn XForceScreenSaver(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XFree(arg0: *const c_void) -> c_int;

    pub fn XFreeColormap(arg0: *const Display, arg1: Colormap) -> c_int;

    pub fn XFreeColors(arg0: *const Display, arg1: Colormap, arg2: *const c_ulong, arg3: c_int, arg4: c_ulong) -> c_int;

    pub fn XFreeCursor(arg0: *const Display, arg1: Cursor) -> c_int;

    pub fn XFreeExtensionList(arg0: *const *const c_char) -> c_int;

    pub fn XFreeFont(arg0: *const Display, arg1: *const XFontStruct) -> c_int;

    pub fn XFreeFontInfo(arg0: *const *const c_char, arg1: *const XFontStruct, arg2: c_int) -> c_int;

    pub fn XFreeFontNames(arg0: *const *const c_char) -> c_int;

    pub fn XFreeFontPath(arg0: *const *const c_char) -> c_int;

    pub fn XFreeGC(arg0: *const Display, arg1: GC) -> c_int;

    pub fn XFreeModifiermap(arg0: *const XModifierKeymap) -> c_int;

    pub fn XFreePixmap(arg0: *const Display, arg1: Pixmap) -> c_int;

    pub fn XGeometry(arg0: *const Display, arg1: c_int, arg2: *const c_char, arg3: *const c_char, arg4: c_uint, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int, arg9: *const c_int, arg10: *const c_int, arg11: *const c_int, arg12: *const c_int) -> c_int;

    pub fn XGetErrorDatabaseText(arg0: *const Display, arg1: *const c_char, arg2: *const c_char, arg3: *const c_char, arg4: *const c_char, arg5: c_int) -> c_int;

    pub fn XGetErrorText(arg0: *const Display, arg1: c_int, arg2: *const c_char, arg3: c_int) -> c_int;

    pub fn XGetFontProperty(arg0: *const XFontStruct, arg1: Atom, arg2: *const c_ulong) -> c_int;

    pub fn XGetGCValues(arg0: *const Display, arg1: GC, arg2: c_ulong, arg3: *const XGCValues) -> c_int;

    pub fn XGetGeometry(arg0: *const Display, arg1: Drawable, arg2: *mut Window, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_uint, arg6: *mut c_uint, arg7: *mut c_uint, arg8: *mut c_uint) -> c_int;

    pub fn XGetIconName(arg0: *const Display, arg1: Window, arg2: *const *const c_char) -> c_int;

    pub fn XGetInputFocus(arg0: *const Display, arg1: *const Window, arg2: *const c_int) -> c_int;

    pub fn XGetKeyboardControl(arg0: *const Display, arg1: *const XKeyboardState) -> c_int;

    pub fn XGetPointerControl(arg0: *const Display, arg1: *const c_int, arg2: *const c_int, arg3: *const c_int) -> c_int;

    pub fn XGetPointerMapping(arg0: *const Display, arg1: *const c_uchar, arg2: c_int) -> c_int;

    pub fn XGetScreenSaver(arg0: *const Display, arg1: *const c_int, arg2: *const c_int, arg3: *const c_int, arg4: *const c_int) -> c_int;

    pub fn XGetTransientForHint(arg0: *const Display, arg1: Window, arg2: *const Window) -> c_int;

    pub fn XGetWindowProperty(arg0: *const Display, arg1: Window, arg2: Atom, arg3: c_long, arg4: c_long, arg5: c_int, arg6: Atom, arg7: *const Atom, arg8: *const c_int, arg9: *const c_ulong, arg10: *const c_ulong, arg11: *const *const c_uchar) -> c_int;

    pub fn XGetWindowAttributes(arg0: *const Display, arg1: Window, arg2: *const XWindowAttributes) -> c_int;

    pub fn XGrabButton(arg0: *const Display, arg1: c_uint, arg2: c_uint, arg3: Window, arg4: c_int, arg5: c_uint, arg6: c_int, arg7: c_int, arg8: Window, arg9: Cursor) -> c_int;

    pub fn XGrabKey(arg0: *const Display, arg1: c_int, arg2: c_uint, arg3: Window, arg4: c_int, arg5: c_int, arg6: c_int) -> c_int;

    pub fn XGrabKeyboard(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_int, arg5: Time) -> c_int;

    pub fn XGrabPointer(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_uint, arg4: c_int, arg5: c_int, arg6: Window, arg7: Cursor, arg8: Time) -> c_int;

    pub fn XGrabServer(arg0: *const Display) -> c_int;

    pub fn XHeightMMOfScreen(arg0: *const Screen) -> c_int;

    pub fn XHeightOfScreen(arg0: *const Screen) -> c_int;

    pub fn XIfEvent(arg0: *const Display, arg1: *const XEvent, arg2: *const u8, arg3: XPointer) -> c_int;

    pub fn XImageByteOrder(arg0: *const Display) -> c_int;

    pub fn XInstallColormap(arg0: *const Display, arg1: Colormap) -> c_int;

    pub fn XKeysymToKeycode(arg0: *const Display, arg1: KeySym) -> KeyCode;

    pub fn XKillClient(arg0: *const Display, arg1: XID) -> c_int;

    pub fn XLookupColor(arg0: *const Display, arg1: Colormap, arg2: *const c_char, arg3: *const XColor, arg4: *const XColor) -> c_int;

    pub fn XLowerWindow(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XMapRaised(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XMapSubwindows(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XMapWindow(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XMaskEvent(arg0: *const Display, arg1: c_long, arg2: *const XEvent) -> c_int;

    pub fn XMaxCmapsOfScreen(arg0: *const Screen) -> c_int;

    pub fn XMinCmapsOfScreen(arg0: *const Screen) -> c_int;

    pub fn XMoveResizeWindow(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint) -> c_int;

    pub fn XMoveWindow(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_int) -> c_int;

    pub fn XNextEvent(arg0: *const Display, arg1: *const XEvent) -> c_int;

    pub fn XNoOp(arg0: *const Display) -> c_int;

    pub fn XParseColor(arg0: *const Display, arg1: Colormap, arg2: *const c_char, arg3: *const XColor) -> c_int;

    pub fn XParseGeometry(arg0: *const c_char, arg1: *const c_int, arg2: *const c_int, arg3: *const c_uint, arg4: *const c_uint) -> c_int;

    pub fn XPeekEvent(arg0: *const Display, arg1: *const XEvent) -> c_int;

    pub fn XPeekIfEvent(arg0: *const Display, arg1: *const XEvent, arg2: *const u8, arg3: XPointer) -> c_int;

    pub fn XPending(arg0: *const Display) -> c_int;

    pub fn XPlanesOfScreen(arg0: *const Screen) -> c_int;

    pub fn XProtocolRevision(arg0: *const Display) -> c_int;

    pub fn XProtocolVersion(arg0: *const Display) -> c_int;

    pub fn XPutBackEvent(arg0: *const Display, arg1: *const XEvent) -> c_int;

    pub fn XPutImage(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: *const XImage, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_uint, arg9: c_uint) -> c_int;

    pub fn XQLength(arg0: *const Display) -> c_int;

    pub fn XQueryBestCursor(arg0: *const Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: *const c_uint, arg5: *const c_uint) -> c_int;

    pub fn XQueryBestSize(arg0: *const Display, arg1: c_int, arg2: Drawable, arg3: c_uint, arg4: c_uint, arg5: *const c_uint, arg6: *const c_uint) -> c_int;

    pub fn XQueryBestStipple(arg0: *const Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: *const c_uint, arg5: *const c_uint) -> c_int;

    pub fn XQueryBestTile(arg0: *const Display, arg1: Drawable, arg2: c_uint, arg3: c_uint, arg4: *const c_uint, arg5: *const c_uint) -> c_int;

    pub fn XQueryColor(arg0: *const Display, arg1: Colormap, arg2: *const XColor) -> c_int;

    pub fn XQueryColors(arg0: *const Display, arg1: Colormap, arg2: *const XColor, arg3: c_int) -> c_int;

    pub fn XQueryExtension(arg0: *const Display, arg1: *const c_char, arg2: *const c_int, arg3: *const c_int, arg4: *const c_int) -> c_int;

    pub fn XQueryKeymap(arg0: *const Display, arg1: *const c_char) -> c_int;

    pub fn XQueryPointer(arg0: *const Display, arg1: Window, arg2: *const Window, arg3: *const Window, arg4: *const c_int, arg5: *const c_int, arg6: *const c_int, arg7: *const c_int, arg8: *const c_uint) -> c_int;

    pub fn XQueryTextExtents(arg0: *const Display, arg1: XID, arg2: *const c_char, arg3: c_int, arg4: *const c_int, arg5: *const c_int, arg6: *const c_int, arg7: *const XCharStruct) -> c_int;

    pub fn XQueryTextExtents16(arg0: *const Display, arg1: XID, arg2: *const XChar2b, arg3: c_int, arg4: *const c_int, arg5: *const c_int, arg6: *const c_int, arg7: *const XCharStruct) -> c_int;

    pub fn XQueryTree(arg0: *const Display, arg1: Window, arg2: *const Window, arg3: *const Window, arg4: *const *const Window, arg5: *const c_uint) -> c_int;

    pub fn XRaiseWindow(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XReadBitmapFile(arg0: *const Display, arg1: Drawable, arg2: *const c_char, arg3: *const c_uint, arg4: *const c_uint, arg5: *const Pixmap, arg6: *const c_int, arg7: *const c_int) -> c_int;

    pub fn XReadBitmapFileData(arg0: *const c_char, arg1: *const c_uint, arg2: *const c_uint, arg3: *const *const c_uchar, arg4: *const c_int, arg5: *const c_int) -> c_int;

    pub fn XRebindKeysym(arg0: *const Display, arg1: KeySym, arg2: *const KeySym, arg3: c_int, arg4: *const c_uchar, arg5: c_int) -> c_int;

    pub fn XRecolorCursor(arg0: *const Display, arg1: Cursor, arg2: *const XColor, arg3: *const XColor) -> c_int;

    pub fn XRefreshKeyboardMapping(arg0: *const XMappingEvent) -> c_int;

    pub fn XRemoveFromSaveSet(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XRemoveHost(arg0: *const Display, arg1: *const XHostAddress) -> c_int;

    pub fn XRemoveHosts(arg0: *const Display, arg1: *const XHostAddress, arg2: c_int) -> c_int;

    pub fn XReparentWindow(arg0: *const Display, arg1: Window, arg2: Window, arg3: c_int, arg4: c_int) -> c_int;

    pub fn XResetScreenSaver(arg0: *const Display) -> c_int;

    pub fn XResizeWindow(arg0: *const Display, arg1: Window, arg2: c_uint, arg3: c_uint) -> c_int;

    pub fn XRestackWindows(arg0: *const Display, arg1: *const Window, arg2: c_int) -> c_int;

    pub fn XRotateBuffers(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XRotateWindowProperties(arg0: *const Display, arg1: Window, arg2: *const Atom, arg3: c_int, arg4: c_int) -> c_int;

    pub fn XScreenCount(arg0: *const Display) -> c_int;

    pub fn XSelectInput(arg0: *const Display, arg1: Window, arg2: c_long) -> c_int;

    pub fn XSendEvent(arg0: *const Display, arg1: Window, arg2: c_int, arg3: c_long, arg4: *const XEvent) -> c_int;

    pub fn XSetAccessControl(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XSetArcMode(arg0: *const Display, arg1: GC, arg2: c_int) -> c_int;

    pub fn XSetBackground(arg0: *const Display, arg1: GC, arg2: c_ulong) -> c_int;

    pub fn XSetClipMask(arg0: *const Display, arg1: GC, arg2: Pixmap) -> c_int;

    pub fn XSetClipOrigin(arg0: *const Display, arg1: GC, arg2: c_int, arg3: c_int) -> c_int;

    pub fn XSetClipRectangles(arg0: *const Display, arg1: GC, arg2: c_int, arg3: c_int, arg4: *const XRectangle, arg5: c_int, arg6: c_int) -> c_int;

    pub fn XSetCloseDownMode(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XSetCommand(arg0: *const Display, arg1: Window, arg2: *const *const c_char, arg3: c_int) -> c_int;

    pub fn XSetDashes(arg0: *const Display, arg1: GC, arg2: c_int, arg3: *const c_char, arg4: c_int) -> c_int;

    pub fn XSetFillRule(arg0: *const Display, arg1: GC, arg2: c_int) -> c_int;

    pub fn XSetFillStyle(arg0: *const Display, arg1: GC, arg2: c_int) -> c_int;

    pub fn XSetFont(arg0: *const Display, arg1: GC, arg2: Font) -> c_int;

    pub fn XSetFontPath(arg0: *const Display, arg1: *const *const c_char, arg2: c_int) -> c_int;

    pub fn XSetForeground(arg0: *const Display, arg1: GC, arg2: c_ulong) -> c_int;

    pub fn XSetFunction(arg0: *const Display, arg1: GC, arg2: c_int) -> c_int;

    pub fn XSetGraphicsExposures(arg0: *const Display, arg1: GC, arg2: c_int) -> c_int;

    pub fn XSetIconName(arg0: *const Display, arg1: Window, arg2: *const c_char) -> c_int;

    pub fn XSetInputFocus(arg0: *const Display, arg1: Window, arg2: c_int, arg3: Time) -> c_int;

    pub fn XSetLineAttributes(arg0: *const Display, arg1: GC, arg2: c_uint, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;

    pub fn XSetModifierMapping(arg0: *const Display, arg1: *const XModifierKeymap) -> c_int;

    pub fn XSetPlaneMask(arg0: *const Display, arg1: GC, arg2: c_ulong) -> c_int;

    pub fn XSetPointerMapping(arg0: *const Display, arg1: *const c_uchar, arg2: c_int) -> c_int;

    pub fn XSetScreenSaver(arg0: *const Display, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> c_int;

    pub fn XSetSelectionOwner(arg0: *const Display, arg1: Atom, arg2: Window, arg3: Time) -> c_int;

    pub fn XSetState(arg0: *const Display, arg1: GC, arg2: c_ulong, arg3: c_ulong, arg4: c_int, arg5: c_ulong) -> c_int;

    pub fn XSetStipple(arg0: *const Display, arg1: GC, arg2: Pixmap) -> c_int;

    pub fn XSetSubwindowMode(arg0: *const Display, arg1: GC, arg2: c_int) -> c_int;

    pub fn XSetTSOrigin(arg0: *const Display, arg1: GC, arg2: c_int, arg3: c_int) -> c_int;

    pub fn XSetTile(arg0: *const Display, arg1: GC, arg2: Pixmap) -> c_int;

    pub fn XSetWindowBackground(arg0: *const Display, arg1: Window, arg2: c_ulong) -> c_int;

    pub fn XSetWindowBackgroundPixmap(arg0: *const Display, arg1: Window, arg2: Pixmap) -> c_int;

    pub fn XSetWindowBorder(arg0: *const Display, arg1: Window, arg2: c_ulong) -> c_int;

    pub fn XSetWindowBorderPixmap(arg0: *const Display, arg1: Window, arg2: Pixmap) -> c_int;

    pub fn XSetWindowBorderWidth(arg0: *const Display, arg1: Window, arg2: c_uint) -> c_int;

    pub fn XSetWindowColormap(arg0: *const Display, arg1: Window, arg2: Colormap) -> c_int;

    pub fn XStoreBuffer(arg0: *const Display, arg1: *const c_char, arg2: c_int, arg3: c_int) -> c_int;

    pub fn XStoreBytes(arg0: *const Display, arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn XStoreColor(arg0: *const Display, arg1: Colormap, arg2: *const XColor) -> c_int;

    pub fn XStoreColors(arg0: *const Display, arg1: Colormap, arg2: *const XColor, arg3: c_int) -> c_int;

    pub fn XStoreName(arg0: *const Display, arg1: Window, arg2: *const c_char) -> c_int;

    pub fn XStoreNamedColor(arg0: *const Display, arg1: Colormap, arg2: *const c_char, arg3: c_ulong, arg4: c_int) -> c_int;

    pub fn XSync(arg0: *const Display, arg1: c_int) -> c_int;

    pub fn XTextExtents(arg0: *const XFontStruct, arg1: *const c_char, arg2: c_int, arg3: *const c_int, arg4: *const c_int, arg5: *const c_int, arg6: *const XCharStruct) -> c_int;

    pub fn XTextExtents16(arg0: *const XFontStruct, arg1: *const XChar2b, arg2: c_int, arg3: *const c_int, arg4: *const c_int, arg5: *const c_int, arg6: *const XCharStruct) -> c_int;

    pub fn XTextWidth(arg0: *const XFontStruct, arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn XTextWidth16(arg0: *const XFontStruct, arg1: *const XChar2b, arg2: c_int) -> c_int;

    pub fn XTranslateCoordinates(arg0: *const Display, arg1: Window, arg2: Window, arg3: c_int, arg4: c_int, arg5: *const c_int, arg6: *const c_int, arg7: *const Window) -> c_int;

    pub fn XUndefineCursor(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XUngrabButton(arg0: *const Display, arg1: c_uint, arg2: c_uint, arg3: Window) -> c_int;

    pub fn XUngrabKey(arg0: *const Display, arg1: c_int, arg2: c_uint, arg3: Window) -> c_int;

    pub fn XUngrabKeyboard(arg0: *const Display, arg1: Time) -> c_int;

    pub fn XUngrabPointer(arg0: *const Display, arg1: Time) -> c_int;

    pub fn XUngrabServer(arg0: *const Display) -> c_int;

    pub fn XUninstallColormap(arg0: *const Display, arg1: Colormap) -> c_int;

    pub fn XUnloadFont(arg0: *const Display, arg1: Font) -> c_int;

    pub fn XUnmapSubwindows(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XUnmapWindow(arg0: *const Display, arg1: Window) -> c_int;

    pub fn XVendorRelease(arg0: *const Display) -> c_int;

    pub fn XWarpPointer(arg0: *const Display, arg1: Window, arg2: Window, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int) -> c_int;

    pub fn XWidthMMOfScreen(arg0: *const Screen) -> c_int;

    pub fn XWidthOfScreen(arg0: *const Screen) -> c_int;

    pub fn XWindowEvent(arg0: *const Display, arg1: Window, arg2: c_long, arg3: *const XEvent) -> c_int;

    pub fn XWriteBitmapFile(arg0: *const Display, arg1: *const c_char, arg2: Pixmap, arg3: c_uint, arg4: c_uint, arg5: c_int, arg6: c_int) -> c_int;

    pub fn XSupportsLocale() -> c_int;

    pub fn XSetLocaleModifiers(arg0: *const c_char) -> *const c_char;

    pub fn XOpenOM(arg0: *const Display, arg1: *const struct__XrmHashBucketRec, arg2: *const c_char, arg3: *const c_char) -> XOM;

    pub fn XCloseOM(arg0: XOM) -> c_int;

    pub fn XSetOMValues(arg0: XOM/* FIXME: variadic function */) -> *const c_char;

    pub fn XGetOMValues(arg0: XOM/* FIXME: variadic function */) -> *const c_char;

    pub fn XDisplayOfOM(arg0: XOM) -> *const Display;

    pub fn XLocaleOfOM(arg0: XOM) -> *const c_char;

    pub fn XCreateOC(arg0: XOM/* FIXME: variadic function */) -> XOC;

    pub fn XDestroyOC(arg0: XOC);

    pub fn XOMOfOC(arg0: XOC) -> XOM;

    pub fn XSetOCValues(arg0: XOC/* FIXME: variadic function */) -> *const c_char;

    pub fn XGetOCValues(arg0: XOC/* FIXME: variadic function */) -> *const c_char;

    pub fn XCreateFontSet(arg0: *const Display, arg1: *const c_char, arg2: *const *const *const c_char, arg3: *const c_int, arg4: *const *const c_char) -> XFontSet;

    pub fn XFreeFontSet(arg0: *const Display, arg1: XFontSet);

    pub fn XFontsOfFontSet(arg0: XFontSet, arg1: *const *const *const XFontStruct, arg2: *const *const *const c_char) -> c_int;

    pub fn XBaseFontNameListOfFontSet(arg0: XFontSet) -> *const c_char;

    pub fn XLocaleOfFontSet(arg0: XFontSet) -> *const c_char;

    pub fn XContextDependentDrawing(arg0: XFontSet) -> c_int;

    pub fn XDirectionalDependentDrawing(arg0: XFontSet) -> c_int;

    pub fn XContextualDrawing(arg0: XFontSet) -> c_int;

    pub fn XExtentsOfFontSet(arg0: XFontSet) -> *const XFontSetExtents;

    pub fn XmbTextEscapement(arg0: XFontSet, arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn XwcTextEscapement(arg0: XFontSet, arg1: *const wchar_t, arg2: c_int) -> c_int;

    pub fn Xutf8TextEscapement(arg0: XFontSet, arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn XmbTextExtents(arg0: XFontSet, arg1: *const c_char, arg2: c_int, arg3: *const XRectangle, arg4: *const XRectangle) -> c_int;

    pub fn XwcTextExtents(arg0: XFontSet, arg1: *const wchar_t, arg2: c_int, arg3: *const XRectangle, arg4: *const XRectangle) -> c_int;

    pub fn Xutf8TextExtents(arg0: XFontSet, arg1: *const c_char, arg2: c_int, arg3: *const XRectangle, arg4: *const XRectangle) -> c_int;

    pub fn XmbTextPerCharExtents(arg0: XFontSet, arg1: *const c_char, arg2: c_int, arg3: *const XRectangle, arg4: *const XRectangle, arg5: c_int, arg6: *const c_int, arg7: *const XRectangle, arg8: *const XRectangle) -> c_int;

    pub fn XwcTextPerCharExtents(arg0: XFontSet, arg1: *const wchar_t, arg2: c_int, arg3: *const XRectangle, arg4: *const XRectangle, arg5: c_int, arg6: *const c_int, arg7: *const XRectangle, arg8: *const XRectangle) -> c_int;

    pub fn Xutf8TextPerCharExtents(arg0: XFontSet, arg1: *const c_char, arg2: c_int, arg3: *const XRectangle, arg4: *const XRectangle, arg5: c_int, arg6: *const c_int, arg7: *const XRectangle, arg8: *const XRectangle) -> c_int;

    pub fn XmbDrawText(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const XmbTextItem, arg6: c_int);

    pub fn XwcDrawText(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const XwcTextItem, arg6: c_int);

    pub fn Xutf8DrawText(arg0: *const Display, arg1: Drawable, arg2: GC, arg3: c_int, arg4: c_int, arg5: *const XmbTextItem, arg6: c_int);

    pub fn XmbDrawString(arg0: *const Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *const c_char, arg7: c_int);

    pub fn XwcDrawString(arg0: *const Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *const wchar_t, arg7: c_int);

    pub fn Xutf8DrawString(arg0: *const Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *const c_char, arg7: c_int);

    pub fn XmbDrawImageString(arg0: *const Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *const c_char, arg7: c_int);

    pub fn XwcDrawImageString(arg0: *const Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *const wchar_t, arg7: c_int);

    pub fn Xutf8DrawImageString(arg0: *const Display, arg1: Drawable, arg2: XFontSet, arg3: GC, arg4: c_int, arg5: c_int, arg6: *const c_char, arg7: c_int);

    pub fn XOpenIM(arg0: *const Display, arg1: *const struct__XrmHashBucketRec, arg2: *const c_char, arg3: *const c_char) -> XIM;

    pub fn XCloseIM(arg0: XIM) -> c_int;

    pub fn XGetIMValues(arg0: XIM/* FIXME: variadic function */) -> *const c_char;

    pub fn XSetIMValues(arg0: XIM/* FIXME: variadic function */) -> *const c_char;

    pub fn XDisplayOfIM(arg0: XIM) -> *const Display;

    pub fn XLocaleOfIM(arg0: XIM) -> *const c_char;

    pub fn XCreateIC(arg0: XIM/* FIXME: variadic function */) -> XIC;

    pub fn XDestroyIC(arg0: XIC);

    pub fn XSetICFocus(arg0: XIC);

    pub fn XUnsetICFocus(arg0: XIC);

    pub fn XwcResetIC(arg0: XIC) -> *const wchar_t;

    pub fn XmbResetIC(arg0: XIC) -> *const c_char;

    pub fn Xutf8ResetIC(arg0: XIC) -> *const c_char;

    pub fn XSetICValues(arg0: XIC/* FIXME: variadic function */) -> *const c_char;

    pub fn XGetICValues(arg0: XIC/* FIXME: variadic function */) -> *const c_char;

    pub fn XIMOfIC(arg0: XIC) -> XIM;

    pub fn XFilterEvent(arg0: *const XEvent, arg1: Window) -> c_int;

    pub fn XmbLookupString(arg0: XIC, arg1: *const XKeyPressedEvent, arg2: *const c_char, arg3: c_int, arg4: *const KeySym, arg5: *const c_int) -> c_int;

    pub fn XwcLookupString(arg0: XIC, arg1: *const XKeyPressedEvent, arg2: *const wchar_t, arg3: c_int, arg4: *const KeySym, arg5: *const c_int) -> c_int;

    pub fn Xutf8LookupString(arg0: XIC, arg1: *const XKeyPressedEvent, arg2: *const c_char, arg3: c_int, arg4: *const KeySym, arg5: *const c_int) -> c_int;

    pub fn XVaCreateNestedList(arg0: c_int/* FIXME: variadic function */) -> XVaNestedList;

    pub fn XRegisterIMInstantiateCallback(arg0: *const Display, arg1: *const struct__XrmHashBucketRec, arg2: *const c_char, arg3: *const c_char, arg4: XIDProc, arg5: XPointer) -> c_int;

    pub fn XUnregisterIMInstantiateCallback(arg0: *const Display, arg1: *const struct__XrmHashBucketRec, arg2: *const c_char, arg3: *const c_char, arg4: XIDProc, arg5: XPointer) -> c_int;

    pub fn XInternalConnectionNumbers(arg0: *const Display, arg1: *const *const c_int, arg2: *const c_int) -> c_int;

    pub fn XProcessInternalConnection(arg0: *const Display, arg1: c_int);

    pub fn XAddConnectionWatch(arg0: *const Display, arg1: XConnectionWatchProc, arg2: XPointer) -> c_int;

    pub fn XRemoveConnectionWatch(arg0: *const Display, arg1: XConnectionWatchProc, arg2: XPointer);

    pub fn XSetAuthorization(arg0: *const c_char, arg1: c_int, arg2: *const c_char, arg3: c_int);

    pub fn _Xmbtowc(arg0: *const wchar_t, arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn _Xwctomb(arg0: *const c_char, arg1: wchar_t) -> c_int;

    pub fn XGetEventData(arg0: *const Display, arg1: *const XGenericEventCookie) -> c_int;

    pub fn XFreeEventData(arg0: *const Display, arg1: *const XGenericEventCookie);

}
