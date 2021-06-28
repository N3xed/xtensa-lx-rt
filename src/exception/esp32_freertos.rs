use super::ExceptionCause;

pub type Context = freertos_esp32_sys::XtExcFrame;

extern "Rust" {
    /// This symbol will be provided by the user via `#[exception]`
    fn __exception(cause: ExceptionCause);
    /// No attribute is supplied for this symbol as the double exception can hardly occur
    fn __double_exception(cause: ExceptionCause);

    /// This symbol will be provided by the user via `#[interrupt(1)]`
    fn __level_1_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(2)]`
    fn __level_2_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(3)]`
    fn __level_3_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(4)]`
    fn __level_4_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(5)]`
    fn __level_5_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(6)]`
    fn __level_6_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(7)]`
    fn __level_7_interrupt(level: u32);
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_exception(cause: ExceptionCause, save_frame: &Context) {
    panic!("Exception: {:?}, {:#08x?}", cause, save_frame)
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_interrupt(level: u32, save_frame: &Context) {
    panic!("Interrupt: {:?}, {:#08x?}", level, save_frame)
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_double_exception(cause: ExceptionCause, save_frame: &Context) {
    panic!("Double Exception: {:?}, {:#08x?}", cause, save_frame)
}