use std::{fs::File, io::BufReader};

use serde_xml_rs::from_reader;

use crate::warhammer_data::ros_parser::ros_parser::Roster;

mod warhammer_data;

use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button)
}

fn main() -> Result<(), PlatformError> {
    // let file = File::open("Astra_1000/Astra_1000.ros")?;
    // let buf = BufReader::new(file);

    // let roster: Roster = from_reader(buf).unwrap();
    // let army = warhammer_data::army::Army::from_roster(
    //     &roster
    // );

    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)?;

    Ok(())
}
