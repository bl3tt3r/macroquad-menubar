use macroquad::prelude::*;
use macroquad_xp_barre_menu::*;

#[macroquad::main("Default")]
async fn main() {
    let mut barre = Barre::default()
        .with_menu(
            Menu::new("Game")
                .with_item(Button::new("New", || println!("New")))
                .with_item(Separator)
                .with_item(Radio::new(vec!["Beginner", "Intermediate", "Expert"], |v| println!("Value: {}", v)))
                .with_item(Button::new("Custom...", || println!("New")))
                .with_item(Separator)
                .with_item(Checkbox::new("Marks (?)", false, |v| println!("Value: {}", v)))
                .with_item(Checkbox::new("Color", false, |v| println!("Value: {}", v)))
                .with_item(Checkbox::new("Sound", false, |v| println!("Value: {}", v)))
                .with_item(Separator)
                .with_item(Button::new("Best Times...", || println!("New")))
                .with_item(Separator)
                .with_item(Button::new("Exit", || std::process::exit(0))),
        )
        .with_menu(Menu::new("Help").with_item(Button::new("Did you really think you would find help here ?", || println!("About"))));

    const FONT_DATA: &[u8] = include_bytes!("style_roboto.ttf");
    let font = load_ttf_font_from_bytes(FONT_DATA).expect("Impossible de charger la font !");

    let settings = Settings {
        font,
        font_size: 14,
        height: 30.0,
        width: 180.0,
        barre_background_color: Color::from_hex(0x24292e),
        barre_border_color: Color::from_hex(0x1d2125),
        item_hover_color: Color::from_hex(0x464c52),
        menu_color: Color::from_hex(0x30363d),
        text_color: WHITE,
        text_hover_color: WHITE,
        menu_shadow_color: Color::from_hex(0x1d2125),
    };

    loop {
        clear_background(BLACK);
        barre.draw(&settings);
        next_frame().await
    }
}
