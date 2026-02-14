#pragma once
#include <stdint.h>
#include "CRC16.h"
#include "frame.hpp"

#define BUFFER_SIZE 512
#define MAGIC_BYTES 0xBABE
#define R_MASK 0x1F
#define G_MASK 0x3F
#define B_MASK 0x1F
#define BITS_IN_BYTE 8
#define BYTE_MASK 0xFF


class Decoder {
public:
    Decoder() : state(DecodeState::FIND_SYNC), width(0), 
    height(0), img_size(0) {}
    void feed(uint8_t byte);


private: 
    void rgb565_to_rgb888(uint16_t rgb565, uint8_t &r, uint8_t &g, uint8_t &b);

    void handle_sync(uint8_t byte);
    void handle_header(uint8_t byte);
    void handle_payload(uint8_t byte);
    void handle_checksum(uint8_t byte);

    FrameHeader header;
    CRC16 crc;
    uint8_t buffer[BUFFER_SIZE];

    uint16_t width;
    uint16_t height;
    size_t img_size;
    uint16_t checksum;

    DecodeState state;
    size_t bytes_read;
};
