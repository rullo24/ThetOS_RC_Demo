#![no_std]
#![no_main]

// STD INCLUDES
use core::panic::PanicInfo;
use core::ptr::{addr_of_mut, null_mut, write_volatile};

// LIBRARY INCLUDES
use nucleo_l152re::{System, TaskId, TaskPriority};
use thetos_entry::entry;

// USER INCLUDES
mod battery;
mod config;
mod diag;
mod motor;
mod shared;
mod tasks;
use battery::TwoCellLipo;
use motor::RcCarDrivetrain;
use tasks::{comms_task, drive_task, heartbeat_task};

// TIM3 base 0x4000_0400 + CCER offset 0x20 (see bsp pwm.rs) -> written directly here,
// not through the Pwm driver, since a panic can happen with no task/RTOS state assumed
const TIM3_CCER: *mut u32 = 0x4000_0420 as *mut u32;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // safety net: force both motor PWM channel outputs off before halting, so a panic
    // never leaves the car driving with no way to stop it. The timer keeps running in
    // hardware even after the CPU halts, so this has to happen here, not just rely on
    // the loop below. Harmless if TIM3 was never clocked or configured.
    unsafe { write_volatile(TIM3_CCER, 0) };
    loop {}
}

// plain [u8; N] statics aren't guaranteed 8-byte (AAPCS) alignment, so this is forced
// explicitly -> every task's stack top/limit must be 8-byte aligned (see arch/cortex-m).
#[repr(align(8))]
struct AlignedStackPool(#[allow(dead_code)] [u8; 8192]);
static mut STACK_POOL: AlignedStackPool = AlignedStackPool([0; 8192]);

// external project: bsp named explicitly, no THETOS_BSP env / build.rs needed
#[entry(bsp = nucleo_l152re)]
fn app_main() -> ! {
    let stack_pool: &mut AlignedStackPool = unsafe { &mut *addr_of_mut!(STACK_POOL) };
    let p_stack_pool: &mut [u8] = &mut stack_pool.0;

    let mut system = match System::new_with_pool(p_stack_pool) {
        Ok(system) => system,
        Err(_) => diag::fail_blink(1), // timer/idle-task init failed
    };

    // this build's hardware: swap either line to retarget a different pack or drivetrain
    let battery = TwoCellLipo;
    let motor = RcCarDrivetrain;
    config::apply_startup_config(&battery, &motor); // seed per-wheel duty from them

    let comms_spawn = system.spawn_task(
        TaskId(1),
        TaskPriority::default(),
        2048,
        comms_task,
        null_mut(),
    );
    if comms_spawn.is_err() {
        diag::fail_blink(2);
    }

    let drive_spawn = system.spawn_task(
        TaskId(2),
        TaskPriority::default(),
        2048,
        drive_task,
        null_mut(),
    );
    if drive_spawn.is_err() {
        diag::fail_blink(3);
    }

    let heartbeat_priority = match TaskPriority::new(20) {
        // below comms/drive -> status only
        Ok(priority) => priority,
        Err(_) => diag::fail_blink(4),
    };

    let heartbeat_spawn = system.spawn_task(
        TaskId(3),
        heartbeat_priority,
        1024,
        heartbeat_task,
        null_mut(),
    );
    if heartbeat_spawn.is_err() {
        diag::fail_blink(5);
    }

    system.run();
}
