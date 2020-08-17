
/***
 *  box
***/

extern crate glib;

use crate::bindings::{
    RofiOrientation, RofiDistance, GList, _cairo,
    RofiOrientation_ROFI_ORIENTATION_HORIZONTAL, RofiOrientation_ROFI_ORIENTATION_VERTICAL,
    rofi_theme_get_distance, widget_padding_get_padding_width,
    rofi_theme_get_boolean, g_list_append, widget_update, widget_draw,
    WidgetType_WIDGET_TYPE_UNKNOWN,
    widget
};
use crate::widget_internal::_widget;
use crate::widget_::{
    WidgetType, widget_init
};

use crate::pub_functions::to_const_i8;

use gtypes::primitive::{ gint, gboolean };
use std::os::raw::*;

pub const DEFAULT_SPACING:cty::c_int = 2;

#[repr(C)]
pub struct box_ {
    widget: *mut _widget,
    type_: RofiOrientation,
    max_size: Option<cty::c_int>,
    spacing: Option<RofiDistance>,
    //children: Option<*mut GList>
    children: Vec<*mut _widget>
}

#[no_mangle]
fn box_draw(wid: *mut _widget, draw: *mut _cairo) {
    let b = box_ {
        widget: wid,
        type_: RofiOrientation_ROFI_ORIENTATION_HORIZONTAL,
        max_size: None,
        spacing: None,
        children: Vec::new() // TODO - how shall there be elements for following for loop?
    };

    for child in b.children {
        unsafe{ widget_draw(child as *mut widget, draw); }
    }
}

#[no_mangle]
fn box_update(widget: *mut _widget) {

}

#[no_mangle]
extern "C" {

    // fn box_resize(widget: *mut _widget, w: i16, h: i16) -> ();
    // fn box_find_mouse_target(wid: *mut _widget, type_: WidgetType, x: gint, y: gint) -> *mut _widget;
    // fn box_get_desired_height(wid: *mut _widget) -> cty::c_int;
    // fn box_get_desired_width(wid: *mut _widget) -> cty::c_int;
    // fn box_set_state(wid: *mut _widget, state: *mut cty::c_char) -> ();
}

#[no_mangle]
pub extern "C" fn box_create(parent: *mut _widget, name: *mut cty::c_char, type_: RofiOrientation) -> box_ {
    let mut v: Vec<*mut _widget> = Vec::new();
    let b = box_ {
        widget: parent.clone(),
        type_: RofiOrientation_ROFI_ORIENTATION_HORIZONTAL,
        max_size: None,
        spacing: None,
        children: v
    };

    unsafe {
        widget_init(b.widget, Some(parent), WidgetType::WIDGET_TYPE_UNKNOWN, name);
        (*b.widget).draw                 = Some(box_draw);
        // (*b.widget).resize               = Some(box_resize);
        (*b.widget).update               = Some(box_update);
        // (*b.widget).find_mouse_target    = Some(box_find_mouse_target);
        // (*b.widget).get_desired_height   = Some(box_get_desired_height);
        // (*b.widget).get_desired_width    = Some(box_get_desired_width);
        // (*b.widget).set_state            = Some(box_set_state);

        //let bindings_wid = std::boxed::Box::<_widget>::into_raw(wid) as *const widget;

        let spacing = rofi_theme_get_distance(
            b.widget as *const widget,
            to_const_i8("spacing"),
            DEFAULT_SPACING);

        let v: Vec<*mut _widget> = Vec::new();
        let b = box_ {
            widget: b.widget,
            type_,
            max_size: None,
            spacing: Some(spacing),
            children: v
        };

        b
    }
}

fn max(a: cty::c_short, b: cty::c_short) -> cty::c_short {
    if a > b { a }
    else { b }
}

#[no_mangle]
pub unsafe extern "C" fn box_add(_box_: *mut box_, child: *mut _widget, expand: gboolean) -> () {
    // Make sure box is width/heigh enough.
    if (*_box_).type_ == RofiOrientation_ROFI_ORIENTATION_VERTICAL {
        let mut width = (*(*_box_).widget).w;
        width = max(
            width,
            (*child).w + widget_padding_get_padding_width((*_box_).widget as *const widget) as i16
        );
        (*(*_box_).widget).w = width;
    }
    else {
        let mut height = (*(*_box_).widget).h;
        height = max(
            height,
            (*child).h + widget_padding_get_padding_width((*_box_).widget as *const widget) as i16
        );
        (*(*_box_).widget).h = height;
    }
    (*child).expand = rofi_theme_get_boolean(child as *const widget, to_const_i8("expand"), expand) != 0;
    //assert_eq!(child.parent == _box.widget_ as *const widget);    // TODO - implement
    (*_box_).children.push(child);
    widget_update((*_box_).widget as *mut widget);
}
