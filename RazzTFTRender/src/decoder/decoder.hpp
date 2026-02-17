#pragma once
#include <stdint.h>
#include "CRC16.h"

#include "../render/renderer.hpp"
#define MAGIC_BYTES 0xBABE
#define BUFFER_SIZE 512 
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

typedef uint16_t RGB;

class Decoder {
public:
    Decoder() : state(DecodeState::FIND_SYNC), width(0), 
    height(0), img_size(0), bytes_read(0), crc(XMODEM_INIT),
    buf_head(0), buf_tail(0), size(0), flush(false), corrupted(false)
    {}


    void feed(uint8_t byte);
    bool get_RGB(RGB&);
    inline DecodeState get_state() { return state; }
    inline bool is_full() { return size == BUFFER_SIZE; }
    inline size_t get_size() { return size; }
    inline bool is_flush() { return flush; }
    inline void clear_corruption() { corrupted = false; }
    inline bool is_corrupted() { return corrupted; }

private: 
    // Handle feed 
    void handle_sync(uint8_t byte);
    void handle_header(uint8_t byte);
    void handle_payload(uint8_t byte);
    void handle_checksum_payload(uint8_t byte);
    bool checksum_check(uint8_t byte);

    // Hard reset
    void reset();

    // Checksum
    CRC16 crc;

    // Data
    uint8_t buffer[BUFFER_SIZE];
    size_t size;
    size_t buf_head;
    size_t buf_tail;

    uint16_t width;
    uint16_t height;
    size_t img_size;
    uint16_t checksum;

    DecodeState state;
    size_t bytes_read;

    bool corrupted; 
    bool flush;
};
