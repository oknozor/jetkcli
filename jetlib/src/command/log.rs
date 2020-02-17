use moins::Color;
use moins::Moins;
use moins::PagerOptions;
use std::collections::HashMap;

pub fn run(issues: &mut String) {
    let mut colors = HashMap::new();
    colors.insert("Description :".to_owned(), Color::LightBlue);
    colors.insert("Status :".to_owned(), Color::LightRed);
    colors.insert("Status description :".to_owned(), Color::LightGreen);

    let options = PagerOptions {
        colors,
        search: false,
        line_number: false,
    };

    Moins::run(issues, Some(options));
}
