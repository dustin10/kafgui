use chrono::{DateTime, Local};
use eframe::egui;
use egui_extras::Column;
use std::collections::HashMap;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        ..Default::default()
    };

    let app = App {
        records: stub_records(),
    };

    eframe::run_simple_native("KafGUI", options, move |ctx, _frame| {
        ctx.set_theme(egui::ThemePreference::Dark);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            egui_flex::Flex::horizontal()
                .w_full()
                .h_full()
                .align_items_content(egui::Align2::LEFT_TOP)
                .show(ui, |flex| {
                    flex.add_ui(egui_flex::item().grow(0.5), |ui| {
                        egui_extras::TableBuilder::new(ui)
                            .id_salt("record-list")
                            .column(Column::auto())
                            .column(Column::auto())
                            .column(Column::auto())
                            .column(Column::auto())
                            //.header(20.0, |mut hdr| {
                            //    hdr.col(|ui| {
                            //        let _ = ui.heading("Partition");
                            //    });
                            //    hdr.col(|ui| {
                            //        let _ = ui.heading("Offset");
                            //    });
                            //    hdr.col(|ui| {
                            //        let _ = ui.heading("Key");
                            //    });
                            //    hdr.col(|ui| {
                            //        let _ = ui.heading("Timestamp");
                            //    });
                            //})
                            .body(|body| {
                                body.rows(20.0, app.records.len(), |mut row| {
                                    let record = app
                                        .records
                                        .get(row.index())
                                        .expect("record exists for row");

                                    row.col(|ui| {
                                        let _ = ui.label(record.partition.to_string());
                                    });
                                    row.col(|ui| {
                                        let _ = ui.label(record.offset.to_string());
                                    });
                                    row.col(|ui| {
                                        let _ = ui.label(record.key.clone().unwrap_or_default());
                                    });
                                    row.col(|ui| {
                                        let _ = ui.label(record.timestamp.to_string());
                                    });
                                })
                            });
                    });

                    flex.add_ui(egui_flex::item().grow(0.5), |ui| {
                        egui_flex::Flex::vertical()
                            //.w_full()
                            .h_full()
                            .align_items_content(egui::Align2::LEFT_TOP)
                            .show(ui, |flex| {
                                flex.add_ui(egui_flex::item().grow(0.1), |ui| {
                                    egui_extras::TableBuilder::new(ui)
                                        .id_salt("record-info")
                                        .column(Column::auto())
                                        .column(Column::auto())
                                        //.header(20.0, |mut hdr| {
                                        //    hdr.col(|ui| {
                                        //        let _ = ui.heading("Info");
                                        //    });
                                        //})
                                        .body(|body| {
                                            body.rows(20.0, 4, |mut row| match row.index() {
                                                0 => {
                                                    row.col(|ui| {
                                                        let _ = ui.label("Topic:");
                                                    });
                                                    row.col(|ui| {
                                                        let _ = ui.label("users");
                                                    });
                                                }
                                                1 => {
                                                    row.col(|ui| {
                                                        let _ = ui.label("Partition:");
                                                    });
                                                    row.col(|ui| {
                                                        let _ = ui.label("0");
                                                    });
                                                }
                                                2 => {
                                                    row.col(|ui| {
                                                        let _ = ui.label("Offset:");
                                                    });
                                                    row.col(|ui| {
                                                        let _ = ui.label("0");
                                                    });
                                                }
                                                3 => {
                                                    row.col(|ui| {
                                                        let _ = ui.label("Timestamp:");
                                                    });
                                                    row.col(|ui| {
                                                        let _ = ui.label(Local::now().to_string());
                                                    });
                                                }
                                                _ => {
                                                    panic!("unexpected record info row")
                                                }
                                            })
                                        });
                                });
                                flex.add_ui(egui_flex::item().grow(0.2), |ui| {
                                    egui_extras::TableBuilder::new(ui)
                                        .id_salt("record-headers")
                                        .column(Column::auto())
                                        .column(Column::auto())
                                        //.header(20.0, |mut hdr| {
                                        //    hdr.col(|ui| {
                                        //        let _ = ui.heading("Info");
                                        //    });
                                        //})
                                        .body(|body| {
                                            body.rows(20.0, 1, |mut row| {
                                                row.col(|ui| {
                                                    let _ = ui.label("foo");
                                                });
                                                row.col(|ui| {
                                                    let _ = ui.label("bar");
                                                });
                                            })
                                        });
                                });
                                flex.add_ui(egui_flex::item().grow(0.7), |ui| {
                                    let mut text = String::from("{}");
                                    ui.text_edit_multiline(&mut text);
                                });
                            });
                    });
                });
        });
    })
}

struct App {
    records: Vec<Record>,
}

struct Record {
    topic: String,
    partition: i32,
    offset: i64,
    key: Option<String>,
    headers: HashMap<String, String>,
    value: Option<String>,
    timestamp: DateTime<Local>,
}

fn stub_records() -> Vec<Record> {
    vec![
        Record {
            topic: String::from("users"),
            partition: 0,
            offset: 0,
            key: Some(String::from("1")),
            headers: HashMap::new(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 1,
            offset: 0,
            key: Some(String::from("2")),
            headers: HashMap::new(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 0,
            offset: 1,
            key: Some(String::from("3")),
            headers: HashMap::new(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 0,
            offset: 2,
            key: Some(String::from("6")),
            headers: HashMap::new(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 2,
            offset: 0,
            key: Some(String::from("4")),
            headers: HashMap::new(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
    ]
}
