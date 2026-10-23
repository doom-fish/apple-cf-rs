use core::ffi::c_void;

extern "C" {
    pub fn acf_cg_context_set_gray_fill_color(context: *mut c_void, gray: f64, alpha: f64);
    pub fn acf_cg_context_set_text_fill_mode(context: *mut c_void);
    pub fn acf_cg_context_set_allows_antialiasing(context: *mut c_void, allowed: bool);
    pub fn acf_cg_context_set_should_antialias(context: *mut c_void, enabled: bool);
    pub fn acf_cg_context_set_allows_font_subpixel_positioning(context: *mut c_void, allowed: bool);
    pub fn acf_cg_context_set_should_subpixel_position_fonts(context: *mut c_void, enabled: bool);
    pub fn acf_cg_context_set_allows_font_subpixel_quantization(
        context: *mut c_void,
        allowed: bool,
    );
    pub fn acf_cg_context_set_should_subpixel_quantize_fonts(context: *mut c_void, enabled: bool);
    pub fn acf_cg_context_set_should_smooth_fonts(context: *mut c_void, enabled: bool);
}
