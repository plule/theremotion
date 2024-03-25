use std::sync::mpsc::Sender;
use std::{sync::mpsc::Receiver, thread};

use crate::thread_conductor;

pub enum Msg {
    Exit,
}

/// Start the leap motion thread
pub fn run(mut tx: Sender<thread_conductor::Msg>, rx: Receiver<Msg>) -> thread::JoinHandle<()> {
    #[cfg(feature = "leap")]
    return thread::Builder::new()
        .name("leap".to_string())
        .spawn(move || {
            let mut connection = leaprs::Connection::create(leaprs::ConnectionConfig::default())
                .expect("Failed to connect");
            connection.open().expect("Failed to open the connection");
            loop {
                if let Ok(Msg::Exit) = rx.try_recv() {
                    return;
                }

                if leap::read_and_update(&mut tx, &mut connection).is_err() {
                    // Conductor thread is not running anymore, exit
                    return;
                }
            }
        })
        .expect("Failed to spawn the hand tracking thread");

    #[cfg(not(feature = "leap"))]
    return thread::Builder::new()
        .name("leap".to_string())
        .spawn(move || {
            if let Ok(Msg::Exit) = rx.recv() {
                return;
            }
        })
        .expect("Failed to spawn the hand tracking thread");
}

#[cfg(feature = "leap")]
mod leap {
    use leaprs::{Connection, Error, EventRef};
    use std::sync::mpsc::{SendError, Sender};

    use crate::thread_conductor::{self, TrackingStatus};

    pub fn read_and_update(
        tx: &mut Sender<thread_conductor::Msg>,
        connection: &mut Connection,
    ) -> Result<(), SendError<thread_conductor::Msg>> {
        match connection.poll(100) {
            Ok(message) => {
                match message.event() {
                    EventRef::Tracking(e) => {
                        // List of visible hands
                        let hands = e.hands();

                        for hand in hands.iter() {
                            tx.send(thread_conductor::Msg::HandUpdate(crate::HandMessage::from(
                                *hand,
                            )))?;
                        }

                        tx.send(thread_conductor::Msg::VisibleHands {
                            left: hands
                                .iter()
                                .any(|h| h.hand_type() == leaprs::HandType::Left),
                            right: hands
                                .iter()
                                .any(|h| h.hand_type() == leaprs::HandType::Right),
                        })?;

                        tx.send(thread_conductor::Msg::TrackingStatus(TrackingStatus::Ok))?;
                    }
                    EventRef::Connection(_) => tx.send(thread_conductor::Msg::TrackingStatus(
                        TrackingStatus::Warning("No device".to_string()),
                    ))?,
                    EventRef::ConnectionLost(_) => {
                        tx.send(thread_conductor::Msg::TrackingStatus(
                            TrackingStatus::Error("Connection lost".to_string()),
                        ))?
                    }
                    EventRef::Device(_) => {
                        tx.send(thread_conductor::Msg::TrackingStatus(TrackingStatus::Ok))?
                    }
                    EventRef::DeviceFailure(_) => {
                        tx.send(thread_conductor::Msg::TrackingStatus(
                            TrackingStatus::Error("Device failure".to_string()),
                        ))?
                    }
                    EventRef::DeviceLost => tx.send(thread_conductor::Msg::TrackingStatus(
                        TrackingStatus::Error("Device disconnected".to_string()),
                    ))?,
                    _ => {}
                }
            }
            Err(err) => match err {
                Error::Timeout => {} // spammey without any device
                Error::NotConnected => tx.send(thread_conductor::Msg::TrackingStatus(
                    TrackingStatus::Warning(err.to_string()),
                ))?,
                _ => tx.send(thread_conductor::Msg::TrackingStatus(
                    TrackingStatus::Error(err.to_string()),
                ))?,
            },
        }
        Ok(())
    }

    impl From<leaprs::HandRef<'_>> for crate::HandMessage {
        fn from(value: leaprs::HandRef<'_>) -> Self {
            crate::HandMessage {
                hand_type: value.hand_type().into(),
                position: value.palm().position().into(),
                velocity: value.palm().velocity().into(),
                rotation: value.arm().rotation().into(),
                pinch: value.pinch_strength,
                grab: value.grab_strength,
            }
        }
    }

    impl From<leaprs::HandType> for crate::HandType {
        fn from(value: leaprs::HandType) -> Self {
            match value {
                leaprs::HandType::Left => crate::HandType::Left,
                leaprs::HandType::Right => crate::HandType::Right,
            }
        }
    }
}
