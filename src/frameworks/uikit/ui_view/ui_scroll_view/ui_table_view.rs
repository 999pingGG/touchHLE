/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITableView`.

use crate::frameworks::uikit::ui_view::UIViewHostObject;
use crate::{impl_HostObject_with_superclass, objc_classes};
use crate::objc::ClassExports;

pub struct UITableViewHostObject {
    superclass: UIViewHostObject,
}
impl_HostObject_with_superclass!(UITableViewHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableView: UIScrollView
@end

};
