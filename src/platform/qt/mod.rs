use qt_gui::QKeySequence;
use qt_widgets::{QApplication, QGridLayout, QLabel, QLineEdit, QShortcut, QWidget};
use qt_core::cpp_core::{Ptr, CppBox, CppDeletable};
use qt_core::{qs, QCoreApplication, QString};
use std::ffi::CStr;
use slint::Window;
use crate::ui::slint::Comandr;
use std::sync::{LazyLock, Mutex};
use std::ptr::addr_of;

//#![feature(thread_local)]

#[thread_local]
static mut SLINT_WINDOW: Option<Comandr> = None;

#[thread_local]
static mut TMP_WINDOW: Option<TmpWindow> = None;

//static SLINT_WINDOW: LazyLock<Mutex<Option<Comandr>>> = LazyLock::new(|| Mutex::new(None));


pub fn comandr_qt_show() -> () {
    unsafe {
        //let core_app = unsafe { qt_core::QCoreApplication::instance() };
        //let app = core_app.dynamic_cast::<QApplication>();

        let active_window_widget = QApplication::active_window();
        let active_window_widget_size = active_window_widget.size();

        let slint_window = crate::ui::slint::Comandr::new().unwrap();

        slint_window.show();
        SLINT_WINDOW = Some(slint_window);

        let grid_widget = QWidget::new_0a();
        let grid_layout = QGridLayout::new_0a();
        grid_widget.set_layout(&grid_layout);
        grid_widget.resize_1a(&active_window_widget_size);
        grid_widget.set_parent_1a(&active_window_widget);
        grid_widget.show();
        grid_widget.set_style_sheet(&qs("border: 2px solid green;"));
        //grid_widget.set_focus_0a();

        // /*
        let mut slint_window_q_widget = get_widget_by_title("comandr_command_palette").unwrap();
        println!("comandr title: {}", qstring_to_rust((*slint_window_q_widget).window_title()));
        (*slint_window_q_widget).focus_widget();
        (*slint_window_q_widget).set_focus_1a(qt_core::FocusReason::ShortcutFocusReason);
        (*slint_window_q_widget).grab_keyboard();
        // */

        let test_widget = QLabel::new();
        test_widget.set_text(&qs("comandr palette"));
        test_widget.set_style_sheet(&qs("background-color: green;"));

        grid_layout.add_widget_3a(slint_window_q_widget, 2, 2);

        //let line_edit = QLineEdit::new();
        //line_edit.set_focus_1a(qt_core::FocusReason::ShortcutFocusReason);
        //line_edit.grab_keyboard();
        //grid_layout.add_widget_3a(&line_edit, 2, 2);

        grid_layout.add_widget_5a(&QWidget::new_0a(), 1, 1, 1, 1);

        grid_layout.add_widget_5a(&QWidget::new_0a(), 1, 3, 1, 1);

        grid_layout.add_widget_5a(&QWidget::new_0a(), 3, 1, 3, 1);

        // to close comandr with escape
        add_shortcut_to_widget(grid_widget.as_ptr(), "Escape", move || close_comandr(grid_widget.as_ptr()));
    }
}

unsafe fn close_comandr(grid_widget: Ptr<QWidget>) {
    match std::ptr::read(addr_of!(SLINT_WINDOW)) {
        Some(slint_window) => slint_window.hide(),
        None => panic!(),
    };

    (*grid_widget).delete();
}

unsafe fn get_class_name(widget: *mut QWidget) -> String {
    let name_string_ptr = (*widget).meta_object().class_name();

    let c_str: &CStr = unsafe { CStr::from_ptr(name_string_ptr) };
    let name_str = c_str.to_str().unwrap();
    let name_string = name_str.to_owned();
    return name_string;
}

unsafe fn add_shortcut_to_widget(widget: Ptr<QWidget>, shortcut:  &str, callback: impl Fn() +'static) {
        let key = QKeySequence::from_q_string(&qs(shortcut));
        let shortcut = QShortcut::new_2a(&key, widget);
        let signal = shortcut.activated();

        let my_slot = qt_core::SlotNoArgs::new(widget, callback);

        signal.connect(&my_slot);
}

pub fn init(widget: Ptr<QWidget>) -> () {
    unsafe {

        let tmp_window = TmpWindow::new().unwrap();
        tmp_window.show();
        let tmp_window_q_widget = get_widget_by_title("comandr_tmp_window").unwrap();
        (*tmp_window_q_widget).hide();
        TMP_WINDOW = Some(tmp_window);

        add_shortcut_to_widget(widget, "CTRL+P", comandr_qt_show)
    }
}

unsafe fn get_widget_by_title(title: &str) -> Option<* mut QWidget> {
    let widgets = QApplication::top_level_widgets();
    let mut ret_widget: *mut QWidget = *widgets.index_mut(0);
    let mut found = false;

    let widgets_size = widgets.size();
    for i in 0..widgets_size {
        let widget = *widgets.index(i);
        let widget_title = qstring_to_rust((*widget).window_title());
        println!("get_widget_by_title: window_title: {}", widget_title);
        println!("class_name: {}", get_class_name(widget));
        if widget_title == title.to_owned() {
            println!("found: {:?}", widget);
            found = true;
            ret_widget = widget
        }
    }
    if found {
        return Some(ret_widget);
    } else {
        return None;
    }
}


slint::slint! {
    export component TmpWindow inherits Window {
        title: "comandr_tmp_window";
        Text {
            text: "hello world";
            color: green;
        }
    }
}

unsafe fn qstring_to_rust(q_string: CppBox<QString>) -> String {
    let mut rust_string = String::new();
    let q_string_size = q_string.size();

    for j in 0..q_string_size {
        let q_char = q_string.index_int(j);
        let rust_char = char::from_u32(q_char.unicode() as u32);
        rust_string.push(rust_char.unwrap());
    }
    return rust_string;
}
