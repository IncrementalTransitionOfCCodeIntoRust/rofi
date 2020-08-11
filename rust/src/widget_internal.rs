/***
 *  _widget_internals
***/

extern crate glib;

pub use crate::bindings::RofiPadding;
pub use crate::widget_::WidgetType;
pub use crate::widget_::WidgetTriggerActionResult;

use cairo_sys::cairo_t;
use std::any::Any;

pub type WidgetTriggerActionCB = Option<fn(wid: *mut _widget, action: u16, x: i16, y: i16, user_data: Option<Box<dyn Any>>) -> WidgetTriggerActionResult>;

#[repr(C)]
pub struct _widget {
    /** The type of the _widget */
    pub type_: WidgetType,
    /** X position relative to parent */
    pub x: i16,
    /** Y position relative to parent */
    pub y: i16,
    /** Width of the _widget */
    pub w: i16,
    /** Height of the _widget */
    pub h: i16,
    /** RofiPadding */
    pub def_margin: RofiPadding,
    pub def_padding: RofiPadding,
    pub def_border: RofiPadding,
    pub def_border_radius: RofiPadding,
    pub margin: RofiPadding,
    pub padding: RofiPadding,
    pub border: RofiPadding,
    pub border_radius: RofiPadding,

    /** enabled or not */
    pub enabled: bool,
    /** Expand the _widget when packed */
    pub expand: bool,
    /** Place _widget at end of parent */
    pub end: bool,
    /** Parent _widget */
    pub parent: Option<*mut _widget>,
    /** Internal */
    pub need_redraw: bool,
    /** get width of _widget implementation function */
    pub get_width: Option<fn(*mut _widget) -> i16>,
    /** get height of _widget implementation function */
    pub get_height: Option<fn(*mut _widget) -> i16>,
    /** draw _widget implementation function */
    pub draw: Option<fn(widget: *mut _widget, draw: *mut cairo_t) -> ()>,
    /** resize _widget implementation function */
    pub resize: Option<fn(*mut _widget, i16, i16) -> ()>,
    /** update _widget implementation function */
    pub update: Option<fn(*mut _widget) -> ()>,

    /** Handle mouse motion, used for dragging */
    pub motion_notify: Option<fn(*mut _widget, x: i16, y: i16) -> bool>,

    pub get_desired_height: Option<fn(*mut _widget) -> i16>,
    pub get_desired_width: Option<fn(*mut _widget) -> i16>,

    pub set_state: Option<fn(*mut _widget, String) -> ()>,  // String was const

    /** _widget find_mouse_target callback */        // TODO tranlate
    pub find_mouse_target: Option<fn(wid: *mut _widget, type_: WidgetType, x: i16, y: i16) -> Option<*mut _widget>>,

    pub trigger_action: WidgetTriggerActionCB,
    /** user data for find_mouse_target and trigger_action callback */
    pub trigger_action_cb_data: Option<Box<dyn Any>>,      // TODO verify type (was void*)

    // /** _widget trigger_action callback */
    //typedef WidgetTriggerActionResult ( *_widget_trigger_action_cb )( _widget *_widget, guint action, gint x, gint y, void *user_data );
    //pub trigger_action: Option<fn(wid: *mut _widget, action: u16, x: i16, y: i16, data: Option<libc::c_int>) -> WidgetTriggerActionResult>,

    /** Free _widget callback */
    pub free: fn(widget: *mut _widget) -> (),

    /** Name of _widget (used for theming) */
    pub name: String,
    pub state: String,   // state was const
}
