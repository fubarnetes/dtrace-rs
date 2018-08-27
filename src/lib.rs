#[macro_use]
extern crate failure;

extern crate libc;
use libc::{c_int, c_void};

mod err;

pub use err::Error;

use std::ffi::CString;
use std::ptr;

/// DTrace library version we are targeting
const DTRACE_VERSION: c_int = 3;

/// Opaque pointer representing the DTrace library handle
type DTraceHandle = *mut c_void;

/// Callback function type for probes
type DTraceProbeCallback =
    unsafe extern "C" fn(DTraceHandle, *const DTraceProbeDesc, *mut c_void) -> c_int;

#[link(name = "dtrace")]
extern "C" {
    fn dtrace_open(version: c_int, flags: c_int, errp: *mut c_int) -> DTraceHandle;
    fn dtrace_close(dtp: DTraceHandle);
    fn dtrace_probe_iter(dtp: DTraceHandle, pdp: *const DTraceProbeDesc, callback: DTraceProbeCallback, arg: *mut c_void);
}

/// Represents a handle on the DTrace library.
pub struct DTrace {
    dtp: DTraceHandle,
}

type DTraceID = u32;

const DTRACE_PROVNAMELEN: usize = 64;
const DTRACE_MODNAMELEN: usize = 64;
const DTRACE_FUNCNAMELEN: usize = 192;
const DTRACE_NAMELEN: usize = 64;

#[repr(C)]
struct DTraceProbeDesc {
    /// Probe Identifier
    dtpd_id: DTraceID,

    /// Probe provider name
    dtpd_provider: [u8; DTRACE_PROVNAMELEN],

    /// Probe module name
    dtpd_mod: [u8; DTRACE_MODNAMELEN],

    /// Probe function name
    dtpd_func: [u8; DTRACE_FUNCNAMELEN],

    /// Probe name
    dtpd_name: [u8; DTRACE_NAMELEN],
}

/// Helper function to convert the first non-nul bytes and convert them into a CString:
fn cstring_from_bytes(bytes: &[u8]) -> CString {
    let bytes: Vec<u8> = bytes
        .to_vec();

    let bytes: Vec<u8> = bytes
        .into_iter()
        .take_while(|c| *c != 0)
        .collect();

    CString::new(bytes).expect("Could not create CString")
}

impl DTrace {
    /// Open a handle to the DTrace library
    ///
    /// # Example
    /// ```
    /// use dtrace::DTrace;
    /// let trace = DTrace::open().unwrap();
    /// ```
    ///
    // FIXME: Pass in flags.
    pub fn open() -> Result<DTrace, Error> {
        let mut err: c_int = 0;
        let flags: c_int = 0;

        match unsafe { dtrace_open(DTRACE_VERSION, flags, &mut err) } {
            dtp if !dtp.is_null() => Ok(DTrace { dtp }),
            _ => Err(match err {
                22 => Error::InvalidArgument,
                err::EDT_VERSION => Error::UnsupportedVersion,
                err::EDT_OVERSION => Error::UnsupportedOSVersion,
                err::EDT_ELFVERSION => Error::UnsupportedLibelfVersion,
                err::EDT_NOENT => Error::DeviceNotAvailable,
                err::EDT_BUSY => Error::DeviceBusy,
                err::EDT_ACCESS => Error::InsufficientPrivileges,
                err::EDT_NOMEM => Error::MemoryAllocation,
                err::EDT_DIFVERS => Error::UnsupportedDIFVersion,
                e => Error::Other(e),
            }),
        }
    }

    // This function gets called once for each probe iterated over.
    //
    // FIXME: Make this pass things into an iterator through the opaque arg.
    extern "C" fn probe_iter_fn(dtp: DTraceHandle, pdp: *const DTraceProbeDesc, arg: *mut c_void) -> c_int {
        let provider = cstring_from_bytes(unsafe{&(*pdp).dtpd_provider});
        let func = cstring_from_bytes(unsafe{&(*pdp).dtpd_func});
        let module = cstring_from_bytes(unsafe{&(*pdp).dtpd_mod});
        let name = cstring_from_bytes(unsafe{&(*pdp).dtpd_name});
        println!("probe: {:?} {:?} {:?} {:?}", provider, module, func, name);
        0
    }

    /// Iterate over all probes
    ///
    /// # Example
    /// ```
    /// use dtrace::DTrace;
    /// let trace = DTrace::open().unwrap();
    /// trace.probes();
    /// ```
    ///
    // FIXME: make this return an iterator
    pub fn probes(&self) {
        unsafe { dtrace_probe_iter(self.dtp, ptr::null(), Self::probe_iter_fn, ptr::null_mut()) };
    }
}

impl Drop for DTrace {
    fn drop(&mut self) {
        unsafe { dtrace_close(self.dtp) };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
