// Super simple todo example

use tuix::*;

use tuix::style::themes::DEFAULT_THEME;
fn main() {


    let tb = Textbox::new("Test");

    let app = Application::new(|wind_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);
        
        //window.set_align_items(state, AlignItems::Center);

        let row = HBox::new().build(state, window, |builder| builder);
        //Textbox::new("Add Item").build(state, row, |builder| builder.set_margin(Length::Pixels(10.0)));
        tb.build(state, row, |builder| builder);
        Button::with_label("Add").build(state, row, |builder| builder.set_margin(Length::Pixels(10.0)));
        
        wind_desc
    });

    app.run();
}