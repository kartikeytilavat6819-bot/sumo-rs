//! Port of foreign/RoutingKit/include/routingkit/id_set_queue.h (IDSetMinQueue).
//! An implicit binary tree of bits over the ids; pop returns the smallest id.

use super::constants::INVALID_ID;
use super::min_max::min_to;

const ROOT: u32 = 1;

fn parent(x: u32) -> u32 {
    x >> 1
}

fn other_child(x: u32) -> u32 {
    x ^ 1
}

fn is_smaller_child(x: u32) -> bool {
    (x & 1) == 0
}

fn smaller_child(x: u32) -> u32 {
    x << 1
}

fn smallest_two_power_no_smaller_than(x: u32) -> u32 {
    let mut y = 1u32;
    while y < x {
        y <<= 1;
    }
    y
}

#[derive(Clone, Debug, Default)]
pub struct IDSetMinQueue {
    max_number_of_ids: u32,
    min_id: u32,
    offset: u32,
    data: Vec<bool>,
}

impl IDSetMinQueue {
    /// IDSetMinQueue(n): the queue may contain ids 0..n-1
    pub fn new(n: u32) -> Self {
        let offset = smallest_two_power_no_smaller_than(n);
        IDSetMinQueue {
            max_number_of_ids: n,
            min_id: INVALID_ID,
            offset,
            data: vec![false; (n + offset + (n & 1)) as usize],
        }
    }

    pub fn id_count(&self) -> u32 {
        self.max_number_of_ids
    }

    /// Adds an id. Adding an id that is already queued does nothing.
    pub fn push(&mut self, id: u32) {
        debug_assert!(id < self.id_count(), "id out of bounds");
        if self.min_id == INVALID_ID {
            self.min_id = id;
        } else {
            min_to(&mut self.min_id, id);
        }
        let mut x = id + self.offset;
        if self.data[x as usize] {
            return;
        }
        loop {
            self.data[x as usize] = true;
            if x == ROOT {
                break;
            }
            x = parent(x);
            if self.data[x as usize] {
                break;
            }
        }
    }

    pub fn empty(&self) -> bool {
        self.min_id == INVALID_ID
    }

    /// Removes and returns the smallest id. Must not be called on an empty queue.
    pub fn pop(&mut self) -> u32 {
        debug_assert!(!self.empty());
        let ret = self.min_id;
        let mut x = self.min_id + self.offset;
        loop {
            self.data[x as usize] = false;
            if x == ROOT {
                self.min_id = INVALID_ID;
                return ret;
            }
            if is_smaller_child(x) {
                let y = other_child(x);
                if self.data[y as usize] {
                    break;
                }
            }
            x = parent(x);
        }
        x = other_child(x);
        loop {
            if x >= self.offset {
                break;
            }
            x = smaller_child(x);
            if !self.data[x as usize] {
                x = other_child(x);
            }
        }
        self.min_id = x - self.offset;
        ret
    }

    pub fn clear(&mut self) {
        while !self.empty() {
            self.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_in_increasing_order_without_duplicates() {
        let mut q = IDSetMinQueue::new(11);
        for id in [7, 3, 9, 3, 0, 10, 5] {
            q.push(id);
        }
        let mut out = Vec::new();
        while !q.empty() {
            out.push(q.pop());
        }
        assert_eq!(out, vec![0, 3, 5, 7, 9, 10]);
        q.push(4);
        q.clear();
        assert!(q.empty());
    }
}
