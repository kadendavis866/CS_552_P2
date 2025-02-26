use signal_hook::consts::signal;
use signal_hook::iterator::Signals;
use std::sync::{Arc, Mutex};
use std::thread;

/// A struct to handle signal processing in the shell.
pub(crate) struct SignalHandler {
    /// A shared thread-safe flag to indicate whether signals should be ignored.
    ignore_signals: Arc<Mutex<bool>>,
}

impl SignalHandler {
    /// Creates a new `SignalHandler` instance. Only one instance should be created per process.
    ///
    /// # Returns
    /// A new `SignalHandler` with the `ignore_signals` flag set to `false`.
    pub(crate) fn new() -> SignalHandler {
        SignalHandler {
            ignore_signals: Arc::new(Mutex::new(false)),
        }
    }

    /// Starts the signal handler in a new thread.
    /// Listens for specific signals and raises them (to be handled normally)
    /// if no child process is running.
    pub(crate) fn start(&self) {
        // list of signals to handle
        let mut signals = Signals::new(&[
            signal::SIGINT,
            signal::SIGTERM,
            signal::SIGQUIT,
            signal::SIGTSTP,
            signal::SIGTTIN,
            signal::SIGTTOU,
        ])
        .expect("error creating signal handler");
        {
            let ignore_signals = Arc::clone(&self.ignore_signals);
            thread::spawn(move || {
                for sig in signals.forever() {
                    let child_running = *ignore_signals.lock().unwrap();
                    if !child_running {
                        match sig {
                            signal::SIGINT => unsafe {
                                libc::raise(libc::SIGINT);
                                break;
                            },
                            signal::SIGTERM => unsafe {
                                libc::raise(libc::SIGTERM);
                                break;
                            },
                            signal::SIGQUIT => unsafe {
                                libc::raise(libc::SIGQUIT);
                                break;
                            },
                            signal::SIGTSTP => unsafe {
                                libc::raise(libc::SIGTSTP);
                                break;
                            },
                            signal::SIGTTIN => unsafe {
                                libc::raise(libc::SIGTTIN);
                                break;
                            },
                            signal::SIGTTOU => unsafe {
                                libc::raise(libc::SIGTTOU);
                                break;
                            },
                            _ => (),
                        }
                    }
                }
            })
        };
    }

    /// Sets the `ignore_signals` flag to the specified value.
    ///
    /// # Arguments
    /// * `value` - A boolean value to set the `ignore_signals` flag.
    pub(crate) fn ignore_signals(&self, value: bool) {
        let mut child_exited = self.ignore_signals.lock().unwrap();
        *child_exited = value;
    }
}
