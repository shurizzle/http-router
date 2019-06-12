#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

extern crate core_foundation_sys;
extern crate core_foundation;
extern crate core_text;
extern crate core_graphics;

extern crate array_tool;
extern crate libc;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::base::{TCFType, ToVoid};
use core_foundation::error::CFErrorRef;
use core_foundation::url::{CFURL, CFURLRef};

use core_foundation_sys::base::OSStatus;

mod font;
mod open;
mod app;

use open::{LSLaunchFlags, Openable};

pub type OptionBits = u32;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LSRolesMask(u32);

impl ::std::fmt::Display for LSRolesMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ::std::fmt::Debug for LSRolesMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<u32> for LSRolesMask {
    fn into(self) -> u32 {
        self.0
    }
}

impl LSRolesMask {
    pub const NONE: LSRolesMask = LSRolesMask(0x00000001);
    pub const VIEWER: LSRolesMask = LSRolesMask(0x00000002);
    pub const EDITOR: LSRolesMask = LSRolesMask(0x00000004);
    pub const SHELL: LSRolesMask = LSRolesMask(0x00000008);
    pub const ALL: LSRolesMask = LSRolesMask(::std::u32::MAX);
}

impl ::std::ops::BitOr for LSRolesMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl ::std::ops::BitAnd for LSRolesMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl ::std::ops::BitXor for LSRolesMask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn LSCopyAllHandlersForURLScheme(inURLScheme: CFStringRef) -> CFArrayRef;
    fn LSCopyApplicationURLsForBundleIdentifier(
        inBundleIdentifier: CFStringRef,
        outError: *mut CFErrorRef,
    ) -> CFArrayRef;
    fn LSCopyDisplayNameForURL(inURL: CFURLRef, outDisplayName: *mut CFStringRef) -> OSStatus;
}

#[derive(Debug, Clone)]
pub struct Browser {
    pub name: String,
    pub bundle_id: String,
    pub path: CFURL,
}

impl Browser {
    pub fn open<T: Openable + ?Sized>(&self, urls: &T) -> Result<(), OSStatus> {
        open::open(
            Some(urls),
            Some(self.path.clone()),
            LSLaunchFlags::DEFAULTS | LSLaunchFlags::LAUNCH_ASYNC,
        )
    }
}

#[allow(dead_code)]
fn get_app_url(name: &str) -> Option<CFURL> {
    let name = CFString::new(name);

    unsafe {
        let names = LSCopyApplicationURLsForBundleIdentifier(
            name.as_concrete_TypeRef(),
            std::ptr::null_mut() as *mut CFErrorRef,
        );

        if names.to_void().is_null() {
            None
        } else {
            Some(
                CFArray::<CFURL>::wrap_under_create_rule(names)
                    .get(0)?
                    .clone(),
            )
        }
    }
}

macro_rules! bundle_ids_for_scheme {
    ( $scheme:ident ) => {{
        let scheme = CFString::new(stringify!($scheme));

        unsafe {
            let bundles = LSCopyAllHandlersForURLScheme(scheme.as_concrete_TypeRef());

            if bundles.to_void().is_null() {
                None
            } else {
                Some(CFArray::<CFString>::wrap_under_create_rule(bundles)
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>())
            }
        }
    }}
}

#[allow(unused_macros)]
macro_rules! bundle_ids_for_mime_type {
    ( $type:ident ) => {{
        let mime = CFString::new(concat!("public.", stringify!($type)));

        unsafe {
            let bundles = LSCopyAllRoleHandlersForContentType(
                mime.as_concrete_TypeRef(),
                LsRoleMask::VIEWER);

            if bundles.to_void().is_null() {
                None
            } else {
                Some(CFArray::<CFString>::wrap_under_create_rule(bundles)
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>())
            }
        }
    }}
}

macro_rules! intersect {
    ( $one:expr, $two:expr ) => {{
        use array_tool::vec::Intersect;
        let one = $one;
        let two = $two;

        match one {
            None => match two {
                None => None,
                Some(two) => Some(two)
            },
            Some(one) => match two {
                None => Some(one),
                Some(two) => Some(one.intersect(two))
            }
        }
    }}
}

#[allow(dead_code)]
fn get_browsers_bundle_identifiers() -> Option<Vec<String>> {
    intersect!(bundle_ids_for_scheme!(http), bundle_ids_for_scheme!(https))
}

#[allow(dead_code)]
fn get_app_name(app_path: &CFURL) -> Option<String> {
    let mut display_name_ref: CFStringRef = unsafe { std::mem::zeroed() };

    let status =
        unsafe { LSCopyDisplayNameForURL(app_path.as_concrete_TypeRef(), &mut display_name_ref) };

    if status == 0 {
        let display_name_ref: CFString =
            unsafe { CFString::wrap_under_create_rule(display_name_ref) };

        Some(display_name_ref.to_string())
    } else {
        None
    }
}

#[allow(dead_code)]
fn get_browsers() -> Option<Vec<Browser>> {
    Some(
        get_browsers_bundle_identifiers()?
            .iter()
            .map(|val| {
                let path = get_app_url(&*val)?;
                let name = get_app_name(&path)?;
                Some(Browser {
                    name: name,
                    bundle_id: val.clone(),
                    path: path,
                })
            })
            .filter(|val| val.is_some())
            .map(|val| val.unwrap())
            .collect(),
    )
}

fn main() {
    // match get_browsers() {
    //     None => println!("None?"),
    //     Some(browsers) => {
    //         println!("{:#?}", browsers);
    //         browsers[5].open(&vec![
    //             "http://www.google.it/",
    //             "https://news.ycombinator.com/",
    //         ]);
    //     }
    // };
    println!("{}", app::url_from_appname("Google Chrome"));
}
