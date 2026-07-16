# Quadruped Robot
 
A Bluetooth-controlled spider robot powered by an STM32 microcontroller and written in Rust.
 
| | |
|---|---|
| **Author** | Alexandru-Bogdan Gherghiceanu |
| **University** | Politehnica București — Faculty of Automatic Control and Computer Science |
| **Course** | Microprocessor Architecture (PM), 2026 |
| **Project page** | [embedded-rust-101.wyliodrin.com](https://embedded-rust-101.wyliodrin.com/docs/acs_cc/project/2026/agherghiceanu) |
 
---
 
## Description
 
This project is a Bluetooth-controlled quadruped (spider) robot that pairs with your phone or tablet over BLE. Once connected, you can control its speed and direction of travel, and issue special commands such as making it wave. The robot uses 8 servomotors — 2 per leg — and an inverse kinematics engine to produce smooth, natural movement.
 
## Motivation
 
A long-standing interest in robotics, combined with years of following Boston Dynamics' work, made a quadruped robot a natural project choice. After watching *Project Hail Mary*, a spider form factor felt like the perfect fit. The chassis is a modified version of the open-source [Sesame spider robot](https://www.doriantodd.com/sesame), adapted to accommodate the chosen components.
 
---
 
## Architecture
 
The system is organized into the following software modules running on the STM32:
 
```
Android/iOS Device
        │  (BLE)
        ▼
  HM-10 Bluetooth Module
        │  (UART / Serial)
        ▼
  Bluetooth Handler Module   ← parses commands (e.g. 'F', 'L', 'W')
        │
        ▼
  Motion Planner Module      ← determines target pose / leg positions (X, Y, Z)
        │
        ▼
  Kinematics Engine (IK)     ← converts positions to servo angles
        │
        ▼
  I2C Controller Module
        │  (I2C)
        ▼
  PCA9685 Servo Driver       ← generates PWM signals
        │
        ▼
  Servomotors x8
```
 
---
 
## Hardware
 
The brain of the robot is the **STM32 Nucleo-U545RE-Q** microcontroller. It receives Bluetooth commands via the HM-10 module (UART), computes the required servo positions, and sends them to a **PCA9685** 16-channel PWM driver over I2C. The driver then actuates the 8 **MG90S** servomotors (2 per leg). Power comes from a **7.4V 1000mAh 2S LiPo** battery, stepped down to the appropriate voltage for the logic and servos via an **XL4015** buck converter.

## Build Progress & Media

A look at the robot from early assembly to a working walking gait.

### Photos

| | |
|---|---|
| ![Parts before assembly](media/parts_disassembled.jpg) | ![PCB and wiring](media/pcb_wiring.jpg) |
| Servos and parts before assembly | PCA9685 driver + power wiring |

![Assembled robot](media/robot_assembled.jpg)

*The robot fully assembled, with the STM32 board mounted on top.*

### Demo

![Walking demo](media/walking_demo.gif)
 
### Bill of Materials
 
| Component | Role | Price (RON) |
|---|---|---|
| STM32 Nucleo-U545RE-Q | Main microcontroller | ~120.00 |
| Servomotor MG90S (x8) | Leg actuation | 19.34 ea. |
| PCA9685 I2C 16-CH Servo Driver | Servo PWM interface | 27.27 |
| XL4015 Buck Converter (8-36V, 5A) | Voltage step-down | 9.96 |
| KCD-1 On/Off Switch | Power switch | 1.85 |
| HM-10 Bluetooth 4.0 Module | BLE communication | 29.99 |
| Electrolytic Capacitor 1000 uF 50V | Voltage buffering | 1.49 |
| 1K Ohm 0.25W Resistor | Signal conditioning | 0.10 |
| Undervoltage Battery Alarm | Low battery warning | 7.99 |
| B3 20W LiPo Charger (2S/3S) | Battery charging | 29.90 |
| Gens Ace 1000mAh 7.4V 30C 2S LiPo (XT60) | Main power source | 58.20 |
| XT60 Connector Set | Battery connection | 8.82 |
| Dupont F-F wires x10 (20cm) | Internal wiring | 2.67 |
| Dupont M-F wires x40 (20cm) | Internal wiring | 19.72 |
 
---
 
## Software
 
The firmware is written in **Rust** using the [Embassy](https://github.com/embassy-rs/embassy) async embedded framework.
 
| Library / Tool | Description | Usage |
|---|---|---|
| `embassy-stm32` | STM32 HAL | Controls board pins; drives PCA9685 via I2C and HM-10 via UART |
| `embassy-executor` | Async executor | Runs the main task loop, processes BLE commands, updates motor angles |
| `embassy-time` | Timekeeping & delays | Controls animation frame timing for walking gait; handles BLE read timeouts |
| `defmt` + `defmt-rtt` | Logging framework | Streams debug info and state changes to a connected laptop terminal |
| `panic-probe` | Panic handler | Captures crashes and prints backtraces for easier debugging |
| `probe-rs` | Flash & debug tool | Compiles, flashes, and runs firmware on the STM32 via `cargo run` |
 
### Building & Flashing
 
Requires a Rust toolchain with the `thumbv8m.main-none-eabihf` target and `probe-rs` installed.
 
```bash
# Add the target (once)
rustup target add thumbv8m.main-none-eabihf
 
# Build and flash
cargo run --release
```
 
---
