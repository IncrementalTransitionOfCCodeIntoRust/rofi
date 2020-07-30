/***
 *  rofi_types
***/

extern crate glib;
extern crate gtypes;

use gtypes::primitive::gboolean;
use glib_sys::GRegex;
use glib_sys::gpointer;

pub use crate::widget_internal::widget;

/**
 * Type of property
 */
enum PropertyType {
    /** Integer */
    P_INTEGER,
    /** Double */
    P_DOUBLE,
    /** String */
    P_STRING,
    /** Character */
    P_CHAR,
    /** Boolean */
    P_BOOLEAN,
    /** Color */
    P_COLOR,
    /** RofiPadding */
    P_PADDING,
    /** Link to global setting */
    P_LINK,
    /** Position */
    P_POSITION,
    /** Highlight */
    P_HIGHLIGHT,
    /** List */
    P_LIST,
    /** Orientation */
    P_ORIENTATION,
    /** Inherit */
    P_INHERIT,
    /** Number of types. */
    P_NUM_TYPES
}

/**
 * This array maps PropertyType to a user-readable name.
 * It is important this is kept in sync.
 */
//extern const char * const PropertyTypeName[P_NUM_TYPES];  // TODO

/** Style of text highlight */
enum RofiHighlightStyle {
    /** no highlight */
    ROFI_HL_NONE            = 0,
    /** bold */
    ROFI_HL_BOLD            = 1,
    /** underline */
    ROFI_HL_UNDERLINE       = 2,
    /** italic */
    ROFI_HL_ITALIC          = 4,
    /** color */
    ROFI_HL_COLOR           = 8,
    /** strikethrough */
    ROFI_HL_STRIKETHROUGH   = 16,
    /** small caps */
    ROFI_HL_SMALL_CAPS      = 32
}

/** Style of line */
pub enum RofiLineStyle {
    /** Solid line */
    ROFI_HL_SOLID,
    /** Dashed line */
    ROFI_HL_DASH
}

/**
 * Distance unit type.
 */
pub enum RofiPixelUnit {
    /** PixelWidth in pixels. */
    ROFI_PU_PX,
    /** PixelWidth in millimeters. */
    ROFI_PU_MM,
    /** PixelWidth in EM. */
    ROFI_PU_EM,
    /** PixelWidget in percentage */
    ROFI_PU_PERCENT,
    /** PixelWidth in CH. */
    ROFI_PU_CH,
}

/**
 * Structure representing a distance.
 */
pub enum RofiDistanceModifier {
    ROFI_DISTANCE_MODIFIER_NONE,
    ROFI_DISTANCE_MODIFIER_ADD,
    ROFI_DISTANCE_MODIFIER_SUBTRACT,
    ROFI_DISTANCE_MODIFIER_DIVIDE,
    ROFI_DISTANCE_MODIFIER_MULTIPLY,
    ROFI_DISTANCE_MODIFIER_MODULO,
    ROFI_DISTANCE_MODIFIER_GROUP,
}

pub struct RofiDistanceUnit {
    /** Distance */
    distance: f64,
    /** Unit type of the distance */
    _type: RofiPixelUnit,
    /** Type */
    modtype: RofiDistanceModifier,
    /** Modifier */
    left: Option<Box<RofiDistanceUnit>>,
    /** Modifier */
    right: Option<Box<RofiDistanceUnit>>
}

pub struct RofiDistance {
    /** Base */
    base: RofiDistanceUnit,
    /** Style of the line (optional)*/
    style: RofiLineStyle
}

/**
 * Type of orientation.
 */
pub enum RofiOrientation {
    ROFI_ORIENTATION_VERTICAL,
    ROFI_ORIENTATION_HORIZONTAL
}

/**
 * Represent the color in theme.
 */
struct ThemeColor {
    /** red channel */
    red: f64,
    /** green channel */
    green: f64,
    /** blue channel */
    blue: f64,
    /**  alpha channel */
    alpha: f64
}

/**
 * RofiPadding
 */
pub struct RofiPadding {
    pub top: RofiDistance,
    pub right: RofiDistance,
    pub bottom: RofiDistance,
    pub left: RofiDistance
}

/**
 * Theme highlight.
 */
struct RofiHighlightColorStyle {
    /** style to display */
    style: RofiHighlightStyle,
    /** Color */
    color: ThemeColor
}

/**
 * Enumeration indicating location or gravity of window.
 *
 * \verbatim WL_NORTH_WEST      WL_NORTH      WL_NORTH_EAST \endverbatim
 * \verbatim WL_EAST            WL_CENTER     WL_EAST \endverbatim
 * \verbatim WL_SOUTH_WEST      WL_SOUTH      WL_SOUTH_EAST\endverbatim
 *
 * @ingroup CONFIGURATION
 */
enum WindowLocation {
    /** Center */
    WL_CENTER     = 0,
    /** Top middle */
    WL_NORTH      = 1,
    /** Middle right */
    WL_EAST       = 2,
    /** Bottom middle */
    WL_SOUTH      = 4,
    /** Middle left */
    WL_WEST       = 8,
    /** Left top corner. */
    WL_NORTH_WEST = 1 | 8,
    /** Top right */
    WL_NORTH_EAST = 1 | 2,
    /** Bottom right */
    WL_SOUTH_EAST = 4 | 2,
    /** Bottom left */
    WL_SOUTH_WEST = 4 | 8
}


/**
 * Property structure.
 */
struct Property {
    /** Name of property */
    name: String,
    /** Type of property. */
    _type: PropertyType,
    /** Value */
    value: _PropertyValue
}

struct Link {
    /** Name */
    name: String,
    /** Cached looked up ref */
    _ref: Option<Box<Property>>,
    /** Property default */
    def_value: Option<Box<Property>>
}

struct _PropertyValue {
    /** integer */
    i: i16,
    /** Double */
    f: f64,
    /** String */
    s: String,
    /** Character */
    c: char,
    /** boolean */
    b: gboolean,
    /** Color */
    color: ThemeColor,
    /** RofiPadding */
    padding: RofiPadding,
    /** Reference */
    link: Option<Box<Link>>,
    /** Highlight Style */
    highlight: RofiHighlightColorStyle,
    /** List */
    list: Vec<Box<widget>>
}

/**
 * Structure to hold a range.
 */
struct rofi_range_pair {
    start: i16,
    stop: i16
}

/**
 * Internal structure for matching.
 */
struct rofi_int_matcher_t {
    regex: Option<Box<GRegex>>,
    invert: gboolean
}

/**
 * Structure with data to process by each worker thread.
 * TODO: Make this more generic wrapper.
 */
struct _thread_state {
    callback: fn(t: &_thread_state, data: gpointer) -> ()
}

//extern GThreadPool *tpool;    // TODO
