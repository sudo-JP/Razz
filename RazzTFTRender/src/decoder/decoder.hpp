#pragma once
#include <stdint.h>
#include "CRC16.h"
#include <stdint.h>

#define BUFFER_SIZE 512
#define MAGIC_BYTES 0xBABE
#define R_MASK 0x1F
#define G_MASK 0x3F
#define B_MASK 0x1F
#define BITS_IN_BYTE 8
#define BYTE_MASK 0xFF

#define XMODEM_INIT 0x1021, 0x0000, 0x0000, false, false

enum class DecodeState {
    FIND_SYNC, // For magic
    READ_HEADER, 
    READ_PAYLOAD, 
    READ_PAYLOAD_CHECKSUM, 
};

class Decoder {
public:
    Decoder() : state(DecodeState::FIND_SYNC), width(0), 
    height(0), img_size(0), bytes_read(0), crc(XMODEM_INIT),
    buf_idx(0) {}
    void feed(uint8_t byte);

    inline DecodeState get_state() { return state; }

private: 
    void rgb565_to_rgb888(uint16_t rgb565, uint8_t &r, uint8_t &g, uint8_t &b);

    void handle_sync(uint8_t byte);
    void handle_header(uint8_t byte);
    void handle_payload(uint8_t byte);
    void handle_checksum_payload(uint8_t byte);
    bool checksum_check(uint8_t byte);

    CRC16 crc;
    uint8_t buffer[BUFFER_SIZE];
    size_t buf_idx;

    uint16_t width;
    uint16_t height;
    size_t img_size;
    uint16_t checksum;

    DecodeState state;
    size_t bytes_read;
};
