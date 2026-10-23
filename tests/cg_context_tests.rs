#![cfg(feature = "cg")]

use apple_cf::cg::CGContext;

#[test]
fn bitmap_context_storage_is_initialized() {
    let context = CGContext::new_rgba8(4, 4).expect("context");
    // SAFETY: the context has no aliases and no drawing occurs while the slice lives.
    let bytes = unsafe { context.as_bytes() };
    assert!(bytes.iter().all(|byte| *byte == 0));
}

#[test]
fn context_clones_share_drawing_state() {
    let context = CGContext::new_rgba8(4, 4).expect("context");
    let alias = context.clone();
    alias.set_rgb_fill_color(1.0, 0.0, 0.0, 1.0);
    alias.fill_rect(0.0, 0.0, 4.0, 4.0);
    drop(alias);

    // SAFETY: drawing is complete and the retained alias has been dropped.
    let bytes = unsafe { context.as_bytes() };
    assert_eq!(&bytes[..4], &[255, 0, 0, 255]);
}

#[test]
fn text_bitmap_formats_and_rasterization_settings() {
    let alpha = CGContext::new_alpha_only(7, 3).expect("alpha-only context");
    assert_eq!(alpha.bytes_per_row(), 7);
    assert_eq!(alpha.alpha_info(), 7);
    alpha.set_gray_fill_color(1.0, 1.0);
    alpha.fill_rect(0.0, 0.0, 7.0, 3.0);
    assert!(unsafe { alpha.as_bytes() }.iter().all(|byte| *byte == 255));

    let color = CGContext::new_premultiplied_last(7, 3).expect("premultiplied context");
    assert_eq!(color.bytes_per_row(), 28);
    assert_eq!(color.alpha_info(), 1);
    color.set_text_fill_mode();
    color.set_allows_antialiasing(true);
    color.set_should_antialias(true);
    color.set_allows_font_subpixel_positioning(true);
    color.set_should_subpixel_position_fonts(true);
    color.set_allows_font_subpixel_quantization(false);
    color.set_should_subpixel_quantize_fonts(false);
    color.set_should_smooth_fonts(true);
    color.set_gray_fill_color(0.5, 1.0);
    color.fill_rect(0.0, 0.0, 7.0, 3.0);
    assert!(unsafe { color.as_bytes() }.iter().any(|byte| *byte != 0));
}
