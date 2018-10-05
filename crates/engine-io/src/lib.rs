//! * Engine-io
//! This is the library providing connectivity with stdin stdout transport.
//! It is based on Actor model https://en.wikipedia.org/wiki/Actor_model
//! leveraging years of battle tested use with several languages and mostly
//! with erlang in the telecomunication industry proving to handle millions
//! of messages.
#![cfg_attr(feature = "flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature = "flame_it", plugin(flamer))]

extern crate actix;
extern crate bytes;
extern crate core;
extern crate crossbeam_channel as channel;
extern crate failure;
#[cfg(feature = "flame_it")]
extern crate flame;
extern crate futures;
extern crate tokio_codec;
#[cfg(not(windows))]
extern crate tokio_file_unix as ufs;
extern crate tokio_io;
extern crate tokio_reactor;
extern crate tokio_stdin_stdout;
#[cfg(windows)]
extern crate winapi;

mod codec;
mod pipe;

use actix::io::{FramedWrite, WriteHandler};
use actix::{Actor, AsyncContext, Context, Handler, StreamHandler};
use channel::Sender;
use core::{ClientAction, PlayerInfo};
use failure::Error;
use std::io;
use tokio_codec::FramedRead;
use tokio_io::AsyncWrite;

// Main Stdin Stdout Actor placeholder
pub struct EnginePipeIo {
  pub in_pipe: bool,
  pub out_pipe: bool,
  pub sender: Sender<PlayerInfo>,
  pub writer:
    Option<actix::io::FramedWrite<pipe::ImplAsyncWriteStream, codec::ClientActionCodec>>,
  pub writer_pipe:
    Option<actix::io::FramedWrite<tokio_stdin_stdout::SendableStdout, codec::ClientActionCodec>>,

}

impl StreamHandler<PlayerInfo, io::Error> for EnginePipeIo {
  fn handle(&mut self, item: PlayerInfo, _ctx: &mut Context<EnginePipeIo>) {
    // println!("Sending: \t {:?}", item);
    self.sender.send(item);
  }

  // fn error(&mut self, err: io::Error, _ctx: &mut Self::Context) -> Running {
  //     println!("Error: \t {}", err);
  //     Running::Stop
  // }

  fn finished(&mut self, _ctx: &mut Self::Context) {
    // if let Some(ref mut w) = self.writer {
    //   w.write(ClientAction::Message(String::from("msg")));
    // }    
    // println!("finished");
  }
}

/// Turn EnginePipeIo into Actor enabled
impl Actor for EnginePipeIo {
  type Context = Context<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    if self.in_pipe {
      let stdin = tokio_stdin_stdout::stdin(0);
      ctx.add_stream(FramedRead::new(
        stdin,
        codec::PlayerInfoCodec(tokio_codec::LinesCodec::new()),
      ));
    } else {
      let stdin = pipe::stdin_stream().unwrap();
      ctx.add_stream(FramedRead::new(
        stdin,
        codec::PlayerInfoCodec(tokio_codec::LinesCodec::new()),
      ));
    }
    if self.out_pipe {
      let stdout = tokio_stdin_stdout::stdout(0).make_sendable();
      let f_stdout = FramedWrite::new(
        stdout,
        codec::ClientActionCodec(tokio_codec::LinesCodec::new()),
        ctx,
      );
      self.writer_pipe = Some(f_stdout);
    } else {
      let stdout = pipe::stdout_stream().unwrap();
      let f_stdout = FramedWrite::new(
        stdout,
        codec::ClientActionCodec(tokio_codec::LinesCodec::new()),
        ctx,
      );
      self.writer = Some(f_stdout);      
    }
  }
}

impl WriteHandler<io::Error> for EnginePipeIo {}

/// Message handling for type ClientAction
impl Handler<ClientAction> for EnginePipeIo {
  type Result = Result<(), Error>;

  // this is the standard method signature for message handling of Actor
  // MessageBox
  #[cfg_attr(feature = "flame_it", flame)]
  fn handle(&mut self, msg: ClientAction, _: &mut Self::Context) -> Self::Result {
    if let Some(ref mut w) = self.writer {
      w.write(msg);
    } else if let Some(ref mut w) = self.writer_pipe {
      w.write(msg);
    }
    // println!("{}", msg);
    Ok(())
  }
}

/// Message handling for type GetPlayerInfoMsg
// impl Handler<GetPlayerInfoMsg> for EnginePipeIo {
//   type Result = Result<PlayerInfo, Error>;

//   #[cfg_attr(feature = "flame_it", flame)]
//   fn handle(&mut self, _msg: GetPlayerInfoMsg, _: &mut Self::Context) -> Self::Result {
//     let mut input = String::new();
//     // there is no problem blocking the thread as it using a real thread-pool
//     io::stdin().read_line(&mut input)?;
//     Ok(PlayerInfo::from_str(input.as_ref())?)
//   }
// }

// Smoke test to see module is building fine
// Decide not go further testing here due to simplicity of functionality
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
