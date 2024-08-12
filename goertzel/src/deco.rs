use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use crate::dtmf::{goertzelme, NULL, OCTOTHORPE, SEXTILE};


const MODE: u8 = 1;

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

    fn poosh(self, dig: u8) -> (Self, Option<char>) {
        let mut c = None;
        let next = match self {
            _ if dig == SEXTILE => Self::default(),
            _ if dig == OCTOTHORPE => {
                c = self.emit();
                Self::default()
            }

            State::Lower((NULL, 0)) if (2..=9).contains(&dig) => State::Lower((dig, 1)),
            State::Lower((n@(7|9), m@(1..=3))) |
            State::Lower((n@(2..=6|8), m@(1..=2))) if n == dig => State::Lower((n, m+1)),
            State::Lower((NULL, 0)) if dig == MODE => State::Upper((NULL, 0)),
            State::Lower((n@(2..=9), _)) if dig != n => {
                c = self.emit();
                Self::default().poosh(dig).0
            }

            State::Upper((NULL, 0)) if (2..=9).contains(&dig) => State::Upper((dig, 1)),
            State::Upper((n@(7|9), m@(1..=3))) |
            State::Upper((n@(2..=6|8), m@(1..=2))) if n == dig => State::Upper((n, m+1)),
            State::Upper((NULL, 0)) if dig == MODE => State::Symbol((NULL, 0)),
            State::Upper((n@(2..=9), _)) if dig != n => {
                c = self.emit();
                Self::default().poosh(dig).0
            }

            State::Symbol((NULL, 0)) if (2..=9).contains(&dig) => State::Symbol((dig, 1)),
            State::Symbol((NULL, 0)) if dig == 0 => {
                c = Some(' ');
                Self::default()
            }
            State::Symbol((n@(2..=9), m@(1..=3))) if n == dig => State::Symbol((n, m+1)),
            State::Symbol((NULL, 0)) if dig == MODE => State::Number,
            State::Symbol((n@(2..=9), _)) if dig != n => {
                c = self.emit();
                Self::default().poosh(dig).0
            }

            State::Number if (0..=9).contains(&dig) => {
                c = char::from_digit(dig.into(), 10);
                Self::default()
            }

            _ if dig == 0 => {
                c = self.emit();
                // TODO(peter): Emit the control character as well
                Self::default()
            }

            _ => panic!("uh oh stinky state"),
        };
        (next, c)
    }

    fn emit(self) -> Option<char> {
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

pub fn ding(sample_ch: UnboundedReceiver<f32>) -> UnboundedReceiver<char> {
    let mut digs_ch = goertzelme(sample_ch);

    let (send_ch, rcv_ch) = unbounded_channel();
    let mut state = State::new();
    tokio::spawn(async move {
        while let Some(dig) = digs_ch.recv().await {
            let pooshed = state.poosh(dig);
            state = pooshed.0;
            if let Some(c) = pooshed.1 {
                send_ch.send(c).unwrap();
            }
        }
    });

    rcv_ch
}
