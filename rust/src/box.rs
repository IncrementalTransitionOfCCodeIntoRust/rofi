//
mod widget;

use std::cmp;

const DEFAULT_SPACING:i32 = 2;


struct _box {
    widget: &widget,
    _type: RofiOrientation,
    max_size: i32,
    // RofiPadding between elements
    spacing: RofiDistance,
    children: Vec<&widget>
}

fn box_get_desired_width(wid: &widget) -> i32 {
    let b = wid as &_box;
    let spacing = distance_get_pixel(b.spacing, b._type);
    let width = 0;

    // Allow user to override
    let w = rofi_theme_get_distance(wid, "width", 0);

    width = distance_get_pixel(w, ROFI_ORIENTATION_HORIZONTAL);
    if width > 0 {
        return width;
    }

    if b._type == ROFI_ORIENTATION_HORIZONTAL {
        let active_widgets = 0;

        for child in b.children {
            if !child.enabled {
                continue;
            }

            active_widgets += 1;

            if child.expand == TRUE {
                width += widget_get_desired_width(child);
                continue;
            }

            width += widget_get_desired_width(child);
        }

        if active_widgets > 0 {
            width += (active_widgets - 1) * spacing;
        }
    } else {
        for child in b.children {
            if !child.enabled {
                continue;
            }

            width = cmp::max(widget_get_desired_width(child), width);
        }
    }

    width += widget_padding_get_padding_width(wid);

    return width;
}

fn box_get_desired_height(wid: &widget) -> i32 {
    let b = wid as &_box;
    let spacing = distance_get_pixel(b.spacing, b._type);
    let height = 0;

    if b._type == ROFI_ORIENTATION_VERTICAL {
        let active_widgets = 0;

        for child in b.children {
            if !child.enabled {
                continue;
            }

            active_widgets += 1;

            height += widget_get_desired_height(child);
        }

        if active_widgets > 0 {
            height += (active_widgets - 1) * spacing;
        }
    } else {
        for child in b.children {
            if !child.enabled {
                continue;
            }

            height = cmp::max(widget_get_desired_height(child), height);
        }
    }

    height += widget_padding_get_padding_height(wid);

    return height;
}

fn vert_calculate_size(b: &_box) -> () {
    let spacing           = distance_get_pixel(b.spacing, ROFI_ORIENTATION_VERTICAL);
    let expanding_widgets = 0;
    let active_widgets    = 0;
    let rem_width         = widget_padding_get_remaining_width(WIDGET(b));
    let rem_height        = widget_padding_get_remaining_height(WIDGET(b));

    for child in b.children {
        if ( child.enabled && child.expand == FALSE ) {
            widget_resize(child, rem_width, widget_get_desired_height(child));
        }
    }

    b.max_size = 0;

    for child in b.children {
        if !child.enabled {
            continue;
        }

        active_widgets += 1;

        if child.expand == TRUE {
            expanding_widgets += 1;
            continue;
        }

        if child.h > 0 {
            b.max_size += child.h;
        }
    }

    if active_widgets > 0 {
        b.max_size += (active_widgets - 1) * spacing;
    }

    if b.max_size > rem_height {
        b.max_size = rem_height;
        g_debug ("Widgets to large (height) for box: %d %d", b.max_size, b.widget.h );
        return;
    }

    if active_widgets > 0 {
        let top = widget_padding_get_top(WIDGET(b));
        let rem = rem_height - b.max_size;
        let index = 0;
        for child in b.children {
            if !child.enabled {
                continue;
            }

            if child.expand == TRUE {
                // Re-calculate to avoid round issues leaving one pixel left.
                let expanding_widgets_size = rem / (expanding_widgets - index);
                widget_move(child, widget_padding_get_left(WIDGET(b)), top);
                top += expanding_widgets_size;
                widget_resize(child, rem_width, expanding_widgets_size);
                top += spacing;
                rem -= expanding_widgets_size;
                index += 1;
            }
            else {
                widget_move(child, widget_padding_get_left(WIDGET(b)), top);
                top += widget_get_height(child);
                top += spacing;
            }
        }
    }

    b.max_size += widget_padding_get_padding_height(WIDGET(b));
}

fn hori_calculate_size(b: &_box) -> () {
    let spacing           = distance_get_pixel(b.spacing, ROFI_ORIENTATION_HORIZONTAL);
    let expanding_widgets = 0;
    let active_widgets    = 0;
    let rem_width         = widget_padding_get_remaining_width(WIDGET(b));
    let rem_height        = widget_padding_get_remaining_height(WIDGET(b));

    for child in b.children {
        if !child.enabled && child.expand == FALSE {
            widget_resize(
                child,
                widget_get_desired_width(child), //child.w,
                rem_height
            );
        }
    }

    b.max_size = 0;

    for child in b.children {
        if !child.enabled {
            continue;
        }

        active_widgets += 1;

        if child.expand == TRUE {
            expanding_widgets += 1;
            continue;
        }
        // Size used by fixed width widgets.
        if child.h > 0 {
            b.max_size += child.w;
        }
    }

    b.max_size += cmp::max(0, (active_widgets - 1) * spacing);

    if b.max_size > rem_width {
        b.max_size = rem_width;
        g_debug("Widgets to large (width) for box: %d %d", b.max_size, b.widget.w);
        //return;
    }

    if active_widgets > 0 {
        let    left  = widget_padding_get_left(WIDGET(b));
        let    rem   = rem_width - b.max_size;
        let    index = 0;

        if rem < 0 {
            rem = 0;
        }

        for child in b.children {
            if !child.enabled {
                continue;
            }

            if child.expand == TRUE {
                // Re-calculate to avoid round issues leaving one pixel left.
                let expanding_widgets_size = rem / (expanding_widgets - index);
                widget_move(child, left, widget_padding_get_top(WIDGET(b)));
                left += expanding_widgets_size;
                widget_resize (child, expanding_widgets_size, rem_height);
                left += spacing;
                rem  -= expanding_widgets_size;
                index += 1;
            }
            else {
                widget_move(child, left, widget_padding_get_top(WIDGET(b)));
                left += widget_get_width( child);
                left += spacing;
            }
        }
    }

    b.max_size += widget_padding_get_padding_width(WIDGET(b));
}

fn box_draw(wid: &widget, draw: &cairo_t) -> () {
    let b = wid as &_box;

    for child in b.children {
        widget_draw(child, draw);
    }
}

fn box_free(wid : &widget) {
    let b = wid as &_box;

    for child in b.children {
        widget_free(child);
    }

    g_list_free(b.children);
    g_free(b);
}

fn box_add (b: &_box, child: &widget, expand: gboolean) -> ()
{
    if b.is_none() {
        return;
    }

    // Make sure box is width/heigh enough.
    if b._type == ROFI_ORIENTATION_VERTICAL {
        let width = b.widget.w;
        width  = cmp::max(width, child.w + widget_padding_get_padding_width(WIDGET(b)));
        b.widget.w = width;
    }
    else {
        let height = b.widget.h;
        height = cmp::max(height, child.h + widget_padding_get_padding_height(WIDGET(b)));
        b.widget.h = height;
    }

    child.expand = rofi_theme_get_boolean(child, "expand", expand);
    g_assert(child.parent == WIDGET(b));
    b.children = b.children.push(child);
    widget_update(WIDGET(b));
}

fn box_resize(widget: &widget, w: i16, h: i16) -> () {
    let b = wid as &_box;

    if b.widget.w != w || b.widget.h != h {
        b.widget.w = w;
        b.widget.h = h;
        widget_update(widget);
    }
}

fn box_find_mouse_target(wid: &widget, _type: WidgetType, x: gint, y: gint) -> &widget {
    let b = wid as &_box;

    for child in b.children {
        if !child.enabled {
            continue;
        }

        if (widget_intersect(child, x, y)) {
            let   rx      = x - child.x;
            let   ry      = y - child.y;
            let target = widget_find_mouse_target(child, _type, rx, ry);
            if target.is_some() {
                return target;
            }
        }
    }

    return None;
}

fn box_set_state(wid: &widget, state: String) -> () {
    let b = wid as &_box;

    for child in b.children {
        widget_set_state(child, state);
    }
}

fn box_create(parent: &widget, name: String, _type: RofiOrientation) -> &_box {
    let b = widget as &_box;

    // Initialize widget.
    widget_init(WIDGET(b), parent, WIDGET_TYPE_UNKNOWN, name);
    b._type                     = _type;
    b.widget.draw               = box_draw;
    b.widget.free               = box_free;
    b.widget.resize             = box_resize;
    b.widget.update             = box_update;
    b.widget.find_mouse_target  = box_find_mouse_target;
    b.widget.get_desired_height = box_get_desired_height;
    b.widget.get_desired_width  = box_get_desired_width;
    b.widget.set_state          = box_set_state;

    b._type = rofi_theme_get_orientation(WIDGET(b), "orientation", b._type);

    b.spacing = rofi_theme_get_distance(WIDGET(b), "spacing", DEFAULT_SPACING);

    return b;
}

fn box_update(wid: &widget) -> () {
    let b = wid as &_box;

    match(b_type) {
        ROFI_ORIENTATION_VERTICAL => vert_calculate_size(b),
        ROFI_ORIENTATION_HORIZONTAL => continue,
        _ => hori_calculate_size(b)
    }

    if wid.parent.is_some() {
        widget_update(wid.parent);
    }
}
