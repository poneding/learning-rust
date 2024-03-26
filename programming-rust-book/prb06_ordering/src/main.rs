use std::cmp::Ordering;
use std::io;
fn main() {
    show_files().expect("none");
}

struct FileInfo {
    name: String,
    timestamp: i64,
}

fn show_files() -> io::Result<()> {
    let mut v = vec![
        FileInfo {
            name: "file1".to_string(),
            timestamp: 1000,
        },
        FileInfo {
            name: "file3".to_string(),
            timestamp: 2000,
        },
        FileInfo {
            name: "file2".to_string(),
            timestamp: 2000,
        },
    ];

    fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        a.timestamp
            .cmp(&b.timestamp)
            .reverse()
            .then(a.name.cmp(&b.name))
    }
    v.sort_by(cmp_by_timestamp_then_name);

    println!("Files:");
    for f in &v {
        println!("{}: {}", f.timestamp, f.name);
    }
    Ok(())
}

// 0..20 => std::ops::Range{start:0, end:20}
