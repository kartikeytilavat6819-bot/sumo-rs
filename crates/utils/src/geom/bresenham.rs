//! Port of utils/geom/Bresenham.{h,cpp}

/// compute the bresenham - interpolation between both values; the callback
/// (BresenhamCallBack::execute) is called on every step
pub fn compute(mut call_back: impl FnMut(i32, i32), val1: i32, val2: i32) {
    let smaller = val1.min(val2);
    let greater = val1.max(val2);
    let mut pos = 0;
    let mut c = smaller;
    for i in 0..greater {
        if smaller == val1 {
            call_back(pos, i);
        } else {
            call_back(i, pos);
        }
        c += 2 * smaller;
        if c >= 2 * greater {
            pos += 1;
            c -= 2 * greater;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn steps() {
        let mut v = Vec::new();
        super::compute(|a, b| v.push((a, b)), 2, 4);
        assert_eq!(v, [(0, 0), (0, 1), (1, 2), (1, 3)]);
    }
}
