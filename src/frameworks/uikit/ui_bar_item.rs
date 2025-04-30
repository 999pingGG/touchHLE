use crate::{objc::{ClassExports, HostObject}, objc_classes};

pub mod ui_bar_button_item;

#[derive(Default)]
struct UIBarItemHostObject {
}
impl HostObject for UIBarItemHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIBarItem: NSObject
@end

};
