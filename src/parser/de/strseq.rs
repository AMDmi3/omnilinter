// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::de::Error;
use serde::Deserializer;

/// serde visitor for flexible string sequences
///
/// Visitor for deserialization of string sequences, represented either
/// as a single string with whitespace separators (`'foo bar baz'`), or
/// a sequence of strings (`['foo', 'bar', 'baz']`).
struct StringSequenceVisitor;

impl<'de> serde::de::Visitor<'de> for StringSequenceVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string with whitespace separated values or sequence of strings")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let res: Vec<_> = s.split_whitespace().map(|s| s.to_string()).collect();
        if res.is_empty() {
            Err(E::custom("empty string not allowed"))
        } else {
            Ok(res)
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res = Vec::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(value) = seq.next_element::<String>()? {
            res.push(value);
        }

        if res.is_empty() {
            Err(A::Error::custom("empty sequence not allowed"))
        } else {
            Ok(res)
        }
    }
}

pub fn deserialize_string_sequence<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(deserializer.deserialize_any(StringSequenceVisitor)?)
}

pub fn deserialize_optional_string_sequence<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserialize_string_sequence(deserializer)?))
}
