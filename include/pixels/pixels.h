#ifndef PIXELS_H
#define PIXELS_H

#include "hardware/pio.h"
#include "ws2812.pio.h"

#define IS_RGBW false
#define NUM_PIXELS 50

#define WS2812_PIN 2

namespace pixels {

void put_pixel(PIO pio, uint sm, uint32_t pixel_grb);

void initialize_pio(PIO *pio, uint *sm, uint *offset);

} // namespace pixels

#endif // PIXELS_H
