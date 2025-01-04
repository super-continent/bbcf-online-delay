use std::{
    ffi::{c_void, OsString},
    io::Read,
    os::windows::ffi::OsStringExt,
};

use retour::static_detour;
use windows::{
    core::{s, IUnknown, GUID, HRESULT, PCWSTR},
    Win32::{
        Foundation::{BOOL, E_FAIL, HINSTANCE, HMODULE},
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryW},
            SystemInformation::GetSystemDirectoryW,
        },
    },
};

const DELAY_FN_OFFSET: usize = 0x37C760;

static_detour! {
    static SetDelay: unsafe extern "C" fn(*mut u8, *mut u8, usize) -> usize;
}

static mut ONLINE_DELAY: usize = 1;

unsafe fn init() {
    std::panic::set_hook(Box::new(|panic_info| {
        let _ = std::fs::write(
            "BBCF-ONLINE-DELAY-PANIC.txt",
            format!("PANIC: {}", panic_info.to_string()),
        );
    }));

    simple_log::file("./bbcf_online_delay.log", "trace", 200, 10).expect("Couldn't initialize logging");

    const CONFIG_FILENAME: &str = "./BBCF_ONLINE_FRAME_DELAY.txt";

    ONLINE_DELAY = std::fs::File::open(CONFIG_FILENAME)
        .and_then(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            contents
                .trim()
                .parse::<usize>()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid number"))
        })
        .unwrap_or_else(|_| {
            // if either parsing fails or the file doesn't exist, we just create the default here
            std::fs::write(CONFIG_FILENAME, "1").expect("Couldn't create config file");
            1
        });

    let module_base = GetModuleHandleA(None).unwrap().0 as usize;
    let delay_fn = std::mem::transmute::<
        usize,
        unsafe extern "C" fn(*mut u8, *mut u8, usize) -> usize,
    >(module_base + DELAY_FN_OFFSET);

    SetDelay
        .initialize(delay_fn, delay_hook)
        .expect("Couldn't initialize SetDelay hook")
        .enable()
        .expect("Couldn't enable SetDelay hook");
}

fn delay_hook(ggpo: *mut u8, handle: *mut u8, delay: usize) -> usize {
    simple_log::log::debug!("handle: {:X}, original_delay: {}", handle as usize, delay);
    unsafe { SetDelay.call(ggpo, handle, ONLINE_DELAY) }
}

#[no_mangle]
extern "system" fn DllMain(_module: HINSTANCE, call_reason: u32, _reserved: *mut c_void) -> BOOL {
    match call_reason {
        1 => unsafe {
            std::thread::spawn(|| init());
        },
        _ => (),
    };

    true.into()
}

#[no_mangle]
pub unsafe extern "system" fn DirectInput8Create(
    inst_handle: HINSTANCE,
    version: u32,
    r_iid: *const GUID,
    ppv_out: *mut *mut c_void,
    p_unk_outer: *mut IUnknown,
) -> HRESULT {
    // type alias to make transmute cleaner
    type DInput8Create = extern "system" fn(
        HINSTANCE,
        u32,
        r_iid: *const GUID,
        *mut *mut c_void,
        *mut IUnknown,
    ) -> HRESULT;

    // Load real dinput8.dll if not already loaded
    let real_dinput8 = get_dinput8_handle();

    let dinput8_create = GetProcAddress(real_dinput8, s!("DirectInput8Create"));

    if !real_dinput8.is_invalid() && !dinput8_create.is_none() {
        let dinput8create_fn = std::mem::transmute::<_, DInput8Create>(dinput8_create.unwrap());
        return dinput8create_fn(inst_handle, version, r_iid, ppv_out, p_unk_outer);
    }

    E_FAIL // Unspecified failure
}

/// Get a handle to the real dinput8 library, if it fails it will return an invalid [`HINSTANCE`]
unsafe fn get_dinput8_handle() -> HMODULE {
    use windows::Win32::Foundation::MAX_PATH;

    const SYSTEM32_DEFAULT: &str = r"C:\Windows\System32";

    let mut buffer = [0u16; MAX_PATH as usize];
    let written_wchars = GetSystemDirectoryW(Some(&mut buffer));

    let system_directory = if written_wchars == 0 {
        SYSTEM32_DEFAULT.into()
    } else {
        // make sure path string does not contain extra trailing nulls
        let str_with_nulls = OsString::from_wide(&buffer)
            .into_string()
            .unwrap_or(SYSTEM32_DEFAULT.into());
        str_with_nulls.trim_matches('\0').to_string()
    };

    let dinput_path = system_directory + r"\dinput8.dll";

    LoadLibraryW(PCWSTR::from_raw(wstring(dinput_path).as_ptr())).unwrap_or(HMODULE::default())
}

#[no_mangle]
pub unsafe extern "system" fn ShowJoyCPL(_hwnd: windows::Win32::Foundation::HWND) {
    return;
}

fn wstring(s: String) -> Vec<u16> {
    let mut res: Vec<_> = s.encode_utf16().collect();
    res.push(0);

    res
}
