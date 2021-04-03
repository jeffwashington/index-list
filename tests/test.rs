use indexlist::*;
use std::collections::HashSet;
use rand::{Rng, seq::SliceRandom};

fn debug_print_indexes(list: &IndexList<u64>) {
    let mut index = list.first_index();
    let mut last = None;
    print!("[ ");
    while let Some(ndx) = index {
        if last.is_some() {
            print!(" >< ");
        }
        print!("{}", ndx);
        debug_assert_eq!(list.prev_index(ndx), last);
        last = index;
        index = list.next_index(ndx);
    }
    println!(" ]");
}

#[test]
fn test_instantiate() {
    let mut list = IndexList::<u64>::new();
    assert_eq!(list.len(), 0);
    assert_eq!(list.capacity(), 0);
    assert_eq!(list.is_index_used(0), false);
    assert_eq!(list.first_index(), None);
    assert_eq!(list.last_index(), None);
    assert_eq!(list.next_index(0), None);
    assert_eq!(list.prev_index(0), None);
    assert_eq!(list.get_index(0), None);
    assert_eq!(list.get_mut_index(0), None);
    assert_eq!(list.remove_first(), None);
    assert_eq!(list.remove_last(), None);
    assert_eq!(list.remove_index(0), None);
    assert_eq!(list.to_vec(), Vec::<&u64>::new());
    list.trim_safe();
    list.trim_swap();
}
#[test]
fn basic_insert_remove() {
    let mut list = IndexList::<u64>::new();
    let count = 9;
    (0..count).for_each(|i| {
        let ndx = list.insert_first(i);
        assert_eq!(list.is_index_used(ndx), true);
    });
    println!("{}", list);
    assert_eq!(list.capacity(), count as usize);
    assert_eq!(list.len(), count as usize);
    list.trim_swap();
    (0..count).rev().for_each(|i| {
        assert_eq!(list.remove_first(), Some(i));
        assert_eq!(list.is_index_used(i as Index), false);
        assert_eq!(list.len(), i as usize);
    });
    assert_eq!(list.remove_first(), None);
    assert_eq!(list.remove_last(), None);
    assert_eq!(list.capacity(), count as usize);
    list.trim_safe();
    assert_eq!(list.capacity(), 0);
}
#[test]
fn test_append() {
    let mut list = IndexList::from(&mut vec!["A", "B", "C"]);
    let mut other = IndexList::from(&mut vec!["D", "E", "F"]);
    list.append(&mut other);
    assert_eq!(list.len(), 6);
    assert_eq!(list.capacity(), 6);
    assert_eq!(list.get_index(3), Some(&"D"));
    let parts: Vec<&str> = list.iter().map(|e| e.as_ref()).collect();
    assert_eq!(parts.join(", "), "A, B, C, D, E, F");
}
#[test]
fn test_trim_swap() {
    let mut rng = rand::thread_rng();
    let mut list = IndexList::<u64>::new();
    for round in 0..4 {
        debug_print_indexes(&list);
        (0..16).for_each(|i| {
            list.insert_last(16 * round + i);
        });
        debug_print_indexes(&list);
        let mut indexes: Vec<usize> = (0..list.capacity()).collect();
        indexes.shuffle(&mut rng);
        (0..8).for_each(|_| {
            list.remove_index(indexes.pop().unwrap());
        });
        debug_print_indexes(&list);
        list.trim_swap();
        assert_eq!(list.capacity(), 8 * (1 + round) as usize);
        assert_eq!(list.len(), list.capacity());
    }
}
#[test]
fn test_single_element() {
    let mut list = IndexList::<u64>::new();
    for num in 0..8 {
        match num & 1 {
            0 => list.insert_first(num),
            _ => list.insert_last(num),
        };
        let val = match num & 2 {
            0 => list.remove_first(),
            _ => list.remove_last(),
        };
        assert_eq!(val, Some(num));
    }
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.capacity(), 1);
    assert_eq!(list.len(), 0);
}
#[test]
fn insert_remove_variants() {
    let count = 256;
    let mut rng = rand::thread_rng();
    let mut list = IndexList::<u64>::new();
    let mut numbers: HashSet<u64> = HashSet::with_capacity(count);
    let mut indexes: Vec<usize> = Vec::with_capacity(count);
    for _ in 0..8 {
        for c in 0..count {
            let num = c as u64;
            numbers.insert(num);
            print!("IndexList#{}:insert ", num);
            match c & 3 {
                0 => {
                    let ndx = list.insert_first(num);
                    println!("first - index {}", ndx);
                    indexes.push(ndx);
                },
                1 => {
                    let that = indexes[rng.gen_range(0..c)];
                    print!("before {} ", that);
                    let ndx = list.insert_before(that, num);
                    println!("- index {}", ndx);
                    indexes.push(ndx);
                },
                2 => {
                    let that = indexes[rng.gen_range(0..c)];
                    print!("after {} ", that);
                    let ndx = list.insert_after(that, num);
                    println!("- index {}", ndx);
                    indexes.push(ndx);
                },
                _ => {
                    let ndx = list.insert_last(num);
                    println!("last - index {}", ndx);
                    indexes.push(ndx);
                },
            }
            print!("IndexList: ");
            debug_print_indexes(&list);
        }
        assert_eq!(list.len(), count);
        for c in (1..=count).rev() {
            let ndx = indexes.swap_remove(rng.gen_range(0..c as usize));
            println!("IndexList - remove {}", ndx);
            let num = list.remove_index(ndx).unwrap();
            //println!("IndexList: {}", list.to_debug_string());
            assert!(numbers.remove(&num));
        }
        assert_eq!(list.capacity(), count);
        assert_eq!(list.len(), 0);
        assert!(numbers.is_empty());
        assert!(indexes.is_empty());
        list.trim_safe();
        assert_eq!(list.capacity(), 0);
    }
}
