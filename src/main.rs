use cursive::views::Dialog;
use cursive::Cursive;
use std::thread;

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("do you like my balls?")
            .title("balls survey")
            .button("Next", chat),
    );

    siv.run();

    api::get_message().await?;

    Ok(())

}

fn chat(s: &mut Cursive) {
    
    s.pop_layer();
    s.add_layer(
        Dialog::text("We are going to login")
            .title("Login")
            .button("Login", |s| s.add_layer(Dialog::info("Login *beep boop*")))
            .button("Take me back to discord", |s| s.quit()),
    );

}
