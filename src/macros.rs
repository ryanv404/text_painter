/// Wrapper around `print!()` to print styled text to the standard out.
#[macro_export]
macro_rules! print_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {
        print!("{}", $crate::Style::new($text).$fg($fg_num).$bg($bg_num));
    };
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {
        print!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    };
    ($fg:ident, $bg:ident, $text:expr) => {
        print!("{}", $crate::Style::new($text).$fg().$bg());
    };
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {
        print!("{}", $crate::Style::new($text).$attr().$fg().$bg());
    };
}

/// Wrapper around `println!()` to print styled text to the standard out with a newline.
#[macro_export]
macro_rules! println_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {
        println!("{}", $crate::Style::new($text).$fg($fg_num).$bg($bg_num));
    };
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {
        println!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    };
    ($fg:ident, $bg:ident, $text:expr) => {
        println!("{}", $crate::Style::new($text).$fg().$bg());
    };
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {
        println!("{}", $crate::Style::new($text).$attr().$fg().$bg());
    };
}

/// Wrapper around `eprint!()` to print styled text to the standard error.
#[macro_export]
macro_rules! eprint_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {
        eprint!("{}", $crate::Style::new($text).$fg($fg_num).$bg($bg_num));
    };
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {
        eprint!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    };
    ($fg:ident, $bg:ident, $text:expr) => {
        eprint!("{}", $crate::Style::new($text).$fg().$bg());
    };
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {
        eprint!("{}", $crate::Style::new($text).$attr().$fg().$bg());
    };
}

/// Wrapper around `eprintln!()` to print styled text to the standard error with a newline.
#[macro_export]
macro_rules! eprintln_style {
    ($fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {
        eprintln!("{}", $crate::Style::new($text).$fg($fg_num).$bg($bg_num));
    };
    ($fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {
        eprintln!(
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        );
    };
    ($fg:ident, $bg:ident, $text:expr) => {
        eprintln!("{}", $crate::Style::new($text).$fg().$bg());
    };
    ($attr:ident, $fg:ident, $bg:ident, $text:expr) => {
        eprintln!("{}", $crate::Style::new($text).$attr().$fg().$bg());
    };
}

/// Wrapper around `write!()` to write styled text to a buffer.
#[macro_export]
macro_rules! write_style {
    ($dest:expr,
     $fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        if let Err(e) = write!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        ) {
            panic!("`write_style!()` error while calling `write!()`. {e}");
        };
    }};
    ($dest:expr,
     $fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        if let Err(e) = write!(
            $dest,
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        ) {
            panic!("`write_style!()` error while calling `write!()`. {e}");
        };
    }};
    ($dest:expr, $fg:ident, $bg:ident, $text:expr) => {{
        if let Err(e) = write!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg().$bg()
        ) {
            panic!("`write_style!()` error while calling `write!()`. {e}");
        };
    }};
    ($dest:expr, $attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        if let Err(e) = write!(
            $dest,
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        ) {
            panic!("`write_style!()` error while calling `write!()`. {e}");
        };
    }};
}

/// Wrapper around `write!()` to write styled text to a buffer with a newline.
#[macro_export]
macro_rules! writeln_style {
    ($dest:expr,
     $fg:ident($fg_num:expr), $bg:ident($bg_num:expr),
     $text:expr) => {{
        if let Err(e) = writeln!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg($fg_num).$bg($bg_num)
        ) {
            panic!("`writeln_style!()` error while calling `writeln!()`. {e}");
        };
    }};
    ($dest:expr,
     $fg:ident($fg_r:expr, $fg_g:expr, $fg_b:expr),
     $bg:ident($bg_r:expr, $bg_g:expr, $bg_b:expr),
     $text:expr) => {{
        if let Err(e) = writeln!(
            $dest,
            "{}",
            $crate::Style::new($text)
                .$fg($fg_r, $fg_g, $fg_b)
                .$bg($bg_r, $bg_g, $bg_b)
        ) {
            panic!("`writeln_style!()` error while calling `writeln!()`. {e}");
        };
    }};
    ($dest:expr, $fg:ident, $bg:ident, $text:expr) => {{
        if let Err(e) = writeln!(
            $dest,
            "{}",
            $crate::Style::new($text).$fg().$bg()
        ) {
            panic!("`writeln_style!()` error while calling `writeln!()`. {e}");
        };
    }};
    ($dest:expr, $attr:ident, $fg:ident, $bg:ident, $text:expr) => {{
        if let Err(e) = writeln!(
            $dest,
            "{}",
            $crate::Style::new($text).$attr().$fg().$bg()
        ) {
            panic!("`writeln_style!()` error while calling `writeln!()`. {e}");
        };
    }};
}

/// Prepend the ANSI Control Sequence Introducer ("\x1b[") sequence to the
/// provided arguments.
#[macro_export]
macro_rules! ansi_csi {
    () => { "\x1b[" };
    ($( $arg:expr ),*) => {
        concat!("\x1b[", $( $arg ),*)
    };
}

/// Constructs an ANSI Select Graphic Rendition sequence by prepending the CSI to
/// the provided arguments followed by appending the character 'm'.
#[macro_export]
macro_rules! ansi_sgr {
    () => {};
    ($( $arg:expr ),*) => {
        concat!("\x1b[", $( $arg ),*, "m")
    };
}
