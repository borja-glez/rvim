use std::{env, fs, path::PathBuf};

use crate::config::TAB_STOP;

pub struct EditorRows {
    row_contents: Vec<Row>,
    pub filename: Option<PathBuf>,
}

impl EditorRows {
    pub fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_contents: Vec::new(),
                filename: None,
            },
            Some(file) => Self::from_file(file.into()),
        }
    }

    fn from_file(file: PathBuf) -> Self {
        let file_contents = fs::read_to_string(&file).expect("Unable to read file");
        Self {
            filename: Some(file),
            row_contents: file_contents
                .lines()
                .map(|it| {
                    let mut row = Row::new(it.into(), String::new());
                    Self::render_row(&mut row);
                    row
                })
                .collect(),
        }
    }

    pub fn get_render(&self, at: usize) -> &String {
        &self.row_contents[at].render
    }

    pub fn get_editor_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }

    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub fn get_row(&self, at: usize) -> &str {
        &self.row_contents[at].row_content
    }

    fn render_row(row: &mut Row) {
        let mut index = 0;
        let capacity = row
            .row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_STOP } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                row.render.push(' ');
                while index % TAB_STOP != 0 {
                    row.render.push(' ');
                    index += 1;
                }
            } else {
                row.render.push(c);
            }
        })
    }
}

pub struct Row {
    pub row_content: Box<str>, // Box<str> instead of String to save memory, immutable string
    render: String,
}

impl Row {
    pub fn new(row_content: Box<str>, render: String) -> Self {
        Self {
            row_content,
            render,
        }
    }
}
