mod components;

use std::collections::HashMap;

use components::rowheader::RowHeader;
use gtk4::{
    prelude::*, Application, ApplicationWindow, Box, Button, DropDown, Entry, ListBox, StringObject,
};
use sourceview5::{self, View};

#[derive(Debug)]
enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
}

#[derive(Debug)]
struct Request {
    method: RequestMethod,
    url: String,
    body: String,
}

struct InterfaceState {
    method: DropDown,
    url: Entry,
    request_body: View,
}

fn mock_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("Accept"), String::from("text/html"));
    map.insert(
        String::from("User-Agent"),
        String::from("request-maker/1.0"),
    );
    map.insert(String::from("Accept-Encoding"), String::from("bzig"));

    map
}

/* fn create_row(header_name: &str, header_value: &str) -> Box {
    let entry_name = Entry::builder().text(header_name).build();
    let entry_value = Entry::builder().text(header_value).build();

    let entry_box = Box::builder()
        .homogeneous(true)
        .spacing(10)
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .margin_bottom(5)
        .build();
    entry_box.append(&entry_name);
    entry_box.append(&entry_value);

    entry_box
} */

fn populate_list(list_box: &ListBox, map: &HashMap<String, String>) {
    for (name, value) in map.iter() {
        // let entry_box = create_row(&name, &value);
        let rowheader = RowHeader::new(&name, &value);
        list_box.append(&rowheader);
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.emarifer.not-postman")
        .build();

    app.connect_activate(|app| {
        sourceview5::init();
        let ui_src = include_str!("../data/ui/prototype.ui");
        let builder = gtk4::Builder::from_string(ui_src);

        let request_method = builder.object::<DropDown>("method").unwrap();
        let request_url = builder.object::<Entry>("url").unwrap();
        let request_body = builder.object::<View>("request_body").unwrap();
        let request_state = InterfaceState {
            method: request_method,
            url: request_url,
            request_body,
        };

        let button_send = builder.object::<Button>("send").unwrap();
        button_send.connect_clicked(move |_| {
            let url = request_state.url.text();
            let method = request_state
                .method
                .selected_item()
                .unwrap()
                .downcast::<StringObject>()
                .unwrap()
                .string();
            let (start, end) = request_state.request_body.buffer().bounds();
            let body = request_state.request_body.buffer().text(&start, &end, true);
            println!("{} {}", method, url);
            println!("{}", body)
        });

        let request_headers_list = builder.object::<ListBox>("request_headers").unwrap();
        let headers = mock_map();
        populate_list(&request_headers_list, &headers);

        let window = builder
            .object::<ApplicationWindow>("win")
            .expect("Couldn't get window");

        window.set_application(Some(app));

        window.present()
    });

    app.run()
}
