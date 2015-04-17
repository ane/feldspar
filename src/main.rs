extern crate feldspar;

use feldspar::analysis;

fn main() {
    let i = analysis::irssi::Irssi;

    let ras = analysis::churn::match_line(&i, "hahaha").unwrap();
    if let Some(res) = analysis::churn::match_line(&i, "13:23 <@ane> hello there i am bob") {
        println!("{}", res);
    }
    println!("{}", ras);
}

