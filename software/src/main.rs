#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_stm32::i2c::{I2c, Config as I2cConfig};
use embassy_stm32::usart::{BufferedUart, Config as UartConfig};
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer, with_timeout};
use embedded_io_async::Read;

bind_interrupts!(struct Irqs {
    LPUART1 => usart::BufferedInterruptHandler<peripherals::LPUART1>;
});

#[derive(PartialEq)]
enum State {
    Stand,
    Forward,
    Backward,
    Left,
    Right,
    Debug,
    Wave
}

fn get_servo_buffer(angles: &[u8; 8]) -> [u8; 61] {
    let mut buf = [0u8; 61];
    buf[0] = 0x06; 

    const MIN_PULSE_US: u32 = 732;
    const MAX_PULSE_US: u32 = 2929;
    const PERIOD_US: u32 = 20000;

    let pca_pins: [usize; 8] = [0, 2, 4, 7, 8, 10, 12, 14];

    for i in 0..8 {
        let pin = pca_pins[i];
        let offset = 1 + pin * 4;
        
        let pulse = angles[i] as u32 * (MAX_PULSE_US - MIN_PULSE_US) / 180 + MIN_PULSE_US;
        let tick = ((pulse * 4096) / PERIOD_US) as u16;

        buf[offset] = 0;
        buf[offset + 1] = 0;
        buf[offset + 2] = (tick & 0xFF) as u8;
        buf[offset + 3] = (tick >> 8) as u8;
    }
    
    buf
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    

    info!("--- PLACA A PORNIT! INCEP INITIALIZAREA I2C ---");


    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    let mut i2c = I2c::new_blocking(p.I2C1, p.PB6, p.PB7, i2c_config);
    let address = 0x40;

    info!("Pornire PCA9685...");
    let _ = i2c.blocking_write(address, &[0x00, 0x10]);
    let _ = i2c.blocking_write(address, &[0xFE, 121]);
    let _ = i2c.blocking_write(address, &[0x00, 0x20]);
    Timer::after_millis(10).await;

    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 9600;
    
    static mut TX_STORAGE: [u8; 16] = [0; 16];
    static mut RX_STORAGE: [u8; 64] = [0; 64];

    let tx_buffer = unsafe { &mut *(&raw mut TX_STORAGE) };
    let rx_buffer = unsafe { &mut *(&raw mut RX_STORAGE) };
    
    let mut serial = BufferedUart::new(
        p.LPUART1, 
        p.PA3, // RX
        p.PA2, // TX
        tx_buffer, 
        rx_buffer, 
        Irqs, 
        uart_config
    ).unwrap();

    let mut state = State::Debug;
    let mut frame: usize = 0;
    let mut b = [0u8; 1]; 

    info!("Comenzi: W(Fata), S(Spate), A(Stanga), D(Dreapta), Q(Debug Montaj), E(Wave), T(Stand)");

    loop {
        if let Ok(Ok(_)) = with_timeout(Duration::from_millis(20), serial.read(&mut b)).await {
            let cmd = b[0] as char;
            match cmd {
                'W' | 'w' => { state = State::Forward; frame = 0; info!("FATA "); }
                'S' | 's' => { state = State::Backward; frame = 0; info!("SPATE "); }
                'A' | 'a' => { state = State::Left; frame = 0; info!("STANGA "); }
                'D' | 'd' => { state = State::Right; frame = 0; info!("DREAPTA "); }
                'Q' | 'q' => { state = State::Debug; frame = 0; info!("Debug Montaj (Toate la 90)"); }
                'E' | 'e' => { state = State::Wave; frame = 0; info!("Wave"); }
                'T' | 't' => { state = State::Stand; frame = 0; info!("Sta in picioare!"); }
                _ => {}
            }
        }

        match state {
            State::Stand => {
                                        // L1 ,L3, L2 ,L4, R1, R3, R2, R4
               let stand_pose = [90, 45,  90, 135, 90, 135,  90, 45]; 
                
                let buf = get_servo_buffer(&stand_pose);
                let _ = i2c.blocking_write(address, &buf);
            }
            State::Forward => {
                let frames = [
                    [110, 80,   70, 135,   110, 135,   110, 45],
                    [110, 45,   70, 135,   110, 135,   110, 45],

                    [110, 45,   70, 135,   110, 135,   70, 80],
                    [110, 45,   70, 135,   110, 135,   70, 45],

                    [110, 45,   70, 135,   70, 100,    70, 45],
                    [110, 45,   70, 135,   70, 135,    70, 45],

                    [110, 45,   110, 100,  70, 135,    70, 45],
                    [110, 45,   110, 135,  70, 135,    70, 45],

                    [70, 45,    70, 135,   110, 135,   110, 45],
                ];
                let buf = get_servo_buffer(&frames[frame]);
                let _ = i2c.blocking_write(address, &buf);
                
                frame += 1;
                if frame >= frames.len() {
                    frame = 0; 
                }
            }
            State::Backward => {
                let frames = [
                    [70, 80,   110, 135,  70, 135,   70, 45],
                    [70, 45,   110, 135,  70, 135,   70, 45],

                    [70, 45,   110, 135,  70, 135,   110, 80],
                    [70, 45,   110, 135,  70, 135,   110, 45],

                    [70, 45,   110, 135,  110, 100,  110, 45],
                    [70, 45,   110, 135,  110, 135,  110, 45],

                    [70, 45,   70, 100,   110, 135,  110, 45],
                    [70, 45,   70, 135,   110, 135,  110, 45],

                    [110, 45,  110, 135,  70, 135,   70, 45],
                ];
                let buf = get_servo_buffer(&frames[frame]);
                let _ = i2c.blocking_write(address, &buf);
                
                frame += 1;
                if frame >= frames.len() {
                    frame = 0; 
                }
            }
            State::Left => {
                let frames = [
                    [70, 80,   110, 135,  110, 135,  110, 45], 
                    [70, 45,   110, 135,  110, 135,  110, 45], 
                    
                    [70, 45,   110, 135,  110, 135,  70, 80], 
                    [70, 45,   110, 135,  110, 135,  70, 45], 
                    
                    [70, 45,   110, 135,  70, 100,   70, 45], 
                    [70, 45,   110, 135,  70, 135,   70, 45], 
                    
                    [70, 45,   70, 100,   70, 135,   70, 45], 
                    [70, 45,   70, 135,   70, 135,   70, 45], 
                    
                    [110, 45,  110, 135,  110, 135,  110, 45],
                ];
                let buf = get_servo_buffer(&frames[frame]);
                let _ = i2c.blocking_write(address, &buf);
                
                frame += 1;
                if frame >= frames.len() {
                    frame = 0;
                }
            }
            State::Right => {
                let frames = [
                    [110, 80,   70, 135,  70, 135,  70, 45], 
                    [110, 45,   70, 135,  70, 135,  70, 45], 
                    
                    [110, 45,   70, 135,  70, 135,  110, 80], 
                    [110, 45,   70, 135,  70, 135,  110, 45], 
                    
                    [110, 45,   70, 135,  110, 100, 110, 45], 
                    [110, 45,   70, 135,  110, 135, 110, 45], 
                    
                    [110, 45,   110, 100, 110, 135, 110, 45], 
                    [110, 45,   110, 135, 110, 135, 110, 45], 
                    
                    [70, 45,    70, 135,  70, 135,  70, 45],
                ];
                let buf = get_servo_buffer(&frames[frame]);
                let _ = i2c.blocking_write(address, &buf);
                
                frame += 1;
                if frame >= frames.len() {
                    frame = 0;
                }
            }
            State::Debug => {
                let buf = get_servo_buffer(&[90, 90, 90, 90,  90, 90, 90, 90]);
                let _ = i2c.blocking_write(address, &buf);
            }
            State::Wave => {
                let frames = [
                    [90, 125,   90, 135,   90, 135,   90, 65],

                    [90, 90,  90, 135,   90, 135,   90, 65],
                    
                    [90, 55,   90, 135,   90, 135,   90, 65],
                    
                    [90, 90,   90, 135,   90, 135,   90, 65],
                ];
                
                let buf = get_servo_buffer(&frames[frame]);
                let _ = i2c.blocking_write(address, &buf);
                
                frame += 1;
                if frame >= frames.len() { frame = 0; }
            }
        }

        Timer::after_millis(150).await;
    }
}