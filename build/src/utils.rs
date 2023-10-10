use std::process::{Command, Stdio};
use std::thread;
use std::io::Write;
use eyre::{ContextCompat, WrapErr};

pub fn chunk_with<I>(iter: impl Iterator<Item = I>, callback: impl Fn(&I) -> bool) -> Vec<Vec<I>> {
    let mut chunks = Vec::new();
    let mut chunk = Vec::new();

    for item in iter {
        if callback(&item) && !chunk.is_empty() {
            chunks.push(chunk);
            chunk = Vec::new();
        }
        chunk.push(item);
    }

    if !chunk.is_empty() {
        chunks.push(chunk);
    }

    chunks
}

pub trait IterExt: Iterator {
    fn chunk_with(self, callback: impl Fn(&Self::Item) -> bool) -> Vec<Vec<Self::Item>>;
}

impl<T: Iterator> IterExt for T {
    fn chunk_with(self, callback: impl Fn(&Self::Item) -> bool) -> Vec<Vec<Self::Item>> {
        chunk_with(self, callback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_works() {
        let data = vec![1, 2, 1, 4, 5, 1, 6, 9];
        let chunks = chunk_with(data.into_iter(), |i| *i == 1);
        assert_eq!(chunks, vec![vec![1, 2], vec![1, 4, 5], vec![1, 6, 9]]);
    }
}

pub fn format_tokens(tokens: String) -> eyre::Result<String> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .wrap_err("Failed to spawn child process")?;

    let mut stdin = child.stdin.take().wrap_err("Failed to open stdin")?;

    thread::spawn(move || {
        stdin
            .write_all(tokens.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output().wrap_err("Failed to read stdout")?;
    let tokens = String::from_utf8(output.stdout).wrap_err("Failed to convert output to string")?;

    Ok(tokens)
}
