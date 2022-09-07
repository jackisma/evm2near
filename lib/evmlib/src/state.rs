// This is free and unencumbered software released into the public domain.

use ethnum::u256;
use std::{collections::HashMap, mem};

pub(crate) type Word = u256;

pub(crate) const ZERO: Word = u256::ZERO;
pub(crate) const ONE: Word = u256::ONE;

pub(crate) const MAX_STACK_DEPTH: usize = 16;
pub(crate) const WORD_SIZE: usize = mem::size_of::<Word>();

pub(crate) struct Stack {
    pub(crate) depth: usize,
    pub(crate) slots: [Word; MAX_STACK_DEPTH],
}

pub(crate) struct Memory {
    pub(crate) bytes: Vec<u8>,
}

pub(crate) struct Storage {
    pub(crate) entries: Option<HashMap<Word, Word>>,
}

pub(crate) struct Machine {
    pub(crate) gas_used: u64,
    pub(crate) gas_limit: u64,
    pub(crate) gas_price: u64,
    pub(crate) stack: Stack,
    pub(crate) memory: Memory,
    pub(crate) storage: Storage,
    pub(crate) call_value: Word,
    pub(crate) call_data: Vec<u8>,
    pub(crate) code: Vec<u8>,
}

impl Stack {
    #[allow(dead_code)]
    pub fn new() -> Stack {
        Stack {
            depth: 0,
            slots: [ZERO; MAX_STACK_DEPTH],
        }
    }

    pub fn clear(&mut self) {
        self.depth = 0;
    }

    pub fn push(&mut self, word: Word) {
        if self.depth == MAX_STACK_DEPTH {
            panic!("stack overflow");
        }
        self.slots[self.depth] = word;
        self.depth += 1;
    }

    pub fn pop(&mut self) -> Word {
        if self.depth == 0 {
            panic!("stack underflow");
        }
        self.depth -= 1;
        let result = self.slots[self.depth];
        self.slots[self.depth] = ZERO;
        result
    }

    pub fn pop2(&mut self) -> (Word, Word) {
        (self.pop(), self.pop())
    }

    pub fn pop3(&mut self) -> (Word, Word, Word) {
        (self.pop(), self.pop(), self.pop())
    }

    pub fn pop4(&mut self) -> (Word, Word, Word, Word) {
        (self.pop(), self.pop(), self.pop(), self.pop())
    }

    pub fn peek(&self) -> Word {
        if self.depth == 0 {
            panic!("stack underflow");
        }
        self.slots[self.depth - 1]
    }

    pub fn peek_n(&self, n: usize) -> Word {
        if n >= self.depth {
            panic!("stack underflow");
        }
        self.slots[self.depth - 1 - n]
    }

    pub fn swap(&mut self, n: usize) {
        if n >= self.depth {
            panic!("stack underflow");
        }
        let tos = self.depth - 1;
        self.slots.swap(tos, tos - n)
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        print!("depth={} slots=[", self.depth);
        for element in self.slots.iter() {
            print!("{},", element);
        }
        println!("]")
    }
}

impl Memory {
    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.bytes.clear()
    }

    pub fn store_byte(&mut self, offset: usize, value: u8) {
        let end_offset = offset + 1;
        self.resize(end_offset);
        self.bytes[offset] = value;
    }

    pub fn store_word(&mut self, offset: usize, value: Word) {
        let end_offset = offset + WORD_SIZE;
        self.resize(end_offset);
        self.bytes[offset..end_offset].copy_from_slice(&value.to_le_bytes());
    }

    pub fn store_slice(&mut self, offset: usize, data: &[u8]) {
        // TODO: checked arithmetic
        let end_offset = offset + data.len();
        self.resize(end_offset);
        self.bytes[offset..end_offset].copy_from_slice(data);
    }

    pub fn store_zeros(&mut self, offset: usize, len: usize) {
        let end_offset = offset + len;
        self.resize(end_offset);
        self.bytes[offset..end_offset].fill(0);
    }

    fn resize(&mut self, end_offset: usize) {
        // resize in increments of the `WORD_SIZE`
        let offset_remainder = end_offset % WORD_SIZE;
        let end_offset = if offset_remainder == 0 {
            end_offset
        } else {
            end_offset + (WORD_SIZE - offset_remainder)
        };

        if end_offset > self.size() {
            self.bytes.resize(end_offset, 0);
        }
    }

    pub fn load_word(&self, offset: usize) -> Word {
        fn to_array(input: &[u8]) -> [u8; WORD_SIZE] {
            input.try_into().expect("slice with incorrect length")
        }
        if offset >= self.size() {
            return ZERO;
        }
        let end_offset = offset + mem::size_of::<Word>();
        Word::from_le_bytes(to_array(&self.bytes[offset..end_offset]))
    }

    pub fn slice(&self, offset: usize, size: usize) -> &[u8] {
        &self.bytes[offset..offset + size]
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!("{:?}", self.bytes)
    }
}

impl Storage {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        if let Some(m) = self.entries.as_mut() {
            m.clear()
        }
    }

    pub fn store_word(&mut self, key: Word, val: Word) {
        if self.entries.is_none() {
            self.entries = Some(HashMap::new());
        }
        self.entries.as_mut().map(|m| m.insert(key, val));
    }

    pub fn load_word(&self, key: Word) -> Word {
        return self
            .entries
            .as_ref()
            .map_or(ZERO, |m| *m.get(&key).unwrap_or(&ZERO));
    }
}

impl Machine {
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.stack.clear();
        self.memory.clear();
        self.storage.clear();
        self.call_data.clear();
    }
}
