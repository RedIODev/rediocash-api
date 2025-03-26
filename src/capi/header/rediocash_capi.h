#ifndef REDIOCASH_CAPI_H 
#define REDIOCASH_CAPI_H

#include <stdbool.h>

typedef enum {
    COMMAND_INIT,
    COMMAND_STATUS
} CommandType;

typedef union {

} CommandData;

typedef struct {
    CommandType type;
    CommandData data;
} Command;

typedef enum {
    STATUS_OK,
    STATUS_UNINIT,
    STATUS_ERROR
} StatusCode;

typedef struct {
    StatusCode statusCode;
    const char *message;
} Status;

typedef union {
    Status status;
} ResponseData;

typedef struct {
    CommandType type;
    ResponseData data;
} Response;

typedef enum {
    RESPONSE_ERROR_UNINIT,
    RESPONSE_ERROR_UNIMPLEMENTED
} ResponseErrorType;

typedef union {

} ResponseErrorData;

typedef struct {
    ResponseErrorType type;
    ResponseErrorData data;
} ResponseError;

typedef struct {
    bool isError;
    union {
        Response response;
        ResponseError error;
    };
} ResponseResult;

ResponseResult plugin_execute(const void *, const Command *);
const char *plugin_name(const void *);
const char *plugin_version(const void *);
void *plugin_create(void);
void plugin_destroy(void *);

void plugin_free(void *);

#endif