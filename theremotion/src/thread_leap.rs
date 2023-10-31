use std::thread;

use crossbeam_channel::{SendError, Sender};
use leaprs::{Connection, ConnectionConfig, Error, Event};
use nalgebra::{UnitQuaternion, Vector3};

use crate::thread_conductor::{self, TrackingStatus};

/// Start the leap motion thread
pub fn run(mut tx: Sender<thread_conductor::Msg>) -> thread::JoinHandle<()> {
    thread::Builder::new()
        .name("leap".to_string())
        .spawn(move || {
            let mut connection =
                Connection::create(ConnectionConfig::default()).expect("Failed to connect");
            connection.open().expect("Failed to open the connection");
            loop {
                if read_and_update(&mut tx, &mut connection).is_err() {
                    // Conductor thread is not running anymore, exit
                    return;
                }
            }
        })
        .expect("Failed to spawn the hand tracking thread")
}

fn read_and_update(
    tx: &mut Sender<thread_conductor::Msg>,
    connection: &mut Connection,
) -> Result<(), SendError<thread_conductor::Msg>> {
    match connection.poll(100) {
        Ok(message) => {
            match message.event() {
                Event::Tracking(e) => {
                    // List of visible hands
                    let hands = e.hands();

                    for hand in hands.iter() {
                        tx.send(thread_conductor::Msg::HandUpdate(crate::HandMessage::from(
                            hand,
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
                Event::Connection(_) => tx.send(thread_conductor::Msg::TrackingStatus(
                    TrackingStatus::Warning("No device".to_string()),
                ))?,
                Event::ConnectionLost(_) => tx.send(thread_conductor::Msg::TrackingStatus(
                    TrackingStatus::Error("Connection lost".to_string()),
                ))?,
                Event::Device(_) => {
                    tx.send(thread_conductor::Msg::TrackingStatus(TrackingStatus::Ok))?
                }
                Event::DeviceFailure(_) => tx.send(thread_conductor::Msg::TrackingStatus(
                    TrackingStatus::Error("Device failure".to_string()),
                ))?,
                Event::DeviceLost => tx.send(thread_conductor::Msg::TrackingStatus(
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

impl From<&leaprs::Hand<'_>> for crate::HandMessage {
    fn from(value: &leaprs::Hand<'_>) -> Self {
        let position = value.palm().position();
        let velocity = value.palm().velocity();
        let rotation = value.arm().rotation();
        crate::HandMessage {
            hand_type: value.hand_type().into(),
            position: Vector3::new(position.x(), position.y(), position.z()),
            velocity: Vector3::new(velocity.x(), velocity.y(), velocity.z()),
            rotation: UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(
                rotation.w(),
                rotation.x(),
                rotation.y(),
                rotation.z(),
            )),
            pinch: value.pinch_strength(),
            grab: value.grab_strength(),
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
