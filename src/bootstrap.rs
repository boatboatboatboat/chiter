/// A macro to create a DLL-Entrypoint for Windowsbinaries
/// It takes a function to call after the injection
///
/// # Example:
/// ```rust
/// fn injected(){
///     ...
/// }
/// make_entrypoint!(injected);
/// ```
#[cfg(windows)]
#[macro_export]
macro_rules! make_entrypoint {
    ($fn:expr) => {
        use winapi::shared::minwindef::HINSTANCE;
        #[no_mangle]
        pub extern "stdcall" fn DllMain(_hinst_dll: HINSTANCE, fdw_reason: u32, _: *mut c_void) {
            if fdw_reason == 1 {
                thread::spawn($fn);
            }
        }
    };
}

/// A macro to create a DLL-Entrypoint for Linuxbinaries
/// It takes a function to call after the injection
/// The function prototype must be extern "C" fn()
///
/// # Example:
/// ```rust
/// pub extern "C" fn injected() {
///     ...
/// }
/// make_entrypoint!(injected);
/// ```
/// Taken from https://github.com/oberien/refunct-tas/blob/master/rtil/src/native/linux/mod.rs#L13-L17
#[cfg(linux)]
#[macro_export]
macro_rules! make_entrypoint {
    ($fn:expr) => {
        #[link_section = ".init_array"]
        pub static INITIALIZE_CTOR: extern "C" fn() = $fn;
    };
}
