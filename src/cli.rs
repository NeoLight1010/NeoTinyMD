fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    title
}

pub fn print_short_banner() {
    println!("{}", get_title());
}

pub fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: neotinymd <filename>.md",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}
