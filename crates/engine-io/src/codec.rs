
use bytes::BytesMut;
use tokio_codec::{Decoder, Encoder, LinesCodec};
use core::{ClientAction, PlayerInfo};
use std::io;

// Encoding Decoding protocol for PlayerInfo data
pub struct PlayerInfoCodec(pub LinesCodec);

impl Decoder for PlayerInfoCodec {
  type Item = PlayerInfo;
  type Error = io::Error;

  // uses an inner LineCodec to convert bytes to String and later convert
  // from string to PlayerInfo
  fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<PlayerInfo>> {
    self
      .0
      .decode(buf)
      // PlayerInfo::from call converter defined in core 
      .map(|ol| ol.map(|l| PlayerInfo::from(l.as_ref())))
  }
}

pub struct ClientActionCodec(pub LinesCodec);

impl Encoder for ClientActionCodec {
  type Item = ClientAction;
  type Error = io::Error;

  // encode to String and later from String to bytes using the inner LinesCodec
  fn encode(&mut self, item: ClientAction, buf: &mut BytesMut) -> Result<(), io::Error> {
    let line: String = format!("{}", item);
    self.0.encode(line, buf)
  }
}