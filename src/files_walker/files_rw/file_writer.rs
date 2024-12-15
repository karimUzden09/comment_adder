use std::fs::File;

use mktemp::Temp;

pub fn t() {
    let mut tmp_path = Temp::new_file().unwrap();

    let mut tmp = File::create(&tmp_path.release()).unwrap();
    //tmp.w
}
