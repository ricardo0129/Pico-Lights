#include <stdio.h>
#include <stdlib.h>

#include "hardware/clocks.h"
#include "hardware/pio.h"
#include "pico/stdlib.h"
#include "tcp_client/tcp_client.h"
#include "ws2812.pio.h"

#include "lwipopts.h"
#include "pico/cyw43_arch.h"

#define IS_RGBW false
#define NUM_PIXELS 50

#define WS2812_PIN 2

void put_pixel(PIO pio, uint sm, uint32_t pixel_grb) {
  pio_sm_put_blocking(pio, sm, pixel_grb << 8u);
}

uint32_t urgb_u32(uint8_t r, uint8_t g, uint8_t b) {
  return ((uint32_t)(g) << 8) | ((uint32_t)(r) << 16) | (uint32_t)(b);
}
const uint32_t color_map[] = {
    urgb_u32(0x00, 0xff, 0x00), // spooky green
    urgb_u32(0xf9, 0x81, 0x28), // pumpkin orange
    urgb_u32(0x5d, 0x25, 0x86), // witchy purple
};

void pattern_snakes(PIO pio, uint sm, uint len, uint t) {
  for (uint i = 0; i < len; ++i) {
    uint pos = (t + i) % len;
    if (i >= 16)
      put_pixel(pio, sm, 0);
    else {
      uint block = i / 4;
      put_pixel(pio, sm, color_map[block]);
    }
  }
}
void fill_solid(PIO pio, uint sm, uint len, uint t) {
  uint32_t color = color_map[0];
  for (uint i = 0; i < len; ++i)
    put_pixel(pio, sm, color);
}

void pattern_sparkle(PIO pio, uint sm, uint len, uint t) {
  if (t % 8)
    return;
  for (uint i = 0; i < len; ++i)
    put_pixel(pio, sm, rand() % 16 ? 0 : 0xffffffff);
}

typedef void (*pattern)(PIO pio, uint sm, uint len, uint t);
const struct {
  pattern pat;
  const char *name;
} pattern_table[] = {
    {pattern_snakes, "Snakes!"},
    {fill_solid, "Solid Color"},
};

int main() {
  stdio_init_all();
  if (cyw43_arch_init()) {
    printf("failed to initialise\n");
    return 1;
  }
  cyw43_arch_enable_sta_mode();

  printf("Connecting to Wi-Fi...\n");
  if (cyw43_arch_wifi_connect_timeout_ms(WIFI_SSID, WIFI_PASSWORD,
                                         CYW43_AUTH_WPA2_AES_PSK, 30000)) {
    printf("failed to connect.\n");
    return 1;
  } else {
    printf("Connected.\n");
  }
  tcp_client::run_tcp_client_test();
  cyw43_arch_deinit();
  return 0;
  printf("WS2812 Smoke Test, using pin %d\n", WS2812_PIN);

  // todo get free sm
  PIO pio;
  uint sm;
  uint offset;

  // This will find a free pio and state machine for our program and load it for
  // us We use pio_claim_free_sm_and_add_program_for_gpio_range (for_gpio_range
  // variant) so we will get a PIO instance suitable for addressing gpios >= 32
  // if needed and supported by the hardware
  bool success = pio_claim_free_sm_and_add_program_for_gpio_range(
      &ws2812_program, &pio, &sm, &offset, WS2812_PIN, 1, true);
  hard_assert(success);

  ws2812_program_init(pio, sm, offset, WS2812_PIN, 800000, IS_RGBW);

  int t = 0;
  while (1) {
    int pat = rand() % count_of(pattern_table);
    int dir = 1;
    puts(pattern_table[pat].name);
    for (int i = 0; i < 1000; ++i) {
      pattern_table[1].pat(pio, sm, NUM_PIXELS, t);
      sleep_ms(10);
      t += dir;
    }
  }

  // This will free resources and unload our program
  pio_remove_program_and_unclaim_sm(&ws2812_program, pio, sm, offset);
}
