use cocoa::base::{NSUInteger, selector, nil, YES, NO};
use cocoa::appkit::{NSApp, NSRect, NSPoint, NSSize,
                    NSAutoreleasePool, NSProcessInfo,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSString,
                    NSMenu, NSMenuItem};
pub fn start() {
    unsafe {
        //let _pool = NSAutoreleasePool::new(nil); Not needed?
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);

        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title = quit_prefix.stringByAppendingString_(
            NSProcessInfo::processInfo(nil).processName()
            );
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            quit_title,
            quit_action,
            quit_key
            ).autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);
        app.setMainMenu_(menubar);
        // create Window
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
            NSTitledWindowMask as NSUInteger,
            NSBackingStoreBuffered,
            NO
            ).autorelease();
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.center();
            let title = NSString::alloc(nil).init_str("Handmade Hero!");
            window.setTitle_(title);
            window.makeKeyAndOrderFront_(nil);
            app.activateIgnoringOtherApps_(YES);
            app.run();
        }
}
