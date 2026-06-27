## Wiring Guide
 
#### Power
 
The LiPo battery connects through the KCD-1 on/off switch to the XL4015 buck converter `IN+` / `IN-`. Set the converter output to **5V** before connecting anything downstream. From the converter `OUT+` / `OUT-`, run power to two destinations: the **PCA9685** `V+` and `GND` rails (servo power), and the **STM32 Nucleo** `+5V` and `GND` pins. The 1000 uF capacitor goes across the PCA9685 power input to smooth voltage spikes when servos move.
 
#### HM-10 Bluetooth Module (J3) — UART
 
| HM-10 pin | STM32 pin |
|---|---|
| TXD (BT_RX net) | PA9 (BT_RX) |
| RXD (BT_TX net) | PA10 (BT_TX) |
| VCC | +5V |
| GND | GND |
 
Note the net labels cross as expected: the module's TX feeds the MCU's RX pin and vice versa.
 
#### PCA9685 Servo Driver (J1) — I2C
 
| PCA9685 pin | STM32 pin |
|---|---|
| SCL | PB6 (I2C_SCL) |
| SDA | PB7 (I2C_SDA) |
| GND | GND |
| OE | GND (tie low to enable outputs) |
| VCC | +5V |
| V+ | +5V from buck converter |
 
VCC powers the PCA9685 logic. V+ powers the servo rail. Both run at 5V here but come from the same buck converter output.
 
#### Servomotors — PCA9685 outputs (J4)
 
The 8 servos connect to the PCA9685 via the J4 port header. Each servo's signal wire maps to a PWM channel net:
 
| J4 pin | Net | Servo |
|---|---|---|
| 1 | PWM_1 | M1 |
| 2 | PWM_2 | M2 |
| 3 | PWM_3 | M3 |
| 4 | PWM_4 | M4 |
| 5 | PWM_5 | M5 |
| 6 | PWM_6 | M6 |
| 7 | PWM_7 | M7 |
| 8 | PWM_8 | M8 |
 
Each servo connector is 3-pin: signal (PWM), VCC (+5V), GND. Power and ground for all servos come from the PCA9685 V+ and GND rails.
