use text_styler::print_style;

#[rustfmt::skip]
macro_rules! print_row_with_bg {
    ($bg:ident, $label:literal) => {
        print_style!(black,          $bg, " XO ");
        print_style!(bright_black,   $bg, " OX ");
        print_style!(white,          $bg, " XO ");
        print_style!(bright_white,   $bg, " OX ");
        print_style!(red,            $bg, " XO ");
        print_style!(bright_red,     $bg, " OX ");
        print_style!(green,          $bg, " XO ");
        print_style!(bright_green,   $bg, " OX ");
        print_style!(blue,           $bg, " XO ");
        print_style!(bright_blue,    $bg, " OX ");
        print_style!(yellow,         $bg, " XO ");
        print_style!(bright_yellow,  $bg, " OX ");
        print_style!(magenta,        $bg, " XO ");
        print_style!(bright_magenta, $bg, " OX ");
        print_style!(cyan,           $bg, " XO ");
        print_style!(bright_cyan,    $bg, " OX ");
        println!("{}", $label);
    }
}

#[rustfmt::skip]
fn main() {
    println!(
        "                      F O R E G R O U N D\n\
        reg bri reg bri reg bri reg bri reg bri reg bri reg bri reg bri"
    );

    print_row_with_bg!(on_black,          " on reg");
    print_row_with_bg!(on_bright_black,   " on bri");
    print_row_with_bg!(on_white,          " on reg");
    print_row_with_bg!(on_bright_white,   " on bri  B");
    print_row_with_bg!(on_red,            " on reg  A");
    print_row_with_bg!(on_bright_red,     " on bri  C");
    print_row_with_bg!(on_green,          " on reg  K");
    print_row_with_bg!(on_bright_green,   " on bri  G");
    print_row_with_bg!(on_blue,           " on reg  R");
    print_row_with_bg!(on_bright_blue,    " on bri  O");
    print_row_with_bg!(on_yellow,         " on reg  U");
    print_row_with_bg!(on_bright_yellow,  " on bri  N");
    print_row_with_bg!(on_magenta,        " on reg  D");
    print_row_with_bg!(on_bright_magenta, " on bri");
    print_row_with_bg!(on_cyan,           " on reg");
    print_row_with_bg!(on_bright_cyan,    " on bri");
}
