#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Instant;

use itertools::Itertools;
use proconio::source::once::OnceSource;

pub fn dir_io_pairs(dir: &str) -> impl Iterator<Item=(impl AsRef<Path>, impl AsRef<Path>)>
{
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new(dir));
    let in_dir_path  = base_path.join(Path::new("in"));
    let out_dir_path = base_path.join(Path::new("out"));

    let pairs = in_dir_path.read_dir().expect("Could not read input directory")
        .map(|dir_entry| {
            let dir_entry = dir_entry.expect("Could not read input directory entry");
            let in_file_path = dir_entry.path();
            let filename = dir_entry.file_name();
            let out_file_path = out_dir_path.join(Path::new(&filename));

            (in_file_path, out_file_path)
        })
        .sorted().rev()
        .collect::<Vec<_>>();

    DirIOPairs { pairs, t: None }
}
struct DirIOPairs {
    pairs: Vec<(PathBuf, PathBuf)>,
    t: Option<Instant>,
}
impl Iterator for DirIOPairs {
    type Item = (PathBuf, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t0) = self.t {
            eprintln!(" dt = {} ms", t0.elapsed().as_millis());
        }
        self.t = Some(Instant::now());

        let u = self.pairs.pop()?;
        eprint!("{} ...", u.0.file_stem().unwrap().to_string_lossy());
        Some(u)
    }
}

pub fn source_from_path(path: impl AsRef<Path>) -> OnceSource<BufReader<File>>
{
    OnceSource::new(BufReader::new(File::open(path).expect("Could not open file")))
}
