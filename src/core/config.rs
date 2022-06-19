//! User facing configuration of the penrose [WindowManager][crate::core::manager::WindowManager].
use crate::{
    core::layout::{side_stack, Layout, LayoutConf},
    draw::{Color, DrawError},
};

use std::convert::TryInto;

__with_builder_and_getters! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Clone, Debug, PartialEq)]
    
    Config;
    
    #[derive(Debug)]
    ConfigBuilder;

    VecImplInto workspaces: String; => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    /// the window classes that will always be considered floating
    VecImplInto floating_classes: String; => vec!["dmenu", "dunst"];

    Concrete layouts: Vec<Layout>; =>
        vec![
            Layout::new("[side]", LayoutConf::default(), side_stack, 1, 0.6),
            Layout::floating("[----]"),
        ];

    ImplTry DrawError; focused_border: Color; => "#cc241d";

    ImplTry DrawError; unfocused_border: Color; => "#3c3836";

    Concrete border_px: u32; => 2;

    Concrete gap_px: u32; => 5;

    Concrete main_ratio_step: f32; => 0.05;

    Concrete show_bar: bool; => true;

    Concrete top_bar: bool; => true;

    Concrete bar_height: u32; => 18;
}

impl Config {
    /// Create a range from 1 -> n_workspaces for use in keybindings
    pub fn ws_range(&self) -> std::ops::Range<usize> {
        1..(self.workspaces.len() + 1)
    }
}

impl ConfigBuilder {
    fn validate(&self) -> std::result::Result<(), String> {
        if self.inner.workspaces.is_empty() {
            return Err("Must supply at least one workspace name".into());
        }

        if self.inner.layouts.is_empty() {
            return Err("Must supply at least one layout function".into());
        }

        if !(0.0..=1.0).contains(&self.inner.main_ratio_step) {
            return Err("main_ratio_step must be in the range 0.0 -> 1.0".into());
        }

        Ok(())
    }
}
