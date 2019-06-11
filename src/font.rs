use core_foundation::base::TCFType;
use core_foundation::string::CFStringRef;
use core_text::font::{CTFont, CTFontUIFontType, CTFontRef, kCTFontApplicationFontType};
use core_graphics::base::CGFloat;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

extern "C" {
    fn CTFontCreateUIFontForLanguage(
        uiType: CTFontUIFontType,
        size: CGFloat,
        language: CFStringRef,
    ) -> CTFontRef;
}

#[allow(dead_code)]
pub fn get_default_font() -> Option<CTFont> {
    unsafe {
        let ctfont_ref =
            CTFontCreateUIFontForLanguage(kCTFontApplicationFontType, 16., ::std::ptr::null());

        if ctfont_ref.is_null() {
            None
        } else {
            Some(CTFont::wrap_under_create_rule(ctfont_ref))
        }
    }
}

#[allow(dead_code)]
pub fn get_default_font_name() -> Option<String> {
    Some(get_default_font()?.display_name().to_string())
}

#[allow(dead_code)]
pub fn get_default_font_path() -> Option<PathBuf> {
    get_default_font()?.url()?.to_path()
}

#[allow(dead_code)]
pub fn get_default_font_data() -> Option<Vec<u8>> {
    let mut buffer = Vec::new();
    match File::open(get_default_font_path()?).and_then(|mut f| f.read_to_end(&mut buffer)) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}
