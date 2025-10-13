#ifndef REQUEST_STRUCT_H
#define REQUEST_STRUCT_H
#include <stdint.h>

const int REQUEST_BODY_SIZE = 8;

struct RequestBody {
  uint32_t position;
  uint32_t color;
};

#endif // REQUEST_STRUCT_H
