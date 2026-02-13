#pragma once
#include <stdint.h>
#include "CRC16.h"
#include "frame.hpp"

#define BUFFER_SIZE 512
#define MAGIC_BYTES 0xBABE
#define R_MASK 0x1F
#define G_MASK 0x3F
#define B_MASK 0x1F


class Decoder {
public:
    Decoder();
    void feed(uint8_t byte);


private: 
    void rgb565_to_rgb888(uint16_t rgb565, uint8_t &r, uint8_t &g, uint8_t &b);
    FrameHeader header;
    CRC16 crc;
    uint8_t buffer[BUFFER_SIZE];
};
