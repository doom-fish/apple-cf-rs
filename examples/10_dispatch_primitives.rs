use apple_cf::dispatch_queue::{DispatchGroup, DispatchSemaphore, DispatchSource};
use std::thread;
use std::time::Duration;

fn main() {
    let group = DispatchGroup::new();
    group.enter();
    group.leave();
    assert!(group.wait(Some(Duration::from_millis(10))));

    let semaphore = DispatchSemaphore::new(0);
    assert_eq!(semaphore.signal(), 0);
    assert!(semaphore.wait(Some(Duration::from_millis(10))));

    let source = DispatchSource::timer(Duration::from_millis(5), Duration::from_millis(1));
    source.resume();
    thread::sleep(Duration::from_millis(25));
    source.cancel();
    assert!(source.fire_count() > 0);
}
