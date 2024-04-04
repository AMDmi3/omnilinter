// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ruleset::enumerator::Enumerator;

#[derive(Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct Regex {
    regex: regex::Regex,
    unique_id: usize,
}

impl Regex {
    pub fn new(re: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: regex::Regex::new(re)?,
            unique_id: usize::MAX,
        })
    }

    pub fn is_match(&self, haystack: &str) -> bool {
        self.regex.is_match(haystack)
    }

    pub fn as_str(&self) -> &str {
        self.regex.as_str()
    }

    pub fn enumerate_with(&mut self, enumerator: &mut Enumerator) {
        self.unique_id = enumerator.get_id(self.regex.as_str());
    }

    #[cfg_attr(not(feature = "matching-cache"), allow(dead_code))]
    pub fn get_unique_id(&self) -> usize {
        debug_assert!(self.unique_id != usize::MAX, "Regex is not enumerated");
        self.unique_id
    }
}
