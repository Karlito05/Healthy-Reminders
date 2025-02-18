// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

use std::error::Error;
slint::include_modules!();

fn play_sound_1() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source_a = SineWave::new(261.63)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    let source_b = SineWave::new(329.63)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    let source_c = SineWave::new(392.0)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    let source_d = SineWave::new(523.25)
        .take_duration(Duration::from_secs_f32(0.75))
        .amplify(0.20);
    sink.append(source_a);
    sink.append(source_b);
    sink.append(source_c);
    sink.append(source_d);
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}

fn play_sound_3() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source_a = SineWave::new(392.00)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    let source_b = SineWave::new(493.88)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    let source_c = SineWave::new(587.33)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    let source_d = SineWave::new(783.99)
        .take_duration(Duration::from_secs_f32(0.75))
        .amplify(0.20);
    sink.append(source_a);
    sink.append(source_b);
    sink.append(source_c);
    sink.append(source_d);
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}

fn play_sound_2() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source_a = SineWave::new(261.63)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    let source_b = SineWave::new(329.63)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);

    sink.append(source_b);
    sink.append(source_a);
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_clicked_eye_snooze_button(move || {
        let ui = ui_handle.unwrap();

        ui.set_enabled_eye_snooze_button(false);
        ui.set_value_eye_timer(901);
        ui.set_clicked_eye_snooze_button_var(true);
        ui.set_current_eye_timer_progress_max(1200);
    });

    let ui_handle = ui.as_weak();
    ui.on_clicked_eye_do_button(move || {
        let ui = ui_handle.unwrap();

        ui.set_enabled_eye_do_button(false);
        ui.set_enabled_eye_snooze_button(false);
        ui.set_value_eye_timer(21);
        ui.set_clicked_eye_do_button_var(true);

        ui.set_current_eye_timer_progress_max(20);
    });

    let ui_handle = ui.as_weak();
    ui.on_ticked_eye_timer(move |int| {
        let ui = ui_handle.unwrap();

        let mut value = int;
        let mut clicked_eye_do_button = ui.get_clicked_eye_do_button_var();
        let mut clicked_eye_snooze_button = ui.get_clicked_eye_snooze_button_var();
        let mut current_eye_timer_progress_max = ui.get_current_eye_timer_progress_max();

        value -= 1;

        if value == 0 && clicked_eye_do_button {
            ui.set_enabled_eye_snooze_button(false);
            ui.set_enabled_eye_do_button(false);

            play_sound_2();

            value = 1200;
            current_eye_timer_progress_max = 1200;
            clicked_eye_do_button = false;
            clicked_eye_snooze_button = false;
        } else if value == 0 && !clicked_eye_do_button && clicked_eye_snooze_button {
            ui.set_enabled_eye_snooze_button(false);

            play_sound_1();

            value = 120;
            clicked_eye_do_button = false;
        } else if value == 0 && !clicked_eye_do_button && !clicked_eye_snooze_button {
            ui.set_enabled_eye_snooze_button(true);
            ui.set_enabled_eye_do_button(true);

            play_sound_1();

            value = 120;
            current_eye_timer_progress_max = 120;
        }

        let mut minutes = ((value - value % 60) / 60).to_string();
        let mut seconds = (value % 60).to_string();

        if minutes == "0"
            || minutes == "1"
            || minutes == "2"
            || minutes == "3"
            || minutes == "4"
            || minutes == "5"
            || minutes == "6"
            || minutes == "7"
            || minutes == "8"
            || minutes == "9"
        {
            minutes = "0".to_owned() + &minutes;
        }

        if seconds == "0"
            || seconds == "1"
            || seconds == "2"
            || seconds == "3"
            || seconds == "4"
            || seconds == "5"
            || seconds == "6"
            || seconds == "7"
            || seconds == "8"
            || seconds == "9"
        {
            seconds = "0".to_owned() + &seconds;
        }

        ui.set_clicked_eye_do_button_var(clicked_eye_do_button);
        ui.set_clicked_eye_snooze_button_var(clicked_eye_snooze_button);
        ui.set_value_eye_timer(value);
        ui.set_text_eye_timer((minutes + ":" + &seconds).into());
        ui.set_progress_percent_eye_timer(value as f32 / current_eye_timer_progress_max as f32);
        ui.set_current_eye_timer_progress_max(current_eye_timer_progress_max);
    });

    let ui_handle = ui.as_weak();
    ui.on_clicked_drink_snooze_button(move || {
        let ui = ui_handle.unwrap();

        ui.set_enabled_drink_snooze_button(false);
        ui.set_value_drink_timer(901);
        ui.set_clicked_drink_snooze_button_var(true);
        ui.set_current_drink_timer_progress_max(1800);
    });

    let ui_handle = ui.as_weak();
    ui.on_clicked_drink_do_button(move || {
        let ui = ui_handle.unwrap();

        ui.set_enabled_drink_snooze_button(false);
        ui.set_value_drink_timer(1);
        ui.set_clicked_drink_do_button_var(true);

        ui.set_current_drink_timer_progress_max(1800);
    });

    let ui_handle = ui.as_weak();
    ui.on_ticked_drink_timer(move |int| {
        let ui = ui_handle.unwrap();

        let mut value = int;
        let mut clicked_drink_do_button = ui.get_clicked_drink_do_button_var();
        let mut clicked_drink_snooze_button = ui.get_clicked_drink_snooze_button_var();
        let mut current_drink_timer_progress_max = ui.get_current_drink_timer_progress_max();

        value -= 1;

        if value == 0 && clicked_drink_do_button {
            ui.set_enabled_drink_snooze_button(false);

            value = 1800;
            current_drink_timer_progress_max = 1800;
            clicked_drink_do_button = false;
            clicked_drink_snooze_button = false;
        } else if value == 0 && !clicked_drink_do_button && clicked_drink_snooze_button {
            ui.set_enabled_drink_snooze_button(false);

            play_sound_3();

            value = 240;
            clicked_drink_do_button = false;
        } else if value == 0 && !clicked_drink_do_button && !clicked_drink_snooze_button {
            ui.set_enabled_drink_snooze_button(true);

            play_sound_3();

            value = 240;
            current_drink_timer_progress_max = 240;
        }

        let mut minutes = ((value - value % 60) / 60).to_string();
        let mut seconds = (value % 60).to_string();

        if minutes == "0"
            || minutes == "1"
            || minutes == "2"
            || minutes == "3"
            || minutes == "4"
            || minutes == "5"
            || minutes == "6"
            || minutes == "7"
            || minutes == "8"
            || minutes == "9"
        {
            minutes = "0".to_owned() + &minutes;
        }

        if seconds == "0"
            || seconds == "1"
            || seconds == "2"
            || seconds == "3"
            || seconds == "4"
            || seconds == "5"
            || seconds == "6"
            || seconds == "7"
            || seconds == "8"
            || seconds == "9"
        {
            seconds = "0".to_owned() + &seconds;
        }

        ui.set_clicked_drink_do_button_var(clicked_drink_do_button);
        ui.set_clicked_drink_snooze_button_var(clicked_drink_snooze_button);
        ui.set_value_drink_timer(value);
        ui.set_text_drink_timer((minutes + ":" + &seconds).into());
        ui.set_progress_percent_drink_timer(value as f32 / current_drink_timer_progress_max as f32);
        ui.set_current_drink_timer_progress_max(current_drink_timer_progress_max);
    });

    ui.run()?;
    Ok(())
}
