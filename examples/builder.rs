use text_styler::Style;

fn main() {
    let builder = Style::new("builder pattern")
        .blue()
        .on_white()
        .bold()
        .build();

    dbg!(builder);
}
