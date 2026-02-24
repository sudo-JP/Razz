# Razz
Razz is a graphics and simulation playground exploring ray tracing, image transformation, and possibly physics

## Plan 
I'm currently following this book: [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html) for this project. 
I finished the first book. I'm planning to finish all three series, then move on to frames by frames rendering. There is another thing I want to to do before I leave ray tracing, that is, render the image on Rust, then stream the data to the Arduino/RP2040 to render an image on 128x160 TFT screen. 

After that, I plan to learn physics, and make a physics simulation. To top everything off, I wanna write an interpreter so I can freely adjust the objects. Ambitious, I know. 

## Challenges 
### Ray Tracing In One Weekend
I understand most of the part in Ray Tracing In One Weekend. Three quarter of the book does required me to do write out the math, or just understanding them in general. However, the latter quarter...is something else. The math gets more abstract, less explanation of why this is the case, and leave more of the math to the mathematicians. It is a valid approach to not fully understand the math from a CS perspective, however, I feel like I've been left too much in the dark. I'll try to brush up my math still for the next chapter. 

### Arduino 
Working with Arduino presented several interconnected challenges spanning hardware constraints, protocol design, and timing issues:

### Hardware Constraints
Arduino Uno has only 2KB of RAM, insufficient for storing a full 128×160 TFT frame buffer (128 × 160 × 3 = 61KB for RGB888). My solution involved two optimizations:
- **Color compression**: Converting RGB888 (3 bytes/pixel) to RGB565 (2 bytes/pixel), reducing payload size by 33%
- **Circular buffer**: Implementing a 512-byte staging buffer with incremental flushing. Buffer chunks are pushed to the display via `pushColor()` as they arrive, avoiding full-frame storage

### Protocol Design
I designed a custom binary serial protocol with the following features:
- **Magic bytes** (0xBABE) for frame synchronization and recovery
- **Header structure** containing image dimensions (width, height) for validation
- **CRC16 checksums** for both header and payload integrity verification
- **State machine parser** (FIND_SYNC → READ_HEADER → READ_PAYLOAD → VERIFY) with automatic fallback to FIND_SYNC on corruption

The biggest debugging challenge was deterministic image corruption caused by sending data before Arduino completed initialization. The display reset (triggered by opening the serial port via DTR signal) takes ~2-5 seconds. Solution: implement a handshake where Arduino sends a ready byte (0xAA) after initialization, and Rust blocks on `read()` before transmitting.

### Hardware Quirks
- **BGR color order**: Display hardware interprets RGB565 as BGR565. Fixed by swapping red/blue channels in the Rust encoder
- **Display initialization**: `screen.begin()` worked reliably for correct scan direction (top-left to bottom-right), while `initR()` variants produced quarter-screen offsets or incorrect orientations
- **setAddrWindow optimization**: Setting the draw window once in `setup()` allows subsequent `pushColor()` calls to stream pixels sequentially without repeated addressing overhead


## Image 
### Ray Tracing In One Weekend
![](./docs/RTIOW.png)

## Video 
### Arduino 
https://github.com/user-attachments/assets/beb13901-d2f5-4c20-9113-03e83370cab0

## Run 
To run, do 
```bash
cargo run --release -- --output test.ppm --width 1000 --height 1000
```

