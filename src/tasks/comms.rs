// LIBRARY INCLUDES
use core::sync::atomic::Ordering;
use nucleo_l152re::{system, Serial, Uart, UartConfig, UninitUart, PA2, PA3};

// USER INCLUDES
use crate::shared::{
    LEFT_CRUISE_DUTY, LEFT_DUTY, LEFT_TURN_DUTY, RIGHT_CRUISE_DUTY, RIGHT_DUTY, RIGHT_TURN_DUTY,
};

const CRUISE_STEP: u16 = u16::MAX / 100; // ~1% per trim press
const IDLE_POLL_MS: u32 = 2; // yield this long when no byte is waiting

// read single-char drive commands over USART2 and update the shared duty targets
pub extern "C" fn comms_task(_arg: *mut ()) -> ! {
    let mut serial = Serial::usart2(PA2, PA3).into_active(UartConfig::default());
    serial.write(b"rc ready: w/a/d/s drive, +/- both, 1/2 left, 3/4 right\r\n");

    loop {
        match serial.read_byte() {
            Ok(Some(byte)) => handle_byte(&mut serial, byte),
            // delay_ms can only fail from a kernel-level fault; degrade to polling
            // flat out rather than halting the whole system over a missed tick
            Ok(None) => {
                let _ = system::delay_ms(IDLE_POLL_MS);
            }
            Err(_) => {
                // receive fault -> fail safe to stopped
                LEFT_DUTY.store(0, Ordering::Relaxed);
                RIGHT_DUTY.store(0, Ordering::Relaxed);
            }
        }
    }
}

fn handle_byte<U: Uart>(serial: &mut U, byte: u8) {
    let left_cruise = LEFT_CRUISE_DUTY.load(Ordering::Relaxed);
    let right_cruise = RIGHT_CRUISE_DUTY.load(Ordering::Relaxed);
    let left_turn = LEFT_TURN_DUTY.load(Ordering::Relaxed);
    let right_turn = RIGHT_TURN_DUTY.load(Ordering::Relaxed);

    let targets = match byte {
        b'w' => Some((left_cruise, right_cruise)),
        b'a' => Some((left_turn, right_cruise)), // left is the inside wheel
        b'd' => Some((left_cruise, right_turn)), // right is the inside wheel
        b's' | b' ' => Some((0, 0)),
        _ => None,
    };
    if let Some((left, right)) = targets {
        LEFT_DUTY.store(left, Ordering::Relaxed);
        RIGHT_DUTY.store(right, Ordering::Relaxed);
        serial.write_byte(byte); // echo as an ack
        return;
    }

    match byte {
        // both motors together -> coarse-tune the overall speed
        b'+' => {
            LEFT_CRUISE_DUTY.store(left_cruise.saturating_add(CRUISE_STEP), Ordering::Relaxed);
            RIGHT_CRUISE_DUTY.store(right_cruise.saturating_add(CRUISE_STEP), Ordering::Relaxed);
            write_both(serial);
        }
        b'-' | b'_' => {
            LEFT_CRUISE_DUTY.store(left_cruise.saturating_sub(CRUISE_STEP), Ordering::Relaxed);
            RIGHT_CRUISE_DUTY.store(right_cruise.saturating_sub(CRUISE_STEP), Ordering::Relaxed);
            write_both(serial);
        }
        // one motor at a time -> fine-trim so left and right drive straight together
        b'1' => {
            LEFT_CRUISE_DUTY.store(left_cruise.saturating_sub(CRUISE_STEP), Ordering::Relaxed);
            write_pct(serial, b"left", LEFT_CRUISE_DUTY.load(Ordering::Relaxed));
        }
        b'2' => {
            LEFT_CRUISE_DUTY.store(left_cruise.saturating_add(CRUISE_STEP), Ordering::Relaxed);
            write_pct(serial, b"left", LEFT_CRUISE_DUTY.load(Ordering::Relaxed));
        }
        b'3' => {
            RIGHT_CRUISE_DUTY.store(right_cruise.saturating_sub(CRUISE_STEP), Ordering::Relaxed);
            write_pct(serial, b"right", RIGHT_CRUISE_DUTY.load(Ordering::Relaxed));
        }
        b'4' => {
            RIGHT_CRUISE_DUTY.store(right_cruise.saturating_add(CRUISE_STEP), Ordering::Relaxed);
            write_pct(serial, b"right", RIGHT_CRUISE_DUTY.load(Ordering::Relaxed));
        }
        // unrecognised -> echo its raw value so a wrong keystroke is visible, not silent
        other => write_unknown_byte(serial, other),
    }
}

fn write_both<U: Uart>(serial: &mut U) {
    write_pct(serial, b"left", LEFT_CRUISE_DUTY.load(Ordering::Relaxed));
    write_pct(serial, b"right", RIGHT_CRUISE_DUTY.load(Ordering::Relaxed));
}

// print "<label>: NN%\r\n" without pulling in core::fmt
fn write_pct<U: Uart>(serial: &mut U, label: &[u8], duty: u16) {
    let pct = (duty as u32 * 100 / u16::MAX as u32) as u8;

    serial.write(label);
    serial.write(b": ");
    if pct >= 100 {
        serial.write(b"100");
    } else if pct >= 10 {
        serial.write_byte(b'0' + pct / 10);
        serial.write_byte(b'0' + pct % 10);
    } else {
        serial.write_byte(b'0' + pct);
    }
    serial.write(b"%\r\n");
}

// print "byte: NNN\r\n" -> lets you see exactly what arrived for an unmapped key
fn write_unknown_byte<U: Uart>(serial: &mut U, byte: u8) {
    serial.write(b"byte: ");
    let hundreds = byte / 100;
    let tens = (byte / 10) % 10;
    let ones = byte % 10;
    if hundreds > 0 {
        serial.write_byte(b'0' + hundreds);
    }
    if hundreds > 0 || tens > 0 {
        serial.write_byte(b'0' + tens);
    }
    serial.write_byte(b'0' + ones);
    serial.write(b"\r\n");
}
