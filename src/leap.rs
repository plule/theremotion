use std::thread;

use leaprs::*;

pub fn start_leap_worker() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        loop {
            if let Ok(message) = connection.poll(1000) {
                match message.event() {
                    Event::Tracking(e) => {
                        for hand in e.hands() {
                            let hand_type = match hand.hand_type() {
                                HandType::Left => "Left",
                                HandType::Right => "Right",
                            };
                            let pos = hand.palm().position();

                            println!(
                                "{}:\tx:{}\ty:{}\tz:{}",
                                hand_type,
                                pos.x(),
                                pos.y(),
                                pos.z()
                            )
                        }
                    }
                    _ => {}
                }
            }
        }
    })
}
