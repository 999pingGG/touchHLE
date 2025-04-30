/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIBarButtonItem`.

use std::default::Default;
use crate::frameworks::foundation::ns_string::to_rust_string;
use crate::frameworks::foundation::NSUInteger;
use crate::{impl_HostObject_with_superclass, msg_super};
use crate::objc::{id, nil, objc_classes, release, retain, ClassExports, NSZonePtr, SEL};

pub type UIBarButtonItemStyle = NSUInteger;
const UIBarButtonItemStylePlain: UIBarButtonItemStyle = 0;
const UIBarButtonItemStyleBordered: UIBarButtonItemStyle = 1;
const UIBarButtonItemStyleDone: UIBarButtonItemStyle = 2;

struct UIBarButtonItemHostObject {
    superclass: super::UIBarItemHostObject,
    /// `NSString*`
    title: id,
    /// `id`
    target: id,
    /// `SEL`
    action: id,
}
impl_HostObject_with_superclass!(UIBarButtonItemHostObject);
impl Default for UIBarButtonItemHostObject {
    fn default() -> Self {
        UIBarButtonItemHostObject {
            superclass: Default::default(),
            title: nil,
            target: nil,
            action: nil,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIBarButtonItem: UIBarItem

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UIBarButtonItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (())dealloc {
    let &UIBarButtonItemHostObject {
        title,
        target,
        action,
        ..
    } = env.objc.borrow(this);

    release(env, title);
    release(env, target);

    msg_super![env; this dealloc]
}

- (id)initWithTitle:(id)title style:(UIBarButtonItemStyle)style target:(id)target action:(id)action {
    log!(
        "TODO: [(UIBarButtonItem*){:?} initWithTitle:\"{}\" style:(UIBarButtonItemStyle){} target:{:?} action:(SEL){:?}]",
        this,
        to_rust_string(env, title),
        style,
        target,
        action
    );

    *env.objc.borrow_mut::<UIBarButtonItemHostObject>(this) = UIBarButtonItemHostObject {
        title,
        target,
        action,
        ..Default::default()
    };

    retain(env, title);
    retain(env, target);

    this
}

- (id)initWithCustomView:(id)customView {   // UIView*
    log!("TODO: [(UIBarButtonItem)*{:?} initWithCustomView:{:?}]", this, customView);
    this
}

@end

};
