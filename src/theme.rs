use std::sync::Arc;
use image::RgbaImage;
use include_dir::{include_dir, Dir, DirEntry};
use prism::Assets;

use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;

use ptsd::{ColorResources, IconResources, FontResources};

pub use ptsd::Color;
use prism::canvas;

use crate::classes::Language;

#[derive(Debug, Clone)]
pub struct Theme(ptsd::Theme, BrandResources);
impl Theme {
    pub fn from(assets: &Dir<'static>, color: Color) -> Self { 
        let (theme, is_dark) = ptsd::Theme::from(assets, color);
        Theme::new(assets, theme, is_dark, color) 
    }

    pub fn dark(assets: &Dir<'static>, color: Color) -> Self { Theme::new(assets, ptsd::Theme::dark(assets, color), true, color) }
    pub fn light(assets: &Dir<'static>, color: Color) -> Self { Theme::new(assets, ptsd::Theme::light(assets, color), false, color) }

    fn new(assets: &Dir<'static>, mut inner: ptsd::Theme, is_dark: bool, color: Color) -> Self {
        Button::map(&mut inner.colors, is_dark, color);
        inner.fonts.insert_font(ptsd::FontStyle::Heading, canvas::Font::from_bytes(assets.get_file("fonts/museo_bold.otf").unwrap().contents()).unwrap());
        inner.fonts.insert_font(ptsd::FontStyle::Label, canvas::Font::from_bytes(assets.get_file("fonts/museo_bold.otf").unwrap().contents()).unwrap());
        inner.fonts.insert_font(ptsd::FontStyle::Text, canvas::Font::from_bytes(assets.get_file("fonts/museo_regular.otf").unwrap().contents()).unwrap());
        Theme(inner, BrandResources::new(assets))
    }

    pub fn colors(&self) -> &ColorResources {&self.0.colors}
    pub fn icons(&self) -> &IconResources {&self.0.icons}
    pub fn fonts(&self) -> &FontResources {&self.0.fonts}
    pub fn brand(&self) -> &BrandResources {&self.1}

    pub fn colors_mut(&mut self) -> &mut ColorResources {&mut self.0.colors}
    pub fn icons_mut(&mut self) -> &mut IconResources {&mut self.0.icons}
    pub fn fonts_mut(&mut self) -> &mut FontResources {&mut self.0.fonts}
    pub fn brand_mut(&mut self) -> &mut BrandResources {&mut self.1}
}

impl Default for Theme {
    fn default() -> Theme {
        let assets = include_dir!("resources");
        let color = Color::from_hex("#00a2ff", 255);
        let inner = ptsd::Theme::dark(&assets, color);
        Theme::new(&assets, inner, true, color)
    }
}

#[derive(Clone, Debug)]
pub struct BrandResources {
    pub wordmark: Arc<RgbaImage>,
    pub logo: Arc<RgbaImage>,
    pub app_icon: Arc<RgbaImage>,
    pub error: Arc<RgbaImage>,
    pub flags: HashMap<String, Arc<RgbaImage>>
}

impl Default for BrandResources {
    fn default() -> Self {
        let dir = include_dir!("resources/brand");
        BrandResources {
            logo: Arc::new(Assets::load_svg(&Assets::load_file(&dir, "logo.svg").unwrap())),
            wordmark: Arc::new(Assets::load_svg(&Assets::load_file(&dir, "wordmark.svg").unwrap())),
            app_icon: Arc::new(Assets::load_svg(&Assets::load_file(&dir, "app_icon.svg").unwrap())),
            error: Arc::new(Assets::load_svg(&Assets::load_file(&dir, "error.svg").unwrap())),
            flags: HashMap::default(),
        }
    }
}

impl BrandResources {
    fn new(dir: &Dir<'static>) -> Self {
        let defaults = BrandResources::default();
        let dir = dir.entries().iter().find_map(|entry| {
            match entry {
                DirEntry::Dir(d) if d.path().file_name().and_then(|n| n.to_str()) == Some("brand") => {
                    Some(d.clone())
                }
                _ => None,
            }
        }).unwrap_or(include_dir!("resources"));

        let flags = dir.entries().iter().find_map(|entry| {
            match entry {
                DirEntry::Dir(d) if d.path().file_name().and_then(|n| n.to_str()) == Some("flags") => {
                    Some(d.clone())
                }
                _ => None,
            }
        });

        let flags: HashMap<String, Arc<RgbaImage>> = flags.map(|l| l.entries().iter().filter_map(|e| match e {
            DirEntry::File(f) => Some(f),
            _ => None,
        }).filter(|p| p.path().to_str().unwrap().ends_with(".svg")).collect::<Vec<_>>()).unwrap_or_default().iter().map(|p| {
            let name = p.path().to_str().unwrap().strip_prefix("brand/flags/").unwrap().strip_suffix(".svg").unwrap().replace(' ', "_");
            (name, Arc::new(Assets::load_svg(p.contents())))
        }).collect();

        println!("FLAGS {:?}", flags.keys());

        BrandResources {
            logo: Assets::load_file(&dir, "brand/logo.svg").map(|f| Arc::new(Assets::load_svg(&f))).unwrap_or(defaults.logo.clone()),
            wordmark: Assets::load_file(&dir, "brand/wordmark.svg").map(|f| Arc::new(Assets::load_svg(&f))).unwrap_or(defaults.wordmark.clone()),
            app_icon: Assets::load_file(&dir, "brand/app_icon.svg").map(|f| Arc::new(Assets::load_svg(&f))).unwrap_or(defaults.app_icon.clone()),
            error: Assets::load_file(&dir, "brand/error.svg").map(|f| Arc::new(Assets::load_svg(&f))).unwrap_or(defaults.error.clone()),
            flags,
        }
    }

    pub fn language(&self, language: Language) -> Arc<RgbaImage> {
        self.flags.get(&match language {
            Language::English => "gb",
            Language::Dutch => "nl",
        }.to_string()).expect("Could not find image for desired language").clone()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ButtonColors {
    pub primary: ButtonColorSet,
    pub secondary: ButtonColorSet,
    pub ghost: ButtonColorSet,
}

#[derive(Copy, Clone, Debug)]
pub struct ButtonColorSet {
    pub default: ButtonColorScheme,
    pub disabled: ButtonColorScheme,
    pub hover: ButtonColorScheme,
    pub pressed: ButtonColorScheme,
}

#[derive(Copy, Clone, Debug)]
pub struct ButtonColorScheme {
    pub background: Color,
    pub label: Color,
    pub outline: Color,
    pub shadow: Color,
}

impl ButtonColorScheme {
    fn from(resources: &ColorResources, variant: Variant, state: State) -> Self {
        ButtonColorScheme {
            background: resources.get(Button(variant, state, Slot::Background)),
            label: resources.get(Button(variant, state, Slot::Label)),
            outline: resources.get(Button(variant, state, Slot::Outline)),
            shadow: resources.get(Button(variant, state, Slot::Shadow))
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant { Primary, Secondary, Ghost }
impl Display for Variant {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {write!(f, "Variant::{}", match self {
    Variant::Primary => "Primary",
    Variant::Secondary => "Secondary",
    Variant::Ghost => "Ghost",
})}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State { Default, Hover, Pressed, Disabled }
impl Display for State {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {write!(f, "State::{}", match self {
    State::Default => "Default",
    State::Hover => "Hover",
    State::Pressed => "Pressed",
    State::Disabled => "Disabled",
})}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Slot { Background, Label, Outline, Shadow }
impl Display for Slot {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {write!(f, "Slot::{}", match self {
    Slot::Background => "Background",
    Slot::Label => "Label",
    Slot::Outline => "Outline",
    Slot::Shadow => "Shadow"
})}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Button(pub Variant, pub State, pub Slot);
impl Display for Button {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> 
    fmt::Result {write!(f, "Button({}, {}, {})", self.0, self.1, self.2)}}

impl Button {
    pub fn get(resources: &ColorResources, variant: Variant) -> ButtonColorSet {
        ButtonColorSet {
            default: ButtonColorScheme::from(resources, variant, State::Default),
            disabled: ButtonColorScheme::from(resources, variant, State::Disabled),
            hover: ButtonColorScheme::from(resources, variant, State::Hover),
            pressed: ButtonColorScheme::from(resources, variant, State::Pressed),
        }
    }

    pub fn map(resources: &mut ColorResources, is_dark: bool, brand: Color) {
        use Slot::*;
        use State::*;
        use Variant::*;

        resources.insert(Button(Primary, Default, Background), brand);
        resources.insert(Button(Primary, Default, Label), brand.contrasted());
        resources.insert(Button(Primary, Default, Outline), Color::TRANSPARENT);
        resources.insert(Button(Primary, Default, Shadow), brand.darken(0.7));
        resources.insert(Button(Primary, Hover, Background), brand.darken(0.75));
        resources.insert(Button(Primary, Hover, Label), brand.contrasted());
        resources.insert(Button(Primary, Hover, Outline), Color::TRANSPARENT);
        resources.insert(Button(Primary, Hover, Shadow), brand.darken(0.6));
        resources.insert(Button(Primary, Pressed, Background), brand.darken(0.7));
        resources.insert(Button(Primary, Pressed, Label), brand.contrasted());
        resources.insert(Button(Primary, Pressed, Outline), Color::TRANSPARENT);
        resources.insert(Button(Primary, Pressed, Shadow), brand.darken(1.0));
        resources.insert(Button(Primary, Disabled, Background), Color::from_hex("#443f3f", 255));
        resources.insert(Button(Primary, Disabled, Label), Color::BLACK);
        resources.insert(Button(Primary, Disabled, Outline), Color::TRANSPARENT);
        resources.insert(Button(Primary, Disabled, Shadow), Color::BLACK);
    
        match is_dark {
            false => {
                resources.insert(Button(Secondary, Default, Background), Color::WHITE);
                resources.insert(Button(Secondary, Default, Label), brand);
                resources.insert(Button(Secondary, Default, Outline), Color::TRANSPARENT);
                resources.insert(Button(Secondary, Default, Shadow), Color::from_hex("#d5caca", 255));
                resources.insert(Button(Secondary, Hover, Background), Color::from_hex("#DDDDDD", 255));
                resources.insert(Button(Secondary, Hover, Label), Color::BLACK);
                resources.insert(Button(Secondary, Hover, Outline), Color::from_hex("#585250", 255));
                resources.insert(Button(Secondary, Hover, Shadow), Color::from_hex("#443f3f", 255));
                resources.insert(Button(Secondary, Pressed, Background), Color::from_hex("#DDDDDD", 255));
                resources.insert(Button(Secondary, Pressed, Label), Color::BLACK);
                resources.insert(Button(Secondary, Pressed, Outline), Color::BLACK);
                resources.insert(Button(Secondary, Pressed, Shadow), Color::from_hex("#443f3f", 255));
                resources.insert(Button(Secondary, Disabled, Background), Color::from_hex("#443f3f", 255));
                resources.insert(Button(Secondary, Disabled, Label), Color::BLACK);
                resources.insert(Button(Secondary, Disabled, Outline), Color::from_hex("#585250", 255));
                resources.insert(Button(Secondary, Disabled, Shadow), Color::BLACK);

                resources.insert(Button(Ghost, Default, Background), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Default, Label), Color::BLACK);
                resources.insert(Button(Ghost, Default, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Default, Shadow), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Hover, Background), Color::from_hex("#DDDDDD", 255));
                resources.insert(Button(Ghost, Hover, Label), Color::BLACK);
                resources.insert(Button(Ghost, Hover, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Hover, Shadow), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Pressed, Background), Color::from_hex("#DDDDDD", 255));
                resources.insert(Button(Ghost, Pressed, Label), Color::BLACK);
                resources.insert(Button(Ghost, Pressed, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Pressed, Shadow), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Disabled, Background), Color::from_hex("#443f3f", 255));
                resources.insert(Button(Ghost, Disabled, Label), Color::BLACK);
                resources.insert(Button(Ghost, Disabled, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Disabled, Shadow), Color::TRANSPARENT);
            },
            true => {
                resources.insert(Button(Secondary, Default, Background), Color::WHITE);
                resources.insert(Button(Secondary, Default, Label), brand);
                resources.insert(Button(Secondary, Default, Outline), Color::TRANSPARENT);
                resources.insert(Button(Secondary, Default, Shadow), Color::from_hex("#d5caca", 255));

                resources.insert(Button(Secondary, Hover, Background), Color::from_hex("#DDDDDD", 255));
                resources.insert(Button(Secondary, Hover, Label), brand);
                resources.insert(Button(Secondary, Hover, Outline), Color::TRANSPARENT);
                resources.insert(Button(Secondary, Hover, Shadow), Color::from_hex("#cac2bf", 255));

                resources.insert(Button(Secondary, Pressed, Background), Color::from_hex("#DDDDDD", 255));
                resources.insert(Button(Secondary, Pressed, Label), brand);
                resources.insert(Button(Secondary, Pressed, Outline), Color::TRANSPARENT);
                resources.insert(Button(Secondary, Pressed, Shadow), Color::from_hex("#cac2bf", 255));

                resources.insert(Button(Secondary, Disabled, Background), Color::from_hex("#443f3f", 255));
                resources.insert(Button(Secondary, Disabled, Label), Color::BLACK);
                resources.insert(Button(Secondary, Disabled, Outline), Color::from_hex("#585250", 255));
                resources.insert(Button(Secondary, Disabled, Shadow), Color::TRANSPARENT);

                resources.insert(Button(Ghost, Default, Background), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Default, Label), Color::WHITE);
                resources.insert(Button(Ghost, Default, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Default, Shadow), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Hover, Background), Color::from_hex("#262322", 255));
                resources.insert(Button(Ghost, Hover, Label), Color::WHITE);
                resources.insert(Button(Ghost, Hover, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Hover, Shadow), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Pressed, Background), Color::from_hex("#262322", 255));
                resources.insert(Button(Ghost, Pressed, Label), Color::WHITE);
                resources.insert(Button(Ghost, Pressed, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Pressed, Shadow), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Disabled, Background), Color::from_hex("#443f3f", 255));
                resources.insert(Button(Ghost, Disabled, Label), Color::BLACK);
                resources.insert(Button(Ghost, Disabled, Outline), Color::TRANSPARENT);
                resources.insert(Button(Ghost, Disabled, Shadow), Color::TRANSPARENT);
            }
        }
    }
}
