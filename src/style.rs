use crossterm_style::{Color, ObjectStyle};
use std::convert::TryInto;
use termimad::*;

pub struct TaskStyle {
  pub name: ObjectStyle,
  pub start_time: ObjectStyle,
  pub end_time: ObjectStyle,
  pub created_time: ObjectStyle,
  pub time_span: ObjectStyle,
}

impl Default for TaskStyle {
  fn default() -> Self {
    let name = ObjectStyle::default().fg(Color::Yellow);
    let start_time = ObjectStyle::default().fg(Color::Magenta);
    let end_time = ObjectStyle::default().fg(Color::Magenta);
    let created_time = ObjectStyle::default().fg(Color::Blue);
    let time_span = ObjectStyle::default().fg(Color::Green);

    TaskStyle {
      name,
      start_time,
      end_time,
      created_time,
      time_span,
    }
  }
}

pub struct TaskListStyle {
  pub name: ObjectStyle,
  pub create_date: ObjectStyle,
  pub id: ObjectStyle,
}

impl Default for TaskListStyle {
  fn default() -> Self {
    let name = ObjectStyle::default().fg(Color::Yellow);
    let create_date = ObjectStyle::default().fg(Color::Magenta);
    let id = ObjectStyle::default().fg(Color::Green);

    TaskListStyle {
      name,
      create_date,
      id,
    }
  }
}

pub struct AppStyle {
  pub task: TaskStyle,
  pub error: ObjectStyle,
  pub cmd: ObjectStyle,
  pub report: MadSkin,
  pub task_list: TaskListStyle,
  pub screen_width: Option<usize>,
}

impl Default for AppStyle {
  fn default() -> Self {
    let cmd = ObjectStyle::default();
    let (width, _) = terminal_size();
    let area: Option<usize> = if width < 4 {
      Some(120)
    } else {
      Some(width.try_into().unwrap())
    };
    let mut report = MadSkin::default();
    report.paragraph.align = Alignment::Center;
    report.table.align = Alignment::Center;
    report.bold.set_fg(Color::Yellow);
    report.italic.object_style = ObjectStyle::default();
    report.italic.set_fg(Color::Magenta);
    report.inline_code.set_fgbg(Color::Reset, Color::Reset);

    AppStyle {
      task: TaskStyle::default(),
      task_list: TaskListStyle::default(),
      error: ObjectStyle::default().fg(Color::Red),
      cmd,
      report,
      screen_width: area,
    }
  }
}
