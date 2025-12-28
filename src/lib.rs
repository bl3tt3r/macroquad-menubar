use macroquad::prelude::*;

#[derive(Default)]
pub struct State {
    actif_menu_id: Option<u8>,
    last_menu_id: u8,
}

pub struct Settings {
    pub font: Font,
    pub font_size: u16,
    pub height: f32,
    pub width: f32,
    pub barre_background_color: Color,
    pub barre_border_color: Color,
    pub item_hover_color: Color,
    pub menu_color: Color,
    pub text_color: Color,
    pub text_hover_color: Color,
    pub menu_shadow_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        let font_bytes: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tahoma.ttf"));
        let font = load_ttf_font_from_bytes(font_bytes).unwrap();
        Self {
            font,
            font_size: 14,
            height: 26.0,
            width: 220.0,
            barre_background_color: Color::from_hex(0xe9e6d3),
            barre_border_color: Color::from_hex(0xa0a0a0),
            item_hover_color: Color::from_hex(0x316ac5),
            menu_color: WHITE,
            text_color: BLACK,
            text_hover_color: WHITE,
            menu_shadow_color: Color::from_hex(0xd2d2d2),
        }
    }
}

impl Settings {
    #[inline]
    pub fn text_params(&self, over: bool) -> TextParams<'_> {
        TextParams {
            font: Some(&self.font),
            font_size: self.font_size,
            color: if over { self.text_hover_color } else { self.text_color },
            ..Default::default()
        }
    }

    #[inline]
    fn text_y_center(&self, origine_y: f32) -> f32 {
        origine_y + (self.height / 2.0) + (self.font_size as f32 / 3.0)
    }
}

pub trait Component {
    fn draw(&mut self, origine: Vec2, settings: &Settings, state: &mut State) -> Vec2;
}

pub struct Checkbox {
    actif: bool,
    label: String,
    on_change: Box<dyn Fn(bool)>,
}

impl Checkbox {
    pub fn new<F>(label: &str, actif: bool, on_change: F) -> Self
    where
        F: Fn(bool) + 'static,
    {
        Self {
            label: label.to_string(),
            actif,
            on_change: Box::new(on_change),
        }
    }
}

impl Component for Checkbox {
    fn draw(&mut self, origine: Vec2, settings: &Settings, _: &mut State) -> Vec2 {
        let mouse_pos = mouse_position().into();
        let rect = Rect::new(origine.x, origine.y, settings.width, settings.height);
        let hover = rect.contains(mouse_pos);

        // Dessiner le fond
        let bg_color = if hover { settings.item_hover_color } else { settings.menu_color };
        draw_rectangle(origine.x, origine.y, settings.width, settings.height, bg_color);

        // Gérer le clic
        if hover && is_mouse_button_pressed(MouseButton::Left) {
            self.actif = !self.actif;
            (self.on_change)(self.actif);
        }

        // Dessiner le texte
        let text_y = settings.text_y_center(origine.y);
        draw_text_ex(&self.label, origine.x + settings.height, text_y, settings.text_params(hover));

        // Dessiner la coche si cet élément est actif
        if self.actif {
            draw_checkmark_smooth(
                origine.x + settings.height * 0.15,
                origine.y + settings.height * 0.15,
                settings.height * 0.7,
                settings,
                hover,
            );
        }

        Vec2::new(settings.width, settings.height)
    }
}

pub struct Radio {
    childrens: Vec<String>,
    actif: Option<String>,
    on_change: Box<dyn Fn(&str)>,
}

impl Radio {
    pub fn new<F>(childrens: Vec<&str>, on_change: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        let childrens: Vec<String> = childrens.iter().map(|s| s.to_string()).collect();
        let actif = childrens.first().cloned();
        Self {
            childrens,
            actif,
            on_change: Box::new(on_change),
        }
    }

    pub fn get_actif(&self) -> Option<&String> {
        self.actif.as_ref()
    }
}

impl Component for Radio {
    fn draw(&mut self, origine: Vec2, settings: &Settings, _: &mut State) -> Vec2 {
        let mouse_pos = mouse_position().into();
        for (i, children) in self.childrens.iter().enumerate() {
            let padding = i as f32 * settings.height;
            let rect = Rect::new(origine.x, origine.y + padding, settings.width, settings.height);
            let hover = rect.contains(mouse_pos);

            // Dessiner le fond
            let bg_color = if hover { settings.item_hover_color } else { settings.menu_color };
            draw_rectangle(origine.x, origine.y + padding, settings.width, settings.height, bg_color);

            // Gérer le clic
            if hover && is_mouse_button_pressed(MouseButton::Left) {
                let value = children.to_string();
                (self.on_change)(&value);
                self.actif = Some(value);
            }

            // Dessiner le texte
            let text_y = settings.text_y_center(origine.y);
            draw_text_ex(children, origine.x + settings.height, text_y + padding, settings.text_params(hover));

            // Dessiner la coche si cet élément est actif
            if self.actif.as_ref().is_some_and(|v| v == children) {
                draw_checkmark_smooth(
                    origine.x + settings.height * 0.15,
                    origine.y + padding + settings.height * 0.15,
                    settings.height * 0.7,
                    settings,
                    hover,
                );
            }
        }

        Vec2::new(settings.width, settings.height * self.childrens.len() as f32)
    }
}

/// Dessine une coche antialiasée ultra-lisse avec plusieurs techniques d'antialiasing
fn draw_checkmark_smooth(x: f32, y: f32, size: f32, settings: &Settings, hovered: bool) {
    let color = if hovered { settings.text_hover_color } else { settings.text_color };

    // Calcul des points de la coche
    let thickness = (size / 12.0).max(1.5);

    // Points de la coche
    let start_x = x + size * 0.15;
    let start_y = y + size * 0.5;

    let mid_x = x + size * 0.4;
    let mid_y = y + size * 0.75;

    let end_x = x + size * 0.85;
    let end_y = y + size * 0.25;

    // Technique 1: Dessiner plusieurs lignes avec alpha décroissant pour créer un flou
    for i in 0..4 {
        let offset = i as f32 * 0.5;
        let alpha_factor = 1.0 - (i as f32 * 0.25);
        let aa_color = Color::new(color.r, color.g, color.b, color.a * alpha_factor);
        let current_thickness = thickness + offset;

        // Bras gauche (descendant)
        draw_line(start_x, start_y, mid_x, mid_y, current_thickness, aa_color);

        // Bras droit (montant)
        draw_line(mid_x, mid_y, end_x, end_y, current_thickness, aa_color);
    }

    // Technique 2: Ajouter des cercles aux jonctions pour lisser les angles
    let circle_radius = thickness * 0.6;
    draw_circle(start_x, start_y, circle_radius, color);
    draw_circle(mid_x, mid_y, circle_radius, color);
    draw_circle(end_x, end_y, circle_radius, color);

    // Technique 3: Ajouter un halo semi-transparent autour
    let halo_color = Color::new(color.r, color.g, color.b, color.a * 0.1);
    draw_line(start_x, start_y, mid_x, mid_y, thickness + 3.0, halo_color);
    draw_line(mid_x, mid_y, end_x, end_y, thickness + 3.0, halo_color);
}

pub struct Separator;

impl Component for Separator {
    fn draw(&mut self, origine: Vec2, settings: &Settings, _state: &mut State) -> Vec2 {
        let height = settings.height * 0.25;
        let half_height = height * 0.5;

        draw_rectangle(origine.x, origine.y, settings.width, height, settings.menu_color);
        draw_line(
            origine.x + height,
            origine.y + half_height,
            origine.x + settings.width - height,
            origine.y + half_height,
            1.0,
            settings.barre_border_color,
        );
        Vec2::new(settings.width, height)
    }
}

pub struct Button {
    label: String,
    on_click: Box<dyn FnMut()>,
}

impl Button {
    pub fn new<F>(label: &str, on_click: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Self {
            label: label.to_string(),
            on_click: Box::new(on_click),
        }
    }
}

impl Component for Button {
    fn draw(&mut self, origine: Vec2, settings: &Settings, state: &mut State) -> Vec2 {
        let mouse_pos = mouse_position().into();
        let width = settings
            .width
            .max(measure_text(&self.label, Some(&settings.font), settings.font_size, 1.0).width + settings.height * 2.0);
        let rect = Rect::new(origine.x, origine.y, width, settings.height);
        let hover = rect.contains(mouse_pos);

        // Dessiner le fond
        let bg_color = if hover { settings.item_hover_color } else { settings.menu_color };
        draw_rectangle(origine.x, origine.y, width, settings.height, bg_color);

        // Gérer le clic
        if hover && is_mouse_button_pressed(MouseButton::Left) {
            (self.on_click)();
            state.actif_menu_id = None;
        }

        // Dessiner le texte
        let text_y = settings.text_y_center(origine.y);
        draw_text_ex(&self.label, origine.x + settings.height, text_y, settings.text_params(hover));

        Vec2::new(width, settings.height)
    }
}

pub struct Menu {
    label: String,
    id: u8,
    childrens: Vec<Box<dyn Component>>,
}

impl Menu {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            id: 0,
            childrens: Vec::new(),
        }
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn with_item<C>(mut self, item: C) -> Self
    where
        C: Component + 'static,
    {
        self.childrens.push(Box::new(item));
        self
    }
}

impl Component for Menu {
    fn draw(&mut self, origine: Vec2, settings: &Settings, state: &mut State) -> Vec2 {
        let text_dims = measure_text(&self.label, Some(&settings.font), settings.font_size, 1.0);
        let width = text_dims.width + settings.height;

        let mouse_pos = mouse_position().into();
        let rect = Rect::new(origine.x, origine.y, width, settings.height);
        let hover = rect.contains(mouse_pos);

        let actif = state.actif_menu_id == Some(self.id);

        // Dessiner le fond si actif ou hover
        if actif || hover {
            draw_rectangle(origine.x, origine.y, width, settings.height, settings.item_hover_color);
        }

        // Dessiner le texte
        let text_y = settings.text_y_center(origine.y);
        draw_text_ex(
            &self.label,
            origine.x + (settings.height * 0.5),
            text_y,
            settings.text_params(actif || hover),
        );

        let mut click_in_menu = false;

        // Dessiner le menu déroulant
        if actif {
            let menu_origine = Vec2::new(origine.x, origine.y + settings.height);
            let bound = vertical_draw(menu_origine, &mut self.childrens, settings, state);

            click_in_menu = Rect::new(origine.x, origine.y + settings.height, bound.x, bound.y).contains(mouse_pos);

            // Bordure et ombre
            draw_rectangle_lines(menu_origine.x, menu_origine.y, bound.x, bound.y, 2.0, settings.barre_border_color);

            let shadow_offset = Vec2::new(1.0, 2.0);
            draw_line(
                menu_origine.x + bound.x + shadow_offset.x,
                menu_origine.y + shadow_offset.y,
                menu_origine.x + bound.x + shadow_offset.x,
                menu_origine.y + bound.y + shadow_offset.y,
                2.0,
                settings.menu_shadow_color,
            );
            draw_line(
                menu_origine.x + shadow_offset.y,
                menu_origine.y + bound.y + shadow_offset.x,
                menu_origine.x + bound.x + shadow_offset.y,
                menu_origine.y + bound.y + shadow_offset.x,
                2.0,
                settings.menu_shadow_color,
            );
        }

        // Gérer les clics
        if is_mouse_button_pressed(MouseButton::Left) && !click_in_menu {
            state.actif_menu_id = match (hover, actif) {
                (true, true) => None,
                (true, false) => Some(self.id),
                (false, true) => None,
                (false, false) => state.actif_menu_id,
            };
        } else if hover && state.actif_menu_id.is_some() && !actif {
            state.actif_menu_id = Some(self.id);
        }

        Vec2::new(width, settings.height)
    }
}

#[derive(Default)]
pub struct Barre {
    childrens: Vec<Menu>,
    state: State,
}

impl Barre {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_menu(mut self, mut menu: Menu) -> Self {
        self.state.last_menu_id += 1;
        menu.id = self.state.last_menu_id;
        self.childrens.push(menu);
        self
    }

    pub fn draw(&mut self, settings: &Settings) {
        self.draw_background(settings);
        horizontal_draw(Vec2::ZERO, &mut self.childrens, settings, &mut self.state);
    }

    #[inline]
    fn draw_background(&self, settings: &Settings) {
        let width = screen_width();
        draw_rectangle(0.0, 0.0, width, settings.height, settings.barre_background_color);
        draw_rectangle(0.0, settings.height - 1.0, width, 1.0, settings.barre_border_color);
    }
}

#[inline]
fn vertical_draw(mut origine: Vec2, childrens: &mut [Box<dyn Component>], settings: &Settings, state: &mut State) -> Vec2 {
    let mut bound = Vec2::ZERO;
    for child in childrens.iter_mut() {
        let size = child.draw(origine, settings, state);
        origine.y += size.y;
        bound.y += size.y;
        bound.x = bound.x.max(size.x);
    }
    bound
}

#[inline]
fn horizontal_draw(mut origine: Vec2, childrens: &mut [Menu], settings: &Settings, state: &mut State) -> Vec2 {
    let mut bound = Vec2::ZERO;
    for child in childrens.iter_mut() {
        let size = child.draw(origine, settings, state);
        origine.x += size.x;
        bound.x += size.x;
        bound.y = bound.y.max(size.y);
    }
    bound
}
