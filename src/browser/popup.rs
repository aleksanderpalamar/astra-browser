use webkit6::NavigationType;

const MIN_WIDTH: i32 = 320;
const MIN_HEIGHT: i32 = 240;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opening {
    Link,
    Script,
}

impl Opening {
    pub fn from_navigation(navigation: NavigationType) -> Self {
        match navigation {
            NavigationType::LinkClicked => Self::Link,
            _ => Self::Script,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Geometry {
    fn is_default_for(self, (width, height): (i32, i32)) -> bool {
        (self.x, self.y, self.width, self.height) == (0, 0, width, height)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Presentation {
    Tab,
    Window { width: i32, height: i32 },
}

pub fn presentation(
    opening: Opening,
    requested: Geometry,
    opener_default_size: (i32, i32),
) -> Presentation {
    if opening == Opening::Link || requested.is_default_for(opener_default_size) {
        return Presentation::Tab;
    }
    Presentation::Window {
        width: requested.width.max(MIN_WIDTH),
        height: requested.height.max(MIN_HEIGHT),
    }
}

#[cfg(test)]
mod tests {
    use webkit6::NavigationType;

    use super::{Geometry, Opening, Presentation, presentation};

    const WINDOW: (i32, i32) = (1280, 800);

    fn geometry(x: i32, y: i32, width: i32, height: i32) -> Geometry {
        Geometry {
            x,
            y,
            width,
            height,
        }
    }

    #[test]
    fn only_link_clicks_count_as_links() {
        assert_eq!(
            Opening::from_navigation(NavigationType::LinkClicked),
            Opening::Link
        );
        assert_eq!(
            Opening::from_navigation(NavigationType::Other),
            Opening::Script
        );
    }

    #[test]
    fn links_always_open_in_a_tab() {
        assert_eq!(
            presentation(Opening::Link, geometry(0, 0, 100, 100), WINDOW),
            Presentation::Tab
        );
    }

    #[test]
    fn scripts_without_size_open_in_a_tab() {
        assert_eq!(
            presentation(Opening::Script, geometry(0, 0, 1280, 800), WINDOW),
            Presentation::Tab
        );
    }

    #[test]
    fn scripts_asking_for_a_size_open_a_popup_window() {
        assert_eq!(
            presentation(Opening::Script, geometry(0, 0, 480, 640), WINDOW),
            Presentation::Window {
                width: 480,
                height: 640
            }
        );
        assert_eq!(
            presentation(Opening::Script, geometry(0, 0, 480, 800), WINDOW),
            Presentation::Window {
                width: 480,
                height: 800
            }
        );
    }

    #[test]
    fn scripts_asking_for_a_position_open_a_popup_window() {
        assert_eq!(
            presentation(Opening::Script, geometry(100, 100, 1280, 800), WINDOW),
            Presentation::Window {
                width: 1280,
                height: 800
            }
        );
    }

    #[test]
    fn tiny_popups_get_a_usable_size() {
        assert_eq!(
            presentation(Opening::Script, geometry(0, 0, 10, 10), WINDOW),
            Presentation::Window {
                width: 320,
                height: 240
            }
        );
    }
}
