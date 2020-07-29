/***
#include <glib.h>
#include <cairo.h>
#include <xcb/xcb.h>
#include <xcb/xproto.h>
#include "keyb.h"
***/ // TODO Check bindings!

/**
 * Abstract structure holding internal state of a widget.
 * Structure is elaborated in widget-internal.h
 */
//typedef struct _widget widget;    // TODO translate

extern crate glib;
extern crate cairo;
extern crate xcb;

pub use base::*;
pub use xproto::*;
use std::cmp;

/**
 * Type of the widget. It is used to bubble events to the relevant widget.
 */
enum WidgetType
{
    /** Default type */
    WIDGET_TYPE_UNKNOWN,
    /** The listview widget */
    WIDGET_TYPE_LISTVIEW         = SCOPE_MOUSE_LISTVIEW,
    /** An element in the listview */
    WIDGET_TYPE_LISTVIEW_ELEMENT = SCOPE_MOUSE_LISTVIEW_ELEMENT,
    /** The input bar edit box */
    WIDGET_TYPE_EDITBOX          = SCOPE_MOUSE_EDITBOX,
    /** The listview scrollbar */
    WIDGET_TYPE_SCROLLBAR        = SCOPE_MOUSE_SCROLLBAR,
    /** A widget allowing user to swithc between modi */
    WIDGET_TYPE_MODE_SWITCHER    = SCOPE_MOUSE_MODE_SWITCHER,
    /** Text-only textbox */
    WIDGET_TYPE_TEXTBOX_TEXT,
}

/**
 * Whether and how the action was handled
 */
enum WidgetTriggerActionResult
{
    /** The action was ignore and should bubble */
    WIDGET_TRIGGER_ACTION_RESULT_IGNORED,
    /** The action was handled directly */
    WIDGET_TRIGGER_ACTION_RESULT_HANDLED,
    /** The action was handled and should start the grab for motion events */
    WIDGET_TRIGGER_ACTION_RESULT_GRAB_MOTION_BEGIN,
    /** The action was handled and should stop the grab for motion events */
    WIDGET_TRIGGER_ACTION_RESULT_GRAB_MOTION_END,
}

/** Macro to get widget from an implementation (e.g. textbox/scrollbar) */
//#define WIDGET( a )    ( (widget *) ( a ) )   // TODO translate, or rewrite when used

//####################################################################################################################

const WIDGET_DEFAULT_PADDING:i32 = 0;
//#define WIDGET_PADDING_INIT { { WIDGET_DEFAULT_PADDING, ROFI_PU_PX, ROFI_DISTANCE_MODIFIER_NONE, NULL, NULL }, ROFI_HL_SOLID }  // TODO translat

fn draw_rounded_rect(d: &cairo_t, x1: f64, y1: f64, x2: f64, y2: f64, r0: f64, r1: f64, r2: f64, r3: f64) -> () {
    if r0 > 0 {
        cairo_move_to(d, x1, y1+r0);
        cairo_arc(d, x1+r0, y1+r0, r0, -G_PI, -G_PI_2);
    } else {
        cairo_move_to(d, x1, y1);
    }

    if r1 > 0 {
        cairo_line_to(d, x2-r1, y1);
        cairo_arc (d, x2-r1, y1+r1, r1, -G_PI_2, 0.0);
    } else {
        cairo_line_to(d, x2, y1);
    }

    if r2 > 0 {
        cairo_line_to(d, x2, y2-r2);
        cairo_arc(d, x2-r2, y2-r2, r2, 0.0, G_PI_2);
    } else {
        cairo_line_to(d, x2, y2);
    }

    if r3 > 0 {
        cairo_line_to (d, x1+r3, y2);
        cairo_arc(d, x1+r3, y2-r3, r3, G_PI_2, G_PI);
    } else {
        cairo_line_to(d, x1, y2);
    }

    cairo_close_path(d);
}

fn widget_init(wid: &widget, parent: &widget, _type: WidgetType, name: String) -> () {
    wid._type = _type;
    wid.parent = parent;
    wid.name = g_strdup(name);
    wid.def_padding         = RofiPadding { WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT };
    wid.def_border          = RofiPadding { WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT };
    wid.def_border_radius   = RofiPadding { WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT };
    wid.def_margin          = RofiPadding { WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT, WIDGET_PADDING_INIT };

    wid.padding       = rofi_theme_get_padding(wid, "padding", wid.def_padding);
    wid.border        = rofi_theme_get_padding(wid, "border", wid.def_border);
    wid.border_radius = rofi_theme_get_padding(wid, "border-radius", wid.def_border_radius);
    wid.margin        = rofi_theme_get_padding(wid, "margin", wid.def_margin);

    // bled by default
    wid.enabled = rofi_theme_get_boolean(wid, "enabled", 1);
}

fn widget_set_state(widget: &widget, state: String) -> () {
    if widget.is_none() {
        return;
    }

    if widget.state == state {
        // Update border.
        widget.border        = rofi_theme_get_padding(widget, "border", widget.def_border);
        widget.border_radius = rofi_theme_get_padding(widget, "border-radius", widget.def_border_radius);
        if ( widget.set_state.is_none()) {
            widget.set_state(widget, state);
        }

        widget_queue_redraw(widget);
    }
}

fn widget_intersect(widget: &widget, x: i32, y: i32) -> i32 {   // &widget was const
    if widget.is_none() {
        return 0;
    }

    if x >= widget.x && x < (widget.x + widget.w ) && y >= widget.y && y < (widget.y + widget.h) {
        return 1;
    }

    return 0;
}

fn widget_resize(widget: &widget, w: i16, h: i16) -> () {
    if widget.is_none() {
        return 0;
    }

    if widget.resize.is_some() {
        if widget.w != w || widget.h != h {
            widget.resize(widget, w, h);
        }
    } else {
        widget.w = w;
        widget.h = h;
    }

    // On a resize we always want to udpate.
    widget_queue_redraw(widget);
}

fn widget_move(widget: &widget, x: i16, y: i16) -> () {
    if widget.is_some() {
        widget.x = x;
        widget.y = y;
    }
}

fn widget_set_type(widget: &widget, _type: WidgetType) -> ()
{
    if widget.is_some() {
        widget._type = _type;
    }
}

fn widget_type(widget: &widget) -> WidgetType {
    if widget.is_none() {
        return WIDGET_TYPE_UNKNOWN;
    }

    return widget._type;
}

fn widget_enabled(widget: &widget) -> gboolean {
    if widget.is_none() {
        return FALSE;
    }

    return widget.enabled;
}

fn widget_set_enabled(widget: &widget, enabled: gboolean) {
    if widget.is_some() {
        if widget.enabled != enabled {
            widget.enabled = enabled;
            widget_update(widget);
            widget_update(widget.parent);
            widget_queue_redraw(widget);
        }
    }
}

fn widget_draw(widget: &widget, d: &cairo_t) {
    if widget.is_none() {
        return;
    }

    if widget.enabled && widget.draw {
        if widget.h < 1 || widget.w < 1 {
            widget.need_redraw = FALSE;
            return;
        }
    }

    // Store current state.
    cairo_save(d);
    let margin_left   = distance_get_pixel ( widget.margin.left, ROFI_ORIENTATION_HORIZONTAL );
    let margin_top    = distance_get_pixel ( widget.margin.top, ROFI_ORIENTATION_VERTICAL );
    let margin_right  = distance_get_pixel ( widget.margin.right, ROFI_ORIENTATION_HORIZONTAL );
    let margin_bottom = distance_get_pixel ( widget.margin.bottom, ROFI_ORIENTATION_VERTICAL );
    let left          = distance_get_pixel ( widget.border.left, ROFI_ORIENTATION_HORIZONTAL );
    let right         = distance_get_pixel ( widget.border.right, ROFI_ORIENTATION_HORIZONTAL );
    let top           = distance_get_pixel ( widget.border.top, ROFI_ORIENTATION_VERTICAL );
    let bottom        = distance_get_pixel ( widget.border.bottom, ROFI_ORIENTATION_VERTICAL );
    let radius_bl     = distance_get_pixel ( widget.border_radius.left, ROFI_ORIENTATION_HORIZONTAL );
    let radius_tr     = distance_get_pixel ( widget.border_radius.right, ROFI_ORIENTATION_HORIZONTAL );
    let radius_tl     = distance_get_pixel ( widget.border_radius.top, ROFI_ORIENTATION_VERTICAL );
    let radius_br     = distance_get_pixel ( widget.border_radius.bottom, ROFI_ORIENTATION_VERTICAL );


    let left_2 = left as f64 / 2;
    let top_2 = top as f64 / 2;
    let right_2 = right as f64 / 2;
    let bottom_2 = bottom as f64 / 2;

    // Calculate the different offsets for the corners.
    let minof_tl = cmp::min(left_2, top_2);
    let minof_tr = cmp::min(right_2, top_2);
    let minof_br = cmp::min(right_2, bottom_2);
    let minof_bl = cmp::min(left_2, bottom_2);

    // Contain border radius in widget space
    let vspace = widget.h - (margin_top + margin_bottom) - (top_2 + bottom_2);
    let hspace = widget.w - (margin_left + margin_right) - (left_2 + right_2);
    let vspace_2 = vspace / 2;
    let hspace_2 = hspace / 2;

    if radius_bl + radius_tl > vspace {
        radius_bl = cmp::min(radius_bl, vspace_2);
        radius_tl = cmp::min(radius_tl, vspace_2);
    }

    if radius_br + radius_tr > vspace {
        radius_br = cmp::min(radius_br, vspace_2);
        radius_tr = cmp::min(radius_tr, vspace_2);
    }

    if radius_tl + radius_tr > hspace {
        radius_tr = cmp::min(radius_tr, hspace_2);
        radius_tl = cmp::min(radius_tl, hspace_2);
    }

    if radius_bl + radius_br > hspace {
        radius_br = cmp::min(radius_br, hspace_2);
        radius_bl = cmp::min(radius_bl, hspace_2);
    }

    // Background painting.
    // Set new x/y position.
    cairo_translate(d, widget.x, widget.y);
    cairo_set_line_width(d, 0);

    fn calc_addtion_val(x: i32) -> i32 {
        match(x > 2) {
            true => x - 1,
            false => match(x == 1) {
                true => 0.5,
                false => 0
            }
        }
    }

    fn calc_substraction_val(x: f64) -> f64 {
        match(x > 1) {
            true => x - 1,
            false => 0
        }
    }

    draw_rounded_rect(
        d,
        margin_left + calc_addtion_val(left),
        margin_top + calc_addtion_val(top),
        widget.w - margin_right  - calc_addtion_val(right),
        widget.h - margin_bottom - calc_addtion_val(bottom),
        radius_tl - calc_substraction_val(minof_tl),
        radius_tr - calc_substraction_val(minof_tr),
        radius_br - calc_substraction_val(minof_br),
        radius_bl - calc_substraction_val(minof_bl)
    );

    cairo_set_source_rgba(d, 1.0, 1.0, 1.0, 1.0);
    rofi_theme_get_color(widget, "background-color", d);
    cairo_fill_preserve(d);
    cairo_clip(d);

    widget.draw(widget, d);
    widget.need_redraw = FALSE;

    cairo_restore(d);

    if left != 0 || top != 0 || right != 0 || bottom != 0 {
        cairo_save(d);
        cairo_translate(d, widget.x, widget.y);
        cairo_new_path(d);
        rofi_theme_get_color(widget, "border-color", d);

        let radius_out_tl = if radius_tl > 0 {
            radius_tl + minof_tl
        } else {
            0
        };

        let radius_int_tl = if radius_tl > 0 {
            radius_tl - minof_tl
        } else {
            0
        };

        let radius_out_tr = if radius_tr > 0 {
            radius_tr + minof_tr
        } else {
            0
        };

        let radius_int_tr = if radius_tr > 0 {
            radius_tr - minof_tr
        } else {
            0
        };

        let radius_out_br = if radius_br > 0 {
            radius_br + minof_br
        } else {
            0
        };

        let radius_int_br = if radius_br > 0 {
            radius_br - minof_br
        } else {
            0
        };

        let radius_out_bl = if radius_bl > 0 {
            radius_bl + minof_bl
        } else {
            0
        };

        let radius_int_bl = if radius_bl > 0 {
            radius_bl - minof_bl
        } else {
            0
        };

        draw_rounded_rect(
            d,
            margin_left,
            margin_top,
            widget.w - margin_right,
            widget.h - margin_top,
            radius_out_tl,
            radius_out_tr,
            radius_out_br,
            radius_out_bl
        );

        cairo_new_sub_path ( d );

        draw_rounded_rect(
            d,
            margin_left + left,
            margin_top  + top,
            widget.w - margin_right - right,
            widget.h - margin_bottom - bottom,
            radius_int_tl,
            radius_int_tr,
            radius_int_br,
            radius_int_bl
        );

        cairo_set_fill_rule(d, CAIRO_FILL_RULE_EVEN_ODD);
        cairo_fill(d);
        cairo_restore(d);
    }
}

fn widget_free(wid: &widget) -> () {
    if wid.is_none() {
        return;
    }

    if wid.name.is_some() {
        g_free(wid.name);
    }

    if wid.free.is_some() {
        wid.free(wid);
    }
}

fn widget_get_height(widget: &widget) -> i32 {
    if widget.is_none() {
        return 0;
    }

    if widget.get_height.is_none() {
        return widget.h;
    }

    return widget.get_height(widget);
}

fn widget_get_width(widget: &widget) -> i32 {
    if widget.is_none() {
        return 0;
    }

    if widget.get_width.is_none() {
        return widget.w;
    }

    return widget.get_width(widget);
}

fn widget_get_x_pos(widget: &widget) -> i32 {
    if widget.is_none() {
        return 0;
    }

    return widget.x;
}

fn widget_get_y_pos(widget: &widget) -> i32 {
    if widget.is_none() {
        return 0;
    }

    return widget.y;
}

fn widget_xy_to_relative(widget: &widget, x: &gint, y: &gint) -> () {
    x -= widget.x;
    y -= widget.y;
    if widget.parent.is_some() {
        widget_xy_to_relative(widget.parent, x, y);
    }
}

fn widget_update(widget: &widget) {
    if widget.is_none() {
        return;
    }

    // When (desired )size of widget changes.
    if ( widget.update.is_some()) {
        widget.update(widget);
    }
}

fn widget_queue_redraw(widget: &wid) {
    if wid.is_none() {
        return;
    }

    let mut iter = wid.iter();

    // Find toplevel widget
    while iter.parent.is_some() {
        iter.need_redraw = TRUE;
        iter = iter.continueparent;
    }

    iter.need_redraw = TRUE;
}

fn widget_need_redraw(widget: &wid) -> gboolean {
    if wid.is_none() {
        return FALSE;
    }

    if !wid.enabled {
        return FALSE;
    }

    return wid.need_redraw;
}

fn widget_find_mouse_target(widget: &wid, _type: WidgetType, x: gint, y: gint) -> &widget {
    if wid.is_none() {
        return None;
    }

    if wid.find_mouse_target != NULL {
        let target = wid.find_mouse_target(wid, _type, x, y);
        if target.is_some() {
            return target;
        }
    }

    if wid._type == _type {
        return &wid;
    }

    return None;
}

fn widget_trigger_action(widget: &wid, action: guint, x: gint, y: gint) -> WidgetTriggerActionResult {
    if wid.is_none() || wid.trigger_action.is_none() {
        return FALSE;
    }

    return wid.trigger_action(wid, action, x, y, wid.trigger_action_cb_data);
}

fn widget_set_trigger_action_handler(widget: &wid, cb: widget_trigger_action_cb, cb_data: libc::c_int ) -> () { // TODO verify cb_data type (was void*)
    if wid.is_some() {
        wid.trigger_action = cb;
        wid.trigger_action_cb_data = cb_data;
    }
}

fn widget_motion_notify(widget: &wid, x: gint, y: gint) -> gboolean {
    if wid.is_none() || wid.motion_notify.is_none() {
        return FALSE;
    }

    return wid.motion_notify(wid, x, y);
}

fn widget_padding_get_left(widget: &wid) -> i32 // TODO &wid was const
{
    if wid.is_none() {
        return 0;
    }

    let distance = distance_get_pixel(wid.padding.left, ROFI_ORIENTATION_HORIZONTAL);
    distance    += distance_get_pixel(wid.border.left,  ROFI_ORIENTATION_HORIZONTAL);
    distance    += distance_get_pixel(wid.margin.left,  ROFI_ORIENTATION_HORIZONTAL);

    return distance;
}

fn widget_padding_get_right(widget: &wid) -> i32 // TODO &wid was const
{
    if wid.is_none() {
        return 0;
    }

    let distance = distance_get_pixel(wid.padding.right, ROFI_ORIENTATION_HORIZONTAL);
    distance    += distance_get_pixel(wid.border.right,  ROFI_ORIENTATION_HORIZONTAL);
    distance    += distance_get_pixel(wid.margin.right,  ROFI_ORIENTATION_HORIZONTAL);

    return distance;
}

fn widget_padding_get_top(widget: &wid) -> i32 // TODO &wid was const
{
    if wid.is_none() {
        return 0;
    }

    let distance = distance_get_pixel(wid.padding.top, ROFI_ORIENTATION_VERTICAL);
    distance    += distance_get_pixel(wid.border.top,  ROFI_ORIENTATION_VERTICAL);
    distance    += distance_get_pixel(wid.margin.top,  ROFI_ORIENTATION_VERTICAL);

    return distance;
}

fn widget_padding_get_bottom(widget: &wid) -> i32 // TODO &wid was const
{
    if wid.is_none() {
        return 0;
    }

    let distance = distance_get_pixel(wid.padding.bottom, ROFI_ORIENTATION_VERTICAL);
    distance    += distance_get_pixel(wid.border.bottom,  ROFI_ORIENTATION_VERTICAL);
    distance    += distance_get_pixel(wid.margin.bottom,  ROFI_ORIENTATION_VERTICAL);

    return distance;
}

fn widget_padding_get_remaining_width(widget: &wid) -> i32 // TODO &wid was const
{
    let width = wid.w;
    width -= widget_padding_get_left(wid);
    width -= widget_padding_get_right(wid);

    return width;
}

fn widget_padding_get_remaining_height(widget: &wid) -> i32 {
    let height = wid.h;
    height -= widget_padding_get_top(wid);
    height -= widget_padding_get_bottom(wid);

    return height;
}

fn widget_padding_get_padding_height(widget: &wid) -> i32 {
    let height = 0;
    height += widget_padding_get_top(wid);
    height += widget_padding_get_bottom(wid);

    return height;
}

fn widget_padding_get_padding_width(widget: &wid) -> i32 {
    let width = 0;
    width += widget_padding_get_left(wid);
    width += widget_padding_get_right(wid);

    return width;
}

fn widget_get_desired_height(widget: &wid) -> i32 {
    if wid.is_none() {
        return 0;
    }

    if wid.get_desired_height.is_none {
        return wid.h;
    }

    return wid.get_desired_height(wid);
}

fn widget_get_desired_width(widget: &wid)-> i32 {
    if wid.is_none() {
        return 0;
    }

    if wid.get_desired_width.is_none() {
        return wid.w;
    }

    return wid.get_desired_width(wid);
}

fn widget_get_absolute_xpos(widget: &wid)-> i32 {
    if wid.is_none() {
        return 0;
    }

    let retv = wid.x;
    if wid.parent.is_some() {
        retv += widget_get_absolute_xpos(wid.parent);
    }

    return retv;
}

fn widget_get_absolute_ypos(widget: &wid)-> i32 {
    if wid.is_none() {
        return 0;
    }

    let retv = wid.widget_get_y_pos;
    if wid.parent.is_some() {
        retv += widget_get_absolute_ypos(wid.parent);
    }

    return retv;
}

//####################################################################################################################

/**
 * @param widget Handle to widget
 *
 * Disable the widget.
 */
fn widget_disable(widget: &widget) -> () {
    widget_set_enabled ( widget, FALSE );
}

/**
 * @param widget Handle to widget
 *
 * Enable the widget.
 */
fn widget_enable(widget: &widget) -> () {
    widget_set_enabled ( widget, TRUE );
}
