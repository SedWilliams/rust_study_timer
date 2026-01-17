use stui_timer::util::{
    io::await_startup_choice,
    types::{EventReader, EventResult},
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

#[cfg(test)]
mod io {
    use super::*;

    #[test]
    fn await_startup_choice_returns_s() {
        let mut reader = TestEventReader::new(vec![make_keypress_event('s')]);
        let result = await_startup_choice(&mut reader).unwrap();
        assert!(result == "s");
    }

    #[test]
    fn await_startup_choice_returns_q() {
        let mut reader = TestEventReader::new(vec![make_keypress_event('q')]);
        let result = await_startup_choice(&mut reader).unwrap();
        assert!(result == "q");
    }

    #[test]
    fn await_startup_choice_returns_v() {
        let mut reader = TestEventReader::new(vec![make_keypress_event('v')]);
        let result = await_startup_choice(&mut reader).unwrap();
        assert!(result == "v");
    }

    #[test]
    fn await_startup_choice_ignores_other_keys() {
        let mut reader = TestEventReader::new(vec![
            make_keypress_event('x'),
            make_keypress_event('z'),
            make_keypress_event('s'),
        ]);
        let result = await_startup_choice(&mut reader).unwrap();
        assert!(result == "s");
    }
}
