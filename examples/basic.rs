extern crate dtrace;

fn main() {
    let trace = dtrace::DTrace::open().expect("Could not open the dtrace library");
    trace.probes();
}
