use core::fmt;
use arrayvec::{Array, ArrayVec};

#[cfg(test)]
extern crate quickcheck;

pub struct LRUCache<A: Array> {
    /// The most-recently-used entry is located at the `head` index
    /// These entries form a linked list. Once an entry is added to
    /// the array, its index never changes.
    entries: ArrayVec<A>,
    /// Index of the first entry in the cache.
    head: u16,
    /// Index of the last entry in the cache.
    tail: u16,
}

#[derive(Debug, Clone)]
pub struct Entry<T> {
    /// The value stored at this entry
    val: T, 
    /// Index of the previous entry in the "linked list"
    prev: u16,
    /// Index of the next entry in the "linked list"
    next: u16,
}

impl<A: Array> Default for LRUCache<A> {
    fn default() -> Self {
        let cache = LRUCache {
            entries: ArrayVec::new(),
            head: 0,
            tail: 0,
        };

        assert!(
            cache.entries.capacity() < u16::max_value() as usize,
            "Capacity overflow"
        );

        cache
    }
}

impl<T, A> Clone for LRUCache<A>
where
    A: Array<Item = Entry<T>>,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            head: self.head,
            tail: self.tail,
        }
    }
}

impl<T, A> fmt::Debug for LRUCache<A>
where
    A: Array<Item = Entry<T>>,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LRUCache")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("entries", &self.entries)
            .finish()
    }
} 

impl<T, A> LRUCache<A>
where
    A: Array<Item = Entry<T>>,
{
    /// Returns the number of elements in the cache
    pub fn num_entries(&self) -> usize {
        self.entries.len()
    }

    /// Returns the most-recently-used entry (the one at the head index)
    pub fn front(&self) -> Option<&T> {
        self.entries
            .get(self.head as usize)
            .map(|e| &e.val)
    }

    /// Returns a mutable reference to the most-recently-used 
    /// entry (the one at the head index)
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.entries
            .get_mut(self.head as usize)
            .map(|e| &mut e.val)
    }
    
    /// Touches the first item in the cache that matches the given
    /// predicate. Returns `true` on a hit, `false` if no matches.
    pub fn touch<F>(&mut self, mut pred: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        match self.iter_mut()
            .find(|&(_, ref x)| pred(x))
        {
            Some((i, _)) => {
                self.touch_index(i);
                true
            },
            None => false,
        }
    }

    /// Touch a given entry at the given index, putting it first
    /// in the list.
    #[inline]
    fn touch_index(&mut self, index: u16) {
        if index != self.head {
            self.remove(index);
            self.push_front(index);
        }
    }
    
    /// Returns the first item in the cache that matches the given 
    /// predicate. If a result is found, it is moved to the head of
    /// the cache.
    pub fn find<F>(&mut self, pred: F) -> Option<&mut T>
    where
        F: FnMut(&T) -> bool,
    {
        if self.touch(pred) {
            self.front_mut()
        } else {
            None
        }
    }

    /// Insert a given value in the cache.
    /// The entry becomes the most-recently-used entry in the cache. If the 
    /// cache is full, the least-recently-used element is removed.
    pub fn insert(&mut self, val: T) {
        let entry = Entry {
            val,
            prev: 0,
            next: 0,
        };

        let new_head = if self.entries.len() == self.entries.capacity() {
            let last_index = self.pop_back();
            self.entries[last_index as usize] = entry;
            last_index
        } else {
            self.entries.push(entry);
            self.entries.len() as u16 - 1
        };

        self.push_front(new_head);
    }
    
    /// Clear all entries from the cache.
    pub fn clear(&mut self) {
        self.entries.clear(); 
    }

    /// Insert a new entry at the head of the cache.
    fn push_front(&mut self, index: u16) {
        if self.entries.len() == 1 {
            self.tail = index;
        } else {
            self.entries[index as usize].next = self.head;
            self.entries[self.head as usize].prev = index;
        }

        self.head = index;
    }

    /// Remove the last entry from the linked list. Returns the index of
    /// the removed entry.
    /// Note that this only unlinks the entry from the list, it doesn't 
    /// remove it from the array.
    fn pop_back(&mut self) -> u16 {
        let old_tail = self.tail;
        let new_tail = self.entries[old_tail as usize].prev;
        self.tail = new_tail;
        old_tail
    }

    /// Iterate mutably over the contents of the cache.
    fn iter_mut(&mut self) -> IterMut<A> {
        IterMut {
            pos: self.head,
            done: self.entries.len() == 0,
            cache: self,
        }
    }

    /// Remove an entry from the linked list.
    /// Note that this only unlinks the entry from the list; it doesn't 
    /// remove it from the array.
    fn remove(&mut self, index: u16) {
        let prev = self.entries[index as usize].prev;
        let next = self.entries[index as usize].next;

        if index == self.head {
            self.head = next;
        } else {
            self.entries[prev as usize].next = next;
        }

        if index == self.tail {
            self.tail = prev;
        } else {
            self.entries[next as usize].prev = prev;
        }
    }
}

/// Mutable iterator over values in the LRUCache, from most-recently-used
/// to least-recently-used.
struct IterMut<'a, A: 'a + Array> {
    cache: &'a mut LRUCache<A>,
    pos: u16,
    done: bool,
}

impl<'a, T, A> Iterator for IterMut<'a, A>
where
    T: 'a,
    A: 'a + Array<Item = Entry<T>>,
{
    type Item = (u16, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None; }

        // Use a raw pointer here because the compiler doesn't know that 
        // subsequent calls cannot alias
        let entry = unsafe {
            &mut *(&mut self.cache.entries[self.pos as usize] as *mut Entry<T>)   
        };

        let index = self.pos;

        if self.pos == self.cache.tail {
            self.done = true;
        }

        self.pos = entry.next;

        Some((index, &mut entry.val))
    }
}
