#[cfg(test)]
mod macro_tests {
    use crate as chiter;
    #[test]
    fn test_functions() {
        let add_one;
        let get_random_number;
        unsafe {
            chiter::make_functions! {
                0xDEADBEEF; fn add_one(i32) -> i32;
                0xDEADBEEFu32 + 70; fn get_random_number() -> i32;
            }
        }
    }
}

mod memory;

#[cfg(target_os = "windows")]
use kernel32::VirtualProtect;

#[rustversion::nightly]
#[cfg(feature = "detour")]
use detour::{Function, GenericDetour};

pub enum SearchError {
    NotFound,
    FromGreaterThanTo,
}

/// Reads 'length' bytes starting at 'address', returns a Vec<u8> with all the bytes.
pub fn read_bytes(address: usize, length: usize) -> Vec<u8> {
    read_object::<Vec<u8>>(address, length)
}

/// Writes 'bytes' starting at 'address'
pub fn write_bytes(address: usize, bytes: &[u8]) {
    write_object(address, bytes.to_vec())
}

///writes an Value to the specified address
///
/// # Arguments
///
/// * `address` Address of the starting point of the Value
/// * `Object` An object that implements Into<Vec<u8>>
///
/// # Example
/// ```
/// //remember, sample_object can be any type that implements Into<Vec<u8>>
/// write_object(0x6473AA4, sample_object)
/// ```
pub fn write_object<T: Into<Vec<u8>>>(address: usize, object: T) {
    let vector = object.into();
    for (idx, byte) in vector.into_iter().enumerate() {
        unsafe { ptr!(address + idx, u8) = byte };
    }
}

///Reads an Value from the specified address
///
/// # Arguments
///
/// * `address` Address of the starting point of the Value
/// * `length` Size of the Value. Note that this is not actually the size of the value, but the size of bytes needed for the From::from conversion
///
/// # Example
/// ```
/// let data_array = read_object<Vec<u8>>(0x6473AA4, 24);
/// ```
pub fn read_object<T: From<Vec<u8>>>(address: usize, length: usize) -> T {
    let mut result = Vec::<u8>::new();
    for index in (0..length).rev() {
        unsafe { result.push(ptr!(address + index, u8)) }
    }
    T::from(result)
}

/// Searches for a pattern and stops at the first occurrence, where:
///
/// ``pattern`` is the pattern to search for,
///
/// ``from`` is the address to start searching from,
///
/// ``to`` is the address to stop searching at,
///
/// and ``wildcard`` is the byte in pattern to ignore.
pub fn search_first(
    pattern: &[u8],
    from: usize,
    to: usize,
    wildcard: u8,
) -> Result<usize, SearchError> {
    if from > to {
        return Err(SearchError::FromGreaterThanTo);
    }
    let length = pattern.len() - 1;
    for position in from..to {
        let bytes = read_bytes(position, length);
        let mut p = 0;
        for byte in &bytes {
            if pattern[p] == wildcard {
                p += 1;
                continue;
            }
            if byte != &pattern[p] {
                break;
            } else if p == length {
                return Ok(position);
            }
            p += 1;
        }
    }
    Err(SearchError::NotFound)
}

/// Searches for a pattern and returns all occurrences.
///
/// Read search_first for further information.
///
/// Take note that the _last_ occurrence is at the top of the vector.
pub fn search(
    pattern: &[u8],
    from: usize,
    to: usize,
    wildcard: u8,
) -> Result<Vec<usize>, SearchError> {
    if from > to {
        return Err(SearchError::FromGreaterThanTo);
    }
    let mut result = Vec::<usize>::new();
    let mut start = from;
    loop {
        if let Ok(position) = search_first(pattern, start, to, wildcard) {
            result.push(position);
            start = position + 1;
        } else {
            if result.len() == 0 {
                return Err(SearchError::NotFound);
            }
            return Ok(result);
        }
    }
}

#[cfg(windows)]
pub struct VTable<'a> {
    //Location of the VTable
    adress: usize,
    //Count of entries in VTable
    size: usize,
    //Internal representation of the Vtableentries
    representation: &'a mut [usize],
}

#[cfg(windows)]
impl<'a> VTable<'a> {
    ///Creates a new VTable-instance for Windows
    ///
    /// ```adress``` is the adress of the vtable, make sure to resolve the pointer to the vtable and not just pass the class inst
    /// ```size``` is the amount of functions held in the vtable
    pub fn new(adress: usize, size: usize) -> VTable<'a> {
        VTable {
            adress: adress,
            size: size,
            representation: unsafe { std::slice::from_raw_parts_mut(adress as *mut usize, size) },
        }
    }

    ///Swaps a vtable entry at the specified index
    /// ```index``` is the index the targeted function is at
    /// ```to_replace``` is a pointer to the function you would to inject
    /// returns the adress of the original function, so you can call it
    pub fn hook(&mut self, index: usize, to_replace: usize) -> Result<usize, std::string::String> {
        if index >= self.size {
            let error_msg: std::string::String = format!(
                "Tried to access out of bound index {} while max was {}",
                index,
                self.size - 1
            );
            return Err(error_msg);
        }

        const PAGE_EXECUTE_READWRITE: u32 = 64;
        let mut old_protect = 0u32;
        let mut new_protect = 0u32;
        //Allowing to write to vtable
        unsafe {
            VirtualProtect(
                self.adress as *mut std::ffi::c_void,
                0x400,
                PAGE_EXECUTE_READWRITE,
                &mut old_protect,
            );
        }
        let orig_adress = self.representation[index];

        self.representation[index] = to_replace;

        unsafe {
            VirtualProtect(
                self.adress as *mut std::ffi::c_void,
                0x400,
                old_protect,
                &mut new_protect,
            );
        }

        Ok(orig_adress)
    }
}

#[rustversion::nightly]
#[cfg(feature = "detour")]
pub struct Hook<T: Function> {
    wrapped_detour: GenericDetour<T>,
}

#[rustversion::nightly]
#[cfg(feature = "detour")]
impl<T: Function> Hook<T> {
    unsafe fn create_new(orig: T, new: T) -> Hook<T> {
        let mut created = Hook::<T> {
            wrapped_detour: GenericDetour::<T>::new(orig, new).unwrap(),
        };
        created.wrapped_detour.enable().unwrap();
        created
    }
}
