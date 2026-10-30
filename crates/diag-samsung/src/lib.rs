// SPDX-License-Identifier: GPL-3.0-or-later

use diag_model::Baseband;

pub fn probe() -> Result<Box<dyn Baseband>, String> {
    Err("Samsung Shannon support is not implemented.".into())
}

// TODO: Need a Samsung Shannon device with root access to reverse engineer and implement.