use core::fmt;

pub mod ctx;
pub use self::ctx::Context;

/// An interrupt controller for a platform.
pub trait Control {
    type Registers: fmt::Debug + fmt::Display;

    /// Disable all interrupts.
    ///
    /// # Safety
    ///
    /// This may cause a fault if called when interrupts are already disabled
    /// (depending on the platform). It does not guarantee that interrupts will
    /// ever be unmasked.
    unsafe fn disable(&mut self);

    /// Enable all interrupts.
    ///
    /// # Safety
    ///
    /// This may cause a fault if called when interrupts are already enabled
    /// (depending on the platform).
    unsafe fn enable(&mut self);

    /// Returns `true` if interrupts are enabled.
    fn is_enabled(&self) -> bool;

    fn register_handlers<H>(&mut self) -> Result<(), RegistrationError>
    where
        H: Handlers<Self::Registers>;

    /// Enter a critical section, returning a guard.
    fn enter_critical(&mut self) -> CriticalGuard<'_, Self> {
        unsafe {
            self.disable();
        }
        CriticalGuard { ctrl: self }
    }
}

pub trait Handlers<R: fmt::Debug + fmt::Display> {
    fn page_fault<C>(cx: C)
    where
        C: ctx::Context<Registers = R> + ctx::PageFault;

    fn code_fault<C>(cx: C)
    where
        C: ctx::Context<Registers = R> + ctx::CodeFault;

    fn double_fault<C>(cx: C)
    where
        C: ctx::Context<Registers = R>;

    fn timer_tick();

    fn keyboard_controller();

    fn test_interrupt<C>(_cx: C)
    where
        C: ctx::Context<Registers = R>,
    {
        // nop
    }
}

/// Errors that may occur while registering an interrupt handler.
#[derive(Clone, Eq, PartialEq)]
pub struct RegistrationError {
    kind: RegistrationErrorKind,
}

#[derive(Debug)]
pub struct CriticalGuard<'a, C: Control + ?Sized> {
    ctrl: &'a mut C,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RegistrationErrorKind {
    Nonexistant,
    AlreadyRegistered,
    Other(&'static str),
}

// === impl CriticalGuard ===

impl<'a, C: Control + ?Sized> Drop for CriticalGuard<'a, C> {
    fn drop(&mut self) {
        unsafe {
            self.ctrl.enable();
        }
    }
}

// === impl RegistrationError ===
impl RegistrationError {
    /// Returns a new error indicating that the registered interrupt vector does
    /// not exist.
    pub fn nonexistant() -> Self {
        Self {
            kind: RegistrationErrorKind::Nonexistant,
        }
    }

    /// Returns a new error indicating that the registered interrupt vector has
    /// already been registered and cannot be registered again.
    pub fn already_registered() -> Self {
        Self {
            kind: RegistrationErrorKind::AlreadyRegistered,
        }
    }

    /// Returns a new platform-specific error with the provided message.
    pub fn other(message: &'static str) -> Self {
        Self {
            kind: RegistrationErrorKind::Other(message),
        }
    }

    pub fn is_nonexistant(&self) -> bool {
        matches!(self.kind, RegistrationErrorKind::Nonexistant)
    }

    pub fn is_already_registered(&self) -> bool {
        matches!(self.kind, RegistrationErrorKind::AlreadyRegistered)
    }
}

impl fmt::Debug for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistrationError")
            .field("kind", &self.kind)
            .finish()
    }
}
