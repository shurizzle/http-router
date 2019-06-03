extern crate core_foundation;

use core_foundation::array::{ CFArray, CFArrayRef };
use core_foundation::string::{ CFString, CFStringRef };
use core_foundation::base::{ TCFType, ToVoid };
use core_foundation::error::{ CFErrorRef };
use core_foundation::url::{ CFURL, CFURLRef };
use libc::c_int;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn LSCopyAllHandlersForURLScheme(inURLScheme: CFStringRef) -> CFArrayRef;
    fn LSCopyApplicationURLsForBundleIdentifier(inBundleIdentifier: CFStringRef, outError: *mut CFErrorRef) -> CFArrayRef;
    fn LSCopyDisplayNameForURL(inURL: CFURLRef, outDisplayName: *mut CFStringRef) -> c_int;
}

#[derive(Debug)]
pub struct Browser {
    pub name: String,
    pub bundle_id: String,
}

fn get_app_url(name: &CFString) -> Option<CFURL> {
    unsafe {
        let names = LSCopyApplicationURLsForBundleIdentifier(name.as_concrete_TypeRef(), std::ptr::null_mut() as *mut CFErrorRef);

        if names.to_void().is_null() {
            None
        } else {
            let names: CFArray<CFURL> = CFArray::wrap_under_create_rule(names);

            Some(names.get(0)?.clone())
        }
    }
}

fn get_browsers_bundle_identifiers() -> Option<CFArray<CFString>> {
    let https = CFString::new("https");

    unsafe {
        let browsers = LSCopyAllHandlersForURLScheme(https.as_concrete_TypeRef());

        if browsers.to_void().is_null() {
            None
        } else {
            Some(CFArray::wrap_under_create_rule(browsers))
        }
    }
}

fn get_app_name(bundle_id: &CFString) -> Option<String> {
    let mut display_name_ref: CFStringRef = unsafe { std::mem::zeroed() };

    let status = unsafe {
        LSCopyDisplayNameForURL(get_app_url(bundle_id)?.as_concrete_TypeRef(), &mut display_name_ref)
    };

    if status == 0 {
        let display_name_ref: CFString = unsafe {
            CFString::wrap_under_create_rule(display_name_ref)
        };

        Some(display_name_ref.to_string())
    } else {
        None
    }
}

fn get_browsers() -> Option<Vec<Browser>> {
    Some(get_browsers_bundle_identifiers()?.iter()
        .map(|val| {
            match get_app_name(&*val) {
                None => None,
                Some(name) => Some(Browser {
                    name: name,
                    bundle_id: val.to_string(),
                })
            }
        })
        .filter(|val| val.is_some())
        .map(|val| val.unwrap())
        .collect())
}

fn main() {
    match get_browsers() {
        None => println!("None?"),
        Some(browsers) => println!("{:#?}", browsers)
    };
}
