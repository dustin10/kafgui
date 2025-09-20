use chrono::{DateTime, Local};
use eframe::egui;
use egui_table::{AutoSizeMode, Column, HeaderRow, TableDelegate};
use std::collections::HashMap;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        ..Default::default()
    };

    let mut app = App {
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

            let headers = vec![HeaderRow::new(20.0)];

            let columns = vec![
                Column::new(20.0),
                Column::new(20.0),
                Column::new(20.0),
                Column::new(20.0),
            ];

            egui_table::Table::new()
                .headers(headers)
                .columns(columns)
                .auto_size_mode(AutoSizeMode::Always)
                .num_rows(app.records.len() as u64)
                .show(ui, &mut app);
        });
    })
}

struct App {
    records: Vec<Record>,
}

impl TableDelegate for App {
    fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::HeaderCellInfo) {
        match cell.group_index {
            0 => {
                let _ = ui.heading("Partition");
            }
            1 => {
                let _ = ui.heading("Offset");
            }
            2 => {
                let _ = ui.heading("Key");
            }
            3 => {
                let _ = ui.heading("Timestamp");
            }
            _ => {}
        }
    }
    fn cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::CellInfo) {
        let record = self
            .records
            .get(cell.row_nr as usize)
            .expect("record for row exists");

        match cell.col_nr {
            0 => {
                let _ = ui.label(record.partition.to_string());
            }
            1 => {
                let _ = ui.label(record.offset.to_string());
            }
            2 => {
                let _ = ui.label(record.key.clone().unwrap_or_default());
            }
            3 => {
                let _ = ui.label(record.timestamp.to_string());
            }
            _ => {}
        }
    }
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
