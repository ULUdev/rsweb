use flate2::read::{DeflateDecoder, GzDecoder};
// use lzw::Decoder as LzwDecoder;
// use lzw::LsbReader;
use super::header::ContentEncodingMethod;
use brotli::Decompressor as BrotliDecompressor;
use std::io::Error;
use std::io::IoSliceMut;
use std::io::Read;

/// an http body
#[derive(Clone, Debug)]
pub struct Body {
    content: Vec<u8>,
    encoding: Option<ContentEncodingMethod>,
}

impl Body {
    /// create an http body from some `content` that is an unencoded/compressed string
    pub fn new(content: String) -> Body {
        Body {
            content: content.bytes().collect(),
            encoding: None,
        }
    }

    /// create an http body from raw bytes (unencoded/compressed)
    pub fn from_bytes(bytes: Vec<u8>) -> Body {
        Body {
            content: bytes,
            encoding: None,
        }
    }

    /// create a body from bytes with a certain encoding/compression
    pub fn from_bytes_with_encoding(bytes: Vec<u8>, encoding: ContentEncodingMethod) -> Body {
        assert!(
            encoding != ContentEncodingMethod::Compress,
            "lzw compression is not supported"
        );
        Body {
            content: bytes,
            encoding: Some(encoding),
        }
    }

    /// return the content as bytes
    pub fn get_bytes(&self) -> Vec<u8> {
        self.content.clone()
    }

    /// try converting the raw bytes content to a string
    pub fn try_to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.content.clone())
    }

    /// decode the content and put it in memory.
    ///
    /// **WARNING**: encoded content might be a large file. Use this function with caution
    pub fn decode_into_memory(&self) -> Result<Vec<u8>, Error> {
        if self.encoding.is_none() {
            return Ok(self.content.clone());
        } else {
            match self.encoding.clone().unwrap() {
                ContentEncodingMethod::Gzip => {
                    let mut gzdecoder = GzDecoder::new(&self.content[..]);
                    let mut v: Vec<u8> = Vec::new();
                    let ios = IoSliceMut::new(&mut v);
                    match gzdecoder.read_vectored(&mut [ios]) {
                        Ok(_) => Ok(v),
                        Err(e) => Err(e),
                    }
                }
                ContentEncodingMethod::Compress => panic!("LZW is currently not supported"),
                ContentEncodingMethod::Deflate => {
                    let mut deflater = DeflateDecoder::new(&self.content[..]);
                    let mut v: Vec<u8> = Vec::new();
                    let ios = IoSliceMut::new(&mut v);
                    match deflater.read_vectored(&mut [ios]) {
                        Ok(_) => Ok(v),
                        Err(e) => Err(e),
                    }
                }
                ContentEncodingMethod::Br => {
                    let mut brdecompressor = BrotliDecompressor::new(&self.content[..], 32);
                    let mut v: Vec<u8> = Vec::new();
                    let ios = IoSliceMut::new(&mut v);
                    match brdecompressor.read_vectored(&mut [ios]) {
                        Ok(_) => Ok(v),
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }

    /// get the length of the decoded body
    ///
    /// Note: doesn't actually decode it fully. It decodes it byte by byte and counts the amount of
    /// bytes read
    pub fn decoded_len(&self) -> usize {
        panic!("not implemented yet!");
        // if self.encoding.is_none() {
        //     return self.content.len()
        // } else {
        //     match self.encoding.clone().unwrap() {
        //         ContentEncodingMethod::Gzip => (),
        //         ContentEncodingMethod::Compress => (),
        //         ContentEncodingMethod::Deflate => (),
        //         ContentEncodingMethod::Br => (),
        //     }
        // }
    }
}
