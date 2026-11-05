pub fn println_url(text: String, url: String) {
    println!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\");
}
