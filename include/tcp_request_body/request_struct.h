#ifndef REQUEST_STRUCT_H
#define REQUEST_STRUCT_H
#include <stdint.h>

const int MAX_BODY_SIZE = 1024;

struct RequestBody {
  uint32_t body_size;
  uint8_t *body_data; // pointer to body data
};

struct PixelUpdate {
  uint32_t position;
  uint32_t color; // RGB format
};

#endif // REQUEST_STRUCT_H
