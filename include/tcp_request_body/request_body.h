#pragma once

#include "lwip/pbuf.h"
#include "lwip/tcp.h"
#include "pico/cyw43_arch.h"
#include "tcp_request_body/request_struct.h"
#include <stdint.h>

namespace request_body {

void deserialize_request_body(uint8_t *data, struct RequestBody *body);

}
