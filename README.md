# ch59x-hal

WIP.

HAL for the CH59x family of microcontrollers.

## Supported Microcontrollers

- CH591, 192K Code Flash + 26K RAM
- CH592, 448K Code Flash + 26K RAM

## Boards

### CH592X-R1-1v0

> **Warning**
> The board has a 8x8 Segment LCD display, but it conflicts with the 2-wire debug interface and Reset pin.
> When using the LCD display, use `wchisp`/USB-ISP to flash and disable the debug interface and reset pin.

- UART1: PA8(RX), PA9(TX)
- LED1 and LED2: NC, use a jumper to connect to PB23 and PA4 respectively
