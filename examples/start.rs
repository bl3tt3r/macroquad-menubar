use macroquad::prelude::*;
use macroquad_xp_barre_menu::*;

#[macroquad::main("Menu Demo")]
async fn main() {
    let mut barre = Barre::default()
        .with_menu(
            Menu::new("Game")
                .with_item(Button::new("New", || println!("New")))
                .with_item(Separator)
                .with_item(Radio::new(vec!["Beginner", "Intermediate", "Expert"], |v| println!("Value: {}", v)))
                .with_item(Checkbox::new("Sound", false, |v| println!("Sound: {}", v))),
        )
        .with_menu(Menu::new("Help").with_item(Button::new("About", || println!("About"))));

    let settings = &Settings::default(); // Style XP par défaut

    loop {
        clear_background(WHITE);
        barre.draw(settings);
        next_frame().await;
    }
}
