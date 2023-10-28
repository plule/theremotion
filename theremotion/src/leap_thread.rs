use std::thread;

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use leaprs::{Connection, ConnectionConfig, Error, Event};
use nalgebra::{UnitQuaternion, Vector3};

use crate::conductor_thread::{self, ConductorMessage, TrackingStatus};

pub enum Message {
    Exit,
}

/// Start the leap motion thread
pub fn run(
    rx: Receiver<Message>,
    mut tx: Sender<conductor_thread::ConductorMessage>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        loop {
            if let Some(update) = rx.try_iter().next() {
                match update {
                    Message::Exit => {
                        log::debug!("Leap thread exiting");
                        return;
                    }
                }
            }

            read_and_update(&mut tx, &mut connection).unwrap();
        }
    })
}

fn read_and_update(
    tx: &mut Sender<conductor_thread::ConductorMessage>,
    connection: &mut Connection,
) -> Result<()> {
    match connection.poll(100) {
        Ok(message) => {
            match message.event() {
                Event::Tracking(e) => {
                    // List of visible hands
                    let hands = e.hands();

                    for hand in hands.iter() {
                        tx.send(ConductorMessage::HandUpdate(crate::HandMessage::from(hand)))?;
                    }

                    tx.send(ConductorMessage::VisibleHands {
                        left: hands
                            .iter()
                            .any(|h| h.hand_type() == leaprs::HandType::Left),
                        right: hands
                            .iter()
                            .any(|h| h.hand_type() == leaprs::HandType::Right),
                    })?;

                    tx.send(ConductorMessage::TrackingStatus(TrackingStatus::Ok))?;
                }
                Event::Connection(_) => tx.send(ConductorMessage::TrackingStatus(
                    TrackingStatus::Warning("No device".to_string()),
                ))?,
                Event::ConnectionLost(_) => tx.send(ConductorMessage::TrackingStatus(
                    TrackingStatus::Error("Connection lost".to_string()),
                ))?,
                Event::Device(_) => {
                    tx.send(ConductorMessage::TrackingStatus(TrackingStatus::Ok))?
                }
                Event::DeviceFailure(_) => tx.send(ConductorMessage::TrackingStatus(
                    TrackingStatus::Error("Device failure".to_string()),
                ))?,
                Event::DeviceLost => tx.send(ConductorMessage::TrackingStatus(
                    TrackingStatus::Error("Device disconnected".to_string()),
                ))?,
                _ => {}
            }
        }
        Err(err) => match err {
            Error::Timeout => {} // spammey without any device
            Error::NotConnected => tx.send(ConductorMessage::TrackingStatus(
                TrackingStatus::Warning(err.to_string()),
            ))?,
            _ => tx.send(ConductorMessage::TrackingStatus(TrackingStatus::Error(
                err.to_string(),
            )))?,
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
