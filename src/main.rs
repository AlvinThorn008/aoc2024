fn main() {
    println!("Hello, world!");
}

use std::time::Instant;

// Run function and return result with seconds duration
pub fn time<F, T>(f: F) -> (T, f64)
  where F: FnOnce() -> T {
  let start = Instant::now();
  let res = f();
  let end = Instant::now();

  let runtime_nanos = (end - start).as_nanos();
//   .expect("Benchmark iter took greater than 2^63 nanoseconds");
  let runtime_secs = runtime_nanos as f64 / 1_000_000_000.0;
  (res, runtime_secs)
}