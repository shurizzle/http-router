use std::mem::transmute;

use objc::runtime::Class;
use objc_foundation::{NSString, NSObject, INSString};
use objc_id::Id;

#[inline]
pub fn class(name: &str) -> *mut Class {
    unsafe { transmute(Class::get(name)) }
}

pub fn url_from_appname(appname: &str) -> String {
    let appname = NSString::from_str(appname);
    let ws_class: Id<Class> = unsafe { Id::from_ptr(class("NSWorkspace")) };
    let ws: Id<NSObject> = unsafe { msg_send![ws_class, sharedWorkspace] };
    let url: Id<NSString> = unsafe { msg_send![ws, fullPathForApplication: appname] };
    url.as_str().to_owned()
}
