macro_rules! test_color {
    ($func:ident:
     $fg:ident($fg_num:literal), $bg:ident($bg_num:literal),
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let output = format!(
                "{}", $crate::style::Style::new($text).$fg($fg_num).$bg($bg_num)
            );
            assert_eq!(output, $expected);
        }
    };
    ($func:ident:
     $fg:ident($fg_r:literal, $fg_g:literal, $fg_b:literal),
     $bg:ident($bg_r:literal, $bg_g:literal, $bg_b:literal),
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let output = format!(
                "{}",
                $crate::style::Style::new($text)
                    .$fg($fg_r, $fg_g, $fg_b)
                    .$bg($bg_r, $bg_g, $bg_b)
            );
            assert_eq!(output, $expected);
        }
    };
    ($func:ident: $attr:ident, $fg:ident, $bg:ident,
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let output = format!(
                "{}", $crate::style::Style::new($text).$attr().$fg().$bg()
            );
            assert_eq!(output, $expected);
        }
    };
    ($func:ident: $fg:ident, $bg:ident, $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let output = format!(
                "{}", $crate::style::Style::new($text).$fg().$bg()
            );
            assert_eq!(output, $expected);
        }
    };
}

macro_rules! test_write_style {
    ($func:ident: $fg:ident($fg_num:literal), $bg:ident($bg_num:literal),
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::write_style!(&mut output, $fg($fg_num), $bg($bg_num), $text);
            assert_eq!(output, $expected.as_bytes());
        }
    };
    ($func:ident:
     $fg:ident($fg_r:literal, $fg_g:literal, $fg_b:literal),
     $bg:ident($bg_r:literal, $bg_g:literal, $bg_b:literal),
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::write_style!(
                &mut output,
                $fg($fg_r, $fg_g, $fg_b),
                $bg($bg_r, $bg_g, $bg_b),
                $text
            );
            assert_eq!(output, $expected.as_bytes());
        }
    };
    ($func:ident: $attr:ident, $fg:ident, $bg:ident,
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::write_style!(&mut output, $attr, $fg, $bg, $text);
            assert_eq!(output, $expected.as_bytes());
        }
    };
    ($func:ident: $fg:ident, $bg:ident, $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::write_style!(&mut output, $fg, $bg, $text);
            assert_eq!(output, $expected.as_bytes());
        }
    };
}

macro_rules! test_writeln_style {
    ($func:ident: $fg:ident($fg_num:literal), $bg:ident($bg_num:literal),
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::writeln_style!(&mut output, $fg($fg_num), $bg($bg_num), $text);
            assert_eq!(output, $expected.as_bytes());
        }
    };
    ($func:ident:
     $fg:ident($fg_r:literal, $fg_g:literal, $fg_b:literal),
     $bg:ident($bg_r:literal, $bg_g:literal, $bg_b:literal),
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::writeln_style!(
                &mut output,
                $fg($fg_r, $fg_g, $fg_b),
                $bg($bg_r, $bg_g, $bg_b),
                $text
            );
            assert_eq!(output, $expected.as_bytes());
        }
    };
    ($func:ident: $attr:ident, $fg:ident, $bg:ident,
     $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::writeln_style!(&mut output, $attr, $fg, $bg, $text);
            assert_eq!(output, $expected.as_bytes());
        }
    };
    ($func:ident: $fg:ident, $bg:ident, $text:literal => $expected:literal) => {
        #[test]
        fn $func() {
            let mut output = Vec::new();
            $crate::writeln_style!(&mut output, $fg, $bg, $text);
            assert_eq!(output, $expected.as_bytes());
        }
    };
}

mod regular_fg {
    test_color!(cu_on_bk: current, on_black, "hi" => "\x1b[40mhi\x1b[0m");
    test_color!(bk_on_bk: black, on_black,   "hi" => "\x1b[30;40mhi\x1b[0m");
    test_color!(rd_on_bk: red, on_black,     "hi" => "\x1b[31;40mhi\x1b[0m");
    test_color!(gr_on_bk: green, on_black,   "hi" => "\x1b[32;40mhi\x1b[0m");
    test_color!(yl_on_bk: yellow, on_black,  "hi" => "\x1b[33;40mhi\x1b[0m");
    test_color!(bl_on_bk: blue, on_black,    "hi" => "\x1b[34;40mhi\x1b[0m");
    test_color!(mg_on_bk: magenta, on_black, "hi" => "\x1b[35;40mhi\x1b[0m");
    test_color!(cy_on_bk: cyan, on_black,    "hi" => "\x1b[36;40mhi\x1b[0m");
    test_color!(wh_on_bk: white, on_black,   "hi" => "\x1b[37;40mhi\x1b[0m");
}

mod bright_fg {
    test_color!(cu_on_bk:    bright_red, on_current,   "hi" => "\x1b[91mhi\x1b[0m");
    test_color!(br_bk_on_bk: bright_black, on_black,   "hi" => "\x1b[90;40mhi\x1b[0m");
    test_color!(br_rd_on_bk: bright_red, on_black,     "hi" => "\x1b[91;40mhi\x1b[0m");
    test_color!(br_gr_on_bk: bright_green, on_black,   "hi" => "\x1b[92;40mhi\x1b[0m");
    test_color!(br_yl_on_bk: bright_yellow, on_black,  "hi" => "\x1b[93;40mhi\x1b[0m");
    test_color!(br_bl_on_bk: bright_blue, on_black,    "hi" => "\x1b[94;40mhi\x1b[0m");
    test_color!(br_mg_on_bk: bright_magenta, on_black, "hi" => "\x1b[95;40mhi\x1b[0m");
    test_color!(br_cy_on_bk: bright_cyan, on_black,    "hi" => "\x1b[96;40mhi\x1b[0m");
    test_color!(br_wh_on_bk: bright_white, on_black,   "hi" => "\x1b[97;40mhi\x1b[0m");
}

mod regular_bg {
    test_color!(bk_on_cu: black, on_current, "hi" => "\x1b[30mhi\x1b[0m");
    test_color!(bk_on_bk: black, on_black,   "hi" => "\x1b[30;40mhi\x1b[0m");
    test_color!(bk_on_rd: black, on_red,     "hi" => "\x1b[30;41mhi\x1b[0m");
    test_color!(bk_on_gr: black, on_green,   "hi" => "\x1b[30;42mhi\x1b[0m");
    test_color!(bk_on_yl: black, on_yellow,  "hi" => "\x1b[30;43mhi\x1b[0m");
    test_color!(bk_on_bl: black, on_blue,    "hi" => "\x1b[30;44mhi\x1b[0m");
    test_color!(bk_on_mg: black, on_magenta, "hi" => "\x1b[30;45mhi\x1b[0m");
    test_color!(bk_on_cy: black, on_cyan,    "hi" => "\x1b[30;46mhi\x1b[0m");
    test_color!(bk_on_wh: black, on_white,   "hi" => "\x1b[30;47mhi\x1b[0m");
}

mod bright_bg {
    test_color!(cu_on_br_bk: current, on_bright_black, "hi" => "\x1b[100mhi\x1b[0m");
    test_color!(bk_on_br_bk: black, on_bright_black,   "hi" => "\x1b[30;100mhi\x1b[0m");
    test_color!(bk_on_br_rd: black, on_bright_red,     "hi" => "\x1b[30;101mhi\x1b[0m");
    test_color!(bk_on_br_gr: black, on_bright_green,   "hi" => "\x1b[30;102mhi\x1b[0m");
    test_color!(bk_on_br_yl: black, on_bright_yellow,  "hi" => "\x1b[30;103mhi\x1b[0m");
    test_color!(bk_on_br_bl: black, on_bright_blue,    "hi" => "\x1b[30;104mhi\x1b[0m");
    test_color!(bk_on_br_mg: black, on_bright_magenta, "hi" => "\x1b[30;105mhi\x1b[0m");
    test_color!(bk_on_br_cy: black, on_bright_cyan,    "hi" => "\x1b[30;106mhi\x1b[0m");
    test_color!(bk_on_br_wh: black, on_bright_white,   "hi" => "\x1b[30;107mhi\x1b[0m");
}

mod color256 {
    test_color!(color256_1: color256(156), on_color256(223),
        "hi" => "\x1b[38;5;156;48;5;223mhi\x1b[0m"
    );
    test_color!(color256_2: color256(0), on_color256(255),
        "hi" => "\x1b[38;5;0;48;5;255mhi\x1b[0m"
    );
    test_color!(color256_3: color256(255), on_color256(0),
        "hi" => "\x1b[38;5;255;48;5;0mhi\x1b[0m"
    );
    test_color!(color256_4: color256(12), on_color256(123),
        "hi" => "\x1b[38;5;12;48;5;123mhi\x1b[0m"
    );
    test_color!(color256_5: color256(1), on_color256(200),
        "hi" => "\x1b[38;5;1;48;5;200mhi\x1b[0m"
    );
}

mod rgb {
    test_color!(rgb_1: rgb(0, 5, 255), on_rgb(43, 34, 214),
        "hi" => "\x1b[38;2;0;5;255;48;2;43;34;214mhi\x1b[0m"
    );
    test_color!(rgb_2: rgb(0, 255, 0), on_rgb(255, 0, 255),
        "hi" => "\x1b[38;2;0;255;0;48;2;255;0;255mhi\x1b[0m"
    );
    test_color!(rgb_3: rgb(255, 0, 255), on_rgb(0, 255, 0),
        "hi" => "\x1b[38;2;255;0;255;48;2;0;255;0mhi\x1b[0m"
    );
    test_color!(rgb_4: rgb(243, 15, 41), on_rgb(1, 2, 3),
        "hi" => "\x1b[38;2;243;15;41;48;2;1;2;3mhi\x1b[0m"
    );
    test_color!(rgb_5: rgb(111, 222, 232), on_rgb(222, 123, 255),
        "hi" => "\x1b[38;2;111;222;232;48;2;222;123;255mhi\x1b[0m"
    );
}

mod text_attrs {
    test_color!(bold:     bold, cyan, on_black,       "hi" => "\x1b[1;36;40mhi\x1b[0m");
    test_color!(dim:      dim, cyan, on_black,        "hi" => "\x1b[2;36;40mhi\x1b[0m");
    test_color!(ital:     italics, cyan, on_black,    "hi" => "\x1b[3;36;40mhi\x1b[0m");
    test_color!(ul:       underlined, cyan, on_black, "hi" => "\x1b[4;36;40mhi\x1b[0m");
    test_color!(inv:      inverted, cyan, on_black,   "hi" => "\x1b[7;36;40mhi\x1b[0m");
    test_color!(hid:      hidden, cyan, on_black,     "hi" => "\x1b[8;36;40mhi\x1b[0m");
    test_color!(strk:     strike, cyan, on_black,     "hi" => "\x1b[9;36;40mhi\x1b[0m");
    test_color!(no_fg:    bold, current, on_black,    "hi" => "\x1b[1;40mhi\x1b[0m");
    test_color!(no_bg:    bold, cyan, on_current,     "hi" => "\x1b[1;36mhi\x1b[0m");
    test_color!(no_fg_bg: bold, current, on_current,  "hi" => "\x1b[1mhi\x1b[0m");
}

mod write_style {
    use std::io::Write;
    test_write_style!(mg_on_bk: magenta, on_black,        "hi" => "\x1b[35;40mhi\x1b[0m");
    test_write_style!(br_rd_on_bk: bright_red, on_black,  "hi" => "\x1b[91;40mhi\x1b[0m");
    test_write_style!(bk_on_br_bl: black, on_bright_blue, "hi" => "\x1b[30;104mhi\x1b[0m");
    test_write_style!(bold: bold, cyan, on_black,         "hi" => "\x1b[1;36;40mhi\x1b[0m");
    test_write_style!(no_fg: bold, current, on_black,     "hi" => "\x1b[1;40mhi\x1b[0m");
    test_write_style!(color256: color256(156), on_color256(223),
        "hi" => "\x1b[38;5;156;48;5;223mhi\x1b[0m"
    );
    test_write_style!(rgb: rgb(0, 5, 255), on_rgb(43, 34, 214),
        "hi" => "\x1b[38;2;0;5;255;48;2;43;34;214mhi\x1b[0m"
    );
}

mod writeln_style {
    use std::io::Write;
    test_writeln_style!(mg_on_bk: magenta, on_black,        "hi" => "\x1b[35;40mhi\x1b[0m\n");
    test_writeln_style!(br_rd_on_bk: bright_red, on_black,  "hi" => "\x1b[91;40mhi\x1b[0m\n");
    test_writeln_style!(bk_on_br_bl: black, on_bright_blue, "hi" => "\x1b[30;104mhi\x1b[0m\n");
    test_writeln_style!(bold: bold, cyan, on_black,         "hi" => "\x1b[1;36;40mhi\x1b[0m\n");
    test_writeln_style!(no_fg: bold, current, on_black,     "hi" => "\x1b[1;40mhi\x1b[0m\n");
    test_writeln_style!(color256: color256(156), on_color256(223),
        "hi" => "\x1b[38;5;156;48;5;223mhi\x1b[0m\n"
    );
    test_writeln_style!(rgb: rgb(0, 5, 255), on_rgb(43, 34, 214),
        "hi" => "\x1b[38;2;0;5;255;48;2;43;34;214mhi\x1b[0m\n"
    );
}
