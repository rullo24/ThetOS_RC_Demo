#![no_std]
#![no_main]

// STD INCLUDES
use core::panic::PanicInfo;
use core::ptr::{addr_of_mut, null_mut};

// LIBRARY INCLUDES
use nucleo_l152re::{System, TaskId, TaskPriority};
use thetos_entry::entry;

// USER INCLUDES
mod tasks;
use tasks::blink_task;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut STACK_POOL: [u8; 4096] = [0; 4096];

// external project: bsp named explicitly, no THETOS_BSP env / build.rs needed
#[entry(bsp = nucleo_l152re)]
fn app_main() -> ! {
    let p_stack_pool = unsafe { &mut *addr_of_mut!(STACK_POOL) };
    let mut system = System::new_with_pool(p_stack_pool).unwrap();
    system
        .spawn_task(
            TaskId(1),
            TaskPriority::default(),
            1024,
            blink_task,
            null_mut(),
        )
        .unwrap();
    system.run();
}
