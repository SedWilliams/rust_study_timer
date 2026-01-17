use stui_timer::util::{
    io::{EventReader, await_yes_no},
    types::EventResult,
};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

struct TestEventReader {
    events: Vec<Event>,
    index: usize,
}

impl TestEventReader {
    fn new(events: Vec<Event>) -> Self {
        Self { events, index: 0 }
    }
}

impl EventReader for TestEventReader {
    fn read_event(&mut self) -> EventResult {
        let event = self.events[self.index].clone();
        self.index += 1;
        Ok(event)
    }
}

fn make_keypress_event(c: char) -> Event {
    Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE))
}

#[test]
fn await_yes_no_returns_y() {
    let mut reader = TestEventReader::new(vec![make_keypress_event('y')]);
    let result = await_yes_no(&mut reader).unwrap();
    assert!(result == "y");
}

#[test]
fn await_yes_no_returns_n() {
    let mut reader = TestEventReader::new(vec![make_keypress_event('n')]);
    let result = await_yes_no(&mut reader).unwrap();
    assert!(result == "n");
}

#[test]
fn await_yes_no_ignores_other_keys() {
    let mut reader = TestEventReader::new(vec![
        make_keypress_event('x'),
        make_keypress_event('q'),
        make_keypress_event('y'),
    ]);
    let result = await_yes_no(&mut reader).unwrap();
    assert!(result == "y");
}
