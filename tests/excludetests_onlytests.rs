use crate::util::{Dir, TestCommand};

const TEST_STR: &'static str = "fn main() {
    // comment 1
}

#[test]
use std::fs;

// comment 2

#[test]
fn test1() {
    // comment 3
}

#[cfg(test)]
mod tests {
    use std::fs;
    // comment 4

    #[test]
    fn test1() {
        // comment 5
    }
}// comment 6";

// This tests that --excludetests works properly (find "comment")
rgtest!(excludetests1, |dir: Dir, mut cmd: TestCommand| {
    dir.create("test", TEST_STR);
    cmd.arg("-n")
        .arg("--excludetests")
        .arg("comment")
        .arg("test");
    eqnice!(
        "2:    // comment 1\n8:// comment 2\n24:}// comment 6\n",
        cmd.stdout()
    );
});

// This tests that --excludetests works properly (find "std::fs")
rgtest!(excludetests2, |dir: Dir, mut cmd: TestCommand| {
    dir.create("test", TEST_STR);
    cmd.arg("-n")
        .arg("--excludetests")
        .arg("std::fs")
        .arg("test");
    cmd.assert_err();
});

// This tests that --onlytests works properly. (find "comment")
rgtest!(onlytests1, |dir: Dir, mut cmd: TestCommand| {
    dir.create("test", TEST_STR);
    cmd.arg("-n").arg("--onlytests").arg("comment").arg("test");
    eqnice!(
        "12:    // comment 3\n18:    // comment 4\n22:        // comment 5\n",
        cmd.stdout()
    );
});

// This tests that --excludetests works properly (find "std::fs")
rgtest!(onlytests2, |dir: Dir, mut cmd: TestCommand| {
    dir.create("test", TEST_STR);
    cmd.arg("-n").arg("--onlytests").arg("std::fs").arg("test");
    eqnice!("6:use std::fs;\n17:    use std::fs;\n", cmd.stdout());
});
