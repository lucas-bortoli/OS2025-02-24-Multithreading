mod game;
mod sound;

use std::io::{self, Write};

use game::State;
use sound::{Audio, AudioManager};

fn main() {
    // limpar console
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    let mut state = State::new();
    let audio_manager = AudioManager::new();

    audio_manager.play(Audio::BackgroundMusic);

    let mut input_buffer = "".to_string();
    'mainloop: loop {
        state.describe();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            input_buffer.clear();
            io::stdin()
                .read_line(&mut input_buffer) // <--------- essa porra faz append na string! não dá overwrite nela! temos que limpar o input buffer sempre
                .expect("Falha ao ler input");

            if input_buffer.trim() == "sair" {
                break 'mainloop;
            }

            let action_screen_number = input_buffer.trim().parse::<usize>();
            if action_screen_number.is_err() {
                println!("Número inválido");
                audio_manager.play(Audio::Invalid);
                continue;
            }

            let result = state.trigger_room_action(action_screen_number.unwrap());
            if result.is_err() {
                println!("{}", result.unwrap_err());
                audio_manager.play(Audio::Invalid);
                continue;
            }

            audio_manager.play(Audio::Door);

            break;
        }
    }

    audio_manager.close();

    println!("Finalizando...");
}
