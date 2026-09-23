use core::fmt;

use spin::LazyLock;
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};

use spin::Mutex;

pub static SERIAL: LazyLock<Mutex<Uart16550Tty<PioBackend>>> = LazyLock::new(|| {
    let serial_port = unsafe {
        Uart16550Tty::new_port(0x3F8, Config::default())
            .expect("should initialize serial device from valid config and valid port")
    };
    Mutex::new(serial_port)
});

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Zapis do portu szeregowego nie powiódł się");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
