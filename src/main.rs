use colors_transform::Rgb;
use core::panic;
use plist::{Dictionary, Value};
use std::{collections::HashMap, env};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <itermcolors>", args[0]);
        return;
    }

    let iterm_theme = Value::from_file(args[1].to_owned())
        .unwrap()
        .as_dictionary()
        .unwrap()
        .clone();

    println!("colors:");
    print_primary_color_section(&iterm_theme);
    print_normal_color_section(&iterm_theme);
    print_bright_color_section(&iterm_theme);
    print_cursor_color_section(&iterm_theme);
    print_selection_color_section(&iterm_theme);
}

fn print_color_section(
    iterm_theme: &Dictionary,
    section_name: &str,
    alacritty_color_name_iterm_theme_color_name_map: &HashMap<&str, String>,
) {
    println!("  {}:", section_name);
    for (alacritty_color_name, iterm_theme_color_name) in
        alacritty_color_name_iterm_theme_color_name_map
    {
        let rgb = fetch_rgb_from_dictionary(&iterm_theme, iterm_theme_color_name);
        println!(
            "    {}: \"{}\"",
            alacritty_color_name,
            rgb_to_alacritty_color_code(&rgb)
        );
    }
}

fn print_selection_color_section(iterm_theme: &Dictionary) {
    let section_name = "selection";
    let alacritty_color_name_iterm_theme_color_name_map = HashMap::from([
        ("text", "Selected Text Color".to_string()),
        ("background", "Selection Color".to_string()),
    ]);
    print_color_section(
        iterm_theme,
        section_name,
        &alacritty_color_name_iterm_theme_color_name_map,
    );
}

fn print_cursor_color_section(iterm_theme: &Dictionary) {
    let section_name = "cursor";
    let alacritty_color_name_iterm_theme_color_name_map = HashMap::from([
        ("text", "Cursor Text Color".to_string()),
        ("cursor", "Cursor Color".to_string()),
    ]);
    print_color_section(
        iterm_theme,
        section_name,
        &alacritty_color_name_iterm_theme_color_name_map,
    );
}

fn print_primary_color_section(iterm_theme: &Dictionary) {
    let section_name = "primary";
    let alacritty_color_name_iterm_theme_color_name_map = HashMap::from([
        ("background", "Background Color".to_string()),
        ("foreground", "Foreground Color".to_string()),
    ]);
    print_color_section(
        iterm_theme,
        section_name,
        &alacritty_color_name_iterm_theme_color_name_map,
    );
}

fn print_normal_color_section(iterm_theme: &Dictionary) {
    let section_name = "normal";
    let mut alacritty_color_name_iterm_theme_color_name_map = HashMap::new();
    for i in 0..=7 {
        let iterm_color_name = format!("Ansi {} Color", i);
        alacritty_color_name_iterm_theme_color_name_map.insert(
            ansi_color_index_to_alacritty_color_name(i),
            iterm_color_name,
        );
    }
    print_color_section(
        iterm_theme,
        section_name,
        &alacritty_color_name_iterm_theme_color_name_map,
    );
}

fn print_bright_color_section(iterm_theme: &Dictionary) {
    let section_name = "bright";
    let mut alacritty_color_name_iterm_theme_color_name_map = HashMap::new();
    for i in 8..=15 {
        let iterm_color_name = format!("Ansi {} Color", i);
        alacritty_color_name_iterm_theme_color_name_map.insert(
            ansi_color_index_to_alacritty_color_name(i),
            iterm_color_name,
        );
    }
    print_color_section(
        iterm_theme,
        section_name,
        &alacritty_color_name_iterm_theme_color_name_map,
    );
}

fn ansi_color_index_to_alacritty_color_name(ansi_color_index: usize) -> &'static str {
    match ansi_color_index % 8 {
        0 => "black",
        1 => "red",
        2 => "green",
        3 => "yellow",
        4 => "blue",
        5 => "magenta",
        6 => "cyan",
        7 => "white",
        _ => panic!("Unreachable"),
    }
}

fn fetch_rgb_from_dictionary(dict: &Dictionary, key: &str) -> Rgb {
    let entry = dict.get(key).unwrap().as_dictionary().unwrap();
    rgb_from_dict_entry(entry)
}

fn rgb_from_dict_entry(entry: &Dictionary) -> Rgb {
    let red = entry.get("Red Component").unwrap().as_real().unwrap() as f32;
    let green = entry.get("Green Component").unwrap().as_real().unwrap() as f32;
    let blue = entry.get("Blue Component").unwrap().as_real().unwrap() as f32;
    return Rgb::from(red * 255.0, green * 255.0, blue * 255.0);
}

fn rgb_to_alacritty_color_code(rgb: &Rgb) -> String {
    format!("0x{}", rem_first(&rgb.to_css_hex_string()))
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
