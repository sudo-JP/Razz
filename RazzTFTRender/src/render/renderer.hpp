#pragma once

#include <SPI.h>
#include <TFT.h>
#include <stdint.h>

// Pin setup
#define CS   10
#define DC   9
#define RST  8

// Dimension
#define WIDTH 160
#define HEIGHT 128

// Coloring
#define CLEAR 0, 0, 0

typedef uint16_t RGB; 

class Renderer {
public:
    Renderer() : screen(CS, DC, RST) {};
    void begin();
    void clear();
    void add_color(RGB);


private: 
    TFT screen;
};
