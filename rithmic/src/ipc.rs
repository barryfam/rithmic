use std::io::{BufReader, BufRead};
use std::process::{Command, Stdio, ChildStdin, ChildStdout, Child};

use proconio::source::once::OnceSource;

/**
Inter-process communication via pipes with a spawned child command

The child's stdin & stdout can be accessed via the `tx` and `rx` fields. See also the [`input()`](Ipc::input) and [`input_line()`](Ipc::input_line) convenience methods. The child process is killed (if still alive) when this object is dropped

# Examples
```
use std::io::Write;
use std::process::Command;
# use rithmic::Ipc;
use proconio::input;

let mut ipc = Ipc::new(
    Command::new("python3")
        .arg("-c")
        .arg("print(int(input()) + 11)")
);

writeln!(&mut ipc.tx, "23");
input! { from ipc.input_line(), x: i32 };

assert_eq!(x, 34);
```
*/
pub struct Ipc {
    child: Child,
    pub tx: ChildStdin,
    pub rx: BufReader<ChildStdout>,
    line_buffer: String,
    raw_buffer: Vec<u8>,
}

impl Ipc {
    /// Spawn a child process and set up piped I/O to it. See [`Ipc`] for an example
    pub fn new(cmd: &mut Command) -> Self {
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().expect("failed to spawn child process");

        let tx = child.stdin.take().expect("failed to open child stdin");
        let rx = BufReader::new(child.stdout.take().expect("failed to open child stdout"));

        Self { child, tx, rx, line_buffer: String::new(), raw_buffer: vec![] }
    }

    /// Wait for the next line from the child process, then create a [`::proconio::source::Source`](https://docs.rs/proconio/latest/proconio/source/index.html) from it. See [`Ipc`] for an example
    pub fn input_line(&mut self) -> impl ::proconio::source::Source<&[u8]>
    {
        self.line_buffer.clear();
        match self.rx.read_line(&mut self.line_buffer) {
            Ok(0) => panic!("read EOF"),
            Err(_) => panic!("error reading input line"),
            _ => OnceSource::new(self.line_buffer.as_bytes())
        }
    }

    /// Read from child process until EOF (blocking), then create a [`::proconio::source::Source`](https://docs.rs/proconio/latest/proconio/source/index.html) from the read data
    pub fn input(&mut self) -> impl ::proconio::source::Source<&[u8]>
    {
        self.raw_buffer.clear();
        match self.rx.read_until(0x00, &mut self.raw_buffer) {
            Ok(0) => panic!("read EOF"),
            Err(_) => panic!("error reading input"),
            _ => OnceSource::new(&self.raw_buffer[..])
        }
    }
}

impl Drop for Ipc {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
