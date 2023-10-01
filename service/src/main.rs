mod application_audio;
use application_audio::*;

fn print_info() -> WindowsResult<()> {
    for device_desc in get_audio_devices()?.keys() {
        println!("{}", device_desc);
    }
    println!();
    for application_audio in get_audio_sessions()? {
        println!("{}: {}%", application_audio.get_process_path()?, application_audio.get_volume()? * 100f32);
    }
    Ok(())
}

fn main() {
    if let Err(e) = print_info() {
        println!("{}", e);
    }
}
