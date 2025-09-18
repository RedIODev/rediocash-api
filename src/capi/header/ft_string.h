#ifndef FT_STRING_H
#define FT_STRING_H

#include "ft_c_types.h"

typedef void (*StringDealloc)(const c8 *);

// Immutable owned String type with variable allocator and fixed length.
// Takes ownership of the passed buffer.
// When a String instance is created with invalid arguments the ownership of the buffer is returned to the caller.
// Rather the String instance is valid can be tested with the isValidString(String*) function.
// All operations on invalid Strings are nops.
typedef struct String_impl String;

// Creates a new String instance from a buffer (taking ownership), the size of the buffer and a function to deallocate the buffer.
String createString(const c8 *, usize, StringDealloc);

// Destroys the String including it's content. The instance is no longer valid after a call to this function.
// Calling destroy on invalid Strings has no effects and is considered a nop (just like any other function).
void destroyString(String *);

// True if the String is valid and contains owned data.
bool isValidString(const String *);

// Gets a character at a given index from a given String.
c8 getCharString(const String *, usize);

usize getLengthString(const String *);

// Gets a view from a given String, start index (inclusive) and end index (exclusive). 
// The view is NOT null terminated.
const c8 *getViewString(const String *, usize, usize);

#endif