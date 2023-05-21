// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use salvo::prelude::StatusCode;

pub fn get_status_code(status_code: StatusCode) -> i16 {
    status_code.as_u16() as i16
}
