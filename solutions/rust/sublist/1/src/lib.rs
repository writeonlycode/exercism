#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if first_list.len() < second_list.len() && is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    } else if second_list.len() < first_list.len() && is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

fn is_sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
    for i in 0..list2.len() {
        if i + list1.len() <= list2.len() {
            if list1 == &list2[i..(i + list1.len())] {
                return true;
            }
        }
    }

    false
}