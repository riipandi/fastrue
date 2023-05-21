// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

pub mod admin;
pub mod error;
pub mod health;
pub mod logout;
pub mod recover;
pub mod root;
pub mod settings;
pub mod signup;
pub mod token;
pub mod user;
pub mod verify;

pub use self::logout::*;
pub use self::recover::*;
pub use self::signup::*;
pub use self::token::*;
pub use self::user::*;
pub use self::verify::*;
