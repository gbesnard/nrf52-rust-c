#ifndef MAIN_H
#define MAIN_H

#include <stdint.h>

#define SUCCESS_CODE 42
#define CONTINUE_CODE 0

typedef enum {
    STATE_INIT = 0,
    STATE_STARTED,
    STATE_STOPPED
} state_t;

typedef struct {
    uint32_t n;
    state_t state;
} foo_struct_t;

void rust_function_cb(void);

#endif /* MAIN_H */
