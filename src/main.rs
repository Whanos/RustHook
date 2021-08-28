use sysinfo::{ProcessExt, System, SystemExt, AsU32};
use djin;

fn main () {
    #[cfg(target_pointer_width = "64")]
    let dll = "guard/target/debug/guard.dll";

    let system = System::new_all();
    for (pid, process) in system.processes() {
        if process.name().contains("process_to_monitor.exe") {
            let handle = djin::open_process(pid.as_u32()).unwrap();
            println!("{:?}", djin::inject_dll(handle, dll, b"DllMain"));
        }
    }
}