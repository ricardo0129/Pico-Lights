#ifndef TCP_STRUCT_H
#define TCP_STRUCT_H

#define BUF_SIZE 2048

#include "hardware/pio.h"
#include "lwip/tcp.h"

struct TCP_CLIENT_T {
  struct tcp_pcb *tcp_pcb;
  ip_addr_t remote_addr;
  uint8_t buffer[BUF_SIZE];
  int buffer_len;
  int sent_len;
  bool complete;
  bool connected;
  // LED pixel state
  PIO pio;
  uint sm;
  uint offset;
  uint32_t *pixels;
};

#endif // TCP_STRUCT_H
