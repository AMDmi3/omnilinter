// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod json;
pub mod stdout;

use crate::location::MatchLocation;

pub trait Reporter {
    fn report(&mut self, location: &MatchLocation, message: &str);
    fn flush(&self);
    fn has_matches(&self) -> bool;
}
