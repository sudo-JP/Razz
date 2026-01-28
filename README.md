# Razz
Razz is a graphics and simulation playground exploring ray tracing, image transformation, and possibly physics

## Plan 
I'm currently following this book: [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) for this project. I'm planning to finish all three series, then move on to frames by frames rendering. There is another thing I want to to do before I leave ray tracing, that is, render the image on Rust, then stream the data to the Arduino/RP2040 to render an image on 128x160 TFT screen. 

After that, I plan to learn physics, and make a physics simulation. To top everything off, I wanna write an interpreter so I can freely adjust the objects. Ambitious, I know. 

## Run 
To run, do 
```bash
cargo run -- --output test.ppm --width 1000 --height 1000
```
