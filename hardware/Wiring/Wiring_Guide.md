## Wiring Guide
 
#### Power
 
The LiPo battery connects through the  on/off switch to the XL4015 buck converter `IN+` / `IN-`. Set the converter output to **5V** before connecting anything downstream. From the converter `OUT+` / `OUT-`, run power to two destinations: the **PCA9685** `V+` and `GND` rails (servo power), and the **STM32 Nucleo** `+5V` and `GND` pins.
 
#### HM-10 Bluetooth Module — UART
 
| HM-10 pin | STM32 pin |
|---|---|
| TXD (BT_RX net) | PA9 (BT_RX) |
| RXD (BT_TX net) | PA10 (BT_TX) |
| VCC | +5V |
| GND | GND |
 
Note the net labels cross as expected: the module's TX feeds the MCU's RX pin and vice versa.
 
#### PCA9685 Servo Driver  — I2C
 
| PCA9685 pin | STM32 pin |
|---|---|
| SCL | PB6 (I2C_SCL) |
| SDA | PB7 (I2C_SDA) |
| GND | GND |
| OE | GND (tie low to enable outputs) |
| VCC | +5V |
| V+ | +5V from buck converter |
 
VCC powers the PCA9685 logic and comes from the STM32 board and runs at 3.3V. V+ powers the servo rail runs at 5V here from the buck converter output.
 
#### Servomotors — PCA9685 outputs 
 
The 8 servos connect to the PCA9685. Each servo's signal wire maps to a PWM channel net.
 
Each servo connector is 3-pin: signal (PWM), VCC (+5V), GND. Power and ground for all servos come from the PCA9685 V+ and GND rails.
