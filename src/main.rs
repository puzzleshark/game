use bevy::prelude::*;
use nokhwa;

fn main() {
    App::new().run();
    println!("this is a test");

    let mut camera = nokhwa::Camera::new(
        0, // index
        Some(nokhwa::CameraFormat::new_from(640, 480, nokhwa::FrameFormat::MJPEG, 30)), // format
    )
    .unwrap();
    // open stream
    camera.open_stream().unwrap();
    loop {
        let frame = camera.frame().unwrap();
        println!("{}, {}", frame.width(), frame.height());
    }
}
