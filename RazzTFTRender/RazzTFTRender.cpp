#include "HardwareSerial.h"
#include "src/decoder/decoder.hpp"
#define SERIAL_BAUD_RATE 115200

Decoder decoder;

void setup() {
    Serial.begin(SERIAL_BAUD_RATE);
    // Magic
    decoder.feed(0xBE);
    decoder.feed(0xBA);

    // Width
    decoder.feed(0x01);
    decoder.feed(0x00);

    // Height
    decoder.feed(0x01);
    decoder.feed(0x00);

    // Checksum
    decoder.feed(0x1A);
    decoder.feed(0x56);
}

void loop() {
    //Serial.println("hello");
    /*while (Serial.available() > 0) {
        decoder.feed(Serial.read());
    }*/
    Serial.println((int)decoder.get_state());
}
