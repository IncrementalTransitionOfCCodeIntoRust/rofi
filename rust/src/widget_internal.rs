/***
 *  widget_internals
***/

extern crate glib;

pub use crate::rofi_types::RofiPadding;
pub use crate::widget::WidgetType;
pub use crate::widget::WidgetTriggerActionResult;

use cairo_sys::cairo_t;
use std::any::Any;

pub type WidgetTriggerActionCB = Option<fn(wid: Box<widget>, action: u16, x: i16, y: i16, user_data: Option<Box<dyn Any>>) -> WidgetTriggerActionResult>;

pub struct widget {
    /** The type of the widget */
    pub _type: WidgetType,
    /** X position relative to parent */
    pub x: i16,
    /** Y position relative to parent */
    pub y: i16,
    /** Width of the widget */
    pub w: i16,
    /** Height of the widget */
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
    /** Expand the widget when packed */
    pub expand: bool,
    /** Place widget at end of parent */
    pub end: bool,
    /** Parent widget */
    pub parent: Option<Box<widget>>,
    /** Internal */
    pub need_redraw: bool,
    /** get width of widget implementation function */
    pub get_width: Option<fn(Box<widget>) -> i16>,
    /** get height of widget implementation function */
    pub get_height: Option<fn(Box<widget>) -> i16>,
    /** draw widget implementation function */
    pub draw: Option<fn(widget: Box<widget>, draw: &cairo_t) -> ()>,
    /** resize widget implementation function */
    pub resize: Option<fn(Box<widget>, i16, i16) -> ()>,
    /** update widget implementation function */
    pub update: Option<fn(Box<widget>) -> ()>,

    /** Handle mouse motion, used for dragging */
    pub motion_notify: fn(Box<widget>, x: i16, y: i16) -> bool,

    pub get_desired_height: fn(Box<widget>) -> i16,
    pub get_desired_width: fn(Box<widget>) -> i16,

    pub set_state: Option<fn(Box<widget>, String) -> ()>,  // String was const

    /** widget find_mouse_target callback */        // TODO tranlate
    pub find_mouse_target: Option<fn(wid: Box<widget>,_type: WidgetType, x: i16, y: i16) -> Box<widget>>,

    pub trigger_action: WidgetTriggerActionCB,
    /** user data for find_mouse_target and trigger_action callback */
    pub trigger_action_cb_data: Option<Box<dyn Any>>,      // TODO verify type (was void*)

    // /** widget trigger_action callback */
    //typedef WidgetTriggerActionResult ( *widget_trigger_action_cb )( widget *widget, guint action, gint x, gint y, void *user_data );
    //pub trigger_action: Option<fn(wid: Box<widget>, action: u16, x: i16, y: i16, data: Option<libc::c_int>) -> WidgetTriggerActionResult>,

    /** Free widget callback */
    pub free: fn(widget: Box<widget>) -> (),

    /** Name of widget (used for theming) */
    pub name: String,
    pub state: String,   // state was const
}
