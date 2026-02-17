#include "renderer.hpp"
#include "utility/Adafruit_ST7735.h"

void Renderer::begin() {
    screen.begin();
    screen.setAddrWindow(0, 0, WIDTH - 1, HEIGHT - 1);
    clear();
}

void Renderer::clear() {
    screen.fillScreen(ST7735_BLACK);
}

void Renderer::add_color(RGB rgb) {
    screen.pushColor(rgb);
}


