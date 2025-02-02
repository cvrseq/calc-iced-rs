#[derive(Clone, Copy, Debug)]
pub struct CustomTheme { 
    background: iced::Color, 
    text: iced::Color, 
    primary: iced::Color, 
    success: iced::Color, 
    danger: iced::Color,
}


impl CustomTheme { 
    pub fn new() -> Self { 
        Self { 
            background: iced::Color::from_rgba8(30, 30, 30, 1.0),
            text: iced::Color::from_rgba8(221, 221, 221, 1.0),
            primary: iced::Color::from_rgba8(10, 132, 255, 1.0),
            success: iced::Color::from_rgba8(48, 209, 81, 1.0),
            danger: iced::Color::from_rgba8(255, 69, 58, 1.0),
        }
    }
}