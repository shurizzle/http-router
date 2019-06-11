extern crate core_foundation;
extern crate libc;

use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::url::{CFURL, CFURLRef};
use core_foundation_sys::base::{CFAllocatorRef, OSStatus};
use libc::c_void;

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

#[allow(dead_code)]
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
struct LSLaunchURLSpec {
    pub app: CFURLRef,
    pub urls: CFArrayRef,
    pub _unused1: *const c_void,
    pub flags: LSLaunchFlags,
    pub _unused2: *const c_void,
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn LSOpenFromURLSpec(
        inLaunchSpec: *const LSLaunchURLSpec,
        outLaunchedURL: *mut CFURLRef,
    ) -> OSStatus;
    fn CFURLCreateWithString(
        allocator: CFAllocatorRef,
        urlString: CFStringRef,
        baseURL: CFURLRef,
    ) -> CFURLRef;
}

pub fn url(url: &str) -> CFURL {
    let url = CFString::new(url);
    unsafe {
        CFURL::wrap_under_create_rule(CFURLCreateWithString(
            ::std::ptr::null(),
            url.as_concrete_TypeRef(),
            ::std::ptr::null(),
        ))
    }
}

pub trait Openable {
    fn into_openable(&self) -> CFArray<CFURL>;
}

impl Openable for Vec<&str> {
    fn into_openable(&self) -> CFArray<CFURL> {
        let v = self.iter().map(|v| url(&**v)).collect::<Vec<CFURL>>();
        CFArray::<CFURL>::from_CFTypes(&v[..])
    }
}

impl Openable for Vec<&String> {
    fn into_openable(&self) -> CFArray<CFURL> {
        let v = self.iter().map(|v| url(&**v)).collect::<Vec<CFURL>>();
        CFArray::<CFURL>::from_CFTypes(&v[..])
    }
}

impl Openable for Vec<String> {
    fn into_openable(&self) -> CFArray<CFURL> {
        let v = self.iter().map(|v| url(v)).collect::<Vec<CFURL>>();
        CFArray::<CFURL>::from_CFTypes(&v[..])
    }
}

impl Openable for &str {
    fn into_openable(&self) -> CFArray<CFURL> {
        vec![self.to_string()].into_openable()
    }
}

impl Openable for str {
    fn into_openable(&self) -> CFArray<CFURL> {
        vec![self].into_openable()
    }
}

impl Openable for String {
    fn into_openable(&self) -> CFArray<CFURL> {
        vec![self].into_openable()
    }
}

#[allow(dead_code)]
struct OpenableContainer {
    array: Option<CFArray<CFURL>>,
    ptr: CFArrayRef,
}

impl OpenableContainer {
    fn new<T>(openable: Option<&T>) -> Self
    where
        T: Openable + ?Sized,
    {
        if openable.is_some() {
            let array = openable.unwrap().into_openable();
            let ptr = array.as_concrete_TypeRef();

            OpenableContainer {
                array: Some(array),
                ptr: ptr,
            }
        } else {
            OpenableContainer {
                array: None,
                ptr: ::std::ptr::null(),
            }
        }
    }

    fn to_ref(&self) -> CFArrayRef {
        self.ptr
    }
}

#[allow(dead_code)]
pub fn open<T>(urls: Option<&T>, app: Option<CFURL>, flags: LSLaunchFlags) -> Result<(), OSStatus>
where
    T: Openable + ?Sized,
{
    let openable = OpenableContainer::new(urls);

    let open_struct = LSLaunchURLSpec {
        app: app.map(|v| v.as_concrete_TypeRef()).unwrap_or_else(
            ::std::ptr::null,
        ),
        _unused1: ::std::ptr::null(),
        urls: openable.to_ref(),
        flags: flags,
        _unused2: ::std::ptr::null(),
    };

    let status = unsafe { LSOpenFromURLSpec(&open_struct, ::std::ptr::null_mut()) };

    if status == 0 { Ok(()) } else { Err(status) }
}
