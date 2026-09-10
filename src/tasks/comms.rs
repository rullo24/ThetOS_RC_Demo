// LIBRARY INCLUDES
use core::sync::atomic::Ordering;
use nucleo_l152re::{system, Serial, Uart, UartConfig, UninitUart, PA2, PA3};

// USER INCLUDES
use crate::shared::{LEFT_DUTY, RIGHT_DUTY};

const CRUISE: u16 = 45_000; // ~69% duty -> forward speed, tune on the bench
const TURN_DROP: u16 = 25_000; // inside wheel slows by this when veering
const IDLE_POLL_MS: u32 = 2; // yield this long when no byte is waiting

// read single-char drive commands over USART2 and update the shared duty targets
pub extern "C" fn comms_task(_arg: *mut ()) -> ! {
    let mut serial = Serial::usart2(PA2, PA3).into_active(UartConfig::default());
    serial.write(b"rc ready: w forward, a left, d right, s stop\r\n");

    loop {
        match serial.read_byte() {
            Ok(Some(byte)) => {
                let targets = match byte {
                    b'w' => Some((CRUISE, CRUISE)),
                    b'a' => Some((CRUISE - TURN_DROP, CRUISE)),
                    b'd' => Some((CRUISE, CRUISE - TURN_DROP)),
                    b's' | b' ' => Some((0, 0)),
                    _ => None,
                };
                if let Some((left, right)) = targets {
                    LEFT_DUTY.store(left, Ordering::Relaxed);
                    RIGHT_DUTY.store(right, Ordering::Relaxed);
                    serial.write_byte(byte); // echo as an ack
                }
            }
            Ok(None) => system::delay_ms(IDLE_POLL_MS).unwrap(), // let drive run
            Err(_) => {
                // receive fault -> fail safe to stopped
                LEFT_DUTY.store(0, Ordering::Relaxed);
                RIGHT_DUTY.store(0, Ordering::Relaxed);
            }
        }
    }
}
