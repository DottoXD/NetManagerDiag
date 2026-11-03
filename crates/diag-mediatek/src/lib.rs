// SPDX-License-Identifier: GPL-3.0-or-later

#![forbid(unsafe_code)]

use diag_model::Baseband;

pub fn probe() -> Result<Box<dyn Baseband>, String> {
    Err("MediaTek support is not implemented.".into())
}

// TODO: Need a MediaTek device with root access to reverse engineer and implement.