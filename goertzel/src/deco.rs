use std::time::Duration;

use tokio::sync::{broadcast, mpsc};
use tokio::time::sleep;
use tracing::debug;

use crate::asyncutil::and_log_err;
use crate::dtmf::{NULL, OCTOTHORPE, SEXTILE};


const MODE: u8 = 1;

const CHAR_TIMEOUT_S: u64 = 3;
const CHARS_CHANNEL_SIZE: usize = 64;


#[derive(PartialEq)]
enum State {
    Lower((u8, u8)),
    Upper((u8, u8)),
    Symbol((u8, u8)),
    Number,
}

impl Default for State {
    fn default() -> Self {
        State::Lower((NULL, 0))
    }
}


// TODO: Wrap this in another struct that can manage the state and handle emitting to channel
impl State {
    fn new() -> Self {
        Self::default()
    }

    fn is_fresh(&self) -> bool {
        *self == State::Lower((NULL, 0))
    }

    fn poosh(self, dig: u8) -> Result<(State, Vec<char>), String> {
        let mut c = Vec::new();
        let next = match self {
            _ if dig == SEXTILE => Self::default(),
            _ if dig == OCTOTHORPE => {
                if let Some(ch) = self.emit() {
                    c.push(ch);
                }
                Self::default()
            }

            State::Lower((n@(2..=9), _)) |
            State::Upper((n@(2..=9), _)) |
            State::Symbol((n@(2..=9), _)) if dig != n => {
                if let Some(ch) = self.emit() {
                    c.push(ch);
                }
                let (s, mut chs) = Self::default().poosh(dig)?;
                c.append(&mut chs);
                s
            }

            State::Lower((NULL, 0)) if (2..=9).contains(&dig) => State::Lower((dig, 1)),
            State::Lower((n@(7|9), m@(1..=3))) |
            State::Lower((n@(2..=6|8), m@(1..=2))) if n == dig => State::Lower((n, m+1)),
            State::Lower((NULL, 0)) if dig == MODE => State::Upper((NULL, 0)),

            State::Upper((NULL, 0)) if (2..=9).contains(&dig) => State::Upper((dig, 1)),
            State::Upper((n@(7|9), m@(1..=3))) |
            State::Upper((n@(2..=6|8), m@(1..=2))) if n == dig => State::Upper((n, m+1)),
            State::Upper((NULL, 0)) if dig == MODE => State::Symbol((NULL, 0)),

            State::Symbol((NULL, 0)) if (2..=9).contains(&dig) => State::Symbol((dig, 1)),
            State::Symbol((NULL, 0)) if dig == 0 => {
                c.push(' ');
                Self::default()
            }
            State::Symbol((n@(2..=9), m@(1..=3))) if n == dig => State::Symbol((n, m+1)),
            State::Symbol((NULL, 0)) if dig == MODE => State::Number,

            State::Number if (0..=9).contains(&dig) => {
                c.push((dig + b'0') as char);
                Self::default()
            }

            _ if dig == 0 => {
                if let Some(ch) = self.emit() {
                    c.push(ch);
                }
                // TODO: Handle this by just closing the send channel instead
                c.push('\0');
                Self::default()
            }

            _ => return Err(String::from("uh oh stinky state")),
        };
        Ok((next, c))
    }

    fn emit(&self) -> Option<char> {
        match self {
            State::Lower((2, c)) => Some((b'a' + c - 1) as char),
            State::Lower((3, c)) => Some((b'd' + c - 1) as char),
            State::Lower((4, c)) => Some((b'g' + c - 1) as char),
            State::Lower((5, c)) => Some((b'j' + c - 1) as char),
            State::Lower((6, c)) => Some((b'm' + c - 1) as char),
            State::Lower((7, c)) => Some((b'p' + c - 1) as char),
            State::Lower((8, c)) => Some((b't' + c - 1) as char),
            State::Lower((9, c)) => Some((b'w' + c - 1) as char),

            State::Upper((2, c)) => Some((b'A' + c - 1) as char),
            State::Upper((3, c)) => Some((b'D' + c - 1) as char),
            State::Upper((4, c)) => Some((b'G' + c - 1) as char),
            State::Upper((5, c)) => Some((b'J' + c - 1) as char),
            State::Upper((6, c)) => Some((b'M' + c - 1) as char),
            State::Upper((7, c)) => Some((b'P' + c - 1) as char),
            State::Upper((8, c)) => Some((b'T' + c - 1) as char),
            State::Upper((9, c)) => Some((b'W' + c - 1) as char),

            State::Symbol((0, 1)) => Some(' '),
            State::Symbol((2, 1)) => Some('!'),
            State::Symbol((2, 2)) => Some('@'),
            State::Symbol((2, 3)) => Some('#'),
            State::Symbol((2, 4)) => Some('$'),
            State::Symbol((3, 1)) => Some('%'),
            State::Symbol((3, 2)) => Some('^'),
            State::Symbol((3, 3)) => Some('&'),
            State::Symbol((3, 4)) => Some('*'),
            State::Symbol((4, 1)) => Some('('),
            State::Symbol((4, 2)) => Some(')'),
            State::Symbol((4, 3)) => Some('`'),
            State::Symbol((4, 4)) => Some('~'),
            State::Symbol((5, 1)) => Some('['),
            State::Symbol((5, 2)) => Some(']'),
            State::Symbol((5, 3)) => Some('{'),
            State::Symbol((5, 4)) => Some('}'),
            State::Symbol((6, 1)) => Some('/'),
            State::Symbol((6, 2)) => Some('\\'),
            State::Symbol((6, 3)) => Some('?'),
            State::Symbol((6, 4)) => Some('|'),
            State::Symbol((7, 1)) => Some('\''),
            State::Symbol((7, 2)) => Some('"'),
            State::Symbol((7, 3)) => Some(';'),
            State::Symbol((7, 4)) => Some(':'),
            State::Symbol((8, 1)) => Some(','),
            State::Symbol((8, 2)) => Some('.'),
            State::Symbol((8, 3)) => Some('<'),
            State::Symbol((8, 4)) => Some('>'),
            State::Symbol((9, 1)) => Some('-'),
            State::Symbol((9, 2)) => Some('_'),
            State::Symbol((9, 3)) => Some('='),
            State::Symbol((9, 4)) => Some('+'),

            _ => None,
        }
    }
}

pub fn ding(mut goertzel_ch: broadcast::Receiver<u8>, mut notgoertzel_ch: broadcast::Receiver<u8>) -> mpsc::Receiver<char> {
    let (goertzeled_send_ch, rcv_ch) = mpsc::channel(CHARS_CHANNEL_SIZE);
    let pulsed_send_ch = goertzeled_send_ch.clone();

    let mut goertzeled_state = State::new();
    let mut pulsed_state = State::new();

    tokio::spawn(and_log_err("deco::ding goertzel", async move {
        loop {
            let dig = goertzel_ch.recv().await?;
            let (new_state, chars) = goertzeled_state.poosh(dig)?;
            goertzeled_state = new_state;
            for c in chars.into_iter() {
                goertzeled_send_ch.try_send(c)?;
            }
        }
    }));

    tokio::spawn(and_log_err("deco::ding pulse", async move {
        loop {
            let mut chars = Vec::new();
            tokio::select! {
                _ = sleep(Duration::from_secs(CHAR_TIMEOUT_S)), if !pulsed_state.is_fresh() => {
                    if let Some(ch) = pulsed_state.emit() {
                        chars.push(ch);
                    }
                    pulsed_state = State::new();
                    debug!("char timeout");
                }
                dig = notgoertzel_ch.recv() => {
                    let (new_state, cs) = pulsed_state.poosh(dig?)?;
                    pulsed_state = new_state;
                    for c in cs {
                        chars.push(c);
                    }
                }
            }
            for c in chars.into_iter() {
                pulsed_send_ch.try_send(c)?;
            }
        }
    }));

    rcv_ch
}
