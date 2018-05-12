/// A macro to convert a pointer into a function
///
/// # Example:
/// ```c
/// // This code is in C.
/// int add_one(int thing) {
///     return thing + 1;
/// }
/// ```
/// ```rust
/// // This code is in Rust. 0xDEADBEEF is the address where add_one starts.
/// let add_one = unsafe { make_fn!(0xDEADBEEF, i32, i32) };
///
/// assert_eq!(add_one(400), 401);
/// ```
#[macro_export]
macro_rules! make_fn {
    ($address:expr, $returntype:ty) => {
        std::mem::transmute::<*const usize, fn() -> $returntype>($address as *const usize)
    };
    ($address:expr, $returntype:ty, $($argument:ty),*) => {
        std::mem::transmute::<*const usize, fn($($argument,)*) -> $returntype>($address as *const usize)
    }
}

/// A macro to write rust-usable pointers in a somewhat nicer way
///
/// # Example:
///
/// ```rust
/// ptr!(0xDEADBEEF, u8) = 255
/// ```
#[macro_export]
macro_rules! ptr {
    ($address:expr, $type:ty) => {
        *($address as *mut $type)
    }
}

pub enum SearchError {
    NotFound,
    FromGreaterThanTo
}

/// Reads 'length' bytes starting at 'address', returns a Vec<u8> with all the bytes.
pub fn read_bytes(address: usize, length: usize) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    for index in (0..length).rev() {
        unsafe { result.push(ptr!(address + index, u8)) }
    }
    result
}

/// Writes 'bytes' starting at 'address'
pub fn write_bytes(address: usize, bytes: &[u8]) {
    for (index, byte) in bytes.into_iter().enumerate() {
        unsafe { ptr!(address + index, u8) = *byte};
    }
}

/// Searches for a pattern and stops at the first occurence, where:
///
/// ``pattern`` is the pattern to search for,
///
/// ``from`` is the address to start searching from,
///
/// ``to`` is the address to stop searching at,
///
/// and ``wildcard`` is the byte in pattern to ignore.
pub fn search_first(pattern: &[u8], from: usize, to: usize, wildcard: u8) -> Result<usize, SearchError>  {
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
                continue
            }
            if byte != &pattern[p] {
                break
            } else if p == length {
                return Ok(position)
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
pub fn search(pattern: &[u8], from: usize, to: usize, wildcard: u8) -> Result<Vec<usize>, SearchError> {
    if from > to {
        return Err(SearchError::FromGreaterThanTo);
    }
    let mut result = Vec::<usize>::new();
    let mut start = from;
    loop {
        if let Ok(position) = search_first(pattern.clone(), start, to, wildcard) {
            result.push(position);
            start = position + 1;
        } else {
            return Ok(result);
        }
    }
}