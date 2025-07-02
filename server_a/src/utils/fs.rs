use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::io::{Read, SeekFrom};
use std::time::UNIX_EPOCH;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};

#[allow(dead_code)]
pub struct FileStream {
    encolding: Option<&'static Encoding>,
    streamer: BufReader<File>,
    buf: Vec<u8>,
    out_line: String,
}

#[allow(dead_code)]
impl FileStream {
    const BUFFER_SIZE: usize = 8 * 1024;
    const EOF: u8 = "\n".as_bytes()[0];
    const WIN_EOF: u8 = "\r".as_bytes()[0];

    pub fn new(
        file_handle: File,
        encolding: Option<&'static Encoding>,
    ) -> Self {
        Self {
            encolding,
            streamer: BufReader::new(file_handle),
            buf: Vec::with_capacity(Self::BUFFER_SIZE),
            out_line: String::with_capacity(Self::BUFFER_SIZE),
        }
    }

    pub async fn reset_count_lines(&mut self) {
        let _ = self.streamer.seek(SeekFrom::Start(0)).await.unwrap();
    }

    pub async fn get_line(&mut self) -> Option<String> {
        self.buf.clear();
        self.out_line.clear();

        let _ = self
            .streamer
            .read_until(Self::EOF, &mut self.buf)
            .await
            .unwrap();

        match self.buf.len() {
            0 => None,
            _ => {
                self.buf = self
                    .buf
                    .iter()
                    .filter(|&&chair| chair != Self::EOF)
                    .copied()
                    .collect::<Vec<u8>>();
                let _ = DecodeReaderBytesBuilder::new()
                    .encoding(self.encolding)
                    .build(&self.buf as &[u8])
                    .read_to_string(&mut self.out_line)
                    .unwrap();

                Some(
                    String::from_utf8(
                        self.out_line
                            .to_string()
                            .as_bytes()
                            .iter()
                            .filter(|&&char| char != Self::WIN_EOF)
                            .copied()
                            .collect::<Vec<_>>(),
                    )
                    .unwrap(),
                )
            }
        }
    }
}

use gutils::{
    anyhow::Result,
    chrono::{DateTime, NaiveDateTime},
};

pub fn get_size(path: &str) -> Result<u64> {
    let size = std::fs::metadata(path)?.len();
    Ok(size)
}

pub fn get_last_modification(path: &str) -> Result<NaiveDateTime> {
    let time = std::fs::metadata(path)?.modified()?;
    let time = DateTime::from_timestamp(
        time.duration_since(UNIX_EPOCH)?.as_secs().try_into()?,
        0,
    );

    if let Some(time) = time {
        Ok(time.naive_local())
    } else {
        Err(anyhow::anyhow!("Error on parse date"))
    }
}
