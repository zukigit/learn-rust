enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y: i64}
}

impl WebEvent {
    fn detect(&self) -> String{
        match self {
            Self::PageLoad => format!("It is pageload"),
            Self::PageUnload => format!("It is pageunload"),
            Self::KeyPress(key) => format!("It is key press, key: {}", key),
            Self::Click { x, y } => format!("It is click, x: {}, y: {}", x, y),
            Self::Paste(str) => format!("It is paste, str: {}", str)
        }
    }
}

fn detect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("It is pageload"),
        WebEvent::PageUnload => println!("It is pageunload"),
        WebEvent::KeyPress(key) => println!("It is key press, key: {}", key),
        WebEvent::Click { x, y } => println!("It is click, x: {}, y: {}", x, y),
        WebEvent::Paste(str) => println!("It is paste, str: {}", str)

    }
}

pub fn enums() {
    let page_load = WebEvent::PageLoad;
    let page_unload = WebEvent::PageUnload;
    let key_press: WebEvent = WebEvent::KeyPress('g');
    let paste: WebEvent = WebEvent::Paste(String::from("it is some copy string"));
    let click: WebEvent = WebEvent::Click { x: 34, y: 65 };

    detect(page_load);
    detect(page_unload);
    detect(key_press);
    detect(paste);
    detect(click);

    let event = WebEvent::PageLoad;
    event.detect();
}