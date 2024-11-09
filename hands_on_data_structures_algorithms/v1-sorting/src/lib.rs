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
        //swapped = false;

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
        let mut v2 = vec![4, 6, 1, 8, 11, 13, 3, 2];
        let mut v3 = vec![
            20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ];
        let mut v4 = vec![
            20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ];

        println!("Regular Bubble Sort");
        bubble_sort(&mut v);
        println!("\nBidirectional Bubble Sort");
        bubble_sort_fb(&mut v2);
        println!(" ");

        println!("Regular Bubble Sort");
        bubble_sort(&mut v3);
        println!("\nBidirectional Bubble Sort");
        bubble_sort_fb(&mut v4);

        assert_eq!(v, vec![1, 2, 3, 4, 6, 8, 11, 13]);
        assert_eq!(v2, vec![1, 2, 3, 4, 6, 8, 11, 13]);
        assert_eq!(
            v3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
        assert_eq!(
            v4,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }
}
