use std::process::Command;

use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "com.carsonetb.Dashboard";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build);

    app.run()
}

fn build(app: &adw::Application) {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let clock = gtk::Label::builder()
        .css_classes(["clock"])
        .label(chrono::Local::now().format("%H:%M:%S %B %d %Y").to_string())
        .margin_top(12)
        .build();

    let cloned = clock.clone();
    glib::timeout_add_seconds_local(1, move || {
        let time_string = chrono::Local::now().format("%H:%M:%S %B %d %Y").to_string();
        cloned.set_label(&time_string);
        glib::ControlFlow::Continue
    });

    let battery = battery::Manager::new()
        .unwrap()
        .batteries()
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let percentage: f32 = (battery.state_of_charge() * 100.0).into();
    let battery_label = gtk::Label::builder()
        .label(format!(
            "{} {percentage:.0}%",
            if battery.state() == battery::State::Charging {
                "Charge"
            } else {
                "Battery"
            }
        ))
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    let battery = gtk::ProgressBar::builder()
        .css_classes(["battery-bar"])
        .orientation(gtk::Orientation::Horizontal)
        .fraction((percentage / 100.0) as f64)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .hexpand(true)
        .build();

    let battery_overlay = gtk::Overlay::new();
    battery_overlay.set_child(Some(&battery));
    battery_overlay.add_overlay(&battery_label);

    let buttons = gtk::Box::builder()
        .css_classes(["buttons"])
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let power = gtk::Button::builder().label("").margin_end(12).build();
    let suspend = gtk::Button::builder().label("󰤁").margin_end(12).build();
    let restart = gtk::Button::builder().label("󰜉").margin_end(12).build();
    let network = gtk::Button::builder().label("").margin_end(12).build();
    let bluetooth = gtk::Button::builder().label("󰂯").margin_end(12).build();
    let volume = gtk::Button::builder().label("").build();

    buttons.append(&power);
    buttons.append(&suspend);
    buttons.append(&restart);
    buttons.append(&network);
    buttons.append(&bluetooth);
    buttons.append(&volume);

    content.append(&clock);
    content.append(&battery_overlay);
    content.append(&buttons);

    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("app.css"));

    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_THEME,
        );
    }

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Dashboard")
        .content(&content)
        .build();

    let controller = gtk::EventControllerKey::new();

    let window_cloned = window.clone();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk::gdk::Key::Escape || key == gtk::gdk::Key::space {
            window_cloned.close();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });

    let window_cloned = window.clone();
    network.connect_clicked(move |_| {
        Command::new("adw-network").spawn().unwrap();
        window_cloned.close();
    });

    let window_cloned = window.clone();
    bluetooth.connect_clicked(move |_| {
        Command::new("bm-sidebar").spawn().unwrap();
        window_cloned.close();
    });

    let window_cloned = window.clone();
    power.connect_clicked(move |_| {
        Command::new("shutdown").args(["now"]).spawn().unwrap();
        window_cloned.close();
    });

    let window_cloned = window.clone();
    suspend.connect_clicked(move |_| {
        Command::new("systemctl").args(["suspend"]).spawn().unwrap();
        window_cloned.close();
    });

    let window_cloned = window.clone();
    restart.connect_clicked(move |_| {
        Command::new("reboot").spawn().unwrap();
        window_cloned.close();
    });

    window.add_controller(controller);

    window.present();
}
