
/***
 *  box
***/

extern crate glib;

use crate::bindings::{
    RofiOrientation, RofiDistance, GList,
    RofiOrientation_ROFI_ORIENTATION_HORIZONTAL,
    rofi_theme_get_distance,
    WidgetType_WIDGET_TYPE_UNKNOWN,
    widget
};
use crate::widget_internal::_widget;
use crate::widget_::{
    WidgetType, widget_init
};

use pub_functions::to_const_i8;

use gtypes::primitive::gint;
use cairo_sys::cairo_t;

const DEFAULT_SPACING:cty::c_int = 2;

#[repr(C)]
struct box_ {
    widget: *mut _widget,
    type_: RofiOrientation,
    max_size: Option<cty::c_int>,
    spacing: Option<RofiDistance>,
    children: Option<*mut GList>
}

#[no_mangle]
fn box_draw(wid: *mut _widget, draw: *mut cairo_t) -> () {
    // let b = box_create(Some(wid), CString::new("some_box").unwrap().as_ptr() as *const cty::c_char, None);

    // for child in b.children {
    //     widget_draw(child.widget, draw);
    // }
    println!("huhuhu");
}

#[no_mangle]
extern "C" {
    //fn box_draw(wid: *mut _widget, draw: *mut cairo_t) -> ();
    // fn box_update(wid: *mut _widget) -> ();
    // fn box_resize(widget: *mut _widget, w: i16, h: i16) -> ();
    // fn box_find_mouse_target(wid: *mut _widget, type_: WidgetType, x: gint, y: gint) -> *mut _widget;
    // fn box_get_desired_height(wid: *mut _widget) -> cty::c_int;
    // fn box_get_desired_width(wid: *mut _widget) -> cty::c_int;
    // fn box_set_state(wid: *mut _widget, state: String) -> ();
}

#[no_mangle]
fn box_create(parent: *mut _widget, name: String, type_: RofiOrientation) -> box_ {
    let b = box_ {
        widget: parent.clone(),
        type_: RofiOrientation_ROFI_ORIENTATION_HORIZONTAL,
        max_size: None,
        spacing: None,
        children: None
    };

    unsafe {
        widget_init(b.widget, Some(parent), WidgetType::WIDGET_TYPE_UNKNOWN, name);
        (*b.widget).draw                 = Some(box_draw);
        // (*b.widget).resize               = Some(box_resize);
        // (*b.widget).update               = Some(box_update);
        // (*b.widget).find_mouse_target    = Some(box_find_mouse_target);
        // (*b.widget).get_desired_height   = Some(box_get_desired_height);
        // (*b.widget).get_desired_width    = Some(box_get_desired_width);
        // (*b.widget).set_state            = Some(box_set_state);

        //let bindings_wid = std::boxed::Box::<_widget>::into_raw(wid) as *const widget;

        let spacing = rofi_theme_get_distance(
            b.widget as *const widget,
            to_const_i8("spacing"),
            DEFAULT_SPACING);

        let b = box_ {
            widget: b.widget,
            type_,
            max_size: None,
            spacing: Some(spacing),
            children: None
        };

        b
    }
}
