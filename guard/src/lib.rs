// winapi
use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPDWORD, LPVOID, MAX_PATH};
use winapi::shared::ntdef::HANDLE;
use winapi::um::consoleapi;
use winapi::um::fileapi::GetFinalPathNameByHandleW;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::processthreadsapi::GetCurrentProcessId;
// minhook
use minhook_sys::{MH_CreateHook, MH_EnableHook, MH_Initialize};
// std
use std::ffi::{c_void, OsString};
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use std::slice;
use std::sync::Mutex;
// ls
use lazy_static::lazy_static;

lazy_static! {
    static ref ORIGINAL: Mutex<Vec<usize>> = Mutex::new(Vec::new());
}

// first typedef the function
type ReadFileFn = extern "system" fn(
    h_file: HANDLE,
    lp_buffer: LPVOID,
    num_bytes_to_read: DWORD,
    num_bytes_read: LPDWORD,
    lp_overlap: LPOVERLAPPED,
) -> BOOL;

// put our hooks in here
unsafe fn enable_hooks() {
    MH_Initialize();

    // get target address (kernel32 dll readfile)
    let target = std::mem::transmute::<_, *mut c_void>(GetProcAddress(
        GetModuleHandleA("kernel32.dll\0".as_ptr() as _),
        "ReadFile\0".as_ptr() as _,
    ) as usize);

    // original will contain the original call we can edit
    let mut original = null_mut();
    let hook = MH_CreateHook(target, hook as _, &mut original);

    // enable hook
    MH_EnableHook(hook as _);

    // lazy static
    ORIGINAL.lock().unwrap().push(original as usize);
}

// util function that will convert a file handle (from hook) to string
unsafe fn get_file_path_from_handle(handle: HANDLE) -> String {
    let mut buf = vec![0u16; MAX_PATH]; // windows uses utf16
    GetFinalPathNameByHandleW(handle, buf.as_mut_ptr(), MAX_PATH as u32,
                              0x8); // call windows function
    buf.retain(|&i| i != 0); // remove all the nulls
    let buf_u16 = slice::from_raw_parts(buf.as_ptr(), buf.len()); // slice it
    let name = OsString::from_wide(buf_u16) // into an OsString
        .as_os_str() // into an &OsStr
        .to_string_lossy() // into a str
        .into_owned(); // into a String
                       // I know that's messy. it works though /shrug
    name
}

// hook function which takes our shit
extern "system" fn hook(
    h_file: HANDLE,                   // File Handle
    lp_buffer: LPVOID,                // Pointer to buffer that receives data
    n_number_of_bytes_to_read: DWORD, // Bytes to read
    lp_number_of_bytes_read: LPDWORD, // Bytes read
    lp_overlapped: LPOVERLAPPED,      // OVERLAPPED Struct
) -> BOOL {
    let original = unsafe {
        std::mem::transmute::<_, ReadFileFn>(ORIGINAL.lock().unwrap()[0])
    };

    unsafe {
        let path = get_file_path_from_handle(h_file);
        println!(
            "Called ReadFile! Path: {:?}, LP_BUFFER: {:?}, BYTES_TO_READ: {:?}, BYTES_READ: {:?}",
            path, lp_buffer, n_number_of_bytes_to_read, lp_number_of_bytes_read
        );
    }
    // call the original function after
    // you could change what it does here
    original(
        h_file,
        lp_buffer,
        n_number_of_bytes_to_read,
        lp_number_of_bytes_read,
        lp_overlapped,
    )
}

// DLL injection point for windows
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, reserved: LPVOID) -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => unsafe {
            consoleapi::AllocConsole();
            let process = GetCurrentProcessId();
            println!("Successfully hooked into PID {:?}!", process);
            enable_hooks();
            println!("Enabled ReadFile Hooks!");
        },
        DLL_PROCESS_DETACH => {
            // one day i'll actually do something with this
        }
        _ => (),
    }
    minwindef::TRUE
}
