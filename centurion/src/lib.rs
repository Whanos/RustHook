#![no_std]
use core::panic::PanicInfo;
use winapi::shared::ntdef::{HANDLE, BOOLEAN};
use core::ffi::c_void;
use winapi::ctypes::c_ulong;


type PcreateProcessNotifyRoutine = extern "system" fn(
    parent_id: HANDLE,
    process_id: HANDLE,
    create: BOOLEAN
) -> c_void;

type PsSetCreateProcessNotifyRoutine = extern "system" fn(
    notify_routine: PcreateProcessNotifyRoutine,
    remove: BOOLEAN
) -> c_ulong;

fn main() {
    let routine_callback: PcreateProcessNotifyRoutine;
    let routine = PsSetCreateProcessNotifyRoutine(routine_callback, )
}

fn pcreate_process_notify_routine(parent_id: HANDLE, process_id: HANDLE, create: BOOLEAN) {

}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Driver entry point
#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    main();
    0 /* STATUS_SUCCESS */
}