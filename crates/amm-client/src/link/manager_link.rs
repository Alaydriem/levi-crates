use std::ffi::c_void;
use std::path::Path;

use amm_abi::types::{AmmCallbacks, AmmCapability, AmmHandle, AmmStatus};
use amm_core::errors::LinkError;
use amm_core::structs::descriptor::Capability;
use amm_core::structs::event::ModEvent;
use amm_core::structs::lease::AcquireOutcome;
use amm_core::structs::manager::TextureGrant;
use curia::{Dispatcher, InstallError, Level, Logger, TracingBridge};
use serde::de::DeserializeOwned;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[cfg(feature = "egui")]
use super::FrameHandler;
use super::{ApiTable, EventDrain, ForwardSink};
#[cfg(feature = "egui")]
use crate::bridge::{EguiFrame, FrameTrampoline};

/// A mod's registration with the manager. A link made with a frame handler owns the boxed frame state the
/// manager's callbacks use; a link without a frame owns no frame state. Drop unregisters, then frees the frame
/// state when there is one.
pub struct ManagerLink {
    table: ApiTable,
    handle: AmmHandle,
    // The frame state the callbacks take as `user`, null for a link without a frame
    user: *mut c_void,
    // Frees `user`; `None` for a link without a frame
    free: Option<unsafe fn(*mut c_void)>,
    drain: EventDrain,
    sink: ForwardSink,
}

// SAFETY: the frame state behind `user`, when there is one, holds an `H: Send` and is reached only by the
// manager's render callbacks and by `Drop`, which runs after unregister; the rest of the link is plain data and a
// shared table
unsafe impl Send for ManagerLink {}

impl ManagerLink {
    // The manager filters by its per-mod level, so the mod side forwards every level.
    const LOG_LEVEL: Level = Level::Trace;
    // Room for the longest mod id the descriptor checker accepts, so one call is enough
    const HOLDER_ID_BYTES: usize = 64;

    /// Registers the mod from its descriptor text, with `handler` as its egui pass, and installs Curia in this
    /// DLL with a sink that forwards to the manager. Curia already being installed counts as success.
    /// `config_dir` is the mod's own LeviLamina config folder, an absolute path; the manager keeps the mod's
    /// `config.hcl` there. A path that is not UTF-8 is `InvalidArgument`.
    #[cfg(feature = "egui")]
    pub fn new<H: FrameHandler + Send + 'static>(
        table: ApiTable,
        descriptor_hcl: &'static str,
        config_dir: &Path,
        handler: H,
    ) -> Result<ManagerLink, LinkError> {
        let user = FrameTrampoline::<H>::into_user(EguiFrame::new(handler));
        let callbacks = FrameTrampoline::<H>::callbacks(user);
        Self::register(table, descriptor_hcl, config_dir, callbacks, Some(FrameTrampoline::<H>::free))
    }

    /// Registers the mod from its descriptor text with no frame and no textures-lost callback, for a mod that
    /// draws nothing through the manager. Logging, `config_dir` and errors are as for `new`. Drop unregisters and
    /// frees nothing.
    pub fn without_frame(
        table: ApiTable,
        descriptor_hcl: &'static str,
        config_dir: &Path,
    ) -> Result<ManagerLink, LinkError> {
        let callbacks = AmmCallbacks { user: std::ptr::null_mut(), frame: None, textures_lost: None };
        Self::register(table, descriptor_hcl, config_dir, callbacks, None)
    }

    // Registers with `callbacks` and installs logging. On any error the manager holds no copy of
    // `callbacks.user`, so `free`, when given, frees it here once.
    fn register(
        table: ApiTable,
        descriptor_hcl: &'static str,
        config_dir: &Path,
        callbacks: AmmCallbacks,
        free: Option<unsafe fn(*mut c_void)>,
    ) -> Result<ManagerLink, LinkError> {
        let handle = match Self::register_handle(&table, descriptor_hcl, config_dir, &callbacks) {
            Ok(handle) => handle,
            Err(error) => {
                if let Some(free) = free {
                    // SAFETY: the manager refused the mod or was never called, so it holds no copy of user,
                    // which is freed here once
                    unsafe { free(callbacks.user) };
                }
                return Err(error);
            }
        };
        let sink = ForwardSink::new(table, handle, Self::LOG_LEVEL);
        Self::install_logging(&sink);
        Ok(Self { table, handle, user: callbacks.user, free, drain: EventDrain::new(), sink })
    }

    fn register_handle(
        table: &ApiTable,
        descriptor_hcl: &'static str,
        config_dir: &Path,
        callbacks: &AmmCallbacks,
    ) -> Result<AmmHandle, LinkError> {
        let Some(register) = table.api().register else {
            return Err(LinkError::from(AmmStatus::Unsupported));
        };
        let Some(config_dir) = config_dir.to_str() else {
            return Err(LinkError::from(AmmStatus::InvalidArgument));
        };
        let mut handle = AmmHandle { slot: u16::MAX, generation: u16::MAX };
        // SAFETY: the descriptor text, the config folder text, the callbacks and the handle all live for the call;
        // the manager copies the callbacks and writes the handle only on success
        let status = unsafe {
            register(
                descriptor_hcl.as_ptr(),
                descriptor_hcl.len(),
                config_dir.as_ptr(),
                config_dir.len(),
                callbacks,
                &mut handle,
            )
        };
        match status {
            AmmStatus::Ok => Ok(handle),
            other => Err(LinkError::from(other)),
        }
    }

    /// A forwarding sink for this mod's log events. It sends through the same handle as the link, and stops when
    /// the link is dropped.
    pub fn sink(&self) -> ForwardSink {
        self.sink.clone()
    }

    /// Asks for a lease on `capability`.
    pub fn acquire(&self, capability: Capability) -> Result<AcquireOutcome, LinkError> {
        let Some(acquire) = self.table.api().acquire else {
            return Err(LinkError::from(AmmStatus::Unsupported));
        };
        // SAFETY: acquire takes plain values; the handle is this link's until drop
        let result = unsafe { acquire(self.handle, AmmCapability::from(capability) as u8) };
        Ok(AcquireOutcome::from(result))
    }

    /// Gives a lease back.
    pub fn release(&self, capability: Capability) {
        if let Some(release) = self.table.api().release {
            // SAFETY: release takes plain values; the handle is this link's until drop
            unsafe { release(self.handle, AmmCapability::from(capability) as u8) };
        }
    }

    /// The id of the mod that holds `capability` now, or `None` when no mod holds it.
    pub fn holder(&self, capability: Capability) -> Option<String> {
        let holder = self.table.api().holder?;
        let capability = AmmCapability::from(capability) as u8;
        let mut buffer = [0u8; Self::HOLDER_ID_BYTES];
        // SAFETY: the buffer has room for its length in bytes for the call
        let len = unsafe { holder(capability, buffer.as_mut_ptr(), buffer.len()) } as usize;
        if len <= buffer.len() {
            return Self::id_text(&buffer[..len]);
        }
        let mut long = vec![0u8; len];
        // SAFETY: the buffer has room for its length in bytes for the call
        let again = unsafe { holder(capability, long.as_mut_ptr(), long.len()) } as usize;
        if again > long.len() {
            return None;
        }
        Self::id_text(&long[..again])
    }

    /// The events queued for this mod, in order. The iterator reads a reused buffer of `EventDrain::CAPACITY`
    /// events; call again to drain the rest.
    pub fn events(&mut self) -> impl Iterator<Item = ModEvent> + '_ {
        self.drain.poll(&self.table, self.handle)
    }

    /// The mod's checked `config.hcl` parsed into its own type.
    pub fn config<T: DeserializeOwned>(&self) -> Result<T, LinkError> {
        let Some(config_document) = self.table.api().config_document else {
            return Err(LinkError::from(AmmStatus::Unsupported));
        };
        let mut ptr = std::ptr::null();
        let mut len = 0;
        // SAFETY: both out pointers live for the call; the handle is this link's until drop
        let status = unsafe { config_document(self.handle, &mut ptr, &mut len) };
        if status != AmmStatus::Ok {
            return Err(LinkError::from(status));
        }
        let bytes: &[u8] = if ptr.is_null() || len == 0 {
            &[]
        } else {
            // SAFETY: the manager owns the text and keeps it valid until the next config_document call
            unsafe { std::slice::from_raw_parts(ptr, len) }
        };
        let text = std::str::from_utf8(bytes).map_err(|error| LinkError::Json { reason: error.to_string() })?;
        hcl::from_str(text).map_err(|error| LinkError::Json { reason: error.to_string() })
    }

    /// Changes one of the mod's own saved values at `path`.
    pub fn set_setting(&self, path: &str, value: &hcl::Value) -> Result<(), LinkError> {
        let Some(set_setting) = self.table.api().set_setting else {
            return Err(LinkError::from(AmmStatus::Unsupported));
        };
        let expression =
            hcl::format::to_string(value).map_err(|error| LinkError::Json { reason: error.to_string() })?;
        // SAFETY: path and the expression text live for the call; the handle is this link's until drop
        let status =
            unsafe { set_setting(self.handle, path.as_ptr(), path.len(), expression.as_ptr(), expression.len()) };
        match status {
            AmmStatus::Ok => Ok(()),
            other => Err(LinkError::from(other)),
        }
    }

    /// Whether `vk` is down. The manager answers only while this mod holds the `Input` lease.
    pub fn key_down(&self, vk: u32) -> bool {
        match self.table.api().key_down {
            // SAFETY: key_down takes plain values; the handle is this link's until drop
            Some(key_down) => unsafe { key_down(self.handle, vk) },
            None => false,
        }
    }

    /// Turns the cursor mode on or off.
    pub fn set_cursor_mode(&self, on: bool) -> Result<(), LinkError> {
        let Some(set_cursor_mode) = self.table.api().set_cursor_mode else {
            return Err(LinkError::from(AmmStatus::Unsupported));
        };
        // SAFETY: set_cursor_mode takes plain values; the handle is this link's until drop
        unsafe { set_cursor_mode(self.handle, u8::from(on)) };
        Ok(())
    }

    /// Asks for a screenshot of the next Present, written as a PNG to `path`, and returns its sequence, which the
    /// `ScreenshotTaken` and `ScreenshotWritten` events carry. `path` is an absolute `.png` file path whose parent
    /// folder exists; the mod must hold an active `Hud` or `Overlay` lease. A path that is not UTF-8 is
    /// `InvalidArgument`; a manager older than ABI 1.2 is `Unsupported`.
    pub fn request_screenshot(&self, path: &Path) -> Result<u32, LinkError> {
        let Some(request_screenshot) = self.table.api().request_screenshot else {
            return Err(LinkError::from(AmmStatus::Unsupported));
        };
        let Some(path) = path.to_str() else {
            return Err(LinkError::from(AmmStatus::InvalidArgument));
        };
        let mut sequence = 0;
        // SAFETY: the path text and the out sequence live for the call; the handle is this link's until drop
        let status = unsafe { request_screenshot(self.handle, path.as_ptr(), path.len(), &mut sequence) };
        match status {
            AmmStatus::Ok => Ok(sequence),
            other => Err(LinkError::from(other)),
        }
    }

    /// True when the manager's table has `request_screenshot`, ABI 1.2 or later. A mod can check it before it
    /// offers a screenshot key.
    pub fn supports_screenshots(&self) -> bool {
        self.table.api().request_screenshot.is_some()
    }

    /// True when a mod other than this one holds `capability` now. False on a manager older than ABI 1.2. Call it
    /// on the game thread: from a frame callback (render thread) the host is busy and the answer is false.
    pub fn held_by_other(&self, capability: Capability) -> bool {
        let Some(held_by_other) = self.table.api().held_by_other else {
            return false;
        };
        // SAFETY: held_by_other takes plain values; the handle is this link's until drop
        let held = unsafe { held_by_other(self.handle, AmmCapability::from(capability) as u8) };
        held == 1
    }

    /// The texture budget the manager gave this mod. A manager older than ABI 1.2 is `Unsupported`.
    pub fn texture_budget(&self) -> Result<TextureGrant, LinkError> {
        let Some(texture_budget) = self.table.api().texture_budget else {
            return Err(LinkError::from(AmmStatus::Unsupported));
        };
        let (mut bytes, mut count) = (0, 0);
        // SAFETY: both out pointers live for the call; the handle is this link's until drop
        let status = unsafe { texture_budget(self.handle, &mut bytes, &mut count) };
        match status {
            AmmStatus::Ok => Ok(TextureGrant { bytes, count }),
            other => Err(LinkError::from(other)),
        }
    }

    // The id text of a holder, or `None` for an empty or non-UTF-8 id.
    fn id_text(bytes: &[u8]) -> Option<String> {
        if bytes.is_empty() {
            return None;
        }
        std::str::from_utf8(bytes).ok().map(str::to_owned)
    }

    fn install_logging(sink: &ForwardSink) {
        match Logger::install(Box::new(Dispatcher::new(vec![sink.clone()]))) {
            Ok(()) | Err(InstallError::AlreadyInstalled) => {}
        }
        match TracingBridge::install_log_capture() {
            Ok(()) | Err(InstallError::AlreadyInstalled) => {}
        }
        // The only failure is a global subscriber already set by this DLL, which counts as installed
        if tracing_subscriber::registry().with(TracingBridge::to_global()).try_init().is_err() {
            curia::debug!("tracing subscriber already set, kept as it is");
        }
    }
}

impl Drop for ManagerLink {
    fn drop(&mut self) {
        let Some(unregister) = self.table.api().unregister else {
            // The manager could still call the callbacks, so the frame state is left alive
            curia::error!("manager table has no unregister, the registration and its frame state, if any, are kept");
            self.sink.detach();
            return;
        };
        // SAFETY: the handle is this link's, and drop runs once
        let status = unsafe { unregister(self.handle) };
        if status != AmmStatus::Ok {
            // The registration may still be live, so the manager could still call the callbacks: the frame state
            // is leaked rather than freed under them
            curia::error!("unregister refused, the registration and its frame state, if any, are kept", { status: status as i32 });
            self.sink.detach();
            return;
        }
        self.sink.detach();
        if let Some(free) = self.free {
            // SAFETY: unregister returned Ok, so the manager no longer calls the callbacks, and user is freed here
            // once
            unsafe { free(self.user) };
        }
    }
}
