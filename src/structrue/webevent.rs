

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: f64, y: f64},
}

fn log_event(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => {
            println!("Page load.");
        },
        WebEvent::PageUnload => {
            println!("Page unload.");
        },
        WebEvent::KeyPress(c) => {
            println!("Press key '{c}'.");
        },
        WebEvent::Paste(s) => {
            println!("Paste the \"{s}\".");
        },
        WebEvent::Click { x, y } => {
            println!("Clicked at {{{x}, {y}}}");
        }
    }
}

pub fn test_webevent() {
    println!("============== No.2 WebEvent ==============");
    log_event(&WebEvent::PageLoad);
    log_event(&WebEvent::PageUnload);
    log_event(&WebEvent::Click { x: 10.2, y: -14.0 });
    log_event(&WebEvent::KeyPress('U'));
    log_event(&WebEvent::Paste(String::from("Hello Rust Enum.")));
}

