use macroquad::prelude::*;
use macroquad_xp_barre_menu::*;

#[macroquad::main("Menu Demo")]
async fn main() {
    let mut barre = Barre::default()
        .with_menu(Menu::new("Game"))
        .with_menu(Menu::new("Menu 1"))
        .with_menu(Menu::new("Menu 2"))
        .with_menu(Menu::new("Menu 3"))
        .with_menu(Menu::new("Menu 4"))
        .with_menu(Menu::new("Menu 5"));

    let settings = &Settings::default();

    loop {
        clear_background(WHITE);
        barre.draw(settings);
        next_frame().await;
    }
}
