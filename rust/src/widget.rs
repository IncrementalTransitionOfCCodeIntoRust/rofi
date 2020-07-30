/***
 *  _widget_internals
***/

extern crate cairo;
extern crate glib;
extern crate xcb;

use crate::rofi_types::*;
use crate::_widget_internal::_widget;
pub use crate::_widget_internal::WidgetTriggerActionCB;
pub use crate::_box::*;

use cairo_sys::*;
use std::any::Any;

pub const PI: f64 = 3.141593;
pub const PI_2: f64 = 1.570796;

/**
 * Type of the wid. It is used to bubble events to the relevant wid.
 */
pub enum WidgetType {
    /** Default type */
    WIDGET_TYPE_UNKNOWN,
    /** The listview _widget */
    WIDGET_TYPE_LISTVIEW = SCOPE_MOUSE_LISTVIEW,
    /** An element in the listview */
    WIDGET_TYPE_LISTVIEW_ELEMENT = SCOPE_MOUSE_LISTVIEW_ELEMENT,
    /** The input bar edit box */
    WIDGET_TYPE_EDITBOX = SCOPE_MOUSE_EDITBOX,
    /** The listview scrollbar */
    WIDGET_TYPE_SCROLLBAR = SCOPE_MOUSE_SCROLLBAR,
    /** A _widget allowing user to swithc between modi */
    WIDGET_TYPE_MODE_SWITCHER = SCOPE_MOUSE_MODE_SWITCHER,
    /** Text-only textbox */
    WIDGET_TYPE_TEXTBOX_TEXT,
}

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

/** Macro to get _widget from an implementation (e.g. textbox/scrollbar) */
//#define WIDGET( a )    ( (_widget *) ( a ) )   // TODO translate, or rewrite when used

//####################################################################################################################

const WIDGET_DEFAULT_PADDING: f64 = 0.0;

//#define WIDGET_PADDING_INIT { { WIDGET_DEFAULT_PADDING, ROFI_PU_PX, ROFI_DISTANCE_MODIFIER_NONE, NULL, NULL }, ROFI_HL_SOLID }  // TODO translate
pub fn widget_padding_init() -> RofiDistance {
    RofiDistance {
        base: RofiDistanceUnit {
            distance: WIDGET_DEFAULT_PADDING,
            _type: RofiPixelUnit::ROFI_PU_PX,
            modtype: RofiDistanceModifier::ROFI_DISTANCE_MODIFIER_NONE,
            left: None,
            right: None,
        },
        style: RofiLineStyle::ROFI_HL_SOLID,
    }
}

// TODO check for safe cairo functions / replacements
unsafe fn draw_rounded_rect(
    d: *mut cairo_t,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    r0: f64,
    r1: f64,
    r2: f64,
    r3: f64,
) -> () {
    if r0 > 0.0 {
        cairo_move_to(d, x1, y1 + r0);
        cairo_arc(d, x1 + r0, y1 + r0, r0, -PI, -PI_2);
    } else {
        cairo_move_to(d, x1, y1);
    }

    if r1 > 0.0 {
        cairo_line_to(d, x2 - r1, y1);
        cairo_arc(d, x2 - r1, y1 + r1, r1, -PI_2, 0.0);
    } else {
        cairo_line_to(d, x2, y1);
    }

    if r2 > 0.0 {
        cairo_line_to(d, x2, y2 - r2);
        cairo_arc(d, x2 - r2, y2 - r2, r2, 0.0, PI_2);
    } else {
        cairo_line_to(d, x2, y2);
    }

    if r3 > 0.0 {
        cairo_line_to(d, x1 + r3, y2);
        cairo_arc(d, x1 + r3, y2 - r3, r3, PI_2, PI);
    } else {
        cairo_line_to(d, x1, y2);
    }

    cairo_close_path(d);
}

pub fn widget_init(wid: Box<_widget>, parent: Option<Box<_widget>>, _type: WidgetType, name: String) -> () {
    wid._type = _type;
    match parent {
        Some(val) => wid.parent = Some(val),
        None => wid.parent = None
    }
    wid.name = name;
    wid.def_padding = RofiPadding {
        top: widget_padding_init(),
        right: widget_padding_init(),
        bottom: widget_padding_init(),
        left: widget_padding_init(),
    };
    wid.def_border = RofiPadding {
        top: widget_padding_init(),
        right: widget_padding_init(),
        bottom: widget_padding_init(),
        left: widget_padding_init(),
    };
    wid.def_border_radius = RofiPadding {
        top: widget_padding_init(),
        right: widget_padding_init(),
        bottom: widget_padding_init(),
        left: widget_padding_init(),
    };
    wid.def_margin = RofiPadding {
        top: widget_padding_init(),
        right: widget_padding_init(),
        bottom: widget_padding_init(),
        left: widget_padding_init(),
    };

    wid.padding = rofi_theme_get_padding(wid, "padding", wid.def_padding);
    wid.border = rofi_theme_get_padding(wid, "border", wid.def_border);
    wid.border_radius = rofi_theme_get_padding(wid, "border-radius", wid.def_border_radius);
    wid.margin = rofi_theme_get_padding(wid, "margin", wid.def_margin);

    // bled by default
    wid.enabled = rofi_theme_get_boolean(wid, "enabled", 1);
}

pub fn widget_set_state(wid: Box<_widget>, state: String) -> () {
    if wid.state == state {
        // Update border.
        wid.border = rofi_theme_get_padding(wid, "border", wid.def_border);
        wid.border_radius = rofi_theme_get_padding(wid, "border-radius", wid.def_border_radius);
        match wid.set_state {
            Some(_impl) => {
                Some(_impl(wid, state));
            },
            None => {}
        }

        widget_queue_redraw(wid);
    }
}

pub fn widget_intersect(wid: Box<_widget>, x: i16, y: i16) -> bool {
    if x >= wid.x && x < (wid.x + wid.w) && y >= wid.y && y < (wid.y + wid.h) { true }
    else { false }
}

pub fn widget_resize(wid: Box<_widget>, w: i16, h: i16) -> () {
    // check whether resize is implemented
    match wid.resize {
        Some(_impl) => {
            if wid.w != w || wid.h != h {
                Some(_impl(wid, w, h));
            }
        },
        None => {
            wid.w = w;
            wid.h = h;
        }
    }

    // On a resize we always want to udpate.
    widget_queue_redraw(wid);
}

pub fn widget_move(wid: Box<_widget>, x: i16, y: i16) -> () {
    wid.x = x;
    wid.y = y;
}

pub fn widget_set_type(wid: Box<_widget>, _type: WidgetType) -> () {
    wid._type = _type;
}

pub fn widget_type(wid: Box<_widget>) -> WidgetType {
    wid._type
}

pub fn widget_enabled(wid: Box<_widget>) -> bool {
    wid.enabled
}

pub fn widget_set_enabled(wid: Box<_widget>, enabled: bool) -> () {
    if wid.enabled != enabled {
        wid.enabled = enabled;
        widget_update(wid);
        match wid.parent {
            Some(val) => {
                widget_update(val);
            },
            None => {}
        }
        widget_queue_redraw(wid);
    }
}

pub fn min(a: f64, b: f64) -> f64 {
    if a <= b { a }
    else { b }
}

pub fn widget_draw(wid: Box<_widget>, d: *mut cairo_t) {
    if wid.enabled {
        match wid.draw {
            Some(_impl) => {
                if wid.h < 1 || wid.w < 1 {
                    wid.need_redraw = false;
                    return;
                }
        
                // Store current state.
                unsafe {
                    cairo_save(d);
                }
        
                let margin_left = distance_get_pixel(
                    wid.margin.left,
                    RofiOrientation::ROFI_ORIENTATION_HORIZONTAL,
                );
                let margin_top = distance_get_pixel(wid.margin.top, RofiOrientation::ROFI_ORIENTATION_VERTICAL);
                let margin_right = distance_get_pixel(
                    wid.margin.right,
                    RofiOrientation::ROFI_ORIENTATION_HORIZONTAL,
                );
                let margin_bottom = distance_get_pixel(
                    wid.margin.bottom,
                    RofiOrientation::ROFI_ORIENTATION_VERTICAL,
                );
                let left = distance_get_pixel(
                    wid.border.left,
                    RofiOrientation::ROFI_ORIENTATION_HORIZONTAL,
                );
                let right = distance_get_pixel(
                    wid.border.right,
                    RofiOrientation::ROFI_ORIENTATION_HORIZONTAL,
                );
                let top = distance_get_pixel(wid.border.top, RofiOrientation::ROFI_ORIENTATION_VERTICAL);
                let bottom = distance_get_pixel(
                    wid.border.bottom,
                    RofiOrientation::ROFI_ORIENTATION_VERTICAL,
                );
                let radius_bl = distance_get_pixel(
                    wid.border_radius.left,
                    RofiOrientation::ROFI_ORIENTATION_HORIZONTAL,
                );
                let radius_tr = distance_get_pixel(
                    wid.border_radius.right,
                    RofiOrientation::ROFI_ORIENTATION_HORIZONTAL,
                );
                let radius_tl = distance_get_pixel(
                    wid.border_radius.top,
                    RofiOrientation::ROFI_ORIENTATION_VERTICAL,
                );
                let radius_br = distance_get_pixel(
                    wid.border_radius.bottom,
                    RofiOrientation::ROFI_ORIENTATION_VERTICAL,
                );
        
                let left_2 = left as f64 / 2.0;
                let top_2 = top as f64 / 2.0;
                let right_2 = right as f64 / 2.0;
                let bottom_2 = bottom as f64 / 2.0;
        
                // Calculate the different offsets for the corners.
                let minof_tl = min(left_2, top_2);
                let minof_tr = min(right_2, top_2);
                let minof_br = min(right_2, bottom_2);
                let minof_bl = min(left_2, bottom_2);
        
                // Contain border radius in _widget space
                let vspace = wid.h - (margin_top + margin_bottom) - (top_2 + bottom_2);
                let hspace = wid.w - (margin_left + margin_right) - (left_2 + right_2);
                let vspace_2 = vspace / 2.0;
                let hspace_2 = hspace / 2.0;
        
                if radius_bl + radius_tl > vspace {
                    radius_bl = min(radius_bl, vspace_2);
                    radius_tl = min(radius_tl, vspace_2);
                }
        
                if radius_br + radius_tr > vspace {
                    radius_br = min(radius_br, vspace_2);
                    radius_tr = min(radius_tr, vspace_2);
                }
        
                if radius_tl + radius_tr > hspace {
                    radius_tr = min(radius_tr, hspace_2);
                    radius_tl = min(radius_tl, hspace_2);
                }
        
                if radius_bl + radius_br > hspace {
                    radius_br = min(radius_br, hspace_2);
                    radius_bl = min(radius_bl, hspace_2);
                }
        
                // Background painting.
                // Set new x/y position.
                unsafe {
                    // TODO
                    cairo_translate(d, wid.x.into(), wid.y.into());
                    cairo_set_line_width(d, 0.0);
                }
        
                pub fn calc_addtion_val(x: f64) -> f64 {
                    match x > 2.0 {
                        true => x - 1.0,
                        false => match x == 1.0 {
                            true => 0.5,
                            false => 0.0,
                        },
                    }
                }
        
                pub fn calc_substraction_val(x: f64) -> f64 {
                    match x > 1.0 {
                        true => x - 1.0,
                        false => 0.0,
                    }
                }
        
                unsafe {
                    draw_rounded_rect(
                        d,
                        margin_left + calc_addtion_val(left),
                        margin_top + calc_addtion_val(top),
                        wid.w - margin_right - calc_addtion_val(right),
                        wid.h - margin_bottom - calc_addtion_val(bottom),
                        radius_tl - calc_substraction_val(minof_tl),
                        radius_tr - calc_substraction_val(minof_tr),
                        radius_br - calc_substraction_val(minof_br),
                        radius_bl - calc_substraction_val(minof_bl),
                    );
        
                    cairo_set_source_rgba(d, 1.0, 1.0, 1.0, 1.0);
                    rofi_theme_get_color(wid, "background-color", d);
                    cairo_fill_preserve(d);
                    cairo_clip(d);
        
                    _impl(wid, d);   // TODO - draw is needed here!
                    wid.need_redraw = false;
        
                    cairo_restore(d);
        
                    if left != 0.0 || top != 0.0 || right != 0.0 || bottom != 0.0 {
                        cairo_save(d);
                        cairo_translate(d, wid.x.into(), wid.y.into());
                        cairo_new_path(d);
                        rofi_theme_get_color(wid, "border-color", d);
        
                        let radius_out_tl = if radius_tl > 0.0 {
                            radius_tl + minof_tl
                        } else {
                            0.0
                        };
        
                        let radius_int_tl = if radius_tl > 0.0 {
                            radius_tl - minof_tl
                        } else {
                            0.0
                        };
        
                        let radius_out_tr = if radius_tr > 0.0 {
                            radius_tr + minof_tr
                        } else {
                            0.0
                        };
        
                        let radius_int_tr = if radius_tr > 0.0 {
                            radius_tr - minof_tr
                        } else {
                            0.0
                        };
        
                        let radius_out_br = if radius_br > 0.0 {
                            radius_br + minof_br
                        } else {
                            0.0
                        };
        
                        let radius_int_br = if radius_br > 0.0 {
                            radius_br - minof_br
                        } else {
                            0.0
                        };
        
                        let radius_out_bl = if radius_bl > 0.0 {
                            radius_bl + minof_bl
                        } else {
                            0.0
                        };
        
                        let radius_int_bl = if radius_bl > 0.0 {
                            radius_bl - minof_bl
                        } else {
                            0.0
                        };
        
                        draw_rounded_rect(
                            d,
                            margin_left,
                            margin_top,
                            wid.w.into() - margin_right,
                            wid.h.into() - margin_top,
                            radius_out_tl,
                            radius_out_tr,
                            radius_out_br,
                            radius_out_bl,
                        );

                        cairo_new_sub_path(d);
        
                        draw_rounded_rect(
                            d,
                            margin_left + left,
                            margin_top + top,
                            wid.w - margin_right - right,
                            wid.h - margin_bottom - bottom,
                            radius_int_tl,
                            radius_int_tr,
                            radius_int_br,
                            radius_int_bl,
                        );
        
                        cairo_set_fill_rule(d, FILL_RULE_EVEN_ODD);
                        cairo_fill(d);
                        cairo_restore(d);
                    }
                } // end unsafe
            } // end Some
            None => {}
        }
    } // end if wid.enabled
} // end _widget_draw

//pub fn widget_free(wid: Box<_widget>) -> ()    // not needed in RUst

pub fn widget_get_height(wid: Box<_widget>) -> i16 {
    match wid.get_height {
        Some(_impl) => {
            Some(_impl(wid)).unwrap()
        },
        None => {
            wid.h
        }
    }
}

pub fn widget_get_width(wid: Box<_widget>) -> i16 {
    match wid.get_width {
        Some(_impl) => {
            Some(_impl(wid)).unwrap()
        },
        None => {
            wid.w
        }
    }
}

pub fn widget_get_x_pos(wid: Box<_widget>) -> i16 {
    wid.x
}

pub fn widget_get_y_pos(wid: Box<_widget>) -> i16 {
    wid.y
}

pub fn widget_xy_to_relative(wid: Box<_widget>, x: i16, y: i16) -> () {
    x -= wid.x;
    y -= wid.y;
    match wid.parent {
        Some(val) => _widget_xy_to_relative(val, x, y),
        None => {}
    }
}

pub fn widget_update(wid: Box<_widget>) -> () {
    match wid.update {
        Some(_impl) => {
            Some(_impl(wid));
        },
        None => {}
    }
}

pub fn top_parent(wid: Box<_widget>) -> Box<_widget> {
    match wid.parent {
        None => wid,
        Some(par) => {
            wid.need_redraw = true;
            top_parent(par)
        }
    }
}

pub fn widget_queue_redraw(wid: Box<_widget>) -> () {
    let top_parent = top_parent(wid);
    top_parent.need_redraw = true;
}

pub fn widget_need_redraw(wid: Box<_widget>) -> bool {
    if !wid.enabled { false }
    else { wid.need_redraw }
}

pub fn widget_find_mouse_target(wid: Box<_widget>, _type: WidgetType, x: i16, y: i16) -> Option<Box<_widget>> {
    match wid.find_mouse_target {
        Some(_impl) => {
            _impl(wid, _type, x, y)
        },
        None => { None }
    }
}

// TODO verify callback style against idiomatic Rust conventions (most probably "closures")
pub fn widget_trigger_action(wid: Box<_widget>, action: u16, x: i16, y: i16) -> WidgetTriggerActionResult {
    match wid.trigger_action {
        Some(_impl) => {
            Some(_impl(wid, action, x, y, wid.trigger_action_cb_data)).unwrap()
        },
        None => { WidgetTriggerActionResult::WIDGET_TRIGGER_ACTION_RESULT_IGNORED }
    }
}

// TODO verify logic & functionality
pub fn widget_set_trigger_action_handler(
    wid: Box<_widget>,
    cb: WidgetTriggerActionCB,
    cb_data: Option<Box<dyn Any>>,
) -> () {
        wid.trigger_action = cb;
        wid.trigger_action_cb_data = cb_data;
}

pub fn widget_motion_notify(wid: Box<_widget>, x: i16, y: i16) -> bool {
    wid.motion_notify(wid, x, y)
}

pub fn widget_padding_get_left(wid: Box<_widget>) -> i16 {
    // TODO &wid was const
    let distance = distance_get_pixel(wid.padding.left, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    distance += distance_get_pixel(wid.border.left, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    distance += distance_get_pixel(wid.margin.left, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    distance
}

pub fn widget_padding_get_right(wid: Box<_widget>) -> i16 {
    // TODO &wid was const
    let distance = distance_get_pixel(wid.padding.right, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    distance += distance_get_pixel(wid.border.right, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    distance += distance_get_pixel(wid.margin.right, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    distance
}

pub fn widget_padding_get_top(wid: Box<_widget>) -> i16 {
    // TODO &wid was const
    let distance = distance_get_pixel(wid.padding.top, RofiOrientation::ROFI_ORIENTATION_VERTICAL);
    distance += distance_get_pixel(wid.border.top, RofiOrientation::ROFI_ORIENTATION_VERTICAL);
    distance += distance_get_pixel(wid.margin.top, RofiOrientation::ROFI_ORIENTATION_VERTICAL);
    distance
}

pub fn widget_padding_get_bottom(wid: Box<_widget>) -> i16 {
    // TODO &wid was const
    let distance = distance_get_pixel(
        wid.padding.bottom,
        RofiOrientation::ROFI_ORIENTATION_VERTICAL,
    );
    distance += distance_get_pixel(
        wid.border.bottom,
        RofiOrientation::ROFI_ORIENTATION_VERTICAL,
    );
    distance += distance_get_pixel(
        wid.margin.bottom,
        RofiOrientation::ROFI_ORIENTATION_VERTICAL,
    );
    distance
}

pub fn widget_padding_get_remaining_width(wid: Box<_widget>) -> i16 {
    // TODO &wid was const
    let width: i16 = wid.w;
    width -= _widget_padding_get_left(wid) as i16;
    width -= _widget_padding_get_right(wid) as i16;
    width
}

pub fn widget_padding_get_remaining_height(wid: Box<_widget>) -> i16 {
    let height: i16 = wid.h;
    height -= _widget_padding_get_top(wid) as i16;
    height -= _widget_padding_get_bottom(wid) as i16;
    height
}

pub fn widget_padding_get_padding_height(wid: Box<_widget>) -> i16 {
    let height: i16 = 0;
    height += _widget_padding_get_top(wid) as i16;
    height += _widget_padding_get_bottom(wid) as i16;
    height
}

pub fn widget_padding_get_padding_width(wid: Box<_widget>) -> i16 {
    let width: i16 = 0;
    width += _widget_padding_get_left(wid) as i16;
    width += _widget_padding_get_right(wid) as i16;
    width
}

pub fn widget_get_desired_height(wid: Box<_widget>) -> i16 {
    match wid.get_desired_height {
        Some(_impl) => {
            _impl(wid)
        },
        None => { wid.h }
    }
}

pub fn widget_get_desired_width(wid: Box<_widget>) -> i16 {
    match wid.get_desired_width {
        Some(_impl) => {
            _impl(wid)
        },
        None => { wid.w }
    }
}

pub fn widget_get_absolute_xpos(wid: Box<_widget>) -> i16 {
    let retv = wid.x;
    match wid.parent {
        Some(par) => {
            retv += _widget_get_absolute_xpos(par);
        },
        None => {}
    }
    retv
}

pub fn widget_get_absolute_ypos(wid: Box<_widget>) -> i16 {
    let retv = _widget_get_y_pos(wid);
    match wid.parent {
        Some(par) => {
            retv += _widget_get_absolute_ypos(par);
        },
        None => {}
    }
    retv
}

//####################################################################################################################

/**
 * @param _widget Handle to _widget
 *
 * Disable the wid.
 */
pub fn widget_disable(wid: Box<_widget>) -> () {
    _widget_set_enabled(wid, false);
}

/**
 * @param _widget Handle to _widget
 *
 * Enable the wid.
 */
pub fn widget_enable(wid: Box<_widget>) -> () {
    _widget_set_enabled(wid, true);
}
