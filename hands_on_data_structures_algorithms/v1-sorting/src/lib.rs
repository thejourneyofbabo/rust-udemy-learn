use std::fmt::Debug;

// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        //println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

pub fn bubble_sort_fb<T: PartialOrd + Debug>(v: &mut [T]) {
    // Bubble sort forward & backward
    let len = v.len();
    if len <= 1 {
        return;
    }

    let mut left = 0;
    let mut right = len - 1;
    let mut swapped = true;

    while swapped {
        swapped = false;

        println!("{:?}", v);
        // Forward pass
        for i in left..right {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }

        right -= 1;
        swapped = false;

        // Backward pass
        for i in (left..right).rev() {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                swapped = true;
            }
        }
        left += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3, 2];
        bubble_sort(&mut v);
        println!("\n");
        let mut v2 = vec![4, 6, 1, 8, 11, 13, 3, 2];
        bubble_sort_fb(&mut v2);
        assert_eq!(v, vec![1, 2, 3, 4, 6, 8, 11, 13]);
        assert_eq!(v2, vec![1, 2, 3, 4, 6, 8, 11, 13]);
    }
}
