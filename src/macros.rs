/// Wrapper around `print!()` to print styled text to the standard out.
#[macro_export]
macro_rules! print_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        print!(
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        );
    }};
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        print!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    }};
    ($fg:ident, $bg:ident, $text:expr) => {{
        print!(
            "{}",
            $crate::Style::new($text).$fg().$bg()
        );
    }};
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        print!(
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        );
    }};
}

/// Wrapper around `println!()` to print styled text to the standard out with a newline.
#[macro_export]
macro_rules! println_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        println!(
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        );
    }};
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        println!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    }};
    ($fg:ident, $bg:ident, $text:expr) => {{
        println!(
            "{}",
            $crate::Style::new($text).$fg().$bg()
        );
    }};
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        println!(
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        );
    }};
}

/// Wrapper around `eprint!()` to print styled text to the standard error.
#[macro_export]
macro_rules! eprint_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        eprint!(
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        );
    }};
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        eprint!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    }};
    ($fg:ident, $bg:ident, $text:expr) => {{
        eprint!(
            "{}",
            $crate::Style::new($text).$fg().$bg()
        );
    }};
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        eprint!(
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        );
    }};
}

/// Wrapper around `eprintln!()` to print styled text to the standard error with a newline.
#[macro_export]
macro_rules! eprintln_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        eprintln!(
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        );
    }};
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        eprintln!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    }};
    ($fg:ident, $bg:ident, $text:expr) => {{
        eprintln!(
            "{}",
            $crate::Style::new($text).$fg().$bg()
        );
    }};
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        eprintln!(
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        );
    }};
}

/// Wrapper around `write!()` to write styled text to a buffer.
#[macro_export]
macro_rules! write_style {
    ($dest:expr,
     $fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        write!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        )
        .unwrap();
    }};
    ($dest:expr,
     $fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        write!(
            $dest,
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        )
        .unwrap();
    }};
    ($dest:expr, $fg:ident, $bg:ident, $text:expr) => {{
        write!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg().$bg()
        )
        .unwrap();
    }};
    ($dest:expr, $attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        write!(
            $dest,
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        )
        .unwrap();
    }};
}

/// Wrapper around `write!()` to write styled text to a buffer with a newline.
#[macro_export]
macro_rules! writeln_style {
    ($dest:expr,
     $fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        writeln!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        )
        .unwrap();
    }};
    ($dest:expr,
     $fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        writeln!(
            $dest,
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        )
        .unwrap();
    }};
    ($dest:expr, $fg:ident, $bg:ident, $text:expr) => {{
        writeln!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg().$bg()
        )
        .unwrap();
    }};
    ($dest:expr, $attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        writeln!(
            $dest,
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        )
        .unwrap();
    }};
}
