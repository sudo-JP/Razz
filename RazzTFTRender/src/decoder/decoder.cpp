#include "decoder.hpp"

void Decoder::rgb565_to_rgb888(uint16_t rgb565, uint8_t &r, uint8_t &g, uint8_t &b) {
    r = ((rgb565 >> 11) & R_MASK) << 3;
    g = ((rgb565 >> 5) & G_MASK) << 2;
    b = (rgb565 & B_MASK) << 3;
}

void Decoder::feed(uint8_t bytes) {

}
