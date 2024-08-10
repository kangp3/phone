use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use crate::dtmf::{goertzelme, NULL, OCTOTHORPE, STAR};


const MODE: u8 = 1;

enum State {
    Lower((u8, usize)),
    Upper((u8, usize)),
    Symbol((u8, usize)),
    Number,
}

const DEFAULT_STATE: State = State::Lower((NULL, 0));


// TODO: Wrap this in another struct that can manage the state and handle emitting to channel
impl State {
    fn new() -> Self {
        State::Lower((NULL, 0))
    }

    fn poosh(self, dig: u8) -> (Self, Option<char>) {
        let mut c = None;
        let next = match self {
            _ if dig == STAR => State::Lower((NULL, 0)),
            _ if dig == OCTOTHORPE => {
                c = self.emit();
                DEFAULT_STATE
            }

            State::Lower((NULL, 0)) if (2..=9).contains(&dig) => State::Lower((dig, 1)),
            State::Lower((n@(7|9), m@(1..=3))) |
            State::Lower((n@(2..=6|8), m@(1..=2))) if n == dig => State::Lower((n, m+1)),
            State::Lower((NULL, 0)) if dig == MODE => State::Upper((NULL, 0)),
            State::Lower((n@(2..=9), _)) if dig != n => {
                c = self.emit();
                DEFAULT_STATE.poosh(dig).0
            }

            State::Upper((NULL, 0)) if (2..=9).contains(&dig) => State::Upper((dig, 1)),
            State::Upper((n@(7|9), m@(1..=3))) |
            State::Upper((n@(2..=6|8), m@(1..=2))) if n == dig => State::Upper((n, m+1)),
            State::Upper((NULL, 0)) if dig == MODE => State::Symbol((NULL, 0)),
            State::Upper((n@(2..=9), _)) if dig != n => {
                c = self.emit();
                DEFAULT_STATE.poosh(dig).0
            }

            State::Symbol((NULL, 0)) if (2..=9).contains(&dig) => State::Symbol((dig, 1)),
            State::Symbol((NULL, 0)) if dig == 0 => {
                c = Some(' ');
                DEFAULT_STATE
            }
            State::Symbol((n@(2..=9), m@(1..=3))) if n == dig => State::Symbol((n, m+1)),
            State::Symbol((NULL, 0)) if dig == MODE => State::Number,
            State::Symbol((n@(2..=9), _)) if dig != n => {
                c = self.emit();
                DEFAULT_STATE.poosh(dig).0
            }

            State::Number if (0..=9).contains(&dig) => {
                c = char::from_digit(dig.into(), 10);
                DEFAULT_STATE
            }

            _ if dig == 0 => {
                c = self.emit();
                // TODO(peter): Emit the control character as well
                DEFAULT_STATE
            }

            _ => panic!("uh oh stinky state"),
        };
        (next, c)
    }

    fn emit(self) -> Option<char> {
        match self {
            State::Lower((2, 1)) => Some('a'),
            State::Lower((2, 2)) => Some('b'),
            State::Lower((2, 3)) => Some('c'),
            State::Lower((3, 1)) => Some('d'),
            State::Lower((3, 2)) => Some('e'),
            State::Lower((3, 3)) => Some('f'),
            State::Lower((4, 1)) => Some('g'),
            State::Lower((4, 2)) => Some('h'),
            State::Lower((4, 3)) => Some('i'),
            State::Lower((5, 1)) => Some('j'),
            State::Lower((5, 2)) => Some('k'),
            State::Lower((5, 3)) => Some('l'),
            State::Lower((6, 1)) => Some('m'),
            State::Lower((6, 2)) => Some('n'),
            State::Lower((6, 3)) => Some('o'),
            State::Lower((7, 1)) => Some('p'),
            State::Lower((7, 2)) => Some('q'),
            State::Lower((7, 3)) => Some('r'),
            State::Lower((7, 4)) => Some('s'),
            State::Lower((8, 1)) => Some('t'),
            State::Lower((8, 2)) => Some('u'),
            State::Lower((8, 3)) => Some('v'),
            State::Lower((9, 1)) => Some('w'),
            State::Lower((9, 2)) => Some('x'),
            State::Lower((9, 3)) => Some('y'),
            State::Lower((9, 4)) => Some('z'),

            State::Upper((2, 1)) => Some('A'),
            State::Upper((2, 2)) => Some('B'),
            State::Upper((2, 3)) => Some('C'),
            State::Upper((3, 1)) => Some('D'),
            State::Upper((3, 2)) => Some('E'),
            State::Upper((3, 3)) => Some('F'),
            State::Upper((4, 1)) => Some('G'),
            State::Upper((4, 2)) => Some('H'),
            State::Upper((4, 3)) => Some('I'),
            State::Upper((5, 1)) => Some('J'),
            State::Upper((5, 2)) => Some('K'),
            State::Upper((5, 3)) => Some('L'),
            State::Upper((6, 1)) => Some('M'),
            State::Upper((6, 2)) => Some('N'),
            State::Upper((6, 3)) => Some('O'),
            State::Upper((7, 1)) => Some('P'),
            State::Upper((7, 2)) => Some('Q'),
            State::Upper((7, 3)) => Some('R'),
            State::Upper((7, 4)) => Some('S'),
            State::Upper((8, 1)) => Some('T'),
            State::Upper((8, 2)) => Some('U'),
            State::Upper((8, 3)) => Some('V'),
            State::Upper((9, 1)) => Some('W'),
            State::Upper((9, 2)) => Some('X'),
            State::Upper((9, 3)) => Some('Y'),
            State::Upper((9, 4)) => Some('Z'),

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
