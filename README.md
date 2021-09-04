# LED Backlighting Controller

This project is designed to control a strip of ws2812 LED's that serve to backlight wall-hung art.

At the moment, in it's earliest viable release form, it has one momentary push button that is used
to cycle through the available solid color lighting choices.

Future updates for this project will support a potentiometer for dimming, and various animation modes.

## Build and Flash

```
cargo build --release
cargo hf2 --release
```

## Video Demo

[![YouTube Demo](https://img.youtube.com/vi/AdlcyJdxIKM/0.jpg)](https://youtu.be/AdlcyJdxIKM)

## Electrical Schematic

![schematic](schematic.svg)

## Power Usage
Power values below from ws2812 strip of 20 led's

| Color | mA @ max brightness | mA @ 50% brightness | mA @ 10% brightness |
| ----- | ------------------- | ------------------- | ------------------- |
| WHITE | 14.30 | 7.26 | 2.05 |
| RED | 5.28 | 3.13 | 1.74 |
| GREEN | 3.11 | 2.25 | 1.65 |
| BLUE | 5.26 | 3.15 | 1.73 |
