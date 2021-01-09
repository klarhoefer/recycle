
use std::env;

use shfileops::recycle;


fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let paths = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    recycle(&paths);
}
