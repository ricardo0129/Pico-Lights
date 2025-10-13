#include <stdio.h>
#include <stdlib.h>

#include "hardware/clocks.h"
#include "pico/stdlib.h"
#include "pixels/pixels.h"
#include "tcp_client/tcp_client.h"

#include "lwipopts.h"
#include "pico/cyw43_arch.h"

int main() {
  stdio_init_all();
  if (cyw43_arch_init()) {
    printf("failed to initialise\n");
    return 1;
  }
  cyw43_arch_enable_sta_mode();

  printf("Connecting to Wi-Fi...\n");
  printf("SSID: %s\n", WIFI_SSID);
  if (cyw43_arch_wifi_connect_timeout_ms(WIFI_SSID, WIFI_PASSWORD,
                                         CYW43_AUTH_WPA2_AES_PSK, 30000)) {
    printf("failed to connect.\n");
    return 1;
  } else {
    printf("Connected.\n");
  }
  printf("WS2812 Smoke Test, using pin %d\n", WS2812_PIN);
  tcp_client::run_tcp_client_test();

  cyw43_arch_deinit();
}
