
/***
 *  widget_
***/

use crate::bindings::{
    BindingsScope_SCOPE_MOUSE_LISTVIEW, BindingsScope_SCOPE_MOUSE_LISTVIEW_ELEMENT, BindingsScope_SCOPE_MOUSE_EDITBOX,
    BindingsScope_SCOPE_MOUSE_SCROLLBAR, BindingsScope_SCOPE_MOUSE_MODE_SWITCHER,
    RofiPadding, RofiDistance, RofiDistanceUnit, RofiDistanceModifier, RofiPixelUnit, RofiLineStyle,
    RofiLineStyle_ROFI_HL_SOLID, RofiPixelUnit_ROFI_PU_PX, RofiDistanceModifier_ROFI_DISTANCE_MODIFIER_NONE,
    rofi_theme_get_padding, rofi_theme_get_boolean,
    widget
};
use crate::widget_internal::_widget;

use pub_functions::to_const_i8;

use std::mem::{self, MaybeUninit};

/**
 * Whether and how the action was handled
 */
pub enum WidgetTriggerActionResult {
    /** The action was ignore and should bubble */
    WIDGET_TRIGGER_ACTION_RESULT_IGNORED,
    /** The action was handled directly */
    WIDGET_TRIGGER_ACTION_RESULT_HANDLED,
    /** The action was handled and should start the grab for motion events */
    WIDGET_TRIGGER_ACTION_RESULT_GRAB_MOTION_BEGIN,
    /** The action was handled and should stop the grab for motion events */
    WIDGET_TRIGGER_ACTION_RESULT_GRAB_MOTION_END,
}

/**
 * Type of the wid. It is used to bubble events to the relevant wid.
 */
pub enum WidgetType {
    /** Default type */
    WIDGET_TYPE_UNKNOWN,
    /** The listview _widget */
    WIDGET_TYPE_LISTVIEW = BindingsScope_SCOPE_MOUSE_LISTVIEW as isize,
    /** An element in the listview */
    WIDGET_TYPE_LISTVIEW_ELEMENT = BindingsScope_SCOPE_MOUSE_LISTVIEW_ELEMENT as isize,
    /** The input bar edit box */
    WIDGET_TYPE_EDITBOX = BindingsScope_SCOPE_MOUSE_EDITBOX as isize,
    /** The listview scrollbar */
    WIDGET_TYPE_SCROLLBAR = BindingsScope_SCOPE_MOUSE_SCROLLBAR as isize,
    /** A _widget allowing user to swithc between modi */
    WIDGET_TYPE_MODE_SWITCHER = BindingsScope_SCOPE_MOUSE_MODE_SWITCHER as isize,
    /** Text-only textbox */
    WIDGET_TYPE_TEXTBOX_TEXT,
}

const WIDGET_DEFAULT_PADDING: f64 = 0.0;

pub fn widget_padding_init() -> RofiDistance {
    RofiDistance {
        base: RofiDistanceUnit {
            distance: WIDGET_DEFAULT_PADDING,
            type_: RofiPixelUnit_ROFI_PU_PX,
            modtype: RofiDistanceModifier_ROFI_DISTANCE_MODIFIER_NONE,
            left: unsafe { MaybeUninit::uninit().assume_init() },
            right: unsafe { MaybeUninit::uninit().assume_init() }
        },
        style: RofiLineStyle_ROFI_HL_SOLID,
    }
}

pub fn widget_init(wid: *mut _widget, parent: Option<*mut _widget>, type_: WidgetType, name: String) -> () {
    unsafe {
        (*wid).type_ = type_;
        (*wid).parent = parent;
        (*wid).name = name;
        (*wid).def_padding = RofiPadding {
            top: widget_padding_init(),
            right: widget_padding_init(),
            bottom: widget_padding_init(),
            left: widget_padding_init(),
        };
        (*wid).def_border = RofiPadding {
            top: widget_padding_init(),
            right: widget_padding_init(),
            bottom: widget_padding_init(),
            left: widget_padding_init(),
        };
        (*wid).def_border_radius = RofiPadding {
            top: widget_padding_init(),
            right: widget_padding_init(),
            bottom: widget_padding_init(),
            left: widget_padding_init(),
        };
        (*wid).def_margin = RofiPadding {
            top: widget_padding_init(),
            right: widget_padding_init(),
            bottom: widget_padding_init(),
            left: widget_padding_init(),
        };

        let bindings_wid = wid as *const widget;
        (*wid).padding= rofi_theme_get_padding(bindings_wid, to_const_i8("padding"), (*bindings_wid).def_padding);
        (*wid).border = rofi_theme_get_padding(bindings_wid, to_const_i8("border"), (*bindings_wid).def_border);
        (*wid).border_radius = rofi_theme_get_padding(bindings_wid, to_const_i8("border-radius"), (*bindings_wid).def_border_radius);
        (*wid).margin = rofi_theme_get_padding(bindings_wid, to_const_i8("margin"), (*bindings_wid).def_margin);

        // bled by default
        let bool_res = rofi_theme_get_boolean(bindings_wid, to_const_i8("enabled"), 1) != 0;
        (*wid).enabled = bool_res as bool;
    }
}
