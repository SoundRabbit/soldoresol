use super::*;

impl RoomModeless {
    pub fn message_style() -> Style {
        style! {
            ".channel-main-message-content *[data-cmd~='nb']" {
                "word-break": "keep-all";
                "white-space": "nowrap";
            }

            ".channel-main-message-content *[data-cmd~='left']" {
                "text-align": "left";
            }

            ".channel-main-message-content *[data-cmd~='right']" {
                "text-align": "right";
            }

            ".channel-main-message-content *[data-cmd~='center']" {
                "text-align": "center";
            }

            ".channel-main-message-content *[data-cmd~='gr']" {
                "display": "grid";
            }

            ".channel-main-message-content *[data-cmd~='box']" {
                "display": "block";
                "overflow-y": "scroll";
                "max-height": "10rem";
                "padding-left": ".35rem";
                "border-left": format!(".35rem solid {}", crate::libs::color::Pallet::gray(3));
            }

            ".channel-main-message-content *[data-cmd~='block']" {
                "display": "block";
            }

            ".channel-main-message-content *[data-cmd~='large']" {
                "font-size": "1.25em";
            }

            ".channel-main-message-content *[data-cmd~='huge']" {
                "font-size": "1.5em";
            }

            ".channel-main-message-content *[data-cmd~='red']" {
                "color": crate::libs::color::Pallet::red(7);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".channel-main-message-content *[data-cmd~='orange']" {
                "color": crate::libs::color::Pallet::orange(5);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".channel-main-message-content *[data-cmd~='yellow']" {
                "color": crate::libs::color::Pallet::yellow(8);
            }

            ".channel-main-message-content *[data-cmd~='green']" {
                "color": crate::libs::color::Pallet::green(7);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".channel-main-message-content *[data-cmd~='blue']" {
                "color": crate::libs::color::Pallet::blue(5);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".channel-main-message-content *[data-cmd~='purple']" {
                "color": crate::libs::color::Pallet::purple(5);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".channel-main-message-content *[data-cmd~='pink']" {
                "color": crate::libs::color::Pallet::pink(7);
                "background-color": crate::libs::color::Pallet::gray(0);
            }

            ".channel-main-message-content *[data-cmd~='bg-dark']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::gray(9);
            }

            ".channel-main-message-content *[data-cmd~='bg-red']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::red(7);
            }

            ".channel-main-message-content *[data-cmd~='bg-orange']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::orange(5);
            }

            ".channel-main-message-content *[data-cmd~='bg-yellow']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::yellow(8);
            }

            ".channel-main-message-content *[data-cmd~='bg-green']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::green(7);
            }

            ".channel-main-message-content *[data-cmd~='bg-blue']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::blue(5);
            }

            ".channel-main-message-content *[data-cmd~='bg-purple']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::purple(5);
            }

            ".channel-main-message-content *[data-cmd~='bg-pink']" {
                "color": crate::libs::color::Pallet::gray(0);
                "background-color": crate::libs::color::Pallet::pink(7);
            }
        }
    }
}
