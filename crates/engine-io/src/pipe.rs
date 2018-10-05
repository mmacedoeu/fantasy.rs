use failure;
#[cfg(not(windows))]
use ufs;
use std::{io, fs};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_reactor::{self, Handle};
#[cfg(not(windows))]
use std::os::unix::prelude::*;
#[cfg(windows)]
use std::os::windows::prelude::*;
#[cfg(windows)]
use std::{ptr, io};
#[cfg(windows)]
use winapi::shared::{
    minwindef::DWORD,
    ntdef::{HANDLE, PHANDLE},
};
#[cfg(windows)]
use winapi::um::{handleapi, namedpipeapi, processenv, winbase};

#[cfg(not(windows))]
pub type ImplAsyncWriteStream = io::BufWriter<tokio_reactor::PollEvented<ufs::File<fs::File>>>;

pub fn stdin_stream() -> Result<impl AsyncRead, failure::Error> {
    #[cfg(not(windows))]
    {
        let file = ufs::raw_stdin()?;
        let file = ufs::File::new_nb(file)?;
        file.into_reader(&Handle::current()).map_err(Into::into)
    }
}

pub fn stdout_stream() -> Result<ImplAsyncWriteStream, failure::Error>{
    #[cfg(not(windows))]
    {
        let file = ufs::raw_stdout()?;
        let file = ufs::File::new_nb(file)?;
        Ok(io::BufWriter::new(file.into_io(&Handle::current())?))
    }    
}

// pub fn pipes() -> Result<(File, File), failure::Error> {
//     #[cfg(not(windows))]
//     {
//         // O_CLOEXEC prevents children from inheriting these pipes. Nix's pipe2() will make a best
//         // effort to make that atomic on platforms that support it, to avoid the case where another
//         // thread forks right after the pipes are created but before O_CLOEXEC is set.
//         let (read_fd, write_fd) = nix::unistd::pipe2(nix::fcntl::OFlag::O_CLOEXEC)?;
//         let rfile = unsafe { File::from_raw_fd(read_fd) };
//         let wfile = unsafe { File::from_raw_fd(write_fd) };
//         Ok((rfile, wfile))
//     }
//     #[cfg(windows)]
//     {
//         let mut read_pipe: HANDLE = ptr::null_mut();
//         let mut write_pipe: HANDLE = ptr::null_mut();

//         let ret = unsafe {
//             // TODO: These pipes do not support IOCP. We might want to emulate anonymous pipes with
//             // CreateNamedPipe, as Rust's stdlib does.
//             namedpipeapi::CreatePipe(
//                 &mut read_pipe as PHANDLE,
//                 &mut write_pipe as PHANDLE,
//                 ptr::null_mut(),
//                 0,
//             )
//         };

//         if ret == 0 {
//             Err(io::Error::last_os_error())
//         } else {
//             unsafe {
//                 Ok((
//                     File::from_raw_handle(read_pipe as _),
//                     File::from_raw_handle(write_pipe as _),
//                 ))
//             }
//         }
//     }
// }
