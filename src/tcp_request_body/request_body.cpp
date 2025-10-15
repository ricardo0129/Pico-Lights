#include "tcp_request_body/request_body.h"
#include <stdint.h>

void request_body::deserialize_pixel_update(uint8_t *data,
                                            struct PixelUpdate *body) {
  /*
   * Read  4 bytes for the position and 4 bytes for the color
   * from the given start of the data buffer.
   */
  body->position = 0;
  for (int i = 0; i < 4; i++) {
    body->position |= ((uint32_t)data[i]) << (i * 8);
  }
  body->color = 0;
  for (int i = 0; i < 4; i++) {
    body->color |= ((uint32_t)data[i + 4]) << (i * 8);
  }
}

void request_body::deserialize_request_body(uint8_t *data,
                                            struct RequestBody *body) {
  body->body_size = 0;
  for (int i = 0; i < 4; i++) {
    body->body_size |= ((uint32_t)data[i]) << (i * 8);
  }
  // Point to the body data after the size
  body->body_data = data + 4;
}
