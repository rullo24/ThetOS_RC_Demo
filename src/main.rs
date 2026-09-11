#![no_std]
#![no_main]

// STD INCLUDES
use core::panic::PanicInfo;
use core::ptr::{addr_of_mut, null_mut};

// LIBRARY INCLUDES
use nucleo_l152re::{System, TaskId, TaskPriority};
use thetos_entry::entry;

// USER INCLUDES
mod battery;
mod config;
mod motor;
mod shared;
mod tasks;
use tasks::{comms_task, drive_task, heartbeat_task};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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
    let mut system = System::new_with_pool(p_stack_pool).unwrap();

    config::apply_startup_config(); // seed per-wheel duty from the active battery + motor runstates

    system
        .spawn_task(
            TaskId(1),
            TaskPriority::default(),
            2048,
            comms_task,
            null_mut(),
        )
        .unwrap();
    system
        .spawn_task(
            TaskId(2),
            TaskPriority::default(),
            2048,
            drive_task,
            null_mut(),
        )
        .unwrap();
    system
        .spawn_task(
            TaskId(3),
            TaskPriority::new(20).unwrap(), // below comms/drive -> status only
            1024,
            heartbeat_task,
            null_mut(),
        )
        .unwrap();

    system.run();
}
