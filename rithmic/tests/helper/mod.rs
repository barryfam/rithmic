#![allow(dead_code)]
#![allow(clippy::print_stderr)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::path::{Path, PathBuf};
use std::time::Instant;

use itertools::Itertools;
use proconio::source::once::OnceSource;
use zip::ZipArchive;

pub fn dir_io_pairs(dir: impl AsRef<Path>) -> impl Iterator<Item=(impl IntoSource, impl IntoSource)>
{
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
    let in_dir_path  = base_path.join("in");
    let out_dir_path = base_path.join("out");

    let pairs = in_dir_path.read_dir().expect("could not read input directory")
        .map(|dir_entry| {
            let dir_entry = dir_entry.expect("could not read input directory entry");
            let in_file_path = dir_entry.path();
            let filename = dir_entry.file_name();
            let out_file_path = out_dir_path.join(filename);

            let desc = in_file_path.file_stem().unwrap().to_string_lossy().to_string();

            (in_file_path, out_file_path, desc)
        })
        .sorted();

    IOPairs::from_iter(pairs)
}

pub fn zip_io_pairs(zipfile: impl AsRef<Path>) -> impl Iterator<Item=(impl IntoSource, impl IntoSource)>
{
    let mut arch = ZipArchive::new(File::open(zipfile).expect("could not open file")).expect("could not read archive");

    let mut pairs = HashMap::<String, [Option<_>; 2]>::new();
    for i in 0..arch.len() {
        let mut f = arch.by_index(i).expect("could not open file inside archive");

        if f.is_dir() { continue }

        let Some((stem, suffix)) = f.name().rsplit_once('.')
            else { continue };

        let e = pairs.entry(stem.to_string()).or_default();
        let io = match suffix {
            "in" => 0,
            "out" => 1,
            _ => continue
        };

        let mut b = Vec::with_capacity(f.size().try_into().expect("file size overflow"));
        f.read_to_end(&mut b).expect("could not read file inside archive");
        e[io] = Some(b);
    }

    let pairs = pairs.into_iter()
        .sorted_by_key(|(k, _)| k.parse::<usize>().unwrap_or(usize::MAX))
        .filter_map(|kuv| if let (k, [Some(u), Some(v)]) = kuv { Some((u, v, k)) } else { None });

    IOPairs::from_iter(pairs)
}

pub trait IntoSource {
    type R: Read;
    fn into_read(self) -> Self::R;

    fn into_source(self) -> OnceSource<BufReader<Self::R>>
    where Self: Sized
    {
        OnceSource::new(BufReader::new(self.into_read()))
    }
}

impl IntoSource for PathBuf {
    type R = File;
    fn into_read(self) -> Self::R {
        File::open(self).expect("could not open file")
    }
}

impl IntoSource for Vec<u8> {
    type R = Cursor<Self>;
    fn into_read(self) -> Self::R {
        Cursor::new(self)
    }
}

struct IOPairs<T: IntoSource>
{
    stack: Vec<(T, T, String)>,
    t: Option<Instant>,
}

impl<T: IntoSource> IOPairs<T> {
    pub fn from_iter(pairs: impl IntoIterator<Item=(T, T, String)>) -> Self
    {
        let mut stack = pairs.into_iter().collect::<Vec<_>>();
        stack.reverse();

        Self { stack, t: None }
    }
}

impl<T: IntoSource> Iterator for IOPairs<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t0) = self.t {
            eprintln!(" dt = {} ms", t0.elapsed().as_millis());
        }
        self.t = Some(Instant::now());

        let u = self.stack.pop()?;
        eprint!("{} ...", u.2);
        Some((u.0, u.1))
    }
}
