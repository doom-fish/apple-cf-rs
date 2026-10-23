// CoreGraphics Bridge - CGRect, CGSize, CGPoint, CGImage

import CoreGraphics
import Foundation
import ImageIO
import UniformTypeIdentifiers

// MARK: - CGRect Bridge

public struct CGRectBridge {
    public var x: Double
    public var y: Double
    public var width: Double
    public var height: Double

    public init(x: Double, y: Double, width: Double, height: Double) {
        self.x = x
        self.y = y
        self.width = width
        self.height = height
    }

    public init(rect: CGRect) {
        x = Double(rect.origin.x)
        y = Double(rect.origin.y)
        width = Double(rect.size.width)
        height = Double(rect.size.height)
    }

    public func toCGRect() -> CGRect {
        CGRect(x: x, y: y, width: width, height: height)
    }
}

// MARK: - CGSize Bridge

public struct CGSizeBridge {
    public var width: Double
    public var height: Double

    public init(width: Double, height: Double) {
        self.width = width
        self.height = height
    }

    public init(size: CGSize) {
        width = Double(size.width)
        height = Double(size.height)
    }

    public func toCGSize() -> CGSize {
        CGSize(width: width, height: height)
    }
}

// MARK: - CGPoint Bridge

public struct CGPointBridge {
    public var x: Double
    public var y: Double

    public init(x: Double, y: Double) {
        self.x = x
        self.y = y
    }

    public init(point: CGPoint) {
        x = Double(point.x)
        y = Double(point.y)
    }

    public func toCGPoint() -> CGPoint {
        CGPoint(x: x, y: y)
    }
}

// MARK: - CGImage Bridge

@_cdecl("cgimage_get_width")
public func getCGImageWidth(_ image: OpaquePointer) -> Int {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()
    return cgImage.width
}

@_cdecl("cgimage_get_height")
public func getCGImageHeight(_ image: OpaquePointer) -> Int {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()
    return cgImage.height
}

@_cdecl("cgimage_release")
public func releaseCGImage(_ image: OpaquePointer) {
    Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).release()
}

@_cdecl("cgimage_retain")
public func retainCGImage(_ image: OpaquePointer) -> OpaquePointer {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()
    return OpaquePointer(Unmanaged.passRetained(cgImage).toOpaque())
}

@_cdecl("cgimage_render_rgba_into")
public func renderCGImageRGBAInto(
    _ image: OpaquePointer,
    _ destBuffer: UnsafeMutableRawPointer,
    _ destCapacity: Int
) -> Int {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()

    let width = cgImage.width
    let height = cgImage.height
    let bytesPerPixel = 4 // RGBA
    let (bytesPerRow, rowOverflow) = width.multipliedReportingOverflow(by: bytesPerPixel)
    let (totalBytes, totalOverflow) = height.multipliedReportingOverflow(by: bytesPerRow)
    guard !rowOverflow, !totalOverflow else {
        return 0
    }

    // Refuse to render into a buffer the caller didn't size correctly. The
    // Rust side allocates exactly width*height*4 bytes; if the image's
    // dimensions changed between the size query and this call (e.g. some
    // future API mutates a CGImage), we abort rather than overflow.
    guard totalBytes <= destCapacity else {
        return 0
    }

    let colorSpace = CGColorSpaceCreateDeviceRGB()
    let bitmapInfo = CGImageAlphaInfo.premultipliedLast.rawValue

    // CGContext draws straight into the caller-provided buffer — no Swift
    // malloc, no extra memcpy. Replaces the previous double-allocate path
    // (CGContext-owned buffer + UnsafeMutableRawPointer.allocate + copyMemory)
    // and removes the matching Rust-side .to_vec() third copy.
    guard let context = CGContext(
        data: destBuffer,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: bytesPerRow,
        space: colorSpace,
        bitmapInfo: bitmapInfo
    ) else {
        return 0
    }

    // .copy unconditionally overwrites destination pixels. CGContext.draw
    // defaults to .normal which blends source-over-dest when the source has
    // any non-opaque alpha — that would make the output depend on whatever
    // was in the (sometimes uninitialised) destination buffer, breaking
    // determinism across rgba_data() (uninit dest) vs rgba_data_into()
    // (caller-supplied dest).
    context.setBlendMode(.copy)
    context.draw(cgImage, in: CGRect(x: 0, y: 0, width: width, height: height))

    return totalBytes
}

@_cdecl("cgimage_render_bgra_into")
public func renderCGImageBGRAInto(
    _ image: OpaquePointer,
    _ destBuffer: UnsafeMutableRawPointer,
    _ destCapacity: Int
) -> Int {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()

    let width = cgImage.width
    let height = cgImage.height
    let bytesPerPixel = 4 // BGRA
    let (bytesPerRow, rowOverflow) = width.multipliedReportingOverflow(by: bytesPerPixel)
    let (totalBytes, totalOverflow) = height.multipliedReportingOverflow(by: bytesPerRow)

    guard !rowOverflow, !totalOverflow, totalBytes <= destCapacity else {
        return 0
    }

    // Native pixel layout for ScreenCaptureKit-produced CGImages — premultipliedFirst
    // + byteOrder32Little = "BGRA" interpreted MSB-first, which matches the
    // CVPixelBufferGet*BaseAddress format `kCVPixelFormatType_32BGRA`. By matching
    // the source layout exactly, CGContext.draw skips the per-pixel
    // colour-channel swap that the RGBA path forces. Measured at ~20 ms
    // saved on a 4K screenshot.
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    let bitmapInfo = CGImageAlphaInfo.premultipliedFirst.rawValue
        | CGBitmapInfo.byteOrder32Little.rawValue

    guard let context = CGContext(
        data: destBuffer,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: bytesPerRow,
        space: colorSpace,
        bitmapInfo: bitmapInfo
    ) else {
        return 0
    }

    // See cgimage_render_rgba_into above — same rationale for .copy.
    context.setBlendMode(.copy)
    context.draw(cgImage, in: CGRect(x: 0, y: 0, width: width, height: height))

    return totalBytes
}

// Legacy two-call API. Retained for back-compat with downstream consumers
// that may have linked against it; new code goes through cgimage_render_rgba_into
// above which avoids a 33 MB-per-4K-frame Swift-side memcpy + malloc.
@_cdecl("cgimage_get_data")
public func getCGImageData(_ image: OpaquePointer, _ outPtr: UnsafeMutablePointer<UnsafeRawPointer?>, _ outLength: UnsafeMutablePointer<Int>) -> Bool {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()

    let width = cgImage.width
    let height = cgImage.height
    let bytesPerPixel = 4 // RGBA
    let (bytesPerRow, rowOverflow) = width.multipliedReportingOverflow(by: bytesPerPixel)
    let (totalBytes, totalOverflow) = height.multipliedReportingOverflow(by: bytesPerRow)
    guard !rowOverflow, !totalOverflow else {
        return false
    }

    let buffer = UnsafeMutableRawPointer.allocate(byteCount: totalBytes, alignment: 1)

    let colorSpace = CGColorSpaceCreateDeviceRGB()
    let bitmapInfo = CGImageAlphaInfo.premultipliedLast.rawValue

    guard let context = CGContext(
        data: buffer,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: bytesPerRow,
        space: colorSpace,
        bitmapInfo: bitmapInfo
    ) else {
        buffer.deallocate()
        return false
    }

    context.draw(cgImage, in: CGRect(x: 0, y: 0, width: width, height: height))

    outPtr.pointee = UnsafeRawPointer(buffer)
    outLength.pointee = totalBytes

    return true
}

@_cdecl("cgimage_free_data")
public func freeCGImageData(_ ptr: UnsafeMutableRawPointer) {
    ptr.deallocate()
}

@_cdecl("cgimage_save_png")
public func saveCGImageToPNG(_ image: OpaquePointer, _ pathPtr: UnsafePointer<CChar>) -> Bool {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()
    let path = String(cString: pathPtr)
    let url = URL(fileURLWithPath: path)

    guard let destination = CGImageDestinationCreateWithURL(url as CFURL, UTType.png.identifier as CFString, 1, nil) else {
        return false
    }

    CGImageDestinationAddImage(destination, cgImage, nil)
    return CGImageDestinationFinalize(destination)
}

/// Save CGImage to file with specified format
/// format: 0=PNG, 1=JPEG, 2=TIFF, 3=GIF, 4=BMP, 5=HEIC
/// quality: 0.0-1.0 for lossy formats (JPEG, HEIC)
@_cdecl("cgimage_save_to_file")
public func saveCGImageToFile(_ image: OpaquePointer, _ pathPtr: UnsafePointer<CChar>, _ format: Int32, _ quality: Float) -> Bool {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()
    let path = String(cString: pathPtr)
    let url = URL(fileURLWithPath: path)

    let utType: UTType
    switch format {
    case 0: utType = .png
    case 1: utType = .jpeg
    case 2: utType = .tiff
    case 3: utType = .gif
    case 4: utType = .bmp
    case 5: utType = .heic
    default: return false
    }

    guard let destination = CGImageDestinationCreateWithURL(url as CFURL, utType.identifier as CFString, 1, nil) else {
        return false
    }

    // Set quality for lossy formats
    var properties: [CFString: Any]? = nil
    if format == 1 || format == 5 { // JPEG or HEIC
        properties = [kCGImageDestinationLossyCompressionQuality: quality]
    }

    CGImageDestinationAddImage(destination, cgImage, properties as CFDictionary?)
    return CGImageDestinationFinalize(destination)
}

@_cdecl("cgimage_hash")
public func cgimageHash(_ image: OpaquePointer) -> Int {
    let cgImage = Unmanaged<CGImage>.fromOpaque(UnsafeRawPointer(image)).takeUnretainedValue()
    return cgImage.hashValue
}

@_cdecl("acf_cg_context_create_bitmap")
public func acfCGContextCreateBitmap(
    _ width: Int,
    _ height: Int,
    _ bytesPerRow: Int,
    _ colorSpace: UnsafeMutableRawPointer?,
    _ bitmapInfo: UInt32
) -> UnsafeMutableRawPointer? {
    guard let colorSpace, width > 0, height > 0, bytesPerRow > 0 else { return nil }
    let space = Unmanaged<CGColorSpace>.fromOpaque(colorSpace).takeUnretainedValue()
    guard let context = CGContext(
        data: nil,
        width: width,
        height: height,
        bitsPerComponent: 8,
        bytesPerRow: bytesPerRow,
        space: space,
        bitmapInfo: bitmapInfo
    ) else {
        return nil
    }
    return Unmanaged.passRetained(context).toOpaque()
}

@_cdecl("acf_cg_context_set_gray_fill_color")
public func acfCGContextSetGrayFillColor(_ context: UnsafeMutableRawPointer?, _ gray: Double, _ alpha: Double) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setFillColor(gray: gray, alpha: alpha)
}

@_cdecl("acf_cg_context_set_text_fill_mode")
public func acfCGContextSetTextFillMode(_ context: UnsafeMutableRawPointer?) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setTextDrawingMode(.fill)
}

@_cdecl("acf_cg_context_set_allows_antialiasing")
public func acfCGContextSetAllowsAntialiasing(_ context: UnsafeMutableRawPointer?, _ allowed: Bool) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setAllowsAntialiasing(allowed)
}

@_cdecl("acf_cg_context_set_should_antialias")
public func acfCGContextSetShouldAntialias(_ context: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setShouldAntialias(enabled)
}

@_cdecl("acf_cg_context_set_allows_font_subpixel_positioning")
public func acfCGContextSetAllowsFontSubpixelPositioning(_ context: UnsafeMutableRawPointer?, _ allowed: Bool) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setAllowsFontSubpixelPositioning(allowed)
}

@_cdecl("acf_cg_context_set_should_subpixel_position_fonts")
public func acfCGContextSetShouldSubpixelPositionFonts(_ context: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setShouldSubpixelPositionFonts(enabled)
}

@_cdecl("acf_cg_context_set_allows_font_subpixel_quantization")
public func acfCGContextSetAllowsFontSubpixelQuantization(_ context: UnsafeMutableRawPointer?, _ allowed: Bool) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setAllowsFontSubpixelQuantization(allowed)
}

@_cdecl("acf_cg_context_set_should_subpixel_quantize_fonts")
public func acfCGContextSetShouldSubpixelQuantizeFonts(_ context: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setShouldSubpixelQuantizeFonts(enabled)
}

@_cdecl("acf_cg_context_set_should_smooth_fonts")
public func acfCGContextSetShouldSmoothFonts(_ context: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let context else { return }
    Unmanaged<CGContext>.fromOpaque(context).takeUnretainedValue().setShouldSmoothFonts(enabled)
}
