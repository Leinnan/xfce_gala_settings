/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use gtk::Orientation::Vertical;
use gtk::{
    Button, ButtonExt, CheckButton, ContainerExt, Inhibit, Label, LabelExt, ToggleButtonExt,
    WidgetExt, Window, WindowType, GtkWindowExt
};
use relm::EventStream;

use std::{env, fs, io};
use std::io::Write;
use Msg::*;

// There will be several widgets involved in this example, but this struct
// will act as a container just for the widgets that we will be updating.
struct Widgets {
    use_gala_checkbox: CheckButton,
    counter_label: Label,
}

// This enum holds the various messages that will be passed between our
// widgets. Note that we aren't deriving `Msg` because this example uses
// the `core` module, which is the basic event-handling library that
// `relm` depends on.
#[derive(Clone, Debug)]
enum Msg {
    ChangeWindowManager,
    Decrement,
    Increment,
    Quit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowManager {
    XFWM4,
    Gala,
    Unknown,
}

// This struct represents the model, and it maintains the state needed to
// populate the widget. The model is updated in the `update` method.
struct Model {
    counter: i32,
}

pub fn get_window_manager_xml_path() -> String {
    let user_folder = env::var("HOME").unwrap_or(String::new());

    assert!(user_folder.len() > 0, "HOME var is not set!");

    let data_folder = env::var("XDG_DATA_HOME").unwrap_or(format!("{}/.config", user_folder));
    format!(
        "{}/xfce4/xfconf/xfce-perchannel-xml/xfce4-session.xml",
        data_folder
    )
}

pub fn create_local_window_manager_config_if_missing() {
    let file = get_window_manager_xml_path();
    let base_file_path = "/etc/xdg/xfce4/xfconf/xfce-perchannel-xml/xfce4-session.xml";

    if std::path::Path::new(&file).exists() {
        return;
    }
    assert!(
        std::path::Path::new(&base_file_path).exists(),
        format!("{} does not exist!", base_file_path)
    );
    let result = fs::copy(base_file_path, file);
    if result.is_err() {
        panic!("Error during file copy!");
    }
}

pub fn get_window_manager_in_config() -> WindowManager {
    let file = get_window_manager_xml_path();
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    if contents.contains("\"xfwm4\"") {
        WindowManager::XFWM4
    } else if contents.contains("\"gala\"") {
        WindowManager::Gala
    } else {
        WindowManager::Unknown
    }
}

pub fn replace_window_manager_in_config() {
    let file = get_window_manager_xml_path();
    let contents = fs::read_to_string(&file).expect("Something went wrong reading the file");
    let mut window_manager = WindowManager::Gala;

    let result = if contents.contains("\"xfwm4\"") {
        contents.replace("\"xfwm4\"", "\"gala\"")
    } else if contents.contains("\"gala\"") {
        window_manager = WindowManager::XFWM4;
        contents.replace("\"gala\"", "\"xfwm4\"")
    } else {
        contents
    };
    fs::write(&file, result);
    use std::process::Command;
    let command_name = if window_manager == WindowManager::Gala {
        "gala"
    } else {
        "xfwm4"
    };
    let output = Command::new(command_name)
        .arg("--replace")
        .spawn()
        .expect("Failed to execute command");
    println!("{:?}",output);
}

pub fn prepare() {
    create_local_window_manager_config_if_missing();

    let cur_window_manager = get_window_manager_in_config();

    println!("Current window manager: {:?}", cur_window_manager);
}

fn main() {
    prepare();
    let start_window_manager = get_window_manager_in_config();
    let is_gala = start_window_manager == WindowManager::Gala;
    let is_unknown = start_window_manager == WindowManager::Unknown;
    gtk::init().expect("gtk::init failed");

    // This is a layout container that will stack widgets vertically.
    let builder = gtk::BoxBuilder::new()
        .orientation(Vertical)
        .vexpand(true)
        .halign(gtk::Align::Fill)
        .spacing(12);

    let vbox = builder.build();
    if is_unknown {
        let category_label = gtk::LabelBuilder::new()
        .justify(gtk::Justification::Center)
        .halign(gtk::Align::Fill)
        .vexpand(true)
        .use_markup(true)
        .label(&format!("<span foreground=\"red\"><big>Cannot determine current window manager,\ncheck file: {}</big></span>", get_window_manager_xml_path()))
        .build();
        vbox.add(&category_label);
    }

    let use_gala_checkbox = gtk::CheckButtonBuilder::new()
        .label("Use Gala window manager")
        .active(is_gala)
        .vexpand(true)
        .sensitive(!is_unknown)
        .halign(gtk::Align::Fill)
        .build();
    vbox.add(&use_gala_checkbox);
    let category_label = gtk::LabelBuilder::new()
        .justify(gtk::Justification::Left)
        .halign(gtk::Align::Start)
        .vexpand(true)
        .use_markup(true)
        .sensitive(!is_unknown)
        .label("<b>Gala options</b>")
        .build();
    vbox.add(&category_label);
    // Add some widgets to the layout.
    let plus_button = Button::with_label("+");
    vbox.add(&plus_button);
    let counter_label = Label::new(Some("0"));
    vbox.add(&counter_label);
    let minus_button = Button::with_label("-");
    vbox.add(&minus_button);

    // As mentioned above, this struct holds the labels that we're going
    // to be updating.
    let widgets = Widgets {
        use_gala_checkbox: use_gala_checkbox,
        counter_label: counter_label,
    };

    // Create a new window and add our layout container to it.
    let window = Window::new(WindowType::Toplevel);
    window.set_border_width(12);
    window.add(&vbox);
    window.set_resizable(false);

    // Now we're going to create two event streams. The first stream will be
    // passed messages directly from the widgets in the application, and will
    // pass the same messages on to the second stream.
    //
    // In this example we're just printing the messages to stdout, but in practice
    // you could take some other kind of action i.e. if stream 1 receives the message `Foo`,
    // send the message `Bar` to stream 2.
    let main_stream = EventStream::new();
    let echo_stream = EventStream::new();

    // Here we add an observer (a closure) to the second event stream. Any message
    // passed to the stream will be passed as an argument to this closure, so adding
    // an observer is how you respond to events generated by your application.
    echo_stream.observe(move |event: &Msg| {
        println!("...Echo: {:?}", event);
    });

    // Here we add an observer to the first stream. First it prints the message, and
    // then it passes a copy of the message to the second stream.
    {
        main_stream.observe(move |event: &Msg| {
            println!("Event: {:?}", event);
            echo_stream.emit(event.clone());
        });
    }

    // Send the `Increment` message when `plus_button` emits the `clicked` signal.
    {
        let stream = main_stream.clone();
        plus_button.connect_clicked(move |_| {
            stream.emit(Increment);
        });
    }

    // Send the `Increment` message when `plus_button` emits the `clicked` signal.
    {
        let stream = main_stream.clone();
        widgets.use_gala_checkbox.connect_clicked(move |_| {
            stream.emit(ChangeWindowManager);
        });
    }

    // Send the `Decrement` message when `minus_button` emits the `clicked` signal.
    {
        let stream = main_stream.clone();
        minus_button.connect_clicked(move |_| {
            stream.emit(Decrement);
        });
    }

    window.show_all();

    // Close the window and quit when the window close button is clicked.
    {
        let stream = main_stream.clone();
        window.connect_delete_event(move |_, _| {
            stream.emit(Quit);
            Inhibit(false)
        });
    }

    // Create the initial state of the model.
    let mut model = Model { counter: 0 };

    // Here we respond to messages that are generated and update the model.
    fn update(event: Msg, model: &mut Model, widgets: &Widgets) {
        match event {
            ChangeWindowManager => {
                {
                    let cur_value = get_window_manager_in_config();
                    println!("{:?}", cur_value);
                }
                replace_window_manager_in_config();
                {
                    let cur_value = get_window_manager_in_config();
                    println!("{:?}", cur_value);
                }
            }
            Decrement => {
                model.counter -= 1;
                widgets.counter_label.set_text(&model.counter.to_string());
            }
            Increment => {
                model.counter += 1;
                widgets.counter_label.set_text(&model.counter.to_string());
            }
            Quit => gtk::main_quit(),
        }
    }

    main_stream.set_callback(move |msg| {
        update(msg, &mut model, &widgets);
    });

    gtk::main();


    let end_window_manager = get_window_manager_in_config();
    if start_window_manager != end_window_manager {
        use std::process::Command;
        let command_name = if end_window_manager == WindowManager::Gala {
            "gala"
        } else {
            "xfwm4"
        };
        let output = Command::new(command_name)
            .arg("--replace")
            .spawn()
            .expect("Failed to execute command");
        println!("{:?}",output);
    }
}
