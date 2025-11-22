use cfonts::{Align, Colors, Fonts, Options, gradient::get_multiple_transition_colors, say};

pub fn print() {
    let mut title = Options {
        text: "JIMO SERVER".into(),
        font: Fonts::FontBlock,
        colors: vec![Colors::Red, Colors::Blue],
        align: Align::Left,
        letter_spacing: 2,
        spaceless: true,
        line_height: 10,
        ..Default::default()
    };
    let gradient = get_multiple_transition_colors(
        &["#ff0000".into(), "#00ff00".into(), "#0000ff".into()],
        11,
        &title,
    );
    title.gradient = gradient;
    let explain=
        Options{
            text:"                                                                ------ Notes Today, Innovations Tomorrow".into(),
            font:Fonts::FontConsole,
            colors:vec![Colors::Cyan],
            spaceless:true,
            align:Align::Left,
            ..Default::default()
        };
    say(title);
    say(explain);
}
