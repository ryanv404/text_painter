use text_styler::{print_style, println_style};

fn main() {
    let total_cols = 80;
    for col in 0..total_cols {
        let col = col as u32;

        let r = 255 - (col * 255 / total_cols);
        let r: u8 = r.try_into().unwrap();

        let g = col * 510 / total_cols;
        let g = if g > 255 { 510 - g } else { g };
        let g: u8 = g.try_into().unwrap();

        let b = col * 255 / total_cols;
        let b: u8 = b.try_into().unwrap();

        if col == (total_cols - 1) {
            println_style!(
                rgb(r, g, b),
                on_rgb(255 - r, 255 - g, 255 - b),
                " "
            );
        } else {
            print_style!(
                rgb(r, g, b),
                on_rgb(255 - r, 255 - g, 255 - b),
                " "
            );
        }
    }
}
