use hotkey;

fn main() {
    let mut hk = hotkey::Listener::new();
    let (registered, _) = hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'A' as u32,
        || println!("Ctrl-Shift-A pressed!"),
    );

    if !registered {
        eprintln!("ERROR: Failed to register hotkey. Has another app already registered it?");
        return;
    }

    hk.listen();
}
