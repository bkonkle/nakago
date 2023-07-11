use std::{
    io,
    panic::{self, PanicInfo},
    pin::Pin,
};

use backtrace::Backtrace;
use crossterm::{execute, style::Print};
use futures::Future;

use crate::inject;

pub fn init_handle_panic<'a>(
    i: &'a mut inject::Inject,
) -> Pin<Box<dyn Future<Output = inject::Result<()>> + 'a>> {
    Box::pin(async move {
        // Process setup
        panic::set_hook(Box::new(handle_panic));

        Ok(())
    })
}

fn handle_panic(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        execute!(
            io::stdout(),
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            ))
        )
        .unwrap();
    }
}
