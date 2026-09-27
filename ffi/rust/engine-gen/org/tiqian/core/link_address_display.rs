use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct LinkAddressDisplay;

impl LinkAddressDisplay {
    pub fn link_address_display_displays_address(display: &str, target: &str) -> bool {
        if u_string::unit_count(&(display)) == 0 || u_string::unit_count(&(target)) == 0 {
            return false;
        }
        if display == target {
            return true;
        }
        return target == format!("{}{}",
            "https://",
            display
        ) || target == format!("{}{}",
            "http://",
            display
        ) || target == format!("{}{}",
            "mailto:",
            display
        );
    }
}
