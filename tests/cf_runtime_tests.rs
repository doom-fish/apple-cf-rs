use apple_cf::cf::{
    CFFileDescriptor, CFMessagePort, CFNotificationCenter, CFRunLoop, CFRunLoopRunResult, CFSocket,
    CFStreamPair, CFString, CFTimer,
};
use apple_cf::cf::CFDictionary;
use std::io::{Read, Write};
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn cf_runtime_wrappers_work() {
    let center = CFNotificationCenter::local();
    center.post(
        &CFString::new("com.doomfish.apple-cf.runtime-tests"),
        None,
        false,
    );

    let timer = CFTimer::new(Duration::from_millis(10), false);
    let run_loop = CFRunLoop::current();
    run_loop.add_timer(&timer);
    assert!(timer.is_valid());
    let result = CFRunLoop::run_in_default_mode(Duration::from_millis(20), true);
    assert!(matches!(
        result,
        CFRunLoopRunResult::Finished
            | CFRunLoopRunResult::Stopped
            | CFRunLoopRunResult::TimedOut
            | CFRunLoopRunResult::HandledSource
    ));

    let name = format!("com.doomfish.apple-cf.echo.test.{}", process::id());
    let _local = CFMessagePort::create_echo_local(&name).expect("local port");
    let remote = CFMessagePort::connect_remote(&name).expect("remote port");
    let reply = remote
        .send_request(b"ping", Duration::from_millis(100))
        .expect("reply");
    assert_eq!(reply, b"ping");

    let pair = CFStreamPair::new(1024);
    assert!(pair.read.open());
    assert!(pair.write.open());
    assert_eq!(pair.write.write(b"ok").expect("write"), 2);
    let mut buffer = [0_u8; 2];
    assert_eq!(pair.read.read(&mut buffer).expect("read"), 2);
    assert_eq!(&buffer, b"ok");
    pair.read.close();
    pair.write.close();

    let socket = CFSocket::udp_ipv4().expect("socket");
    assert!(socket.is_valid());
    assert!(socket.native() >= 0);

    let stdin = std::io::stdin();
    let fd = CFFileDescriptor::from_borrowed_fd(std::os::fd::AsFd::as_fd(&stdin))
        .expect("file descriptor");
    assert_eq!(fd.native_descriptor(), 0);
}

#[test]
fn message_port_names_with_nul_are_rejected() {
    assert!(CFMessagePort::create_echo_local("com.doomfish.apple-cf\0echo").is_none());
    assert!(CFMessagePort::connect_remote("com.doomfish.apple-cf\0echo").is_none());
}

#[test]
fn owned_file_descriptor_is_closed_when_the_last_reference_drops() {
    let (local, mut peer) = UnixStream::pair().expect("socket pair");
    peer.set_read_timeout(Some(Duration::from_secs(5)))
        .expect("read timeout");
    let descriptor = CFFileDescriptor::from_owned_fd(OwnedFd::from(local)).expect("descriptor");
    let clone = descriptor.clone();
    drop(descriptor);
    drop(clone);
    let mut buffer = [0_u8; 1];
    assert_eq!(peer.read(&mut buffer).expect("peer read"), 0);
}

#[test]
fn owned_file_descriptor_is_closed_on_invalidate() {
    let (local, mut peer) = UnixStream::pair().expect("socket pair");
    peer.set_read_timeout(Some(Duration::from_secs(5)))
        .expect("read timeout");
    let descriptor = CFFileDescriptor::from_owned_fd(OwnedFd::from(local)).expect("descriptor");
    descriptor.invalidate();
    let mut buffer = [0_u8; 1];
    assert_eq!(peer.read(&mut buffer).expect("peer read"), 0);
}

#[test]
fn borrowed_file_descriptor_is_never_closed() {
    let (mut local, mut peer) = UnixStream::pair().expect("socket pair");
    let descriptor = CFFileDescriptor::from_borrowed_fd(local.as_fd()).expect("descriptor");
    assert_eq!(descriptor.native_descriptor(), local.as_raw_fd());
    descriptor.invalidate();
    drop(descriptor);
    local.write_all(b"x").expect("write");
    let mut buffer = [0_u8; 1];
    peer.read_exact(&mut buffer).expect("read");
    assert_eq!(&buffer, b"x");
}

#[test]
fn local_notification_observer_receives_posts_until_dropped() {
    let center = CFNotificationCenter::local();
    let name = CFString::new(&format!(
        "com.doomfish.apple-cf.observer.{}",
        process::id()
    ));
    let received = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&received);
    let observer = center.add_observer(&name, move |name, user_info| {
        sink.lock()
            .expect("sink")
            .push((name.to_string(), user_info.map(CFDictionary::len)));
    });
    let key = CFString::new("key");
    let value = CFString::new("value");
    let user_info = CFDictionary::from_pairs(&[(&key, &value)]);
    center.post(&name, Some(&user_info), true);
    center.post(&name, None, true);
    drop(observer);
    center.post(&name, None, true);
    assert_eq!(
        *received.lock().expect("received"),
        vec![(name.to_string(), Some(1)), (name.to_string(), None)]
    );
}

#[test]
fn panicking_notification_observer_does_not_unwind_into_core_foundation() {
    let center = CFNotificationCenter::local();
    let name = CFString::new(&format!(
        "com.doomfish.apple-cf.panicking-observer.{}",
        process::id()
    ));
    let calls = Arc::new(Mutex::new(0_u32));
    let counter = Arc::clone(&calls);
    let _observer = center.add_observer(&name, move |_, _| {
        *counter.lock().expect("counter") += 1;
        panic!("observer panic");
    });
    center.post(&name, None, true);
    center.post(&name, None, true);
    assert_eq!(*calls.lock().expect("calls"), 2);
}
