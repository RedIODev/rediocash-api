#ifndef REDIOCASH_CAPI_H 
#define REDIOCASH_CAPI_H

#include <stdbool.h>

typedef const char *(*ObjToStringFunc)(void*);

typedef void (*ObjFunc)(void*);

typedef struct {
    ObjToStringFunc name;
    ObjToStringFunc version;
    ObjFunc register_events;
    ObjFunc substribe_events;
} PluginVtable;

typedef struct {
    PluginVtable vtable;
    void *object;
} CPlugin;

typedef struct {
    void *inner;
} CEvents;

typedef struct {
    CEvents events;
} CInitData;

typedef struct {
    const void *event;
} CEvent;

typedef struct {
    void *event;
} CEventMut;

//required to be defined by the c plugin and must be able to free all data passed through the api related to events.
void deallocate_event_data(void*);

#endif