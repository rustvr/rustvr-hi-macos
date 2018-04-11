//  Copyright 2018 RustVR
//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate rustvr_core;

pub struct PluginHiMacos {
}

impl rustvr_core::Plugin for PluginHiMacos {
}

impl rustvr_core::PluginHi for PluginHiMacos {
}

pub fn make_plugin() -> PluginHiMacos {
    PluginHiMacos {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
