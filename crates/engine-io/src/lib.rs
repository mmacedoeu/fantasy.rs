//! * Engine-io
//! This is the library providing connectivity with stdin stdout transport.
//! It is based on Actor model https://en.wikipedia.org/wiki/Actor_model
//! leveraging years of battle tested use with several languages and mostly
//! with erlang in the telecomunication industry proving to handle millions
//! of messages.
#![cfg_attr(feature="flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;
extern crate actix;
extern crate bytes;
extern crate core;
extern crate failure;
extern crate futures;
extern crate tokio_codec;

use actix::{Actor, Handler, SyncContext};
use bytes::BytesMut;
use core::{ClientAction, GetPlayerInfoMsg, PlayerInfo};
use failure::Error;
use std::io;
use std::str::FromStr;
use tokio_codec::{Decoder, Encoder, LinesCodec};

// Main Stdin Stdout Actor placeholder
pub struct EnginePipeIo;

/// Turn EnginePipeIo into Actor enabled
impl Actor for EnginePipeIo {
  // Define this Actor as being Sync, which means it will
  // use true thread-pool instead of light task / green threads
  // This design decision is based on common practices to handle
  // Io on one or two real thread's to achieve high throughput 
  type Context = SyncContext<Self>;
}

impl EnginePipeIo {
  // Inner basic implementation of sending data to stdout
  pub fn send(&mut self, action: ClientAction) -> Result<(), Error> {
    println!("{}", action);
    Ok(())
  }
}

/// Message handling for type ClientAction
impl Handler<ClientAction> for EnginePipeIo {
  type Result = Result<(), Error>;

  // this is the standard method signature for message handling of Actor
  // MessageBox
  #[cfg_attr(feature = "flame_it", flame)]
  fn handle(&mut self, msg: ClientAction, _: &mut Self::Context) -> Self::Result {
    Ok(self.send(msg)?)
  }
}

/// Message handling for type ClientAction
impl Handler<GetPlayerInfoMsg> for EnginePipeIo {
  type Result = Result<PlayerInfo, Error>;

  #[cfg_attr(feature = "flame_it", flame)]
  fn handle(&mut self, _msg: GetPlayerInfoMsg, _: &mut Self::Context) -> Self::Result {
    let mut input = String::new();
    // there is not problem blocking the thread as it using a real thread-pool
    io::stdin().read_line(&mut input)?;
    Ok(PlayerInfo::from_str(input.as_ref())?)
  }
}

// Encoding Decoding protocol for PlayerInfo data
pub struct PlayerInfoCodec(pub LinesCodec);

impl Decoder for PlayerInfoCodec {
  type Item = PlayerInfo;
  type Error = std::io::Error;

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

// Smoke test to see module is building fine
// Decide not go further testing here due to simplicity of functionality
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
