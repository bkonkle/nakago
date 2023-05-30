use std::{
    io,
    panic::{self, PanicInfo},
};

use async_trait::async_trait;
use backtrace::Backtrace;
use crossterm::{execute, style::Print};

use crate::{Hook, InjectResult};

/// An Init Hook for handling panics
#[derive(Default)]
pub struct InitHandlePanic {}

#[async_trait]
impl Hook for InitHandlePanic {
    async fn handle(&self, _i: &mut crate::Inject) -> InjectResult<()> {
        // Process setup
        panic::set_hook(Box::new(handle_panic));

        Ok(())
    }
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
