//! `UINavigationItem`.

use crate::objc::{id, ClassExports, HostObject};
use crate::objc_classes;

#[derive(Default)]
pub struct UINavigationItemHostObject {
//     // /// The root view.
//     // /// `UIView*`
//     // view: id,
//     // /// Nib name to be used at the load
//     // /// of the root view, may be nil.
//     // /// `NSString*`
//     // nib_name: id,
//     // /// Bundle to be used for loading
//     // /// the nib by name, may be nil.
//     // /// `NSBundle*`
//     // bundle: id,
}
impl HostObject for UINavigationItemHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UINavigationItem: NSObject

- (())setRightBarButtonItem:(id)item {
    log!("TODO: [(UINavigationItem*){:?} setRightBarButtonItem:?]", this); // TODO
}
    
- (())setLeftBarButtonItem:(id)item {
    log!("TODO: [(UINavigationItem*){:?} setLeftBarButtonItem:?]", this); // TODO
}

@end

};
