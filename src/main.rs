extern crate core_foundation;
extern crate array_tool;
extern crate core_foundation_sys;
extern crate libc;

use core_foundation::array::{ CFArray, CFArrayRef };
use core_foundation::string::{ CFString, CFStringRef };
use core_foundation::base::{ TCFType, ToVoid };
use core_foundation::error::{ CFErrorRef };
use core_foundation::url::{ CFURL, CFURLRef };

use core_foundation_sys::base::{ CFAllocatorRef, OSStatus };
use libc::c_void;

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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LSLaunchFlags(u32);

impl ::std::fmt::Display for LSLaunchFlags {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ::std::fmt::Debug for LSLaunchFlags {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<u32> for LSLaunchFlags {
    fn into(self) -> u32 {
        self.0
    }
}

impl LSLaunchFlags {
    pub const DEFAULTS: LSLaunchFlags = LSLaunchFlags(0x00000001);
    pub const LAUNCH_AND_PRINTS: LSLaunchFlags = LSLaunchFlags(0x00000002);
    pub const LAUNCH_AND_DISPLAY_ERRORS: LSLaunchFlags = LSLaunchFlags(0x00000040);
    pub const LAUNCH_DONT_ADD_TO_RECENTS: LSLaunchFlags = LSLaunchFlags(0x00000100);
    pub const LAUNCH_DONT_SWITCH: LSLaunchFlags = LSLaunchFlags(0x00000200);
    pub const LAUNCH_ASYNC: LSLaunchFlags = LSLaunchFlags(0x00010000);
    pub const LAUNCH_NEW_INSTANCE: LSLaunchFlags = LSLaunchFlags(0x00080000);
    pub const LAUNCH_AND_HIDE: LSLaunchFlags = LSLaunchFlags(0x00100000);
    pub const LAUNCH_AND_HIDE_OTHERS: LSLaunchFlags = LSLaunchFlags(0x00200000);
}

impl ::std::ops::BitOr for LSLaunchFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        LSLaunchFlags(self.0 | rhs.0)
    }
}

impl ::std::ops::BitAnd for LSLaunchFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        LSLaunchFlags(self.0 & rhs.0)
    }
}

impl ::std::ops::BitXor for LSLaunchFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        LSLaunchFlags(self.0 ^ rhs.0)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct LSLaunchURLSpec {
    pub app: CFURLRef,
    pub urls: CFArrayRef,
    pub _unused1: *const c_void,
    pub flags: LSLaunchFlags,
    pub _unused2: *const c_void,
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn LSCopyAllHandlersForURLScheme(inURLScheme: CFStringRef) -> CFArrayRef;
    fn LSCopyApplicationURLsForBundleIdentifier(inBundleIdentifier: CFStringRef, outError: *mut CFErrorRef) -> CFArrayRef;
    fn LSCopyDisplayNameForURL(inURL: CFURLRef, outDisplayName: *mut CFStringRef) -> OSStatus;
    fn LSOpenFromURLSpec(inLaunchSpec: *const LSLaunchURLSpec, outLaunchedURL: *mut CFURLRef) -> OSStatus;
    fn CFURLCreateWithString(allocator: CFAllocatorRef, urlString: CFStringRef, baseURL: CFURLRef) -> CFURLRef;
}

fn url(url: &str) -> CFURL {
    let url = CFString::new(url);
    unsafe {
        CFURL::wrap_under_create_rule(
            CFURLCreateWithString(std::ptr::null(), url.as_concrete_TypeRef(), std::ptr::null()))
    }
}

pub trait Openable {
    fn into_openable(&self) -> Vec<String>;
}

impl Openable for Vec<&str> {
    fn into_openable(&self) -> Vec<String> {
        self.iter().map(|v| v.to_string()).collect::<Vec<String>>()
    }
}

impl Openable for Vec<String> {
    fn into_openable(&self) -> Vec<String> {
        self.to_vec()
    }
}

impl Openable for &str {
    fn into_openable(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl Openable for str {
    fn into_openable(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl Openable for String {
    fn into_openable(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

#[derive(Debug, Clone)]
pub struct Browser {
    pub name: String,
    pub bundle_id: String,
    pub path: CFURL,
}

impl Browser {
    pub fn open<T: Openable + ?Sized>(&self, urls: &T) -> Result<(), OSStatus> {
        let orig = urls.into_openable().iter().map(|v| url(v)).collect::<Vec<CFURL>>();
        let urls = CFArray::<CFURL>::from_CFTypes(&orig[..]);

        let open_struct = LSLaunchURLSpec {
            app: self.path.as_concrete_TypeRef(),
            _unused1: ::std::ptr::null(),
            urls: urls.as_concrete_TypeRef(),
            flags: LSLaunchFlags::DEFAULTS | LSLaunchFlags::LAUNCH_ASYNC,
            _unused2: ::std::ptr::null(),
        };

        let status = unsafe {
            LSOpenFromURLSpec(&open_struct, std::ptr::null_mut())
        };

        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}

fn get_app_url(name: &str) -> Option<CFURL> {
    let name = CFString::new(name);

    unsafe {
        let names = LSCopyApplicationURLsForBundleIdentifier(
            name.as_concrete_TypeRef(),
            std::ptr::null_mut() as *mut CFErrorRef
        );

        if names.to_void().is_null() {
            None
        } else {
            Some(CFArray::<CFURL>::wrap_under_create_rule(names).get(0)?.clone())
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
            let bundles = LSCopyAllRoleHandlersForContentType(mime.as_concrete_TypeRef(), LsRoleMask::VIEWER);

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

fn get_browsers_bundle_identifiers() -> Option<Vec<String>> {
    intersect!(
        bundle_ids_for_scheme!(http),
        bundle_ids_for_scheme!(https)
    )
}

fn get_app_name(app_path: &CFURL) -> Option<String> {
    let mut display_name_ref: CFStringRef = unsafe { std::mem::zeroed() };

    let status = unsafe {
        LSCopyDisplayNameForURL(
            app_path.as_concrete_TypeRef(),
            &mut display_name_ref
        )
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
        .collect())
}

fn main() {
    match get_browsers() {
        None => println!("None?"),
        Some(browsers) => {
            println!("{:#?}", browsers);
            browsers[1].open(&vec!["http://www.google.it/", "https://news.ycombinator.com/"]);
        }
    };
}
