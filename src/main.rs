mod analysis;

use analysis::*;

fn main() {
    let i = analysis::irssi::Irssi;

    let res = match_line(&i, "hehehe").unwrap();
    let ras = match_line(&i, "hahaha").unwrap();
    println!("{}", res);
    println!("{}", ras);
}
