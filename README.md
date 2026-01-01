
# macroquad_xp_barre_menu


[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
![Rust Edition](https://img.shields.io/badge/edition-2024-orange)
![Macroquad](https://img.shields.io/badge/macroquad-0.4.14-success)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?logo=rust)

> Barre de menu style Windows XP pour Macroquad — Simple, légère et nostalgique ! 🪟

![Fenetre avec style windows XP par defaut](docs/default.png)

---

##  🎯 À propos

macroquad_xp_barre_menu est une bibliothèque Rust qui fournit une barre de menu style Windows XP pour vos jeux et applications Macroquad. Elle s'intègre parfaitement dans la boucle de rendu immediate-mode de Macroquad et propose un style rétro authentique avec la police Tahoma et les couleurs classiques de Windows XP.

Idéale pour :

- 🎮 Jeux rétro ou nostalgiques
- 🛠️ Outils de développement 2D
- 📱 Applications avec interface simple
- 🎨 Projets nécessitant un style visuel distinctif

---

## ✨ Fonctionnalités

- ✅ Barre de menu horizontale avec menus déroulants
- ✅ 4 types d'items :
    - Button : action au clic
    - Checkbox : interrupteur on/off
    - Radio : sélection exclusive
    - Separator : séparateur visuel
- ✅ Style Windows XP authentique par défaut (police Tahoma, couleurs fidèles)
- ✅ Entièrement personnalisable (couleurs, tailles, polices)
- ✅ Multiplateforme : Windows, Linux, macOS, WASM, Android, iOS
- ✅ Zero dependencies en dehors de Macroquad
- ✅ API simple et intuitive avec pattern builder

---

## 📦 Installation

Ajoutez cette dépendance à votre `Cargo.toml` :

```toml
[dependencies]
macroquad = "0.4.14"
macroquad_xp_barre_menu = { git = "https://github.com/bl3tt3r/macroquad-barre-menu" }
```

> 💡 Note : Le package n'est pas encore publié sur crates.io. Utilisez la dépendance Git en attendant.

---

## 🚀 Démarrage rapide

```rust
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
                .with_item(Checkbox::new("Sound", false, |v| println!("Sound: {}", v)))
        )
        .with_menu(
            Menu::new("Help")
                .with_item(Button::new("About", || println!("About")))
        );

    let settings = &Settings::default(); // Style XP par défaut

    loop {
        clear_background(WHITE);
        barre.draw(settings);
        next_frame().await;
    }
}
```

![Fenetre avec style windows XP et un contenue minimal](docs/start.png)


---

## 📂 Exemples

Le dépôt contient plusieurs exemples prêts à l'emploi :

### 1. Style par défaut (Windows XP)

`examples/default.rs`

```bash
cargo run --example default
```

![Capture de l'exemple 'default'](docs/default.png)

### 2. Démarrage rapide

`examples/start.rs`

```bash
cargo run --example start
```

![Capture de l'exemple 'start'](docs/start.png)

###  Thème personnalisé (sombre)

`examples/style.rs`

```bash
cargo run --example style
```

![Capture de l'exemple 'style'](docs/style.png)

---

## ✅ Compatibilité

- Macroquad **0.4.14+**
- Plateformes : **Windows, Linux, macOS, WASM, Android, iOS** (héritées de Macroquad).

---

## 🎨 Personnalisation

Le style par défaut utilise le thème Windows XP, mais vous pouvez créer votre propre apparence :

```rust
use macroquad::prelude::*;
use macroquad_xp_barre_menu::*;

#[macroquad::main("Custom Theme")]
async fn main() {
    let mut barre = Barre::default()
        .with_menu(Menu::new("Game").with_item(Button::new("New", || println!("New"))));

    // Exemple d'un Settings custom (type "sombre")
    let font_bytes: &[u8] = include_bytes!("style_roboto.ttf");
    let font = load_ttf_font_from_bytes(font_bytes).expect("font not found");

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
        next_frame().await;
    }
}
```

### Configuration des couleurs

| Propriété | Description | Valeur par défaut (XP) |
|-----------|-------------|------------------------|
| `barre_background_color` | Couleur de fond de la barre | `#ECE9D8` |
| `barre_border_color` | Bordure de la barre | `#0054E3` |
| `item_hover_color` | Surbrillance au survol | `#3399FF` |
| `menu_color` | Fond des menus déroulants | `#FFFFFF` |
| `text_color` | Couleur du texte | `#000000` |
| `text_hover_color` | Texte au survol | `#FFFFFF` |
| `menu_shadow_color` | Ombre des menus | `#808080` |


---

## 🧰 Utilisation / API rapide

### Ajouter des **menus** à la barre

```rust
let mut barre = Barre::default()
    .with_menu(Menu::new("Game"))
    .with_menu(Menu::new("Menu 1"));
    .with_menu(Menu::new("Menu 2"));
    .with_menu(Menu::new("Menu 3"));
    .with_menu(Menu::new("Menu 4"));
    .with_menu(Menu::new("Menu 5"));
```

![Capture avec plusieurs menus](docs/menus.png)

### Ajouter des **items** dans un menu

Chaîne `with_item(...)` pour empiler des items :

```rust
let menu = Menu::new("Game")
    .with_item(Button::new("New", || println!("New")))
    .with_item(Separator)
    .with_item(Radio::new(vec!["Beginner", "Intermediate", "Expert"], |v| println!("Value: {}", v)))
    .with_item(Checkbox::new("Sound", false, |enabled| println!("Sound: {}", enabled)));
```

![Capture d'un menu ouvert avec plusieurs items](docs/items.png)

### Dessiner la barre (dans la boucle de rendu)

```rust
let settings = &Settings::default();
loop {
    clear_background(WHITE);
    barre.draw(settings);
    next_frame().await;
}
```

---

## 📜 Licence

Ce projet est distribué sous **Apache License 2.0**. Vous pouvez utiliser, modifier, distribuer et vendre le logiciel, sous réserve d’indiquer les changements, conserver les avis d’attribution et respecter la licence.
Aucune garantie ni obligation de support n’est fournie.
Voir le fichier [`LICENSE`](./LICENSE) pour le texte complet.

---

<div align="center">
Si ce projet vous est utile, n'oubliez pas de lui donner une ⭐ !
Fait avec ❤️ et 🦀 par la communauté Rust
</div>
