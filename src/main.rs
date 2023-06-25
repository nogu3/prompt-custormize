const RESET: &str = r"%{\e[0m%}";
const HALF_CIRCLE_LEFT: &str = r"\ue0b6";
const HALF_CIRCLE_RIGHT: &str = r"\ue0b4";
const USER_NAME: &str = "%n ";
const TRIANGLE_LEFT: &str = r"\ue0b0";

mod color;

fn main() {
    let mut prompt: String = String::from("");

    prompt.push_str(&create_prompt(
        HALF_CIRCLE_LEFT,
        Some(color::DEEP_DARK_GRAY),
        None,
    ));
    prompt.push_str(&create_prompt(
        USER_NAME,
        Some(color::DARK_GRAY),
        Some(color::DEEP_DARK_GRAY),
    ));
    prompt.push_str(&create_prompt(
        TRIANGLE_LEFT,
        Some(color::DEEP_DARK_GRAY),
        Some(color::GRAY),
    ));
    prompt.push_str(&create_prompt(
        " ~ ",
        Some(color::DARK_GRAY),
        Some(color::GRAY),
    ));
    prompt.push_str(&create_prompt(
        TRIANGLE_LEFT,
        Some(color::GRAY),
        Some(color::LIGHT_GRAY),
    ));
    prompt.push_str(&create_prompt(
        TRIANGLE_LEFT,
        Some(color::LIGHT_GRAY),
        Some(color::LIGHT_BLUE),
    ));
    prompt.push_str(&create_prompt(
        TRIANGLE_LEFT,
        Some(color::LIGHT_BLUE),
        Some(color::GREEN_BLUE),
    ));
    prompt.push_str(&create_prompt(
        TRIANGLE_LEFT,
        Some(color::GREEN_BLUE),
        Some(color::BLUE),
    ));
    prompt.push_str(&create_prompt(
        " %T ",
        Some(color::SUPER_LIGHT_BLUE),
        Some(color::BLUE),
    ));
    prompt.push_str(&create_prompt(
        HALF_CIRCLE_RIGHT,
        Some(color::BLUE),
        None,
    ));
    println!("{}", prompt);
}

fn create_prompt(
    content: &str,
    text_color: Option<color::Color>,
    back_color: Option<color::Color>,
) -> String {
    let mut result: String = String::from("");

    match text_color {
        Some(value) => result.push_str(&create_text_color_prompt(value)),
        None => {}
    }

    match back_color {
        Some(value) => result.push_str(&create_back_color_prompt(value)),
        None => {}
    }

    result.push_str(content);
    result.push_str(RESET);

    result.to_string()
}

fn create_text_color_prompt(color: color::Color) -> String {
    return format!(r"%{{\e[38;2;{}m%}}", color.to_string()).to_string();
}

fn create_back_color_prompt(color: color::Color) -> String {
    return format!(r"%{{\e[48;2;{}m%}}", color.to_string()).to_string();
}
