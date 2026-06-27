# Quadruped_Robot

A Bluetooth-controlled spider robot powered by an STM32 microcontroller and written in Rust.

Author: Alexandru-Bogdan Gherghiceanu

University: Politehnica București — Faculty of Automatic Control and Computer Science

Course: Microprocessor Architecture (PM), 2026

Project page: embedded-rust-101.wyliodrin.com


Description

This project is a Bluetooth-controlled quadruped (spider) robot that pairs with your phone or tablet over BLE. Once connected, you can control its speed and direction of travel, and issue special commands such as making it wave. The robot uses 8 servomotors — 2 per leg — and an inverse kinematics engine to produce smooth, natural movement.

Motivation

A long-standing interest in robotics, combined with years of following Boston Dynamics' work, made a quadruped robot a natural project choice. After watching Project Hail Mary, a spider form factor felt like the perfect fit. The chassis is a modified version of the open-source Sesame spider robot, adapted to accommodate the chosen components.


Architecture

The system is organized into the following software modules running on the STM32:

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


Hardware

The brain of the robot is the STM32 Nucleo-U545RE-Q microcontroller. It receives Bluetooth commands via the HM-10 module (UART), computes the required servo positions, and sends them to a PCA9685 16-channel PWM driver over I2C. The driver then actuates the 8 MG90S servomotors (2 per leg). Power comes from a 7.4V 1000mAh 2S LiPo battery, stepped down to the appropriate voltage for the logic and servos via an XL4015 buck converter.

Bill of Materials

ComponentRolePrice (RON)STM32 Nucleo-U545RE-QMain microcontroller~120.00Servomotor MG90S (x8)Leg actuation19.34 ea.PCA9685 I2C 16-CH Servo DriverServo PWM interface27.27XL4015 Buck Converter (8-36V, 5A)Voltage step-down9.96KCD-1 On/Off SwitchPower switch1.85HM-10 Bluetooth 4.0 ModuleBLE communication29.99Electrolytic Capacitor 1000 uF 50VVoltage buffering1.491K Ohm 0.25W ResistorSignal conditioning0.10Undervoltage Battery AlarmLow battery warning7.99B3 20W LiPo Charger (2S/3S)Battery charging29.90Gens Ace 1000mAh 7.4V 30C 2S LiPo (XT60)Main power source58.20XT60 Connector SetBattery connection8.82Dupont F-F wires x10 (20cm)Internal wiring2.67Dupont M-F wires x40 (20cm)Internal wiring19.72


Software

The firmware is written in Rust using the Embassy async embedded framework.

Library / ToolDescriptionUsageembassy-stm32STM32 HALControls board pins; drives PCA9685 via I2C and HM-10 via UARTembassy-executorAsync executorRuns the main task loop, processes BLE commands, updates motor anglesembassy-timeTimekeeping & delaysControls animation frame timing for walking gait; handles BLE read timeoutsdefmt + defmt-rttLogging frameworkStreams debug info and state changes to a connected laptop terminalpanic-probePanic handlerCaptures crashes and prints backtraces for easier debuggingprobe-rsFlash & debug toolCompiles, flashes, and runs firmware on the STM32 via cargo run

Building & Flashing

Requires a Rust toolchain with the thumbv8m.main-none-eabihf target and probe-rs installed.

bash# Add the target (once)
rustup target add thumbv8m.main-none-eabihf

# Build and flash
cargo run --release
