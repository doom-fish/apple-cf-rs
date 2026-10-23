import Dispatch
import Foundation

private final class DataAddSourceHolder {
    private enum State {
        case suspended
        case active
        case cancelled
    }

    private let lock = NSLock()
    private let source: DispatchSourceUserDataAdd
    private var state = State.suspended

    init(
        queue: DispatchQueue,
        context: UnsafeMutableRawPointer,
        event: @escaping @convention(c) (UnsafeMutableRawPointer?, UInt64) -> Void,
        release: @escaping @convention(c) (UnsafeMutableRawPointer?) -> Void
    ) {
        source = DispatchSource.makeUserDataAddSource(queue: queue)
        source.setEventHandler { [weak self] in
            guard let self else { return }
            self.lock.lock()
            let active = self.state == .active
            self.lock.unlock()
            if active {
                event(context, UInt64(self.source.data))
            }
        }
        source.setCancelHandler {
            release(context)
        }
    }

    deinit {
        cancel()
    }

    func resume() {
        lock.lock()
        guard state == .suspended else {
            lock.unlock()
            return
        }
        state = .active
        source.activate()
        lock.unlock()
    }

    func cancel() {
        lock.lock()
        guard state != .cancelled else {
            lock.unlock()
            return
        }
        let needsActivation = state == .suspended
        state = .cancelled
        source.cancel()
        if needsActivation {
            source.activate()
        }
        lock.unlock()
    }

    func mergeData(_ value: UInt64) {
        guard value != 0 else { return }
        lock.lock()
        if state != .cancelled {
            source.add(data: UInt(value))
        }
        lock.unlock()
    }
}

@_cdecl("acf_dispatch_source_data_add_is_main_thread")
public func acf_dispatch_source_data_add_is_main_thread() -> Bool {
    Thread.isMainThread
}

@_cdecl("acf_dispatch_source_data_add_create")
public func acf_dispatch_source_data_add_create(
    _ queue: UnsafeMutableRawPointer,
    _ context: UnsafeMutableRawPointer,
    _ event: (@convention(c) (UnsafeMutableRawPointer?, UInt64) -> Void)?,
    _ release: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
) -> UnsafeMutableRawPointer? {
    guard let event, let release else { return nil }
    let queue = Unmanaged<DispatchQueue>.fromOpaque(queue).takeUnretainedValue()
    return Unmanaged.passRetained(
        DataAddSourceHolder(queue: queue, context: context, event: event, release: release)
    ).toOpaque()
}

@_cdecl("acf_dispatch_source_data_add_resume")
public func acf_dispatch_source_data_add_resume(_ source: UnsafeMutableRawPointer) {
    Unmanaged<DataAddSourceHolder>.fromOpaque(source).takeUnretainedValue().resume()
}

@_cdecl("acf_dispatch_source_data_add_cancel")
public func acf_dispatch_source_data_add_cancel(_ source: UnsafeMutableRawPointer) {
    Unmanaged<DataAddSourceHolder>.fromOpaque(source).takeUnretainedValue().cancel()
}

@_cdecl("acf_dispatch_source_data_add_merge")
public func acf_dispatch_source_data_add_merge(_ source: UnsafeMutableRawPointer, _ value: UInt64) {
    Unmanaged<DataAddSourceHolder>.fromOpaque(source).takeUnretainedValue().mergeData(value)
}
