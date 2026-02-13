#pragma once
#include <stdint.h>

struct FrameHeader {
    uint16_t magic;
    uint16_t width;
    uint16_t height;
    uint16_t header_checksum;
};

enum class DecodeState {
    FIND_SYNC, // For magic
    READ_HEADER, 
    READ_PAYLOAD, 
    READ_PAYLOAD_CHECKSUM, 
};
