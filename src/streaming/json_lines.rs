//! Newline-delimited JSON framing over a byte stream.

use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, Bytes, BytesMut};
use futures_core::Stream;
use serde::de::DeserializeOwned;

use crate::error::Error;

/// Adapts a stream of byte chunks (as produced by
/// [`reqwest::Response::bytes_stream`]) into a stream of JSON values, one
/// per newline-terminated line.
///
/// Lines may be split across chunk boundaries and multiple lines may share
/// a chunk; both are handled. Empty lines are skipped, `\r\n` is accepted,
/// and a trailing unterminated line is parsed when the inner stream ends.
pub(crate) struct JsonLines<S, T> {
    inner: S,
    buffer: BytesMut,
    ended: bool,
    _item: PhantomData<fn() -> T>,
}

impl<S, T> JsonLines<S, T> {
    pub(crate) fn new(inner: S) -> Self {
        JsonLines {
            inner,
            buffer: BytesMut::new(),
            ended: false,
            _item: PhantomData,
        }
    }
}

impl<S, T> JsonLines<S, T>
where
    T: DeserializeOwned,
{
    /// Takes the next complete line out of the buffer, if any.
    fn next_buffered_line(&mut self) -> Option<Result<T, Error>> {
        while let Some(newline) = self.buffer.iter().position(|&b| b == b'\n') {
            let line = self.buffer.split_to(newline + 1);
            match parse_line(&line[..newline]) {
                Some(result) => return Some(result),
                None => continue, // blank line
            }
        }
        None
    }

    /// Parses whatever remains in the buffer once the inner stream ended.
    fn final_line(&mut self) -> Option<Result<T, Error>> {
        let rest = self.buffer.split();
        parse_line(&rest)
    }
}

fn parse_line<T: DeserializeOwned>(mut line: &[u8]) -> Option<Result<T, Error>> {
    if line.ends_with(b"\r") {
        line = &line[..line.len() - 1];
    }
    if line.iter().all(u8::is_ascii_whitespace) {
        return None;
    }
    Some(
        serde_json::from_slice(line).map_err(|source| Error::Decode {
            source,
            body: String::from_utf8_lossy(line).into_owned(),
        }),
    )
}

impl<S, T> Stream for JsonLines<S, T>
where
    S: Stream<Item = reqwest::Result<Bytes>> + Unpin,
    T: DeserializeOwned,
{
    type Item = Result<T, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            if let Some(line) = this.next_buffered_line() {
                return Poll::Ready(Some(line));
            }
            if this.ended {
                return Poll::Ready(this.final_line());
            }
            match Pin::new(&mut this.inner).poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    this.buffer.extend_from_slice(chunk.chunk());
                }
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(Error::Transport(e)))),
                Poll::Ready(None) => {
                    this.ended = true;
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Line {
        n: u32,
    }

    async fn collect(chunks: Vec<&'static [u8]>) -> Vec<Result<Line, Error>> {
        let inner = futures_util::stream::iter(
            chunks
                .into_iter()
                .map(|c| Ok::<_, reqwest::Error>(Bytes::from_static(c))),
        );
        JsonLines::new(inner).collect().await
    }

    #[tokio::test]
    async fn multiple_lines_in_one_chunk() {
        let items = collect(vec![b"{\"n\":1}\n{\"n\":2}\n{\"n\":3}\n"]).await;
        let values: Vec<u32> = items.into_iter().map(|r| r.unwrap().n).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn line_split_across_chunks() {
        let items = collect(vec![b"{\"n\"", b":1", b"}\n{\"n\":2}", b"\n"]).await;
        let values: Vec<u32> = items.into_iter().map(|r| r.unwrap().n).collect();
        assert_eq!(values, vec![1, 2]);
    }

    #[tokio::test]
    async fn crlf_and_empty_lines_are_tolerated() {
        let items = collect(vec![b"{\"n\":1}\r\n\r\n\n{\"n\":2}\r\n"]).await;
        let values: Vec<u32> = items.into_iter().map(|r| r.unwrap().n).collect();
        assert_eq!(values, vec![1, 2]);
    }

    #[tokio::test]
    async fn trailing_unterminated_line_is_parsed_at_end() {
        let items = collect(vec![b"{\"n\":1}\n{\"n\":", b"2}"]).await;
        let values: Vec<u32> = items.into_iter().map(|r| r.unwrap().n).collect();
        assert_eq!(values, vec![1, 2]);
    }

    #[tokio::test]
    async fn malformed_line_yields_decode_error_and_continues() {
        let items = collect(vec![b"not json\n{\"n\":7}\n"]).await;
        assert!(matches!(items[0], Err(Error::Decode { .. })));
        assert_eq!(items[1].as_ref().unwrap().n, 7);
    }
}
