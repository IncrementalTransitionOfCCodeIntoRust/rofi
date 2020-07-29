//
struct widget
{
    /** The type of the widget */
    _type: WidgetType,
    /** X position relative to parent */
    x: i16,
    /** Y position relative to parent */
    y: i16,
    /** Width of the widget */
    w: i16,
    /** Height of the widget */
    h: i16,
    /** RofiPadding */
    def_margin: RofiPadding,
    def_padding: RofiPadding,
    def_border: RofiPadding,
    def_border_radius: RofiPadding,
    margin: RofiPadding,
    padding: RofiPadding,
    border: RofiPadding,
    border_radius: RofiPadding,

    /** enabled or not */
    enabled: gboolean,
    /** Expand the widget when packed */
    expand: gboolean,
    /** Place widget at end of parent */
    end: gboolean,
    /** Parent widget */
    parent: &_widget,
    /** Internal */
    need_redraw: gboolean,
    /** get width of widget implementation function */
    get_width: fn(&_widget) -> i32,
    /** get height of widget implementation function */
    get_height: fn(&_widget) -> i32,
    /** draw widget implementation function */
    draw: fn(widget: &widget, draw: &cairo_t) -> (),
    /** resize widget implementation function */
    resize: fn(&_widget, i16, i16) -> (),
    /** update widget implementation function */
    update: fn(&_widget) -> (),

    /** Handle mouse motion, used for dragging */
    motion_notify: fn(&_widget, x: gint, y: gint) -> gboolean,

    get_desired_height: fn(&_widget) -> i32,
    get_desired_width: fn(&_widget) -> i32,

    set_state: fn(&_widget, String) -> (),  // String was const

    /** widget find_mouse_target callback */
    find_mouse_target: widget_find_mouse_target_cb,
    /** widget trigger_action callback */
    trigger_action: widget_trigger_action_cb,
    /** user data for find_mouse_target and trigger_action callback */
    trigger_action_cb_data: libc::c_int,      // TODO verify type (was void*)

    /** Free widget callback */
    free: fn(widget: &_widget) -> (),

    /** Name of widget (used for theming) */
    name: String,
    state: String,   // state was const
}
