use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Mutex;

lazy_static! {
    static ref HANDLE_STORAGE: Mutex<HashMap<i32, usize>> = Mutex::new(HashMap::new());
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TestStruct {
    pub a: i32,
    pub b: u32,
}

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) {
    let buf_name = unsafe { CStr::from_ptr(name).to_bytes() };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    println!("Hello {}!", str_name);
}

#[no_mangle]
pub extern "C" fn get_handle_from_struct(s: TestStruct) -> i32 {
    println!("get_handle_from_struct: s.a = {}; s.b = {}", s.a, s.b);

    let id = 1;
    println!("get_handle_from_struct: id = {}", id);

    let p = &s as *const TestStruct;
    println!("get_handle_from_struct: raw pointer = {:p}", p);

    let pi = p as usize;
    println!("get_handle_from_struct: usize pointer = {}", pi);

    HANDLE_STORAGE.lock().unwrap().insert(id, pi);

    id
}

#[no_mangle]
pub extern "C" fn get_struct_from_handle(handle: i32) -> TestStruct {
    unsafe {
        println!("get_struct_from_handle: handle = {}", handle);

        let storage = HANDLE_STORAGE.lock().unwrap();
        let pi = storage[&handle] as usize;
        println!("get_struct_from_handle: usize pointer = {}", pi);

        let p = pi as *const TestStruct;
        println!("get_struct_from_handle: raw pointer = {:p}", p);

        let s: TestStruct = *p;
        println!("get_struct_from_handle: s.a = {}; s.b = {}", s.a, s.b);

        s
    }
}

#[no_mangle]
pub extern "C" fn gomain() {
    oldmain().ok();
}

fn oldmain() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Progrium Test")
        .with_decorations(false)
        .with_transparent(true)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
      .with_transparent(true)
      //.with_url("https://progrium.com")?
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
