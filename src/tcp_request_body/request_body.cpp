#include "tcp_request_body/request_body.h"
#include <stdint.h>

void request_body::deserialize_request_body(uint8_t *data,
                                            struct RequestBody *body) {
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
