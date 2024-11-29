mod pareto_front_builder;
pub use pareto_front_builder::{ParetoFrontBuilder, ParetoFrontId, ParetoValue};

struct Entry<T> {
    item: T,
    depth: u8,
    parent_index: usize,
}

pub struct Backtracking<T: Copy> {
    entries: Vec<Entry<T>>,
}

impl<T: Copy> Backtracking<T> {
    pub const SENTINEL: usize = usize::MAX;

    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get_depth(&self, index: usize) -> u8 {
        if index == Self::SENTINEL {
            0
        } else {
            self.entries[index].depth
        }
    }

    pub fn get_items(&self, mut index: usize) -> impl Iterator<Item = T> {
        let mut items = Vec::new();
        while index != Self::SENTINEL {
            items.push(self.entries[index].item);
            index = self.entries[index].parent_index;
        }
        items.into_iter().rev()
    }

    pub fn push(&mut self, item: T, parent_index: usize) -> usize {
        let depth = if parent_index == Self::SENTINEL {
            1
        } else {
            self.entries[parent_index].depth + 1
        };
        self.entries.push(Entry {
            item,
            depth,
            parent_index,
        });
        self.entries.len() - 1
    }
}
