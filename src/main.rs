#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{addr_of_mut, null_mut};

use nucleo_l152re::{
    system, GpioLevel, OutputPin, OutputStyle, System, TaskId, TaskPriority, UninitPin, PA5,
};
use thetos_entry::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut STACK_POOL: [u8; 4096] = [0; 4096];

extern "C" fn blink_task(_arg: *mut ()) -> ! {
    let mut ld2 = PA5.into_output(OutputStyle::PushPull);
    loop {
        ld2.set(GpioLevel::High);
        system::delay_ms(500).unwrap();
        ld2.set(GpioLevel::Low);
        system::delay_ms(500).unwrap();
    }
}

// external project: bsp named explicitly, no THETOS_BSP env / build.rs needed
#[entry(bsp = nucleo_l152re)]
fn app_main() -> ! {
    let p_stack_pool = unsafe { &mut *addr_of_mut!(STACK_POOL) };
    let mut system = System::new_with_pool(p_stack_pool).unwrap();
    system
        .spawn_task(TaskId(1), TaskPriority::default(), 1024, blink_task, null_mut())
        .unwrap();
    system.run();
}
