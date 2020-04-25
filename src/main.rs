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
enum Sort<T: PartialOrd + Clone + Debug> {
    Selection(Vec<T>),
    Insertion(Vec<T>),
    Merge(Vec<T>),
}

impl<T: PartialOrd + Clone + Debug> Sort<T> {
    fn sort(&mut self) {
        match self {
            Sort::Selection(_) => self.selection_sort(),
            Sort::Insertion(_) => self.insertion_sort(),
            Sort::Merge(_) => self.merge_sort(),
        }
    }

    fn get_vec(&self) -> &Vec<T> {
        match self {
            Sort::Selection(vec) => vec,
            Sort::Insertion(vec) => vec,
            Sort::Merge(vec) => vec,
        }
    }

    fn get_vec_mut(&mut self) -> &mut Vec<T> {
        match self {
            Sort::Selection(vec) => vec,
            Sort::Insertion(vec) => vec,
            Sort::Merge(vec) => vec,
        }
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

    // Merge sort and helper methods
    fn merge_sort(&mut self) {
        let item_length = self.get_vec().len();

        self.merge_sort_sublist(0, item_length);
    }

    fn merge_sort_sublist(&mut self, start: usize, length: usize) {
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
        self.merge_sort_sublist(a_start, a_length);
        self.merge_sort_sublist(b_start, b_length);

        // Merge sorted sublists
        self.merge_sort_merge((a_start, a_length), (b_start, b_length));
    }

    fn merge_sort_merge(&mut self, a: (usize, usize), b: (usize, usize)) {
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
            self.merge_sort_merge((orig_b_start, orig_b_length - b_length), (b_start, b_length));
        } 

        let items_mut = self.get_vec_mut();
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
        let mut sort: Option<Sort<u16>> = None;

        match algo.as_str() {
            "selection" => {
                sort = Some(Sort::Selection(generate_random_ints(Some(count))));
                println!("Selection sorting {} random numbers", count);
            },
            "insertion" => {
                sort = Some(Sort::Insertion(generate_random_ints(Some(count))));
                println!("Insertion sorting {} random numbers", count);
            },
            "merge" => {
                sort = Some(Sort::Merge(generate_random_ints(Some(count))));
                println!("Merge sorting {} random numbers", count);
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

