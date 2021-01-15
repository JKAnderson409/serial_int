#![cfg(test)]

use crate::{serial, Serial, SerialGenerator};

fn creates_unique_values_until_end<T: Serial + Ord>(start: T) {
    let mut gen = SerialGenerator::with_init_value(start);
    let mut used = Vec::new();

    while gen.has_remaining_increments() {
        let serial = gen.generate();

        assert!(!used.contains(&serial));

        used.push(serial);
    }
}

fn recreates_end_value<T: Serial + std::fmt::Debug>(init: T, end: T) {
    let mut gen = SerialGenerator::with_init_value(init.clone());

    assert_eq!(init, gen.generate());
    assert_eq!(end, gen.generate());
    assert_eq!(end, gen.generate());
    assert_eq!(end, gen.generate());
}

#[test]
fn u8_creates_unique_values_until_end() {
    creates_unique_values_until_end(u8::MAX - 5);
}

#[test]
fn u16_creates_unique_values_until_end() {
    creates_unique_values_until_end(u16::MAX - 5);
}

#[test]
fn u32_creates_unique_values_until_end() {
    creates_unique_values_until_end(u32::MAX - 5);
}

#[test]
fn u64_creates_unique_values_until_end() {
    creates_unique_values_until_end(u64::MAX - 5);
}

#[test]
fn u128_creates_unique_values_until_end() {
    creates_unique_values_until_end(u128::MAX - 5);
}

#[test]
fn usize_creates_unique_values_until_end() {
    creates_unique_values_until_end(usize::MAX - 5);
}


#[test]
fn u8_recreates_end_value() {
    recreates_end_value(u8::MAX - 1, u8::MAX);
}

#[test]
fn u16_recreates_end_value() {
    recreates_end_value(u16::MAX - 1, u16::MAX);
}

#[test]
fn u32_recreates_end_value() {
    recreates_end_value(u32::MAX - 1, u32::MAX);
}

#[test]
fn u64_recreates_end_value() {
    recreates_end_value(u64::MAX - 1, u64::MAX);
}

#[test]
fn u128_recreates_end_value() {
    recreates_end_value(u128::MAX - 1, u128::MAX);
}

#[test]
fn usize_recreates_end_value() {
    recreates_end_value(usize::MAX - 1, usize::MAX);
}
