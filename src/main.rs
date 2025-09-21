use chrono::{DateTime, Local};
use eframe::egui;
use egui_taffy::{taffy, tui, TuiBuilderLogic};
use std::collections::HashMap;
use taffy::prelude::{fr, length, percent, span};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        ..Default::default()
    };

    let state = State {
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

            tui(ui, "records")
                .reserve_available_space()
                .style(taffy::Style {
                    display: taffy::Display::Grid,
                    grid_template_columns: vec![fr(1.0), fr(1.0)],
                    grid_template_rows: vec![fr(1.0), fr(2.0), fr(7.0)],
                    gap: length(8.0),
                    size: percent(1.0),
                    align_items: Some(taffy::AlignItems::Stretch),
                    align_content: Some(taffy::AlignContent::Stretch),
                    justify_items: Some(taffy::AlignItems::Stretch),
                    ..Default::default()
                })
                .show(|tui| {
                    tui.style(taffy::Style {
                        grid_column: span(1),
                        grid_row: span(3),
                        ..Default::default()
                    })
                    .add_with_border(|tui| {
                        tui.ui(|ui| {
                            egui_extras::TableBuilder::new(ui)
                                .id_salt("record-list")
                                .column(egui_extras::Column::auto())
                                .column(egui_extras::Column::auto())
                                .column(egui_extras::Column::auto())
                                .column(egui_extras::Column::auto())
                                .header(20.0, |mut hdr| {
                                    hdr.col(|ui| {
                                        let _ = ui.heading("Partition");
                                    });
                                    hdr.col(|ui| {
                                        let _ = ui.heading("Offset");
                                    });
                                    hdr.col(|ui| {
                                        let _ = ui.heading("Key");
                                    });
                                    hdr.col(|ui| {
                                        let _ = ui.heading("Timestamp");
                                    });
                                })
                                .body(|body| {
                                    body.rows(20.0, state.records.len(), |mut row| {
                                        let record = state
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
                                            let _ =
                                                ui.label(record.key.clone().unwrap_or_default());
                                        });
                                        row.col(|ui| {
                                            let _ = ui.label(record.timestamp.to_string());
                                        });
                                    })
                                });
                        });
                    });

                    tui.style(taffy::Style {
                        grid_column: span(1),
                        grid_row: span(1),
                        ..Default::default()
                    })
                    .add_with_border(|tui| {
                        tui.ui(|ui| {
                            let record = state.records.first().expect("record exists");

                            egui_extras::TableBuilder::new(ui)
                                .id_salt("record-info")
                                .column(egui_extras::Column::auto())
                                .column(egui_extras::Column::auto())
                                .body(|body| {
                                    body.rows(20.0, 4, |mut row| match row.index() {
                                        0 => {
                                            row.col(|ui| {
                                                let _ = ui.label("Topic:");
                                            });
                                            row.col(|ui| {
                                                let _ = ui.label(&record.topic);
                                            });
                                        }
                                        1 => {
                                            row.col(|ui| {
                                                let _ = ui.label("Partition:");
                                            });
                                            row.col(|ui| {
                                                let _ = ui.label(record.partition.to_string());
                                            });
                                        }
                                        2 => {
                                            row.col(|ui| {
                                                let _ = ui.label("Offset:");
                                            });
                                            row.col(|ui| {
                                                let _ = ui.label(record.offset.to_string());
                                            });
                                        }
                                        3 => {
                                            row.col(|ui| {
                                                let _ = ui.label("Timestamp:");
                                            });
                                            row.col(|ui| {
                                                let _ = ui.label(record.timestamp.to_string());
                                            });
                                        }
                                        _ => {
                                            panic!("unexpected record info row")
                                        }
                                    })
                                });
                        });
                    });

                    tui.style(taffy::Style {
                        grid_column: span(1),
                        grid_row: span(1),
                        ..Default::default()
                    })
                    .add_with_border(|tui| {
                        tui.ui(|ui| {
                            let record = state.records.first().expect("record exists");

                            egui_extras::TableBuilder::new(ui)
                                .id_salt("record-headers")
                                .column(egui_extras::Column::auto())
                                .column(egui_extras::Column::auto())
                                .header(20.0, |mut hdr| {
                                    hdr.col(|ui| {
                                        let _ = ui.heading("Key");
                                    });
                                    hdr.col(|ui| {
                                        let _ = ui.heading("Value");
                                    });
                                })
                                .body(|body| {
                                    body.rows(20.0, record.headers.len(), |mut row| {
                                        // TODO: cleanup
                                        let (key, value) =
                                            record.headers.iter().next().expect("header exists");

                                        row.col(|ui| {
                                            let _ = ui.label(key);
                                        });
                                        row.col(|ui| {
                                            let _ = ui.label(value);
                                        });
                                    });
                                });
                        });
                    });

                    tui.style(taffy::Style {
                        grid_column: span(1),
                        grid_row: span(1),
                        ..Default::default()
                    })
                    .add_with_border(|tui| {
                        tui.ui(|ui| {
                            let record = state.records.first().expect("record exists");
                            let mut value = record.value.clone().unwrap_or_default();
                            ui.text_edit_multiline(&mut value);
                        });
                    });
                });
        });
    })
}

struct State {
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
    let mut headers = HashMap::new();
    headers.insert(String::from("foo"), String::from("bar"));

    vec![
        Record {
            topic: String::from("users"),
            partition: 0,
            offset: 0,
            key: Some(String::from("1")),
            headers: headers.clone(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 1,
            offset: 0,
            key: Some(String::from("2")),
            headers: headers.clone(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 0,
            offset: 1,
            key: Some(String::from("3")),
            headers: headers.clone(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 0,
            offset: 2,
            key: Some(String::from("6")),
            headers: headers.clone(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
        Record {
            topic: String::from("users"),
            partition: 2,
            offset: 0,
            key: Some(String::from("4")),
            headers: headers.clone(),
            value: Some(String::from("{}")),
            timestamp: Local::now(),
        },
    ]
}
