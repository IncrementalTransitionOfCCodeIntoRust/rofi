/***
 *  theme - partial implementation
***/

pub use crate::rofi_types::RofiOrientation;
pub use crate::rofi_types::RofiDistance;
pub use crate::rofi_types::RofiDistanceUnit;
pub use crate::rofi_types::RofiPixelUnit;


fn get_pixels(unit: Box<RofiDistanceUnit>, ori: RofiOrientation) -> i16 {
    let val = unit.distance;

    if unit._type == RofiPixelUnit::ROFI_PU_EM {
        //val = unit.distance * textbox_get_estimated_char_height ();
    }
}

fn distance_unit_get_pixel2(unit: Box<RofiDistanceUnit>, ori: RofiOrientation) -> i16 {
    match unit.modtype {
        ROFI_DISTANCE_MODIFIER_GROUP => {
            match unit.left {
                Some(val) => { return distance_unit_get_pixel2(val, ori) },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_ADD => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            return distance_unit_get_pixel2(val_left, ori) + distance_unit_get_pixel2(val_right, ori) },
                        None => {}
                    }
                },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_SUBTRACT => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            return distance_unit_get_pixel2(val_left, ori) - distance_unit_get_pixel2(val_right, ori); },
                        None => {}
                    }
                },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_MULTIPLY => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            return distance_unit_get_pixel2(val_left, ori) * distance_unit_get_pixel2(val_right, ori); },
                        None => {}
                    }
                },
                None => {}
            }
        }
        ROFI_DISTANCE_MODIFIER_DIVIDE => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            let a = distance_unit_get_pixel2(val_left, ori);
                            let b = distance_unit_get_pixel2(val_right, ori);
                            if b != 0 { return a / b }
                            else { return a }
                        },
                        None => {}
                    }
                },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_MODULO => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            let a = distance_unit_get_pixel2(val_left, ori);
                            let b = distance_unit_get_pixel2(val_right, ori);
                            if b != 0 { return a % b }
                            else { return 0 }
                        },
                        None => {}
                    }
                },
                None => {}
            }
        },
        _ => { }
    }

    return get_pixels(unit, ori);
}


fn distance_unit_get_pixel(unit: RofiDistanceUnit, ori: RofiOrientation) -> i16 {
    match unit.modtype {
        ROFI_DISTANCE_MODIFIER_GROUP => {
            match unit.left {
                Some(val) => { return distance_unit_get_pixel2(val, ori) },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_ADD => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            return distance_unit_get_pixel2(val_left, ori) + distance_unit_get_pixel2(val_right, ori) },
                        None => {}
                    }
                },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_SUBTRACT => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            return distance_unit_get_pixel2(val_left, ori) - distance_unit_get_pixel2(val_right, ori); },
                        None => {}
                    }
                },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_MULTIPLY => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            return distance_unit_get_pixel2(val_left, ori) * distance_unit_get_pixel2(val_right, ori); },
                        None => {}
                    }
                },
                None => {}
            }
        }
        ROFI_DISTANCE_MODIFIER_DIVIDE => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            let a = distance_unit_get_pixel2(val_left, ori);
                            let b = distance_unit_get_pixel2(val_right, ori);
                            if b != 0 { return a / b }
                            else { return a }
                        },
                        None => {}
                    }
                },
                None => {}
            }
        },
        ROFI_DISTANCE_MODIFIER_MODULO => {
            match unit.left {
                Some(val_left) => {
                    match unit.right {
                        Some(val_right) => {
                            let a = distance_unit_get_pixel2(val_left, ori);
                            let b = distance_unit_get_pixel2(val_right, ori);
                            if b != 0 { return a % b }
                            else { return 0 }
                        },
                        None => {}
                    }
                },
                None => {}
            }
        },
        _ => { }
    }

    return -1;  // TODO think of useful return here!
}

pub fn distance_get_pixel(d: Box<RofiDistance>, ori: RofiOrientation ) -> i16 {
    return distance_unit_get_pixel(d.base, ori);
}
