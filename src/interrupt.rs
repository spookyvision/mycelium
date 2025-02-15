use core::marker::PhantomData;
use hal_core::interrupt::{self, ctx};

pub struct Handlers {
    _p: (),
}

// TODO(eliza): ag.
static mut TIMER: usize = 0;

impl interrupt::Handlers for Handlers {
    fn page_fault<C>(cx: C)
    where
        C: ctx::Context + ctx::PageFault,
    {
        tracing::error!(registers = ?cx.registers(), "page fault");
        loop {}
    }

    fn code_fault<C>(cx: C)
    where
        C: ctx::Context + ctx::CodeFault,
    {
        tracing::error!(kind = ?cx.kind(), registers = ?cx.registers(), "code fault");
        loop {}
    }

    fn timer_tick() {
        let timer = unsafe {
            TIMER += 1;
            TIMER
        };
        let seconds_hand = timer % 8;
        match seconds_hand {
            0 => {
                tracing::trace!("timer tick");
            }
            4 => {
                tracing::trace!("timer tock");
            }
            _ => {}
        }
    }

    fn keyboard_controller(scancode: u8) {
        tracing::info!(
            // for now
            "got scancode {}. the time is now: {}",
            scancode,
            unsafe { TIMER }
        );
    }

    fn test_interrupt<C>(cx: C)
    where
        C: ctx::Context,
    {
        tracing::info!(registers=?cx.registers(), "lol im in ur test interrupt");
    }
}
