#ifndef FT_CAPI_H 
#define FT_CAPI_H

#include "ft_c_types.h"
#include "ft_string.h"
#include <stddef.h>

// The Type used for handling messages.
// The first String is the argument is the arguments of the message to be handled.
// The String is owned by the caller.
// The second argument is used as an output.
// If the handler finishes successfully it should return true and the output parameter contains the result.
// If the handler encounters an error handling the request it should return false and the output parameter is an error message.
typedef bool (*Handler)(String, String *);

bool s(String s) {
    String s = createString("", 0, NULL);
}

#endif