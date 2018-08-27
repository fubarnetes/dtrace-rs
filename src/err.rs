use libc::c_int;

/// client is requesting unsupported version
pub const EDT_VERSION: c_int = 1000;

/// version string is invalid or overflows
pub const EDT_VERSINVAL: c_int = 1001;

/// requested API version is not defined
pub const EDT_VERSUNDEF: c_int = 1002;

/// requested API version has been reduced
pub const EDT_VERSREDUCED: c_int = 1003;

/// libctf called failed (dt_ctferr has more)
pub const EDT_CTF: c_int = 1004;

/// error in D program compilation
pub const EDT_COMPILER: c_int = 1005;

/// tuple register allocation failure
pub const EDT_NOTUPREG: c_int = 1006;

/// memory allocation failure
pub const EDT_NOMEM: c_int = 1007;

/// integer limit exceeded
pub const EDT_INT2BIG: c_int = 1008;

/// string limit exceeded
pub const EDT_STR2BIG: c_int = 1009;

/// unknown module name
pub const EDT_NOMOD: c_int = 1010;

/// unknown provider name
pub const EDT_NOPROV: c_int = 1011;

/// unknown probe name
pub const EDT_NOPROBE: c_int = 1012;

/// unknown symbol name
pub const EDT_NOSYM: c_int = 1013;

/// no symbol corresponds to address
pub const EDT_NOSYMADDR: c_int = 1014;

/// unknown type name
pub const EDT_NOTYPE: c_int = 1015;

/// unknown variable name
pub const EDT_NOVAR: c_int = 1016;

/// unknown aggregation name
pub const EDT_NOAGG: c_int = 1017;

/// improper use of type name scoping operator
pub const EDT_BADSCOPE: c_int = 1018;

/// overspecified probe description
pub const EDT_BADSPEC: c_int = 1019;

/// bad macro variable in probe description
pub const EDT_BADSPCV: c_int = 1020;

/// invalid probe identifier
pub const EDT_BADID: c_int = 1021;

/// module is not currently loaded
pub const EDT_NOTLOADED: c_int = 1022;

/// module does not contain any CTF data
pub const EDT_NOCTF: c_int = 1023;

/// module and program data models don't match
pub const EDT_DATAMODEL: c_int = 1024;

/// library has newer DIF version than driver
pub const EDT_DIFVERS: c_int = 1025;

/// unrecognized aggregating action
pub const EDT_BADAGG: c_int = 1026;

/// file i/o error
pub const EDT_FIO: c_int = 1027;

/// invalid DIF program
pub const EDT_DIFINVAL: c_int = 1028;

/// invalid DIF size
pub const EDT_DIFSIZE: c_int = 1029;

/// failed to copyin DIF program
pub const EDT_DIFFAULT: c_int = 1030;

/// bad probe description
pub const EDT_BADPROBE: c_int = 1031;

/// bad probe description globbing pattern
pub const EDT_BADPGLOB: c_int = 1032;

/// declaration scope stack underflow
pub const EDT_NOSCOPE: c_int = 1033;

/// declaration stack underflow
pub const EDT_NODECL: c_int = 1034;

/// record list does not match statement
pub const EDT_DMISMATCH: c_int = 1035;

/// record data offset error
pub const EDT_DOFFSET: c_int = 1036;

/// record data alignment error
pub const EDT_DALIGN: c_int = 1037;

/// invalid dtrace_setopt option name
pub const EDT_BADOPTNAME: c_int = 1038;

/// invalid dtrace_setopt option value
pub const EDT_BADOPTVAL: c_int = 1039;

/// invalid dtrace_setopt option context
pub const EDT_BADOPTCTX: c_int = 1040;

/// failed to fork preprocessor
pub const EDT_CPPFORK: c_int = 1041;

/// failed to exec preprocessor
pub const EDT_CPPEXEC: c_int = 1042;

/// preprocessor not found
pub const EDT_CPPENT: c_int = 1043;

/// unknown preprocessor error
pub const EDT_CPPERR: c_int = 1044;

/// external symbol table overflow
pub const EDT_SYMOFLOW: c_int = 1045;

/// operation illegal when tracing is active
pub const EDT_ACTIVE: c_int = 1046;

/// destructive actions not allowed
pub const EDT_DESTRUCTIVE: c_int = 1047;

/// no anonymous tracing state
pub const EDT_NOANON: c_int = 1048;

/// can't claim anon state and enable probes
pub const EDT_ISANON: c_int = 1049;

/// END enablings exceed size of prncpl buffer
pub const EDT_ENDTOOBIG: c_int = 1050;

/// failed to load type for printf conversion
pub const EDT_NOCONV: c_int = 1051;

/// incomplete printf conversion
pub const EDT_BADCONV: c_int = 1052;

/// invalid library ERROR action
pub const EDT_BADERROR: c_int = 1053;

/// abort due to error
pub const EDT_ERRABORT: c_int = 1054;

/// abort due to drop
pub const EDT_DROPABORT: c_int = 1055;

/// abort explicitly directed
pub const EDT_DIRABORT: c_int = 1056;

/// invalid return value from callback
pub const EDT_BADRVAL: c_int = 1057;

/// invalid normalization
pub const EDT_BADNORMAL: c_int = 1058;

/// enabling exceeds size of buffer
pub const EDT_BUFTOOSMALL: c_int = 1059;

/// invalid truncation
pub const EDT_BADTRUNC: c_int = 1060;

/// device busy (active kernel debugger)
pub const EDT_BUSY: c_int = 1061;

/// insufficient privileges to use DTrace
pub const EDT_ACCESS: c_int = 1062;

/// dtrace device not available
pub const EDT_NOENT: c_int = 1063;

/// abort due to systemic unresponsiveness
pub const EDT_BRICKED: c_int = 1064;

/// failed to load hard-wired definitions
pub const EDT_HARDWIRE: c_int = 1065;

/// libelf is out-of-date w.r.t libdtrace
pub const EDT_ELFVERSION: c_int = 1066;

/// attempt to buffer output without handler
pub const EDT_NOBUFFERED: c_int = 1067;

/// description matched unstable set of probes
pub const EDT_UNSTABLE: c_int = 1068;

/// invalid setopt library action
pub const EDT_BADSETOPT: c_int = 1069;

/// invalid stack program counter size
pub const EDT_BADSTACKPC: c_int = 1070;

/// invalid aggregation variable identifier
pub const EDT_BADAGGVAR: c_int = 1071;

/// client is requesting deprecated version
pub const EDT_OVERSION: c_int = 1072;

/// failed to enable probe
pub const EDT_ENABLING_ERR: c_int = 1073;

/// no probes sites for declared provider
pub const EDT_NOPROBES: c_int = 1074;

/// failed to load a module
pub const EDT_CANTLOAD: c_int = 1075;

#[derive(Debug, PartialEq, Fail)]
pub enum Error {
    #[fail(display = "Invalid argument")]
    InvalidArgument,

    #[fail(
        display = "The value of the version argument is greater than the current DTrace version number."
    )]
    UnsupportedVersion,

    #[fail(
        display = "The value of the version argument is less than the minimum committed DTrace version number."
    )]
    UnsupportedOSVersion,

    #[fail(display = "The libelf library is out of date with respect to the libdtrace library.")]
    UnsupportedLibelfVersion,

    #[fail(display = "The DTrace device, /dev/dtrace is not available.")]
    DeviceNotAvailable,

    #[fail(
        display = "The DTrace device is busy because, for example, the kernel debugger is active."
    )]
    DeviceBusy,

    #[fail(display = "The user has insufficient privileges to use DTrace.")]
    InsufficientPrivileges,

    #[fail(display = "The system was unable to allocate memory while processing this function.")]
    MemoryAllocation,

    #[fail(display = "The libdtrace library uses a newer DIF version than the DTrace driver.")]
    UnsupportedDIFVersion,

    #[fail(display = "A call into the backend type system failed.")]
    TypeError,

    #[fail(display = "Other DTrace Error: {}", _0)]
    Other(c_int),
}
