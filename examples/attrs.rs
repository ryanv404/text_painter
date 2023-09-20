use text_styler::{print_style, println_style};

macro_rules! print_row_with_attr {
    ($attr:ident, $text:literal) => {
        print_style!($attr,   black, on_current,   &format!("{} ", $text));
        print_style!($attr,   white, on_current,   &format!("{} ", $text));
        print_style!($attr,   red, on_current,     &format!("{} ", $text));
        println_style!($attr, green, on_current,   &format!("{}",  $text));
        print_style!($attr,   blue, on_current,    &format!("{} ", $text));
        print_style!($attr,   yellow, on_current,  &format!("{} ", $text));
        print_style!($attr,   cyan, on_current,    &format!("{} ", $text));
        println_style!($attr, magenta, on_current, &format!("{}",  $text));
        println!();
    };
}

fn main() {
    print_row_with_attr!(bold,        "I'm bold");
    print_row_with_attr!(dim,         "I'm dimmed");
    print_row_with_attr!(underlined,  "I'm underlined");
    print_row_with_attr!(italics,     "I'm italicized");
    print_row_with_attr!(inverted,    "I'm inverted");
    print_row_with_attr!(strike,      "I'm crossed out");
    print_row_with_attr!(hidden,      "I'm hidden");
    println!(r"   /\                            /\ ");
    println!(r"    \_  Last 2 rows are hidden  _/ ");
}
