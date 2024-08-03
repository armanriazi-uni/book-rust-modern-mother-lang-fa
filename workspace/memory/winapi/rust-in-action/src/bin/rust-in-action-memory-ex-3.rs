#![allow(dead_code, unused_variables)]


/// rust-in-action-memory-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-memory-winapi_bin --bin  rust-in-action-memory-ex-3```
///
/// ## What
/// `Memory Scan 2`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
///
// extern crate winapi;
// use std::ptr::{null_mut};
// use winapi::um::memoryapi::{VirtualQueryEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE};
// use winapi::um::processthreadsapi::{OpenProcess, GetCurrentProcess};
// use winapi::um::sysinfoapi::{GetSystemInfo, GetCurrentProcessId};
// use winapi::um::winnt::{PROCESS_ALL_ACCESS, MEMORY_BASIC_INFORMATION, SIZE_T, SYSTEM_INFO};
// use winapi::shared::minwindef::{DWORD, LPVOID, LPVOID, PVOID, ULONG_PTR};

fn main() {
    unimplemented!();
    // const MEMINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    // let current_process_id: DWORD;
    // let current_process_handle: HANDLE;
    // let mutable_base_address: PVOID;
    // let mutable_system_info: SYSTEM_INFO;
    // let mutable_memory_info: MEMORY_BASIC_INFORMATION;

    // unsafe {
    //     mutable_base_address = null_mut();
    //     mutable_system_info = std::mem::zeroed();
    //     mutable_memory_info = std::mem::zeroed();
    //     current_process_id = GetCurrentProcessId();
    //     current_process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, current_process_id);
    //     GetSystemInfo(&mut mutable_system_info);
    //     println!("Current Process ID: {}", current_process_id);
    //     println!("Current Process Handle: {:?}", current_process_handle);
    //     println!("System Info: {:?}", mutable_system_info);
    //     let region_size = VirtualQueryEx(current_process_handle, mutable_base_address, &mut mutable_memory_info, MEMINFO_SIZE as SIZE_T);
    //     if region_size != 0 {
    //         println!("Memory Region at {:p}: {:?}", mutable_base_address, mutable_memory_info);
    //     }
    // }
}
