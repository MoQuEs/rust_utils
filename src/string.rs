use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

pub fn mask_if_email(potential_email: impl AsRef<str>) -> String {
    if potential_email.as_ref().contains('@') {
        let mut parts = potential_email.as_ref().split('@');

        let mut new_string = String::new();
        new_string.push_str(&parts.next().unwrap()[0..1]);
        new_string.push_str("***OMITTED***");
        new_string.push('@');
        new_string.push_str(parts.next().unwrap());

        return new_string;
    }

    potential_email.as_ref().to_string()
}

pub fn slugify_for_filename(text: impl AsRef<str>) -> String {
    // '<>:"/\|?*' are not allowed in Windows filenames
    text.as_ref()
        .replace(['<', '>', ':', '"', '/', '\\', '?', '*', ' '], "_")
}

pub trait MaskIfEmail {
    fn mask_if_email(&self) -> String;
}

pub trait SlugifyForFilename {
    fn slugify_for_filename(&self) -> String;
}

impl MaskIfEmail for String {
    fn mask_if_email(&self) -> String {
        mask_if_email(self)
    }
}

impl MaskIfEmail for str {
    fn mask_if_email(&self) -> String {
        mask_if_email(self)
    }
}

impl SlugifyForFilename for String {
    fn slugify_for_filename(&self) -> String {
        slugify_for_filename(self)
    }
}

impl SlugifyForFilename for str {
    fn slugify_for_filename(&self) -> String {
        slugify_for_filename(self)
    }
}

#[derive(Clone)]
pub struct StringStreamSafe {
    pub inner: Arc<Mutex<String>>,
    pub enable_write: Arc<AtomicBool>,
}

impl StringStreamSafe {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn clear(&self) {
        self.inner.lock().unwrap().clear();
    }

    #[inline]
    pub fn write_from_stream(&self, s: &mut impl Read) -> std::io::Result<usize> {
        if !self.enable_write.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(0);
        }

        let mut inner = self.inner.lock().unwrap();
        s.read_to_string(&mut inner)
    }

    #[inline]
    pub fn dump(&self) -> String {
        self.inner.lock().unwrap().clone()
    }
}

impl Default for StringStreamSafe {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(String::with_capacity(0))),
            enable_write: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl Write for StringStreamSafe {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.enable_write.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(0);
        }

        let add = String::from_utf8_lossy(buf);

        let mut inner = self.inner.lock().unwrap();
        inner.reserve(add.len());
        inner.push_str(&add);

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for StringStreamSafe {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self.inner.lock().unwrap();
        let len = std::cmp::min(buf.len(), inner.len());
        let (a, b) = inner.as_bytes().split_at(len);

        buf[..len].copy_from_slice(a);
        *inner = String::from_utf8_lossy(b).to_string();

        Ok(len)
    }
}

impl Display for StringStreamSafe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.lock().unwrap();
        f.write_str(&inner)
    }
}

impl Debug for StringStreamSafe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.lock().unwrap();
        f.debug_tuple("StringStreamSafe").field(&inner).finish()
    }
}

impl From<String> for StringStreamSafe {
    fn from(value: String) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
            enable_write: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl From<&str> for StringStreamSafe {
    fn from(value: &str) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value.to_string())),
            enable_write: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl StringStreamSafe {
    pub fn deep_clone(&self) -> Self {
        Self {
            inner: Arc::new(Mutex::new(self.inner.lock().unwrap().clone())),
            enable_write: Arc::new(AtomicBool::new(
                self.enable_write.load(std::sync::atomic::Ordering::Relaxed),
            )),
        }
    }
}

unsafe impl Send for StringStreamSafe {}
unsafe impl Sync for StringStreamSafe {}

#[cfg(feature = "string-logger")]
impl log4rs::encode::Write for StringStreamSafe {}

#[derive(Clone)]
pub struct StringStream {
    inner: String,
    enable_write: bool,
}

impl StringStream {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn clear(&mut self) {}

    #[inline]
    pub fn write_from_stream(&mut self, s: &mut impl Read) -> std::io::Result<usize> {
        if !self.enable_write {
            return Ok(0);
        }

        s.read_to_string(&mut self.inner)
    }

    #[inline]
    pub fn dump(&self) -> String {
        self.inner.clone()
    }
}

impl Default for StringStream {
    fn default() -> Self {
        Self {
            inner: String::with_capacity(0),
            enable_write: true,
        }
    }
}

impl Write for StringStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.enable_write {
            return Ok(0);
        }

        let add = String::from_utf8_lossy(buf);

        self.inner.reserve(add.len());
        self.inner.push_str(&add);

        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for StringStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.inner.len());
        let (a, b) = self.inner.as_bytes().split_at(len);

        buf[..len].copy_from_slice(a);
        self.inner = String::from_utf8_lossy(b).to_string();

        Ok(len)
    }
}

impl Display for StringStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl Debug for StringStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StringStream").field(&self.inner).finish()
    }
}

impl From<String> for StringStream {
    fn from(value: String) -> Self {
        Self {
            inner: value,
            enable_write: true,
        }
    }
}

impl From<&str> for StringStream {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
            enable_write: true,
        }
    }
}

impl StringStream {
    pub fn deep_clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            enable_write: self.enable_write,
        }
    }
}

#[cfg(feature = "string-logger")]
impl log4rs::encode::Write for StringStream {}

#[cfg(feature = "string-logger")]
#[derive(Debug)]
pub struct StringStreamAppender {
    writer: Arc<Mutex<StringStream>>,
    encoder: Box<dyn log4rs::encode::Encode>,
    target: Box<String>,
}

#[cfg(feature = "string-logger")]
impl StringStreamAppender {
    pub fn new(
        writer: Arc<Mutex<StringStream>>,
        encoder: Box<dyn log4rs::encode::Encode>,
        target: Box<String>,
    ) -> Self {
        Self {
            writer,
            encoder,
            target,
        }
    }

    pub fn writer(mut self, writer: Arc<Mutex<StringStream>>) -> Self {
        self.writer = writer;
        self
    }

    pub fn encoder(mut self, encoder: Box<dyn log4rs::encode::Encode>) -> Self {
        self.encoder = encoder;
        self
    }

    pub fn target(mut self, target: Box<String>) -> Self {
        self.target = target;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[cfg(feature = "string-logger")]
unsafe impl Send for StringStreamAppender {}

#[cfg(feature = "string-logger")]
unsafe impl Sync for StringStreamAppender {}

#[cfg(feature = "string-logger")]
impl log4rs::append::Append for StringStreamAppender {
    fn append(&self, record: &log::Record) -> anyhow::Result<()> {
        if !self.target.is_empty() && record.target() != *self.target {
            return Ok(());
        }

        let mut w = self.writer.lock().unwrap();
        self.encoder.encode(&mut *w, record)?;

        Ok(())
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_email() {
        let value = mask_if_email("asd@a.tsa");
        assert_eq!(value, "a***OMITTED***@a.tsa");
    }

    #[test]
    fn is_not_email() {
        let value = mask_if_email("asda.tsa");
        assert_eq!(value, "asda.tsa");
    }

    #[test]
    fn is_email_str() {
        let value = "asd@a.tsa".mask_if_email();
        assert_eq!(value, "a***OMITTED***@a.tsa".to_string());
    }

    #[test]
    fn is_not_email_str() {
        let value = "asda.tsa".mask_if_email();
        assert_eq!(value, "asda.tsa".to_string());
    }

    #[test]
    fn is_email_string() {
        let value = "asd@a.tsa".to_string().mask_if_email();
        assert_eq!(value, "a***OMITTED***@a.tsa".to_string());
    }

    #[test]
    fn is_not_email_string() {
        let value = "asda.tsa".to_string().mask_if_email();
        assert_eq!(value, "asda.tsa".to_string());
    }

    #[test]
    fn slugify_for_filename_string() {
        let value = "a b".to_string().slugify_for_filename();
        assert_eq!(value, "a_b".to_string());
    }

    #[test]
    fn slugify_for_filename_str() {
        let value = "a b".slugify_for_filename();
        assert_eq!(value, "a_b".to_string());
    }

    #[test]
    fn slugify_for_filename_string_with_special_chars() {
        let value = "a b<>:\"/\\|?*".to_string().slugify_for_filename();
        assert_eq!(value, "a_b______|__".to_string());
    }

    #[test]
    fn slugify_for_filename_str_with_special_chars() {
        let value = "a b<>:\"/\\|?*".slugify_for_filename();
        assert_eq!(value, "a_b______|__".to_string());
    }

    #[test]
    fn string_stream_write() {
        let mut stream = StringStream::new();

        let result = stream.write(b"hello");
        assert_eq!(result.unwrap(), 5);

        let mut s = String::new();
        let result = stream.read_to_string(&mut s);
        assert_eq!(result.unwrap(), 5);
        assert_eq!(&s, "hello");
    }

    #[test]
    fn string_stream_read() {
        let mut stream = StringStream::new();
        stream.write(b"hello").unwrap();
        stream.write(b"hello").unwrap();

        let mut buffer = [0; 5];
        let result = stream.read(&mut buffer);
        assert_eq!(result.unwrap(), 5);
        assert_eq!(&buffer, b"hello");

        let mut buffer = Vec::new();
        let result = stream.read_to_end(&mut buffer);
        assert_eq!(result.unwrap(), 5);
        assert_eq!(&buffer, b"hello");
    }

    #[test]
    fn string_stream_read_to_end() {
        let mut stream = StringStream::new();
        stream.write(b"hello").unwrap();
        stream.write(b"hello").unwrap();

        let mut buffer = Vec::new();
        let result = stream.read_to_end(&mut buffer);
        assert_eq!(result.unwrap(), 10);
        assert_eq!(&buffer, b"hellohello");

        let mut buffer = Vec::new();
        let result = stream.read_to_end(&mut buffer);
        assert_eq!(result.unwrap(), 0);
        assert_eq!(&buffer, b"");
    }
}
