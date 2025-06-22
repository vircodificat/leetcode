fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let n = nums1.len() + nums2.len();

    let mut merged: Vec<i32> = Vec::with_capacity(n);

    let mut nums1_iter = nums1.into_iter().peekable();
    let mut nums2_iter = nums2.into_iter().peekable();

    loop {
        match (nums1_iter.peek(), nums2_iter.peek()) {
            (Some(_), None) => merged.push(nums1_iter.next().unwrap()),
            (None, Some(_)) => merged.push(nums2_iter.next().unwrap()),
            (Some(a), Some(b)) => {
                if a <= b {
                    merged.push(nums1_iter.next().unwrap());
                } else {
                    merged.push(nums2_iter.next().unwrap());
                }
            }
            (None, None) => return merged,
        }
    }
}
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n = nums1.len() + nums2.len();

    if n == 0 {
        return 0.0;
    }

    let merged = merge(nums1, nums2);

    if n == 1 {
        return f64::try_from(merged[0]).unwrap();
    }

    let mid = merged.len() / 2;

    if mid % 2 == 0 {
        let lo_mid = f64::try_from(merged[mid    ]).unwrap();
        let hi_mid = f64::try_from(merged[mid - 1]).unwrap();
        (lo_mid + hi_mid) / 2.0
    } else {
        merged[mid].try_into().unwrap()
    }
}

fn main () {
    let result = find_median_sorted_arrays(vec![1, 3], vec![2]);
    dbg!(result);
}
