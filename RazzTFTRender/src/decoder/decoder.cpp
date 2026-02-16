#include "decoder.hpp"

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

/*
 * Precondition: size < BUFFER_SIZE 
 * */
void Decoder::handle_payload(uint8_t byte) {
    buffer[buf_tail] = byte;
    buf_tail = (buf_tail + 1) % BUFFER_SIZE; 
    size++;
    crc.add(byte);

    if (size == BUFFER_SIZE) {
        flush = true;
    }
    if (bytes_read == img_size) {
        // Flush 
        flush = true;
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
        reset();
        return false;
    }
    return true;
}

void Decoder::handle_checksum_payload(uint8_t byte) {
    if (bytes_read == 0) {
        bytes_read++;
        checksum = (byte & BYTE_MASK);
    } else {
        if (!checksum_check(byte)) {
            corrupted = true;
            reset();
        } else {
            flush = true;
        }

        // Ready for the next byte 
        state = DecodeState::FIND_SYNC;
    }
}

/*
 * Precondition: flush == true && not empty
 * */
bool Decoder::get_RGB(RGB &rgb) {
    // If size is not even, theres corruption, reset everything
    if ((size & 1) == 1) {
        reset();
        return false;
    }

    rgb = buffer[buf_head]; 
    buf_head = (buf_head + 1) % BUFFER_SIZE;
    rgb |= (buffer[buf_head] << BITS_IN_BYTE);
    buf_head = (buf_head + 1) % BUFFER_SIZE;
    size -= 2;
    if (size == 0) flush = false;

    return true;
}

void Decoder::reset() {
    buf_head = 0;
    buf_tail = 0;
    size = 0;
    state = DecodeState::FIND_SYNC;
    crc.restart();
}
