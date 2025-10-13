#include "pixels/pixels.h"
#include <stdint.h>

void pixels::put_pixel(PIO pio, uint sm, uint32_t pixel_grb) {
  pio_sm_put_blocking(pio, sm, pixel_grb << 8u);
}

void pixels::initialize_pio(PIO *pio, uint *sm, uint *offset) {
  // todo get free sm

  // This will find a free pio and state machine for our program and load it for
  // us We use pio_claim_free_sm_and_add_program_for_gpio_range (for_gpio_range
  // variant) so we will get a PIO instance suitable for addressing gpios >= 32
  // if needed and supported by the hardware
  bool success = pio_claim_free_sm_and_add_program_for_gpio_range(
      &ws2812_program, pio, sm, offset, WS2812_PIN, 1, true);
  hard_assert(success);

  ws2812_program_init(*pio, *sm, *offset, WS2812_PIN, 800000, IS_RGBW);
}
