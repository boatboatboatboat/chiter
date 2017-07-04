#[macro_export]
macro_rules! make_func {
    ($addy:expr, $res:ty, $($arg:ty),*) => {
        std::mem::transmute::<*const u32, fn($($arg,)*) -> $res>($addy as *const u32)
    }
}

pub fn read_bytes(address: u32, length: u8) -> Vec<u8> {
    let mut res = Vec::<u8>::new(); //T-T-TURBOFISH
    let mut i = 0;
    while i < length {
        unsafe{res.push(*((address + i as u32) as *const u8))};
        i += 1;
    };
    res
}

pub fn read_u32(address: u32) -> u32 {
    let res = unsafe { *((address as u32) as *mut u32) };
    res
}

pub fn write_bytes(address: u32, bytes: Vec<u8>) {
    let mut i = 0;
    for byte in bytes.into_iter() {
        unsafe { *((address + i as u32) as *mut u8) = byte };
        i += 1;
    }
}

pub fn write_u32(address: u32, writeme: u32) {
    unsafe { *((address as u32) as *mut u32) = writeme };
}

pub fn scan_pattern(pattern: String, from: u32, to: u32) -> u32 {
    unimplemented!();
    1
}
