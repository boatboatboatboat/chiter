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

///A macro to interop with c++ virtual funcs
/// 
/// Supplied by balen (https://www.unknowncheats.me/forum/members/1860455.html)
/// 
/// # Example:
/// ```rust
///  create_function_definitions! {
///  ("thiscall" trace_ray[5] -> bool;(ray, &mut Ray);(mask, u32);(filter, &mut TraceFilter);(trace, &mut Trace))
/// }
///  
/// pub fn new() -> Self {
/// let address = create_interface("engine.dll", "EngineTraceClient004");
///
///  Self {
/// this_ptr: ThisPtr::new(address),
///        }
/// }
/// ```
#[macro_export]
macro_rules! create_function_definitions {
        ($(($calling_convention:tt $visible:vis $function_name:ident[$index:literal] -> $return_type:ty;$(($param_name:ident,$param_type:ty));*)),*) =>{
        $(
            $visible fn $function_name(&self, $($param_name: $param_type),*) -> $return_type{
        unsafe {
        use ::winapi::shared::minwindef::LPVOID;
        use ::std::mem::transmute;
        use crate::util::get_vfunc_address;
        if $calling_convention == "thiscall" {
            return transmute::<LPVOID, extern $calling_convention fn(LPVOID, $($param_type),*) -> $return_type>(get_vfunc_address(self.this_ptr.get(), $index))(self.this_ptr.get(), $($param_name),*);
        }

        transmute::<LPVOID, extern $calling_convention fn($($param_type),*) -> $return_type>(get_vfunc_address(self.this_ptr.get(), $index))($($param_name),*)
    }
    }
        )*

    };

    }

/// A macro to convert multiple pointers into functions
///
/// take note: untested
///
/// # Example:
/// ```c
/// // This code is in C.
/// int add_one(int thing) {
///     return thing + 1;
/// }
///
/// int get_random_number() {
///     return 4;
/// }
/// ```
/// ```rust
/// // This code is in Rust.
/// // 0xDEADBEEF is the address where add_one starts,
/// // and 0x0DEADBEEF + 70 is the address where get_random_number starts.
///
/// let add_one;
/// let get_random_number;
///
/// unsafe {
///     make_functions! {
///         0xDEADBEEF; fn add_one(i32) -> i32;
///         0xDEADBEEF + 70; fn get_random_number() -> i32
///     }
/// }
///
/// assert_eq!(add_one(400), 401);
/// assert_eq!(get_random_number(), 4);
/// ```
// TO-DO: fix "unexpected end of macro invocation" when ending with a ;
#[macro_export]
macro_rules! make_functions {
    ( $( $address:expr; fn $fn_name:ident( $($argument:ty),* ) -> $returntype:ty);* ;) => {
        $(
            $fn_name = std::mem::transmute::<*const usize, fn( $($argument),* ) -> $returntype>($address as *const usize);
        );*
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
    };
}
