use game_engine::{app::App, CaptureInput, CaptureList, Events, GameLoop, ToCaptureList};

fn inputs() -> CaptureList {
    [
        CaptureInput {key: game_engine::KeyCode::KeyE, function: test}
    ]
    .to_list()
}

fn test() {
    println!("pressed");
}

fn main() {
    GameLoop::new()
        .capture_events(inputs);
}