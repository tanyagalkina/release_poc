use std::path::Path;

pub fn _is_not_greater_then_20(test: f64) -> bool {
    test <= 20.0
}

fn _read_file_rm_last(path: &Path) -> String {
    let mut res = std::fs::read_to_string(path).unwrap();
    res.pop();
    res
}

fn _go_to_this_func() {}

fn _go_to_that_func() {}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("+++ exited with {:?} +++", 42);
}

// clean comment, shoud look like this
// why auto bumping does not work ?
// one more smart-release try
// no, this does not work
// maybe we should not use git gui to generate releases, maybe thi sis what breaks the
// will release now with cargo release + generate CHANGELOG with cargo changelog
// installed correct gh tool
// after release with gh, maybe smart-release will retrive the correct version ? 
#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn _read_file_rm_last_test() {
        let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
        file.write_str("1234abcd").unwrap();
        assert_eq!(_read_file_rm_last(file.path()), "1234abc")
    }

    // is cool ... glint ... commitizen 
    #[test]
    fn go_to_this_func_test() {
        matches!(_go_to_this_func(), ());
        matches!(_go_to_that_func(), ());
    }
}
