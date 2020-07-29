//
extern crate glib;

/**
 * Type of property
 */
enum PropertyType
{
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
enum RofiLineStyle
{
    /** Solid line */
    ROFI_HL_SOLID,
    /** Dashed line */
    ROFI_HL_DASH
}

/**
 * Distance unit type.
 */
enum RofiPixelUnit
{
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
enum RofiDistanceModifier
{
    ROFI_DISTANCE_MODIFIER_NONE,
    ROFI_DISTANCE_MODIFIER_ADD,
    ROFI_DISTANCE_MODIFIER_SUBTRACT,
    ROFI_DISTANCE_MODIFIER_DIVIDE,
    ROFI_DISTANCE_MODIFIER_MULTIPLY,
    ROFI_DISTANCE_MODIFIER_MODULO,
    ROFI_DISTANCE_MODIFIER_GROUP,
}

struct RofiDistanceUnit
{
    /** Distance */
    distance: f64,
    /** Unit type of the distance */
    _type: RofiPixelUnit,
    /** Type */
    modtype: RofiDistanceModifier,

    /** Modifier */
    left: &RofiDistanceUnit,

    /** Modifier */
    right: &RofiDistanceUnit,

}

struct RofiDistance
{
    /** Base */
    base: RofiDistanceUnit,
    /** Style of the line (optional)*/
    style: RofiLineStyle
}

/**
 * Type of orientation.
 */
enum RofiOrientation
{
    ROFI_ORIENTATION_VERTICAL,
    ROFI_ORIENTATION_HORIZONTAL
}

/**
 * Represent the color in theme.
 */
struct ThemeColor
{
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
struct RofiPadding
{
    top: RofiDistance,
    right: RofiDistance,
    bottom: RofiDistance,
    left: RofiDistance
}

/**
 * Theme highlight.
 */
struct RofiHighlightColorStyle
{
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
enum WindowLocation
{
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
    WL_NORTH_WEST = WL_NORTH | WL_WEST,
    /** Top right */
    WL_NORTH_EAST = WL_NORTH | WL_EAST,
    /** Bottom right */
    WL_SOUTH_EAST = WL_SOUTH | WL_EAST,
    /** Bottom left */
    WL_SOUTH_WEST = WL_SOUTH | WL_WEST
}

struct link
{
    /** Name */
    name: String,
    /** Cached looked up ref */
    _ref: &Property,
    /** Property default */
    def_value: &Property
}

union _PropertyValue
{
    /** integer */
    i: i32,
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
    link: &link,
    /** Highlight Style */
    highlight: RofiHighlightColorStyle,
    /** List */
    list: Vec
}

/**
 * Property structure.
 */
struct Property
{
    /** Name of property */
    name: String,
    /** Type of property. */
    _type: PropertyType,
    /** Value */
    value: PropertyValue
}

/**
 * Structure to hold a range.
 */
struct rofi_range_pair
{
    start: i32,
    stop: i32
}

/**
 * Internal structure for matching.
 */
struct rofi_int_matcher_t
{
    regex: &GRegex,
    invert: gboolean
}

/**
 * Structure with data to process by each worker thread.
 * TODO: Make this more generic wrapper.
 */
struct _thread_state
{
    callback: fn(t: &_thread_state, data: gpointer) -> ()
}

//extern GThreadPool *tpool;    // TODO
