#include <rofi-types.h>
#include "widget_internal_.h"

typedef struct box_ box_;

box_* box_create(_widget* parent, const char* name, RofiOrientation type_);

void box_add(box_* _box_, _widget* child, gboolean expand);
