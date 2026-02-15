#include "decoder.hpp"

void Decoder::rgb565_to_rgb888(uint16_t rgb565, uint8_t &r, uint8_t &g, uint8_t &b) {
    r = ((rgb565 >> 11) & R_MASK) << 3;
    g = ((rgb565 >> 5) & G_MASK) << 2;
    b = (rgb565 & B_MASK) << 3;
}

void Decoder::feed(uint8_t byte) {
    switch (state) {
        case DecodeState::FIND_SYNC: handle_sync(byte); break;
        case DecodeState::READ_HEADER: handle_header(byte); break;
        case DecodeState::READ_PAYLOAD: handle_payload(byte); break;
        case DecodeState::READ_PAYLOAD_CHECKSUM: handle_checksum_payload(byte); break;
    }
}

// Sync magic have 2 bytes, we transition at 1 
// Also dont wanna use increment before check cuz slow (hardware)
void Decoder::handle_sync(uint8_t byte) {
    if ((bytes_read == 0 && byte != (MAGIC_BYTES & 0xFF)) ||
            (bytes_read == 1 && byte != (MAGIC_BYTES >> BITS_IN_BYTE))) {
        bytes_read = 0;
        return;
    } else if (bytes_read == 1) {
        crc.add(byte);
        state = DecodeState::READ_HEADER;
        bytes_read = 0;
        return;
    }
    crc.add(byte);
    bytes_read++;
    return;
}

// Exclude magic, we have 
// 2 bytes for width -> up to 1
// 2 bytes for height -> up to 3
// 2 bytes checksum -> up to 5
// Logic
void Decoder::handle_header(uint8_t byte) {
    // WIDTH
    if (bytes_read == 0) {
        width = (byte & BYTE_MASK);
    } else if (bytes_read == 1) {
        width |= (byte << BITS_IN_BYTE);
    } 

    // HEIGHT
    else if (bytes_read == 2) {
        height = (byte & BYTE_MASK);
    } else if (bytes_read == 3) {
        height |= (byte << BITS_IN_BYTE);
    } 

    // CHECKSUM
    else if (bytes_read == 4) {
        checksum = (byte & BYTE_MASK);
    } else if (bytes_read == 5) {
        if (!checksum_check(byte)) return;

        // Change state 
        state = DecodeState::READ_PAYLOAD;
        img_size = (width * height * 2) - 1; 
        crc.restart();
        return;
    }
    
    // CHECKSUM ACCUMULATION 
    if (bytes_read < 4) {
        crc.add(byte);
    }
    bytes_read++;
    return;
}

void Decoder::handle_payload(uint8_t byte) {
    if (buf_idx == BUFFER_SIZE) {
        // Flush

        buf_idx = 0;
    } 

    buffer[buf_idx++] = byte;
    crc.add(byte);

    if (bytes_read == img_size) {
        // Flush 
        bytes_read = 0;
        state = DecodeState::READ_PAYLOAD_CHECKSUM;
        return;
    }
    bytes_read++;
    return;
}

bool Decoder::checksum_check(uint8_t byte) {
    checksum |= (byte << BITS_IN_BYTE);
    bytes_read = 0;
    // Check checksum
    if (checksum != crc.calc()) {
        crc.restart();
        state = DecodeState::FIND_SYNC;
        return false;
    }
    return true;
}

void Decoder::handle_checksum_payload(uint8_t byte) {
    if (bytes_read == 0) {
        bytes_read++;
        checksum = (byte & BYTE_MASK);
    } else {
        // NOTE: if this fail, return a signal to display to not render
        checksum_check(byte);

        // Ready for the next byte 
        state = DecodeState::FIND_SYNC;
    }
}
