use xdg::BaseDirectories;

pub fn get_xdg_dirs() -> xdg::BaseDirectories {
    BaseDirectories::with_prefix("mist_bar")
} 
