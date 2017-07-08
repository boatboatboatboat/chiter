#[macro_export]
/// Creates a function from an address where:
/// $addy = function address
/// $res = return type
/// $(arg)* = argument types
/// Example:
/// ```rust
/// // in C: int foo() { return 1; }
/// make_func!(address_of_foo, isize, )
/// // in C: int bar(int baz, int foobar) { return baz + foobar; }
/// make_func!(address_of_bar, isize, isize, isize)
/// ```
macro_rules! make_func {
    ($addy:expr, $res:ty, $($arg:ty),*) => {
        std::mem::transmute::<*const usize, fn($($arg,)*) -> $res>($addy as *const usize)
    }
}

// an enum for endianess
pub enum Endian { Big, Little }

pub struct memory {
    end: Endian
}

impl Memory {
    /// Returns a struct thing with all the functions, prepared w/ endianess.
    pub fn new(endian: Endian) -> Memory {
        Memory {end: endian}
    }
    // Reads "length" amount of bytes starting at address.
    pub fn read_bytes(&self, address: usize, length: usize) -> Vec<u8> {
        let mut result = Vec::<u8>::new(); //TURBO
        let mut i; // we know we're going to index it rn.
        match self.end {
            Endian::Big => {
                i = 0;
                while i <= length {
                    unsafe { result.push(*((address + i) as *const u8)) };
                    i += 1;
                }
            },
            Endian::Little => {
                i = 1;
                while i <= length {
                    unsafe { result.push(*((address + length - i) as *const u8)) };
                    i += 1;
                }
            }
        }
        result
    }
    // Writes bytes, starting at address.
    pub fn write_bytes(&self, address: usize, bytes: &[u8]) {
        let length = bytes.len();
        let mut i;
        match self.end {
            Endian::Big => {
                i = 1; // this should fix it LOL
                for byte in bytes.into_iter() {
                    println!("GenAddr: {:x}", unsafe { *((address + i) as *mut u8) });
                    println!("GenByte: {:x}", *byte);
                    unsafe { *((address + i) as *mut u8) = *byte };
                    i += 1;
                }
            },
            Endian::Little => {
                i = 1;
                for byte in bytes.into_iter() {
                    unsafe { *((address + length - i) as *mut u8) = *byte };
                    i += 1;
                }
            }
        }
    }
    /// Searches for a matching pattern between the addresses "from" and "to".
    pub fn search(&self, pattern: &[u8], from: usize, to: usize, wildcard: u8) -> usize {
        if from >= to {
            panic!("the from address is higher than the to address");
        }
        let length = pattern.len() - 1;
        for position in from..to {
            let bytes = self.read_bytes(position, length);
            let mut p = 0;
            for byte in &bytes {
                if pattern[p] == wildcard {
                    p += 1;
                    continue
                }
                if byte != &pattern[p] {
                    break
                } else if p == length {
                    return position - 1
                }
                p += 1;
            }
        }
        0
    }
}
