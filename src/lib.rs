use std::{isize, rc::Rc};

fn quick_sort<'a>(arr: &'a mut Vec<usize>, low: usize, high: usize) -> () {
    // return from function if it's < 2, it's already sorted either desc or asc, 1 or 0 shouldn't
    // be iterated over
    if arr.len() <= 2 {
        return;
    }

    let (mut l_pointer, mut r_pointer, pivot, mut temp) = (low, high, arr[low], 0);
    while l_pointer < r_pointer {
        while arr[l_pointer] <= arr[pivot] {
            l_pointer += 1;
        }
        while arr[r_pointer] > arr[pivot] {
            r_pointer -= 1;
        }
        if l_pointer < r_pointer {
            temp = arr[l_pointer];
            arr[l_pointer] = arr[r_pointer];
            arr[r_pointer] = temp;
        }
    }
    temp = arr[pivot];
    arr[pivot] = arr[r_pointer];
    arr[r_pointer] = temp;
    quick_sort(arr, low, r_pointer - 1);
    quick_sort(arr, l_pointer + 1, high);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_array_sorted() {
        let mut v: Vec<usize> = vec![1, 2, 3, 4];
        quick_sort(&mut v, 0, 3);
        assert_eq!(v, [1, 2, 3, 4]);
    }

    #[test]
    fn sort_array_simple() {
        let mut v: Vec<usize> = vec![3, 1, 2, 4];
        quick_sort(&mut v, 0, 3);
        assert_eq!(v, [1, 2, 3, 4]);
    }

    #[test]
    fn return_array_of_two() {
        let mut v: Vec<usize> = vec![1, 2];
        quick_sort(&mut v, 0, 1);
        assert_eq!(v, [1, 2]);
    }

    // #[test]
    // fn partition_simple() {
    //     let v = vec![1, 2, 3, 4];
    //     let left = Rc::new(&v[0..2]);
    //     let right = Rc::new(&v[2..4]);
    //     assert_eq!(partition(&v, 2, 4), (left, right));
    // }
}
