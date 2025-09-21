use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader, IsTerminal},
};

pub struct InputStream {
    buf_read: Box<dyn BufRead>,
    is_terminal: bool,
}

impl InputStream {
    pub fn from_file(f: File) -> Self {
        InputStream {
            buf_read: Box::new(BufReader::new(f)),
            is_terminal: false,
        }
    }

    pub fn from_stdin() -> Self {
        let stdin = std::io::stdin();
        let is_terminal = stdin.is_terminal();
        InputStream {
            buf_read: Box::new(stdin.lock()),
            is_terminal,
        }
    }

    pub fn read_line(&mut self, s: &mut String) -> usize {
        self.buf_read.read_line(s).unwrap_or_default()
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }
}

thread_local! {
    pub static IN_STREAM: RefCell<Option<InputStream>> = const { RefCell::new(None )};
}

pub fn in_stream_from_stdin() {
    IN_STREAM.replace(Some(InputStream::from_stdin()));
}

pub fn in_steam_from_file(file: File) {
    IN_STREAM.replace(Some(InputStream::from_file(file)));
}

pub fn in_stream_is_terminal() -> bool {
    let mut it = false;

    IN_STREAM.with_borrow(|i| {
        if let Some(in_stream) = i {
            it = in_stream.is_terminal();
        }
    });

    it
}

pub fn in_stream_read_line(s: &mut String) -> usize {
    let mut n = 0;

    IN_STREAM.with_borrow_mut(|v| {
        if let Some(in_stream) = v {
            n = in_stream.read_line(s);
        }
    });
    n
}
