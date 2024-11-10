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

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half,
    // sort the right half, O(n*ln(n)) = O(n+2*(n/2)+4*n(4)..)
    // 1024 * 10 < 1024 * 1024
    // bring the sorted halfs together O(n)

    println!("MS:{:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len()); // Result Vector
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // Bring them together again add whichever is lowest the front of a or the front of b
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3, 2];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 2, 3, 4, 6, 8, 11, 13]);
    }

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
