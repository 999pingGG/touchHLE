/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITableViewController`.

use crate::frameworks::uikit::ui_view_controller::UIViewControllerHostObject;
use crate::objc::{id, impl_HostObject_with_superclass, objc_classes, release, ClassExports};

#[derive(Default)]
struct UITableViewControllerHostObject {
    superclass: UIViewControllerHostObject,
    /// `UITableView*`
    tableView: id,
}
impl_HostObject_with_superclass!(UITableViewControllerHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableViewController: UIViewController

+ (id)alloc {
    let host_object = Box::<UITableViewControllerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithStyle:(i32)style {
    assert!(style == 0 || style == 1);
    let styleString = if style == 0 { "UITableView.Style.plain" } else { "UITableView.Style.grouped" };
    log!("TODO: [(UITableViewController*){:?} initWithStyle:{}]", this, styleString);
    this
}

- (id)tableView {
    env.objc.borrow::<UITableViewControllerHostObject>(this).tableView
}

- (())dealloc {
    release(env, env.objc.borrow::<UITableViewControllerHostObject>(this).tableView);

    env.objc.dealloc_object(this, &mut env.mem);
}

@end

};
