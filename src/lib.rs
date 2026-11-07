#![no_std]
use gravel::ui::layer::text::{TextLayer, TextLayerHandle};
use gravel::ui::window::{Window, WindowRef, WindowHandlers};
use gravel::ui::clicks::ClickRecognizerRef;
use gravel::graphics::geometry::GRect;
use gravel::prelude::WindowHandle;
use gravel::prelude::LayerHandle;
use gravel::foundation::platform;
use gravel::graphics::bitmap::Raw8Bit;
use gravel::graphics::bitmap::{BitmapV2, DynamicBitmap};
use gravel::graphics::color::GColor;
use gravel::prelude::*;
use gravel::ui::layer::BitmapLayer;
use gravel::ui::window_stack;
use core::ffi::CStr;

const SELECT: &CStr = c"Select";
const UP: &CStr = c"Up";
const DOWN: &CStr = c"Down";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prv_select_click_handler<T>(s_click: ClickRecognizerRef, mut s_window: WindowRef<T>) {
    unsafe {
        let s_text_layer: &mut TextLayer = core::mem::transmute(s_window.user_data_mut());
        s_text_layer.set_text(c"Select".as_ptr());
        s_text_layer.get_layer().mark_dirty();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prv_up_click_handler<T>(s_click: ClickRecognizerRef, mut s_window: WindowRef<T>) {
    unsafe {
        let s_text_layer: &mut TextLayer = core::mem::transmute(s_window.user_data_mut());
        s_text_layer.set_text(c"Up".as_ptr());
        s_text_layer.get_layer().mark_dirty();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prv_down_click_handler<T>(s_click: ClickRecognizerRef, mut s_window: WindowRef<T>) {
    unsafe {
        let s_text_layer: &mut TextLayer = core::mem::transmute(s_window.user_data_mut());
        s_text_layer.set_text(c"Down".as_ptr());
        s_text_layer.get_layer().mark_dirty();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prv_click_provider<T>(mut s_window: WindowRef<T>) {
    s_window.single_click_subscribe(gravel::ui::clicks::ButtonId::Select, prv_select_click_handler);
    s_window.single_click_subscribe(gravel::ui::clicks::ButtonId::Up, prv_up_click_handler);
    s_window.single_click_subscribe(gravel::ui::clicks::ButtonId::Down, prv_down_click_handler);
}

struct s_window_handle {}
impl<T> WindowHandlers<T> for s_window_handle {
    extern "C" fn load(s_window: WindowRef<T>) {
        let s_window_layer = s_window.root_layer();
        unsafe {
            let s_text_layer: &TextLayer = core::mem::transmute(s_window.user_data());
            s_window_layer.add_child(&s_text_layer.get_layer());
        }
    }
    extern "C" fn appear(s_window: WindowRef<T>) {}
    extern "C" fn disappear(s_window: WindowRef<T>) {}
    extern "C" fn unload(s_window: WindowRef<T>) {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> isize {
    let size = platform::DISPLAY_SIZE;
    let mut frame = GRect::default();
    frame.size = size;
    let mut s_text_layer = TextLayer::new(frame).unwrap();
    s_text_layer.set_text(c"Press a button".as_ptr());
    s_text_layer.set_text_alignment(gravel_sys::graphics::text::GTextAlignment::GTextAlignmentCenter);
    let mut s_window: Window<TextLayer> = Window::new_with(s_text_layer);
    s_window.set_click_config_provider(prv_click_provider);
    s_window.set_window_handlers::<s_window_handle>();
    window_stack::push(&s_window, true);
    gravel::foundation::app::event_loop();
    0
}
