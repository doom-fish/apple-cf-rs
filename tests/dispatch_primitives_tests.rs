use apple_cf::dispatch_queue::{
    dispatch_apply, dispatch_async, dispatch_async_and_wait, DispatchGroup, DispatchQoS,
    DispatchQueue, DispatchSemaphore, DispatchSource,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[test]
fn dispatch_sync_primitives_work() {
    let group = DispatchGroup::new();
    group.enter();
    group.leave();
    assert!(group.wait(Some(Duration::from_millis(10))));

    let semaphore = DispatchSemaphore::new(0);
    assert_eq!(semaphore.signal(), 0);
    assert!(semaphore.wait(Some(Duration::from_millis(10))));

    let source = DispatchSource::timer(Duration::from_millis(5), Duration::from_millis(1));
    source.resume();
    thread::sleep(Duration::from_millis(30));
    source.cancel();
    assert!(source.fire_count() > 0);

    let queue = DispatchQueue::new(
        "com.doomfish.apple-cf.dispatch-primitives-tests",
        DispatchQoS::UserInitiated,
    );
    let counter = Arc::new(AtomicUsize::new(0));
    let async_group = DispatchGroup::new();
    async_group.enter();
    let async_group_done = async_group.clone();
    let async_counter = Arc::clone(&counter);
    dispatch_async(&queue, move || {
        async_counter.fetch_add(1, Ordering::SeqCst);
        async_group_done.leave();
    });
    assert!(async_group.wait(Some(Duration::from_secs(1))));

    let waited_counter = Arc::clone(&counter);
    dispatch_async_and_wait(&queue, move || {
        waited_counter.fetch_add(1, Ordering::SeqCst);
    });
    assert_eq!(counter.load(Ordering::SeqCst), 2);

    let total = Arc::new(AtomicUsize::new(0));
    let total_clone = Arc::clone(&total);
    dispatch_apply(4, &queue, move |index| {
        total_clone.fetch_add(index + 1, Ordering::SeqCst);
    });
    assert_eq!(total.load(Ordering::SeqCst), 10);
}

#[test]
fn dispatch_source_lifecycle_is_balanced_and_idempotent() {
    drop(DispatchSource::timer(
        Duration::from_millis(5),
        Duration::from_millis(1),
    ));

    let cancelled_before_resume =
        DispatchSource::timer(Duration::from_millis(5), Duration::from_millis(1));
    cancelled_before_resume.cancel();
    cancelled_before_resume.cancel();
    cancelled_before_resume.resume();
    drop(cancelled_before_resume);

    let source = DispatchSource::timer(Duration::from_millis(1), Duration::from_millis(1));
    let resume_threads: Vec<_> = (0..8)
        .map(|_| {
            let source = source.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    source.resume();
                }
            })
        })
        .collect();
    for thread in resume_threads {
        thread.join().expect("resume thread");
    }

    thread::sleep(Duration::from_millis(20));
    let retained = source.clone();
    drop(source);
    assert!(retained.fire_count() > 0);

    let cancel_threads: Vec<_> = (0..8)
        .map(|_| {
            let source = retained.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    source.cancel();
                }
            })
        })
        .collect();
    for thread in cancel_threads {
        thread.join().expect("cancel thread");
    }

    retained.cancel();
    retained.resume();
}

#[test]
fn dispatch_source_fire_count_is_monotonic_across_threads() {
    let source = DispatchSource::timer(Duration::from_millis(1), Duration::from_millis(1));
    source.resume();

    let readers: Vec<_> = (0..4)
        .map(|_| {
            let source = source.clone();
            thread::spawn(move || {
                let mut previous = 0;
                for _ in 0..1_000 {
                    let current = source.fire_count();
                    assert!(current >= previous);
                    previous = current;
                }
            })
        })
        .collect();

    for reader in readers {
        reader.join().expect("fire-count reader");
    }
    source.cancel();
}
