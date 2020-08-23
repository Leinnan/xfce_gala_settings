
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Settings {
    pub dynamic_workspaces: bool,
    pub edge_tiling: bool,
    pub animations: bool
}

impl Settings {
    pub fn load() -> Settings {
        let dynamic_workspaces = dconf_rs::get_boolean("/org/pantheon/desktop/gala/behavior/dynamic-workspaces").unwrap_or(false);
        let edge_tiling = dconf_rs::get_boolean("/org/gnome/mutter/edge-tiling").unwrap_or(false);
        let animations = dconf_rs::get_boolean("/org/pantheon/desktop/gala/animations/enable-animations").unwrap_or(false);
        Settings {
            dynamic_workspaces: dynamic_workspaces,
            edge_tiling: edge_tiling,
            animations: animations
        }
    }

    pub fn save(self) {
        dconf_rs::set_boolean("/org/pantheon/desktop/gala/behavior/dynamic-workspaces",self.dynamic_workspaces);
        dconf_rs::set_boolean("/org/gnome/mutter/edge-tiling",self.edge_tiling);
        dconf_rs::set_boolean("/org/pantheon/desktop/gala/animations/enable-animations",self.animations);
    }
}