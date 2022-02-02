use lazy_static::lazy_static;
use libc::{c_int, c_void};
use std::collections::HashMap;
use std::mem;
use std::sync::Mutex;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
    webview::WebViewBuilder,
};

lazy_static! {
    static ref HANDLERS: Mutex<HashMap<i32, &'static Window>> = Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern "C" fn register_callback(cb: extern "C" fn(i: c_int) -> c_void) {
    println!("register_callback");
    cb(2 as c_int);
    //let closure: &mut &mut dyn FnMut(c_int) = unsafe { mem::transmute(cb) };
    //closure(2)
}

#[no_mangle]
pub extern "C" fn start_loop() {
    internal_start_loop().ok();
}

fn internal_start_loop() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Progrium Test")
        .with_decorations(true)
        .with_transparent(true)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_transparent(true)
        .with_url(
            r#"data:text/html,
            <!doctype html>
            <html>
                <body style="font-family: -apple-system, BlinkMacSystemFont, avenir next, avenir, segoe ui, helvetica neue, helvetica, Ubuntu, roboto, noto, arial, sans-serif; background-color:rgba(87,87,87,0.5);"></body>
                <script>
                    window.onload = function() {
                        document.body.innerHTML = `<div style="padding: 30px">Transparency Test<br><br>${navigator.userAgent}</div>`;
                    };
                </script>
            </html>"#,
        )?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Started"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
