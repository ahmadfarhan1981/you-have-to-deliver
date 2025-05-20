use tracing::info;
use unicode_segmentation::UnicodeSegmentation;
use crate::sim::utils::term::{bold, green, red, yellow};

pub fn print_banner() {
    //
    // const BANNER_LINE_1: &str = r#"░▒▓█▓▒░░▒▓█▓▒░                                ░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░      ░▒▓████████▓▒░▒▓██████▓▒░       ░▒▓███████▓▒░░▒▓████████▓▒░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓███████▓▒░  ░▒▓█▓▒░  "#;
    // const BANNER_LINE_2: &str = r#"░▒▓█▓▒░░▒▓█▓▒░                                ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░                ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░  "#;
    // const BANNER_LINE_3: &str = r#"░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒▒▓█▓▒░░▒▓█▓▒░                ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░  "#;
    // const BANNER_LINE_4: &str = r#" ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓████████▓▒░▒▓████████▓▒░░▒▓█▓▒▒▓█▓▒░░▒▓██████▓▒░           ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓██████▓▒░ ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒▒▓█▓▒░░▒▓██████▓▒░ ░▒▓███████▓▒░  ░▒▓█▓▒░  "#;
    // pub const BANNER_LINE_5: &str = r#"   ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▓█▓▒░ ░▒▓█▓▒░                ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░ ░▒▓█▓▓█▓▒░ ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░  "#;
    // const BANNER_LINE_6: &str = r#"   ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▓█▓▒░ ░▒▓█▓▒░                ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░ ░▒▓█▓▓█▓▒░ ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░   "#;
    // const BANNER_LINE_7: &str = r#"   ░▒▓█▓▒░    ░▒▓██████▓▒░ ░▒▓██████▓▒░       ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓██▓▒░  ░▒▓████████▓▒░         ░▒▓█▓▒░   ░▒▓██████▓▒░       ░▒▓███████▓▒░░▒▓████████▓▒░▒▓████████▓▒░▒▓█▓▒░  ░▒▓██▓▒░  ░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░  "#;
    //
    let phrase = format!(
        "   {} {} {}",
        yellow("YOU"),
        bold(&red("HAVE")),
        green("to DELIVER!"),

    );
    // fn print_colored_banner_line(line: &str, a: usize, b: usize) {
    //     let graphemes: Vec<_> = line.graphemes(true).collect();
    //
    //     let left = graphemes[..a].concat();
    //     let mid = graphemes[a..b].concat();
    //     let right = graphemes[b..].concat();
    //
    //     info!(
    //         "{}{}{}",
    //         left.cyan(),
    //         mid.red(),
    //         right.cyan()
    //     );
    // }
    //
    // print_colored_banner_line(BANNER_LINE_1, 44, 103);
    // print_colored_banner_line(BANNER_LINE_2, 44, 103);
    // print_colored_banner_line(BANNER_LINE_3, 44, 103);
    // print_colored_banner_line(BANNER_LINE_4, 44, 103);
    // print_colored_banner_line(BANNER_LINE_5, 44, 103);
    // print_colored_banner_line(BANNER_LINE_6, 44, 103);
    // print_colored_banner_line(BANNER_LINE_7, 44, 103);

    info!("{}", phrase);
}
