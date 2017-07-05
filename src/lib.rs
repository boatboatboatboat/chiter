#[macro_export]
macro_rules! make_func {
    ($addy:expr, $res:ty, $($arg:ty),*) => {
        std::mem::transmute::<*const usize, fn($($arg,)*) -> $res>($addy as *const usize)
    }
}

pub fn read_bytes(address: usize, length: usize) -> Vec<u8> {
    let mut res = Vec::<u8>::new(); //T-T-TURBOFISH
    let mut i = 1;
    while i <= length {
        unsafe{res.push(*((address + (length - i)) as *const u8))};
        i += 1;
    };
    res
}

pub fn read_usize(address: usize) -> usize {
    let res = unsafe { *((address as usize) as *mut usize) };
    res
}

pub fn write_bytes(address: usize, bytes: &[u8]) {
    let mut i = 1;
    let length = bytes.len();
    for byte in bytes.into_iter() {
        unsafe { *((address + (length - i) as usize) as *mut u8) = *byte };
        i += 1;
    }
}

pub fn write_usize(address: usize, writeme: usize) {
    unsafe { *((address as usize) as *mut usize) = writeme };
}


pub fn search(pattern: &[u8], from: usize, to: usize, wildcard: u8) -> usize {
    if from >= to {
        panic!("the from address is higher than the to address");
    }
    for position in from..to {
        let ressy = read_bytes(position, pattern.len());
        let mut p = 0;
        for res_byte in &ressy {
            if pattern[p] == wildcard {
                p += 1;
                continue // Wildcard, just skip.
            }
            if res_byte != &pattern[p] {
                break // Pattern does not match.
            } else if p == pattern.len() - 1 {
                return position // We found it!
            }
            p += 1;
        }
    }
    0 // Return 0 if we don't find anything :c
}
