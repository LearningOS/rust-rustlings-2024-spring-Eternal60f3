/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
use std::mem::swap;
use std::cmp::PartialOrd;
use std::fmt::Display;

fn partition<T: PartialOrd>(array: &mut [T], mut lidx: usize, mut ridx: usize) -> usize {
    let base_idx = lidx;
    while lidx < ridx {
        while lidx < ridx && array[base_idx] <= array[ridx] {
            ridx -= 1;
        }
        while lidx < ridx && array[lidx] <= array[base_idx] {
            lidx += 1;
        }
        array.swap(lidx, ridx);
    }
    array.swap(base_idx, lidx);
    lidx
}

fn quick_sort<T: PartialOrd + Display>(array: &mut [T], lidx: usize, ridx: usize) {
    if lidx >= ridx {
        return;
    }
    let mid = partition(array, lidx, ridx);
    if mid > 0 {
        quick_sort(array, lidx, mid - 1);
    } 
    if mid < ridx {
        quick_sort(array, mid + 1, ridx);
    }
}

fn sort<T: PartialOrd + Display>(array: &mut [T]){
	//TODO
    quick_sort(array, 0, array.len() - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}