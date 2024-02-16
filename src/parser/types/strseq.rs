// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// serde visitor for string sequences which may be represented as single string
///
/// Visitor for deserialization of string sequences, represented either as a single
/// string with whitespace separators (`'foo bar baz'`), or a sequence of strings
/// (`['foo', 'bar', 'baz']`).
struct WhitespaceSeparatedStringOrStringSequenceVisitor;

impl<'de> serde::de::Visitor<'de> for WhitespaceSeparatedStringOrStringSequenceVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string with whitespace separated values or sequence of strings")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(s.split_whitespace().map(|s| s.to_string()).collect())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(value) = seq.next_element::<String>()? {
            res.push(value);
        }
        Ok(res)
    }
}

#[derive(PartialEq, Default, Debug)]
pub struct StringSequence(Vec<String>);

impl IntoIterator for StringSequence {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'de> Deserialize<'de> for StringSequence {
    fn deserialize<D>(deserializer: D) -> Result<StringSequence, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(StringSequence {
            0: deserializer.deserialize_any(WhitespaceSeparatedStringOrStringSequenceVisitor)?,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct NonEmptyStringSequence(Vec<String>);

impl IntoIterator for NonEmptyStringSequence {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'de> Deserialize<'de> for NonEmptyStringSequence {
    fn deserialize<D>(deserializer: D) -> Result<NonEmptyStringSequence, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values =
            deserializer.deserialize_any(WhitespaceSeparatedStringOrStringSequenceVisitor)?;
        if values.is_empty() {
            Err(D::Error::custom("empty sequence not allowed"))
        } else {
            Ok(NonEmptyStringSequence { 0: values })
        }
    }
}
