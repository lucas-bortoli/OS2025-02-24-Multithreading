use std::sync::mpsc;
use std::thread::{self, JoinHandle};

pub enum Audio {
    BackgroundMusic,
    Door,
    Invalid,
}

pub enum Message {
    PlaySndBackgroundMusic,
    PlaySndDoor,
    PlaySndInvalid,
    Close,
}

pub struct AudioManager {
    audio_thread: JoinHandle<()>,
    tx: mpsc::Sender<Message>,
}

impl AudioManager {
    pub fn new() -> AudioManager {
        let (tx, rx) = mpsc::channel::<Message>();
        let spawned = thread::spawn(move || {
            let (mut manager, backend) = awedio::start().expect("Failed to start awedio");

            loop {
                let received = rx.recv();
                if received.is_err() {
                    println!("Channel closed. Breaking out.");
                    break;
                }

                match received.unwrap() {
                    Message::PlaySndBackgroundMusic => manager.play(
                        awedio::sounds::open_file("./bgm.mp3").expect("Failed to open audio file"),
                    ),
                    Message::PlaySndDoor => manager.play(
                        awedio::sounds::open_file("./door.wav").expect("Failed to open audio file"),
                    ),
                    Message::PlaySndInvalid => manager.play(
                        awedio::sounds::open_file("./invalid.wav")
                            .expect("Failed to open audio file"),
                    ),
                    Message::Close => {
                        println!("Closing audio thread!!");
                        break;
                    }
                }
            }
        });

        return AudioManager {
            audio_thread: spawned,
            tx: tx,
        };
    }

    pub fn play(&self, audio: Audio) {
        let msg = match audio {
            Audio::BackgroundMusic => Message::PlaySndBackgroundMusic,
            Audio::Door => Message::PlaySndDoor,
            Audio::Invalid => Message::PlaySndInvalid,
        };

        self.tx
            .send(msg)
            .expect("Failed to send play command to thread.");
    }

    pub fn close(&self) {
        if self.audio_thread.is_finished() {
            return;
        }

        self.tx
            .send(Message::Close)
            .expect("Failed to send close coomand.");
    }
}
