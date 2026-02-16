#include "Arduino.h"
#include "HardwareSerial.h"
#include "src/render/renderer.hpp"
#include "src/decoder/decoder.hpp"
#define SERIAL_BAUD_RATE 115200

Decoder decoder;
Renderer render;
bool flag = false;

void setup() {
    pinMode(3, OUTPUT);
    Serial.begin(SERIAL_BAUD_RATE);
    render.begin();
}

void loop() {
    if (Serial.available() && !decoder.is_full()) {
        decoder.feed(Serial.read());
    }

    if (decoder.is_corrupted()) {
        // Reset
        render.clear();
        decoder.clear_corruption();
    }

    while (decoder.is_flush() && decoder.get_size() > 0) {
        RGB rgb;
        if (!decoder.get_RGB(rgb)) {
            render.clear();
            break;
        }
        render.add_color(rgb);
    }
}
