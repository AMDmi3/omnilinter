// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

pub struct Enumerator {
    next_id: usize,
    assigned_ids: HashMap<String, usize>,
}

impl Enumerator {
    pub fn new() -> Enumerator {
        Enumerator {
            next_id: 0,
            assigned_ids: HashMap::new(),
        }
    }

    pub fn get_id(&mut self, value: &str) -> usize {
        *self.assigned_ids.entry(value.into()).or_insert_with(|| {
            let id = self.next_id;
            self.next_id += 1;
            id
        })
    }

    pub fn get_count(&self) -> usize {
        self.next_id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn enumerator() {
        let mut e = Enumerator::new();
        assert_eq!(e.get_id("foo"), 0);
        assert_eq!(e.get_id("bar"), 1);
        assert_eq!(e.get_id("baz"), 2);
        assert_eq!(e.get_id("foo"), 0);
        assert_eq!(e.get_id("bar"), 1);
        assert_eq!(e.get_id("baz"), 2);
    }
}
