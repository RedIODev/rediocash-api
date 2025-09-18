
#include "header/ft_string.h"
#include <stddef.h>
#include <string.h>

struct String_impl {
    StringDealloc dealloc_fn;
    const c8 *data;
    usize length;
};

String createString(const c8 *data, usize length, StringDealloc deallocator) {
    if (data == NULL || deallocator == NULL) {
        return (String) {data: NULL, length: 0, dealloc_fn: no_op};
    }

    return (String) {data: data, length: length, dealloc_fn: deallocator};
}

void destroyString(String *str) {
    if (!isValidString(str)) {
        return;
    }

    str->length = 0;
    const c8 *tmp_data = str->data;
    StringDealloc tmp_fn = str->dealloc_fn;
    str->data = NULL;
    str->dealloc_fn = NULL;
    tmp_fn(tmp_data);
}

bool isValidString(const String *str) {
    return str->data != NULL && str->dealloc_fn != NULL;
}

c8 getCharString(const String *str, usize index) {
    if (!isValidString(str)) {
        return NULL;
    }

    if (str->length >= index) {
        return NULL;
    }

    return str->data[index];
}

usize getLengthString(const String *str) {
    return str->length;
}

const c8 *getViewString(const String *str, usize start, usize end) {
    if (!isValidString(str)) {
        return NULL;
    }
    if (start >= end || start >= str->length || end > str->length) {
        return NULL;
    }
    return str->data + start;

}

void no_op(const c8 *ignored) {}