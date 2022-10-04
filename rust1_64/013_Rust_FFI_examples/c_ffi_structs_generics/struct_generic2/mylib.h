#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct MyValue_String MyValue_String;

typedef struct MyValue_u32 MyValue_u32;

MyValue_String *myvalue_create_str(void);

MyValue_u32 *myvalue_create_u32(void);

void myvalue_free_str(MyValue_String *v_ptr);

void myvalue_free_u32(MyValue_u32 *v_ptr);

void myvalue_print_str(MyValue_String *v_ptr);

void myvalue_print_u32(MyValue_u32 *v_ptr);

