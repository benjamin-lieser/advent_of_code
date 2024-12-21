use std::{array, collections::VecDeque};

use advent_of_code::*;

enum Input {
    Dir(Dir),
    Press
}

trait Pad : Sized {
    fn input(&self, input : Input) -> (Option<Input>, Option<Self>);
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Keypad {
    pos : Pos,
    to_ress: [char;4]
}

impl Pad for Keypad {
    fn input(&self, input : Input) -> (Option<Input>, Option<Self>) {
        match input {
            Input::Dir(dir) => {
                if KEYPAD.borrow().get(self.pos + dir) == None || KEYPAD.borrow().get(self.pos + dir) == Some('#') {
                    return (None, None)
                } else {
                    let mut new_pad = self.clone();
                    new_pad.pos = self.pos + dir;
                    return (None, Some(new_pad))
                }
            }
            Input::Press => {
                let what = KEYPAD.borrow().get(self.pos).unwrap();
                if self.to_ress[0] == what {
                    let mut new_pad = self.clone();
                    new_pad.to_ress[..3].copy_from_slice(&self.to_ress[1..]);
                    new_pad.to_ress[3] = 'X';
                    return (None, Some(new_pad))
                } else {
                    return (None,None)
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Control {
    pos : Pos
}

impl Pad for Control {
    fn input(&self, input : Input) -> (Option<Input>, Option<Self>) {
        match input {
            Input::Dir(dir) => {
                if CONTROL.borrow().get(self.pos + dir) == None || CONTROL.borrow().get(self.pos + dir) == Some('#') {
                    return (None, None)
                } else {
                    let mut new_pad = self.clone();
                    new_pad.pos = self.pos + dir;
                    return (None, Some(new_pad))
                }
            }
            Input::Press => {
                let what = CONTROL.borrow().get(self.pos).unwrap();
                match what {
                    'A' => return (Some(Input::Press), Some(self.clone())),
                    x => return (Some(Input::Dir(Dir::from_char2(x).unwrap())), Some(self.clone()))
                }
            }

        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct State {
    keypad: Keypad,
    control1: Control,
    control2: Control,
}

impl State {
    fn is_done(&self) -> bool{
        self.keypad.to_ress == ['X', 'X', 'X', 'X']
    }
}

fn next_state(state: State, press: char) -> Option<State> {
    let mut new_state = state.clone();
    let input = match press {
        'A' => Input::Press,
        x => Input::Dir(Dir::from_char2(x).unwrap())
    };

    let (next_input, new_control2) = new_state.control2.input(input);
    if let Some(x) = new_control2 {
        new_state.control2 = x;
    } else {
        return None;
    }

    if next_input.is_none() {
        return Some(new_state);
    }

    let(next_input, new_control1) = new_state.control1.input(next_input.unwrap());
        
    if let Some(x) = new_control1 {
        new_state.control1 = x;
    } else {
        return None;
    }

    if next_input.is_none() {
        return Some(new_state);
    }


    let(_, new_keybad) = new_state.keypad.input(next_input.unwrap());

    if let Some(x) = new_keybad {
        new_state.keypad = x;
    } else {
        return None;
    }

    Some(new_state)
}

static KEYPAD: Global<Grid<char>> = Global::new();
static CONTROL: Global<Grid<char>> = Global::new();

fn main() {
    let input = get_input(2024, 21);

    KEYPAD.set(Grid::from_str("789\n456\n123\n#0A\n"));
    CONTROL.set(Grid::from_str("#^A\n<v>\n"));

    let mut acc = 0;

    for line in input.lines() {
        let keypad = Keypad{pos : (2,3), to_ress: array::from_fn(|i| line.chars().nth(i).unwrap())};
        let control = Control{pos : (2,0)};
        let state = State{keypad:keypad, control1: control, control2: control};

        let mut visit = HashSet::new();

        let mut queue = VecDeque::<(State, int)>::new();

        queue.push_back((state, 0));

        let mut length = 0;

        while let Some((x, dist)) = queue.pop_front() {
            if x.is_done() {
                length = dist;
                break;
            }
            visit.insert(x);

            for i in ['A', '>', '<', '^', 'v'] {
                //dbg!(i, x);
                let next_state = next_state(x, i);
                //dbg!(next_state);
                if next_state.is_some() && !visit.contains(&next_state.unwrap()) {
                    queue.push_back((next_state.unwrap(), dist + 1));
                }
            }

        }

        acc += length * line[..3].parse::<int>().unwrap();
        

    }

    dbg!(acc);

}
