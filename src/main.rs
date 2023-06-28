use getch::Getch;
use std::io::{self};
use std::ffi::CString;
use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError};
use windows::{
    core::PCSTR,
    Win32::Foundation::{CloseHandle, HANDLE, WAIT_ABANDONED, WAIT_OBJECT_0},
    Win32::System::Threading::{CreateMutexA, ReleaseMutex, WaitForSingleObject},
};

const INFINITE: u32 = 0xFFFFFFFF;

struct Handle (HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

struct Guard (Handle);

impl Drop for Guard {
    fn drop(&mut self) {
        unsafe {
            ReleaseMutex((self.0).0);
        }
    }
}

fn main() -> io::Result<()> {
    
    let cname = CString::new("ROBLOX_singletonMutex").unwrap();
    let cname_bytes = cname.as_bytes_with_nul();

    let mutex = unsafe { CreateMutexA(None, true, PCSTR::from_raw(cname_bytes.as_ptr() as *const u8)) }
    .unwrap_or_else(|_| panic!("failed to create global mutex"));

    unsafe {
        if GetLastError() == ERROR_ALREADY_EXISTS {
            panic!("Mutex is already exist");
             }
    }

    let mutex = Handle(mutex);
    
    match unsafe { WaitForSingleObject(mutex.0, INFINITE) } {
        WAIT_OBJECT_0 | WAIT_ABANDONED => (),
        err => panic!(
            "WaitForSingleObject failed on global mutex : {} (ret={:x})",
            io::Error::last_os_error(),
            err.0
        ),
    }
    
    let guard = Guard(mutex);

    println!("You can now open multiple ROBLOX clients.");
    println!("Closing this window will close all but one client.");
    println!("Press any key to close..");
    
    Getch::new().getch().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    drop(guard);
    Ok(())
    
}

