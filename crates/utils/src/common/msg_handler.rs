//! Port of utils/common/MsgHandler.{h,cpp}
//!
//! The three handler singletons live in static mutexes; use `write_message` / `write_warning` /
//! `write_error` (WRITE_MESSAGE / WRITE_WARNING / WRITE_ERROR), the `write_warningf!` family
//! (WRITE_WARNINGF, with aggregation) or `with_instance` for everything else.
//!
//! Retrievers are output device names ("stdout", "stderr" or a file path).
// shortcut: devices are a minimal table here (stdout, stderr, null, plain files) because
// utils/iodevices is not ported; route device_inform through OutputDevice::getDevice once it is
// (output-prefix, sockets, gz). The handler factory (GUI/TraCI) and gettext are not ported.
use super::std_defs::set_g_locale_initialized;
use super::string_utils;
use super::util_exceptions::ProcessError;
use crate::options::options_cont::with_options;
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    Message,
    Warning,
    Error,
    Debug,
    GlDebug,
}

pub struct MsgHandler {
    my_type: MsgType,
    was_informed: bool,
    aggregation_threshold: i32,
    aggregation_count: BTreeMap<String, i32>,
    retrievers: Vec<String>,
    initial_messages: Vec<String>,
}

static MESSAGE_INSTANCE: Mutex<Option<MsgHandler>> = Mutex::new(None);
static WARNING_INSTANCE: Mutex<Option<MsgHandler>> = Mutex::new(None);
static ERROR_INSTANCE: Mutex<Option<MsgHandler>> = Mutex::new(None);
static AM_PROCESSING_PROCESS: AtomicBool = AtomicBool::new(false);
static WRITE_DEBUG_MESSAGES: AtomicBool = AtomicBool::new(false);
static WRITE_DEBUG_GL_MESSAGES: AtomicBool = AtomicBool::new(false);
static WRITE_TIMESTAMPS: AtomicBool = AtomicBool::new(false);
static WRITE_PROCESS_ID: AtomicBool = AtomicBool::new(false);
// TL("Error: ") / TL("Warning: ") without translation
const ERROR_PREFIX: &str = "Error: ";
const WARNING_PREFIX: &str = "Warning: ";

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn slot(t: MsgType) -> &'static Mutex<Option<MsgHandler>> {
    match t {
        MsgType::Message => &MESSAGE_INSTANCE,
        MsgType::Warning => &WARNING_INSTANCE,
        MsgType::Error => &ERROR_INSTANCE,
        _ => panic!("only message, warning and error handlers exist"),
    }
}

/// getMessageInstance() / getWarningInstance() / getErrorInstance(), creating it on first use.
/// Do not call `with_instance` for the same type from inside `f` (the lock is not reentrant).
pub fn with_instance<R>(t: MsgType, f: impl FnOnce(&mut MsgHandler) -> R) -> R {
    let mut g = lock(slot(t));
    f(g.get_or_insert_with(|| MsgHandler::new(t)))
}

// ---------------------------------------------------------------------------
// minimal output devices
// ---------------------------------------------------------------------------
static DEVICES: Mutex<BTreeMap<String, Box<dyn Write + Send>>> = Mutex::new(BTreeMap::new());

/// Makes sure the named device exists (OutputDevice::getDevice).
pub fn get_device(name: &str) -> Result<(), ProcessError> {
    let mut devs = lock(&DEVICES);
    if devs.contains_key(name) {
        return Ok(());
    }
    let dev: Box<dyn Write + Send> = match name {
        "stdout" => Box::new(std::io::stdout()),
        "stderr" => Box::new(std::io::stderr()),
        "nul" | "NUL" | "/dev/null" => Box::new(std::io::sink()),
        // C++ OutputDevice buffers file writes until closeAll; keep the same
        // behaviour so a log file stays empty until the devices are closed.
        _ => Box::new(std::io::BufWriter::new(std::fs::File::create(name).map_err(|e| {
            ProcessError::IOError(format!("Could not build output file '{name}' ({e})."))
        })?)),
    };
    devs.insert(name.to_string(), dev);
    Ok(())
}

/// Registers a custom writer under a device name (for tests or later OutputDevice integration).
pub fn register_device(name: &str, dev: Box<dyn Write + Send>) {
    lock(&DEVICES).insert(name.to_string(), dev);
}

/// OutputDevice::inform
fn device_inform(name: &str, msg: &str, progress: bool) {
    if get_device(name).is_err() {
        return;
    }
    let mut devs = lock(&DEVICES);
    let dev = devs.get_mut(name).unwrap();
    let line = if progress { msg.to_string() } else { format!("{msg}\n") };
    let _ = dev.write_all(line.as_bytes());
    // no per-write flush: C++ OutputDevice::inform buffers and only flushes on closeAll
}

/// Flush all message devices (the log/message-log/error-log files).
/// Called by OutputDevice::close_all, matching C++ OutputDevice::closeAll.
pub fn flush_devices() {
    let mut devs = lock(&DEVICES);
    for dev in devs.values_mut() {
        let _ = dev.flush();
    }
}

impl MsgHandler {
    fn new(t: MsgType) -> Self {
        let mut h = MsgHandler {
            my_type: t,
            was_informed: false,
            aggregation_threshold: -1,
            aggregation_count: BTreeMap::new(),
            retrievers: Vec::new(),
            initial_messages: Vec::new(),
        };
        if t == MsgType::Message {
            h.add_retriever("stdout");
        } else {
            h.add_retriever("stderr");
        }
        h
    }

    pub fn enable_debug_messages(enable: bool) {
        WRITE_DEBUG_MESSAGES.store(enable, Relaxed);
    }

    pub fn enable_debug_gl_messages(enable: bool) {
        WRITE_DEBUG_GL_MESSAGES.store(enable, Relaxed);
    }

    pub fn write_debug_messages() -> bool {
        WRITE_DEBUG_MESSAGES.load(Relaxed)
    }

    pub fn write_debug_gl_messages() -> bool {
        WRITE_DEBUG_GL_MESSAGES.load(Relaxed)
    }

    pub fn insert_line_breaks(msg: &str, line_width: usize) -> String {
        if msg.len() <= line_width {
            return msg.to_string();
        }
        let mut msg = msg.as_bytes().to_vec();
        let find = |m: &[u8], c: u8, from: usize| m.iter().skip(from).position(|&x| x == c).map(|p| p + from);
        let mut next_line_break = find(&msg, b'\n', 0);
        let mut space_after_line = find(&msg, b' ', line_width);
        while let Some(sal) = space_after_line {
            let pos = if next_line_break.is_none_or(|n| n > sal) {
                msg[sal] = b'\n';
                sal + 1
            } else {
                next_line_break.unwrap() + 1
            };
            space_after_line = find(&msg, b' ', pos + line_width);
            next_line_break = find(&msg, b'\n', pos);
        }
        String::from_utf8_lossy(&msg).into_owned()
    }

    pub fn remove_retriever_from_all_instances(out: &str) {
        for t in [MsgType::Error, MsgType::Warning, MsgType::Message] {
            if let Some(h) = lock(slot(t)).as_mut() {
                h.remove_retriever(out);
            }
        }
    }

    /// without gettext only the prefixes are (re)set
    pub fn setup_i18n(_locale: &str) {
        set_g_locale_initialized(true);
    }

    pub fn init_output_options() -> Result<(), ProcessError> {
        get_device("stdout")?;
        get_device("stderr")?;
        let (aggregate, no_warnings, log, log_ts, log_pid, message_log, error_log, verbose) = with_options(|oc| {
            let s = |n: &str| if oc.is_set(n, false).unwrap() { Some(oc.get_string(n).unwrap()) } else { None };
            let log = s("log");
            (
                oc.get_int("aggregate-warnings").unwrap(),
                oc.get_bool("no-warnings").unwrap(),
                log.clone(),
                log.is_some() && oc.get_bool("log.timestamps").unwrap(),
                log.is_some() && oc.get_bool("log.processid").unwrap(),
                s("message-log"),
                s("error-log"),
                oc.get_bool("verbose").unwrap(),
            )
        });
        with_instance(MsgType::Warning, |h| h.set_aggregation_threshold(aggregate));
        with_instance(MsgType::Error, |h| h.set_aggregation_threshold(aggregate));
        if no_warnings {
            with_instance(MsgType::Warning, |h| h.remove_retriever("stderr"));
        }
        if let Some(log_file) = log {
            get_device(&log_file)?;
            with_instance(MsgType::Error, |h| h.add_retriever(&log_file));
            if !no_warnings {
                with_instance(MsgType::Warning, |h| h.add_retriever(&log_file));
            }
            with_instance(MsgType::Message, |h| h.add_retriever(&log_file));
            if log_ts {
                WRITE_TIMESTAMPS.store(true, Relaxed);
            }
            if log_pid {
                WRITE_PROCESS_ID.store(true, Relaxed);
            }
        }
        if let Some(log_file) = message_log {
            get_device(&log_file)?;
            with_instance(MsgType::Message, |h| h.add_retriever(&log_file));
        }
        if let Some(log_file) = error_log {
            get_device(&log_file)?;
            with_instance(MsgType::Error, |h| h.add_retriever(&log_file));
            with_instance(MsgType::Warning, |h| h.add_retriever(&log_file));
        }
        if verbose {
            with_instance(MsgType::Error, |h| h.initial_messages.push("Repeating initial error messages:".into()));
        } else {
            with_instance(MsgType::Message, |h| h.remove_retriever("stdout"));
        }
        Ok(())
    }

    pub fn cleanup_on_end() {
        for t in [MsgType::Message, MsgType::Warning, MsgType::Error] {
            *lock(slot(t)) = None;
        }
    }

    pub fn inform(&mut self, msg: &str, add_type: bool) {
        if add_type && !self.initial_messages.is_empty() && self.initial_messages.len() < 5 {
            self.initial_messages.push(msg.to_string());
        }
        // beautify progress output
        if AM_PROCESSING_PROCESS.load(Relaxed) {
            AM_PROCESSING_PROCESS.store(false, Relaxed);
            if self.my_type == MsgType::Message {
                self.inform("", true);
            } else {
                with_instance(MsgType::Message, |h| h.inform("", true));
            }
        }
        let msg = self.build(msg, add_type);
        for r in &self.retrievers {
            device_inform(r, &msg, false);
        }
        self.was_informed = true;
    }

    /// informf: the format string is the aggregation key
    pub fn informf(&mut self, format: &str, args: &[&dyn super::to_string::ToStringPrec]) {
        if !self.aggregation_threshold_reached(format) {
            let msg = string_utils::format(format, args);
            self.inform(&msg, true);
        }
    }

    pub fn begin_process_msg(&mut self, msg: &str, add_type: bool) {
        let msg = self.build(msg, add_type);
        for r in &self.retrievers {
            device_inform(r, &msg, true);
            AM_PROCESSING_PROCESS.store(true, Relaxed);
        }
        self.was_informed = true;
    }

    pub fn end_process_msg2(&mut self, success: bool, duration: i64) {
        if success {
            if duration > -1 {
                self.end_process_msg(&format!(" done ({duration}ms)."));
            } else {
                self.end_process_msg(" done.");
            }
        } else {
            self.end_process_msg(" failed.");
        }
    }

    pub fn end_process_msg(&mut self, msg: &str) {
        for r in &self.retrievers {
            device_inform(r, msg, false);
        }
        self.was_informed = true;
        AM_PROCESSING_PROCESS.store(false, Relaxed);
    }

    pub fn clear(&mut self, reset_informed: bool) {
        if self.aggregation_threshold >= 0 {
            let counts = std::mem::take(&mut self.aggregation_count);
            for (k, v) in &counts {
                if *v > self.aggregation_threshold {
                    self.inform(&format!("{v} total messages of type: {k}"), true);
                }
            }
        }
        self.aggregation_count.clear();
        if !reset_informed && self.initial_messages.len() > 1 {
            let was_informed = self.was_informed;
            for msg in std::mem::take(&mut self.initial_messages) {
                self.inform(&msg, false);
            }
            self.was_informed = was_informed;
        }
        if reset_informed {
            self.was_informed = false;
        }
    }

    pub fn add_retriever(&mut self, retriever: &str) {
        if !self.is_retriever(retriever) {
            self.retrievers.push(retriever.to_string());
        }
    }

    pub fn remove_retriever(&mut self, retriever: &str) {
        if let Some(i) = self.retrievers.iter().position(|r| r == retriever) {
            self.retrievers.remove(i);
        }
    }

    pub fn is_retriever(&self, retriever: &str) -> bool {
        self.retrievers.iter().any(|r| r == retriever)
    }

    pub fn was_informed(&self) -> bool {
        self.was_informed
    }

    pub fn set_aggregation_threshold(&mut self, thresh: i32) {
        self.aggregation_threshold = thresh;
    }

    pub fn get_aggregation_threshold(&self) -> i32 {
        self.aggregation_threshold
    }

    fn build_process_id_prefix(&self) -> String {
        format!("[PID: {}] ", std::process::id())
    }

    fn build(&self, msg: &str, add_type: bool) -> String {
        let mut prefix = String::new();
        if WRITE_TIMESTAMPS.load(Relaxed) {
            prefix += &format!("[{}] ", string_utils::iso_time_string(None));
        }
        if WRITE_PROCESS_ID.load(Relaxed) {
            prefix += &self.build_process_id_prefix();
        }
        if add_type {
            match self.my_type {
                MsgType::Message => {}
                MsgType::Warning => prefix += WARNING_PREFIX,
                MsgType::Error => prefix += ERROR_PREFIX,
                MsgType::Debug => prefix += "Debug: ",
                MsgType::GlDebug => prefix += "GLDebug: ",
            }
        }
        prefix + msg
    }

    fn aggregation_threshold_reached(&mut self, format: &str) -> bool {
        if self.aggregation_threshold < 0 {
            return false;
        }
        let c = self.aggregation_count.entry(format.to_string()).or_insert(0);
        let reached = *c >= self.aggregation_threshold;
        *c += 1;
        reached
    }
}

/// WRITE_WARNING(msg)
pub fn write_warning(msg: &str) {
    with_instance(MsgType::Warning, |h| h.inform(msg, true));
}
/// WRITE_MESSAGE(msg)
pub fn write_message(msg: &str) {
    with_instance(MsgType::Message, |h| h.inform(msg, true));
}
/// WRITE_ERROR(msg)
pub fn write_error(msg: &str) {
    with_instance(MsgType::Error, |h| h.inform(msg, true));
}
/// PROGRESS_BEGIN_MESSAGE(msg)
pub fn progress_begin_message(msg: &str) {
    with_instance(MsgType::Message, |h| h.begin_process_msg(&format!("{msg} ..."), true));
}
/// PROGRESS_DONE_MESSAGE()
pub fn progress_done_message() {
    with_instance(MsgType::Message, |h| h.end_process_msg2(true, -1));
}
/// PROGRESS_TIME_MESSAGE(before): `before` comes from SysUtils::getCurrentMillis()
pub fn progress_time_message(duration_ms: i64) {
    with_instance(MsgType::Message, |h| h.end_process_msg2(true, duration_ms));
}
/// PROGRESS_FAILED_MESSAGE()
pub fn progress_failed_message() {
    with_instance(MsgType::Message, |h| h.end_process_msg2(false, -1));
}

/// WRITE_WARNINGF(format, args...): '%' placeholders, aggregated by format string
#[macro_export]
macro_rules! write_warningf {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::common::msg_handler::with_instance($crate::common::msg_handler::MsgType::Warning, |h| {
            h.informf($fmt, &[$(&$arg as &dyn $crate::common::to_string::ToStringPrec),*])
        })
    };
}
/// WRITE_MESSAGEF(format, args...)
#[macro_export]
macro_rules! write_messagef {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::common::msg_handler::with_instance($crate::common::msg_handler::MsgType::Message, |h| {
            h.informf($fmt, &[$(&$arg as &dyn $crate::common::to_string::ToStringPrec),*])
        })
    };
}
/// WRITE_ERRORF(format, args...)
#[macro_export]
macro_rules! write_errorf {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::common::msg_handler::with_instance($crate::common::msg_handler::MsgType::Error, |h| {
            h.informf($fmt, &[$(&$arg as &dyn $crate::common::to_string::ToStringPrec),*])
        })
    };
}
/// TLF(format, args...) == StringUtils::format (no translation)
#[macro_export]
macro_rules! tlf {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        $crate::common::string_utils::format($fmt, &[$(&$arg as &dyn $crate::common::to_string::ToStringPrec),*])
    };
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::Arc;

    /// a device whose content can be read back
    #[derive(Clone, Default)]
    pub(crate) struct Capture(pub Arc<Mutex<Vec<u8>>>);
    impl Write for Capture {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            lock(&self.0).extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl Capture {
        pub(crate) fn text(&self) -> String {
            String::from_utf8(lock(&self.0).clone()).unwrap()
        }
    }

    #[test]
    fn prefix_and_aggregation() {
        // a private handler, so the global ones stay untouched by this test
        let cap = Capture::default();
        register_device("test-msg-handler", Box::new(cap.clone()));
        let mut h = MsgHandler::new(MsgType::Warning);
        h.remove_retriever("stderr");
        h.add_retriever("test-msg-handler");
        h.inform("plain", true);
        h.inform("raw", false);
        h.set_aggregation_threshold(1);
        h.informf("x %", &[&1]);
        h.informf("x %", &[&2]);
        h.informf("x %", &[&3]);
        assert!(h.was_informed());
        h.clear(true);
        assert!(!h.was_informed());
        assert_eq!(cap.text(), "Warning: plain\nraw\nWarning: x 1\nWarning: 3 total messages of type: x %\n");
    }

    #[test]
    fn repeats_collapse_into_total_after_threshold() {
        // C++ MsgHandler::aggregationThresholdReached: the first `threshold` repeats are printed,
        // clear() then reports "N total messages of type: <format>" for every key with N > threshold
        let cap = Capture::default();
        register_device("test-msg-agg", Box::new(cap.clone()));
        let mut h = MsgHandler::new(MsgType::Warning);
        h.remove_retriever("stderr");
        h.add_retriever("test-msg-agg");
        h.set_aggregation_threshold(5);
        for i in 0..6 {
            h.informf("Could not build program '%' for traffic light '%'", &[&String::from("0"), &i]);
        }
        h.clear(true);
        assert_eq!(
            cap.text(),
            "Warning: Could not build program '0' for traffic light '0'\n\
             Warning: Could not build program '0' for traffic light '1'\n\
             Warning: Could not build program '0' for traffic light '2'\n\
             Warning: Could not build program '0' for traffic light '3'\n\
             Warning: Could not build program '0' for traffic light '4'\n\
             Warning: 6 total messages of type: Could not build program '%' for traffic light '%'\n"
        );
    }

    #[test]
    fn every_line_is_newline_terminated() {
        let cap = Capture::default();
        register_device("test-msg-nl", Box::new(cap.clone()));
        let mut h = MsgHandler::new(MsgType::Warning);
        h.remove_retriever("stderr");
        h.add_retriever("test-msg-nl");
        h.inform("first", true);
        h.inform("second", true);
        assert_eq!(cap.text(), "Warning: first\nWarning: second\n");
    }

    #[test]
    fn line_breaks() {
        assert_eq!(MsgHandler::insert_line_breaks("aaa bbb ccc", 3), "aaa\nbbb\nccc");
        assert_eq!(MsgHandler::insert_line_breaks("short", 10), "short");
    }
}
