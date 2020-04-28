use std::env;
use std::process;
use std::fmt::Debug;
use rand::prelude::*;
use rand::distributions::{Distribution, Standard};

fn generate_random_ints<T>(length: Option<usize>) -> Vec<T>
    where
        Standard: Distribution<T>
{
    let vec_length = if let Some(length) = length { length } else { 16 };
    let mut vec = Vec::with_capacity(vec_length);

    for _ in 0..vec_length {
        vec.push(rand::random::<T>());
    }

    vec
}

#[derive(Debug)]
enum SortMethod {
    Selection,
    Insertion,
    MergeInPlace,
    MergeSublist,
}

#[derive(Debug)]
struct Sort<T: PartialOrd + Clone + Debug> {
    items: Vec<T>,
    method: SortMethod,
}

impl<T: PartialOrd + Clone + Debug> Sort<T> {
    fn new(items: Vec<T>, method: SortMethod) -> Self {
        Sort {
            items,
            method,
        }
    }

    fn sort(&mut self) {
        match self.method {
            SortMethod::Selection => self.selection_sort(),
            SortMethod::Insertion => self.insertion_sort(),
            SortMethod::MergeInPlace => self.merge_sort_inplace(),
            SortMethod::MergeSublist => self.merge_sort_sublist(),
        }
    }

    fn get_vec(&self) -> &Vec<T> {
        &self.items
    }

    fn get_vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }

    fn print(&self) {
        println!("Items: {:?}", self.get_vec());
    }

    // Selection sort and helper methods
    fn selection_sort(&mut self) {
        let items = self.get_vec();

        for i in 0..items.len() {
            let min_pos = self.sel_min_pos(i);
            self.sel_swap(i, min_pos);
        }
    }

    fn sel_min_pos(&self, from: usize) -> usize {
        let items = self.get_vec();

        let mut pos = from;
        for i in (from + 1)..items.len() {
            if items.get(i) < items.get(pos) {
                pos = i;
            }
        }

        pos
    }

    fn sel_swap(&mut self, a: usize, b: usize) {
        let items = self.get_vec_mut();
        items.swap(a, b);
    }

    // Insertion sort and helper methods
    fn insertion_sort(&mut self) {
        let item_length = self.get_vec().len();

        let mut i = 1;
        loop {
            if i == item_length {
                break;
            }

            let larger = self.ins_find_larger_pos(i);

            if larger < i {
                self.ins_insert(i, larger);
                i -= 1;
            }

            i += 1;
        }
    }

    fn ins_find_larger_pos(&self, pos: usize) -> usize {
        let items = self.get_vec();

        let cur = items.get(pos);
        let mut larger = pos;
        for i in (0..pos).rev() {
            if items.get(i) < cur {
                break;
            }

            if items.get(i) > cur {
                larger = i;
            }
        }

        larger
    }

    fn ins_insert(&mut self, old: usize, new: usize) {
        let items = self.get_vec_mut();
        let old_val = items.remove(old);
        items.insert(new, old_val);
    }

    // Merge sorting in-place and helper methods

    fn merge_sort_inplace(&mut self) {
        let item_length = self.get_vec().len();

        self.merge_sort_inplace_sublist(0, item_length);
    }

    fn merge_sort_inplace_sublist(&mut self, start: usize, length: usize) {
        // Sublist of one item, nothing to sort
        if length <= 1 {
            return;
        }

        // Split this sublist into two
        let a_start = start;
        let a_length = length / 2;
        let b_start = a_start + a_length;
        let b_length = length - a_length;

        // Recursively sort sublists
        self.merge_sort_inplace_sublist(a_start, a_length);
        self.merge_sort_inplace_sublist(b_start, b_length);

        // Merge sorted sublists
        self.merge_sort_inplace_merge((a_start, a_length), (b_start, b_length));
    }

    fn merge_sort_inplace_merge(&mut self, a: (usize, usize), b: (usize, usize)) {
        // let items = self.get_vec();
        let items_mut = self.get_vec_mut();
        let (mut a_start, mut a_length) = a;
        let (mut b_start, mut b_length) = b;

        let mut orig_a_start = a_start;
        let mut orig_a_length = a_length;
        let mut orig_b_start = b_start;
        let mut orig_b_length = b_length;

        while a_length > 0 && b_length > 0 {
            let a = items_mut[a_start].clone();
            let b = items_mut[b_start].clone();

            if a <= b {
                a_start += 1;
                a_length -= 1;
            } else {
                items_mut.remove(b_start);
                items_mut.insert(a_start, b);

                b_start += 1;
                b_length -= 1;
            }
        }

        drop(items_mut);

        // Merge sort the elements displaced from A into B
        if b_length > 0 && b_start != orig_b_start {
            self.merge_sort_inplace_merge((orig_b_start, orig_b_length - b_length), (b_start, b_length));
        } 

        let items_mut = self.get_vec_mut();
    }

    // Merge sorting with sublists and helper methods

    fn merge_sort_sublist(&mut self) {
        let mut old_items: Vec<T> = vec![];
        std::mem::swap::<Vec<T>>(&mut old_items, &mut self.items);

        self.items = Self::merge_sort_sublist_sort(old_items);
    }

    fn merge_sort_sublist_sort(items: Vec<T>) -> Vec<T> {
        if items.len() <= 1 {
            return items;
        }

        let size = items.len();
        let mut left = Vec::with_capacity(size / 2);
        let mut right = Vec::with_capacity(size - (size / 2));

        for (idx, item) in items.iter().enumerate() {
            if idx < size / 2 {
                left.push(item.clone());
            } else {
                right.push(item.clone());
            }
        }

        left = Self::merge_sort_sublist_sort(left);
        right = Self::merge_sort_sublist_sort(right);

        Self::merge_sort_sublist_merge(left, right)
    }

    fn merge_sort_sublist_merge(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut source: &mut Vec<T> = &mut vec![];

        while left.len() > 0 && right.len() > 0 {
            if left[0] <= right[0] {
                source = &mut left;
            } else {
                source = &mut right;
            }

            result.push(source.remove(0));
        }

        while left.len() > 0 {
            result.push(left.remove(0));
        }

        while right.len() > 0 {
            result.push(right.remove(0));
        }

        result
    }
}

fn main() {
    let sort_algo = env::args().nth(1);

    let default_count = 16;
    let count = if let Some(arg) = env::args().nth(2) {
        if let Ok(count) = arg.parse() {
            count
        } else {
            default_count
        }
    } else {
        default_count
    };

    if let Some(algo) = sort_algo {
        let mut sort: Option<Sort<u8>> = None;

        match algo.as_str() {
            "selection" => {
                sort = Some(Sort::new(generate_random_ints(Some(count)), SortMethod::Selection));
                println!("Selection sorting {} random numbers", count);
            },
            "insertion" => {
                sort = Some(Sort::new(generate_random_ints(Some(count)), SortMethod::Insertion));
                println!("Insertion sorting {} random numbers", count);
            },
            "merge-in-place" => {
                sort = Some(Sort::new(generate_random_ints(Some(count)), SortMethod::MergeInPlace));
                println!("Merge sorting in-place {} random numbers", count);
            },
            "merge-sublist" => {
                sort = Some(Sort::new(generate_random_ints(Some(count)), SortMethod::MergeSublist));
                println!("Merge sorting by allocating sublists {} random numbers", count);
            },
            _ => {
                println!("No such algorithm: {}", algo);
                process::exit(1);
            },
        }

        // Run selection sort
        let mut sort = sort.unwrap();
        print!("Unsorted ");
        sort.print();

        sort.sort();
        print!("Sorted ");
        sort.print();
    } else {
        println!("You must provide a sorting algorithm!");
        process::exit(1);
    }
}

