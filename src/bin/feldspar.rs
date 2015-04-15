extern crate feldspar;

use feldspar::*;

fn main() {
    let i = analysis::irssi::Irssi;

    let res = analysis::churn::match_line(&i, "hehehe").unwrap();
    let ras = analysis::churn::match_line(&i, "hahaha").unwrap();
    println!("{}", res);
    println!("{}", ras);
}

