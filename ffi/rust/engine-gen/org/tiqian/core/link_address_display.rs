use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct LinkAddressDisplay;

impl LinkAddressDisplay {
    pub fn link_address_display_displays_address(display: &UStr, target: &UStr) -> bool {
        if u_string::unit_count(&(display)) == 0 || u_string::unit_count(&(target)) == 0 {
            return false;
        }
        if display == target {
            return true;
        }
        return target == { let mut __s = UString::new(); __s += &(UString::from("https://")); __s += display; __s } || target == { let mut __s = UString::new(); __s += &(UString::from("http://")); __s += display; __s } || target == { let mut __s = UString::new(); __s += &(UString::from("mailto:")); __s += display; __s };
    }
}
