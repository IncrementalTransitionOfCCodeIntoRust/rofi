/***
 *  box
***/

pub use crate::rofi_types::*;
pub use crate::widget_internal::_widget;
pub use crate::_widget::*;

use std::cmp;
use std::option::Option;
use cairo_sys::cairo_t;
use glib::g_debug;

const DEFAULT_SPACING:i32 = 2;

struct _box {
    widget: Box<_widget>,
    _type: RofiOrientation,
    max_size: i16,
    // RofiPadding between elements
    spacing: Option<RofiDistance>,
    children: Vec<Box<_box>>
}

fn box_get_desired_width(wid: Box<_widget>) -> i16 {
    let b = box_create(Some(wid), "some_box".to_string(), None);

    let spacing = distance_get_pixel(b.spacing, b._type);
    let width = 0;

    // Allow user to override
    let w = rofi_theme_get_distance(wid, "width", 0);

    width = distance_get_pixel(w, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    if width > 0 {
        return width
    }

    if b._type == RofiOrientation::ROFI_ORIENTATION_HORIZONTAL {
        let active_widgets = 0;

        for child in b.children {
            if !child.widget.enabled {
                continue;
            }

            active_widgets += 1;

            if child.widget.expand == true {
                width += widget_get_desired_width(child.widget);
                continue;
            }

            width += widget_get_desired_width(child.widget);
        }

        if active_widgets > 0 {
            width += (active_widgets - 1) * spacing;
        }
    } else {
        for child in b.children {
            if !child.widget.enabled {
                continue;
            }

            width = cmp::max(widget_get_desired_width(child.widget), width);
        }
    }

    width += widget_padding_get_padding_width(wid);

    return width;
}

fn box_get_desired_height(wid: Box<_widget>) -> i16 {
    let b = box_create(Some(wid), "some_box".to_string(), None);
    let spacing = distance_get_pixel(b.spacing, b._type);
    let height = 0;

    if b._type == RofiOrientation::ROFI_ORIENTATION_VERTICAL {
        let active_widgets = 0;

        for child in b.children {
            if !child.widget.enabled {
                continue;
            }

            active_widgets += 1;

            height += widget_get_desired_height(child.widget);
        }

        if active_widgets > 0 {
            height += (active_widgets - 1) * spacing;
        }
    } else {
        for child in b.children {
            if !child.widget.enabled {
                continue;
            }

            height = cmp::max(widget_get_desired_height(child.widget), height);
        }
    }

    height += widget_padding_get_padding_height(wid);

    return height;
}

fn vert_calculate_size(b: Box<_box>) -> () {
    let spacing           = distance_get_pixel(b.spacing, RofiOrientation::ROFI_ORIENTATION_VERTICAL);
    let expanding_widgets = 0;
    let active_widgets    = 0;
    let rem_width         = widget_padding_get_remaining_width(b.widget);
    let rem_height        = widget_padding_get_remaining_height(b.widget);

    for child in b.children {
        if child.widget.enabled && !child.widget.expand {
            widget_resize(child.widget, rem_width, widget_get_desired_height(child.widget));
        }
    }

    b.max_size = 0;

    for child in b.children {
        if !child.widget.enabled {
            continue;
        }

        active_widgets += 1;

        if child.widget.expand {
            expanding_widgets += 1;
            continue;
        }

        if child.widget.h > 0 {
            b.max_size += child.widget.h;
        }
    }

    if active_widgets > 0 {
        b.max_size += (active_widgets - 1) * spacing;
    }

    if b.max_size > rem_height {
        b.max_size = rem_height;
        g_debug!("Widgets too large (height) for box: ", "{} {}", b.max_size, b.widget.h );
        return;
    }

    if active_widgets > 0 {
        let top = widget_padding_get_top(b.widget);
        let rem = rem_height - b.max_size;
        let index = 0;
        for child in b.children {
            if !child.widget.enabled {
                continue;
            }

            if child.widget.expand {
                // Re-calculate to avoid round issues leaving one pixel left.
                let expanding_widgets_size = rem / (expanding_widgets - index);
                widget_move(child.widget, widget_padding_get_left(b.widget), top);
                top += expanding_widgets_size;
                widget_resize(child.widget, rem_width, expanding_widgets_size);
                top += spacing;
                rem -= expanding_widgets_size;
                index += 1;
            }
            else {
                widget_move(child.widget, widget_padding_get_left(b.widget), top);
                top += widget_get_height(child.widget);
                top += spacing;
            }
        }
    }

    b.max_size += widget_padding_get_padding_height(b.widget);
}

fn hori_calculate_size(b: Box<_box>) -> () {
    let spacing           = distance_get_pixel(b.spacing, RofiOrientation::ROFI_ORIENTATION_HORIZONTAL);
    let expanding_widgets = 0;
    let active_widgets    = 0;
    let rem_width         = widget_padding_get_remaining_width(b.widget);
    let rem_height        = widget_padding_get_remaining_height(b.widget);

    for child in b.children {
        if !child.widget.enabled && !child.widget.expand {
            widget_resize(
                child.widget,
                widget_get_desired_width(child.widget), //child.w,
                rem_height
            );
        }
    }

    b.max_size = 0;

    for child in b.children {
        if !child.widget.enabled {
            continue;
        }

        active_widgets += 1;

        if child.widget.expand {
            expanding_widgets += 1;
            continue;
        }
        // Size used by fixed width widgets.
        if child.widget.h > 0 {
            b.max_size += child.widget.w;
        }
    }

    b.max_size += cmp::max(0, (active_widgets - 1) * spacing);

    if b.max_size > rem_width {
        b.max_size = rem_width;
        g_debug!("Widgets to large (width) for box: ", "{} {}", b.max_size, b.widget.w);
        //return;
    }

    if active_widgets > 0 {
        let    left  = widget_padding_get_left(b.widget);
        let    rem   = rem_width - b.max_size;
        let    index = 0;

        if rem < 0 {
            rem = 0;
        }

        for child in b.children {
            if !child.widget.enabled {
                continue;
            }

            if child.widget.expand {
                // Re-calculate to avoid round issues leaving one pixel left.
                let expanding_widgets_size = rem / (expanding_widgets - index);
                widget_move(child.widget, left, widget_padding_get_top(b.widget));
                left += expanding_widgets_size;
                widget_resize (child.widget, expanding_widgets_size, rem_height);
                left += spacing;
                rem  -= expanding_widgets_size;
                index += 1;
            }
            else {
                widget_move(child.widget, left, widget_padding_get_top(b.widget));
                left += widget_get_width( child.widget);
                left += spacing;
            }
        }
    }

    b.max_size += widget_padding_get_padding_width(b.widget);
}

fn box_draw(wid: Box<_widget>, draw: *mut cairo_t) -> () {
    let b = box_create(Some(wid), "some_box".to_string(), None);

    for child in b.children {
        widget_draw(child.widget, draw);
    }
}

fn box_add (b: &_box, child: Box<_box>, expand: bool) -> () {
    // Make sure box is width/heigh enough.
    if b._type == RofiOrientation::ROFI_ORIENTATION_VERTICAL {
        let width = b.widget.w;
        width  = cmp::max(width, child.widget.w + widget_padding_get_padding_width(b.widget));
        b.widget.w = width;
    }
    else {
        let height = b.widget.h;
        height = cmp::max(height, child.widget.h + widget_padding_get_padding_height(b.widget));
        b.widget.h = height;
    }

    child.widget.expand = rofi_theme_get_boolean(child.widget, "expand", expand);
    //assert_eq!(child.widget.parent, b.widget);    TODO
    b.children.push(child);
    widget_update(b.widget);
}

fn box_resize(wid: Box<_widget>, w: i16, h: i16) -> () {
    let b = box_create(Some(wid), "some_box".to_string(), None);

    if b.widget.w != w || b.widget.h != h {
        b.widget.w = w;
        b.widget.h = h;
        widget_update(b.widget);
    }
}

fn box_find_mouse_target(wid: Box<_widget>, _type: WidgetType, x: i16, y: i16) -> Option<Box<_widget>> {
    let b = box_create(Some(wid), "some_box".to_string(), None);

    for child in b.children {
        if !child.widget.enabled {
            continue;
        }

        if widget_intersect(child.widget, x, y) {
            let   rx      = x - child.widget.x;
            let   ry      = y - child.widget.y;
            let target = widget_find_mouse_target(child.widget, _type, rx, ry);
            if target.is_some() { return target; }
        }
    }

    None
}

fn box_set_state(wid: Box<_widget>, state: String) -> () {
    let b = box_create(Some(wid), "some_box".to_string(), None);

    for child in b.children {
        widget_set_state(child.widget, state);
    }
}

fn box_create(parent: Option<Box<_widget>>, name: String, _type: Option<RofiOrientation>) -> Box<_box>{
    let b: Box<_box>;

    // Initialize widget.
    widget_init(b.widget, parent, WidgetType::WIDGET_TYPE_UNKNOWN, name);
    match _type {
        Some(orient) => {
            b._type = orient;
        },
        None => {
            b._type = RofiOrientation::ROFI_ORIENTATION_HORIZONTAL;
        }    // Don't know whether this makes sense here
    }
    b.widget.draw               = Some(box_draw);
    b.widget.resize             = Some(box_resize);
    b.widget.update             = Some(box_update);
    b.widget.find_mouse_target  = Some(box_find_mouse_target);
    b.widget.get_desired_height = Some(box_get_desired_height);
    b.widget.get_desired_width  = Some(box_get_desired_width);
    b.widget.set_state          = Some(box_set_state);

    b._type = rofi_theme_get_orientation(b.widget, "orientation", b._type);

    b.spacing = rofi_theme_get_distance(b.widget, "spacing", DEFAULT_SPACING);

    b
}

fn box_update(wid: Box<_widget>) -> () {
    let b = box_create(Some(wid), "some_box".to_string(), None);

    match b._type {
        RofiOrientation::ROFI_ORIENTATION_VERTICAL => vert_calculate_size(b),
        _ => hori_calculate_size(b)
    }

    match wid.parent {
        Some(par) => widget_update(par),
        None => {}
    }
}
