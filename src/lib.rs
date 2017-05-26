/// Sums two slices
pub fn sum_arrays(a: &[i32], b: &[i32], out: &mut [i32]) {
    for (o, (x, y)) in out.iter_mut().zip(a.iter().zip(b.iter())) {
        *o =
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = vec![1; 30];
        let b = (0..30).collect::<Vec<_>>();
        let mut c = vec![0; 30];
        let expected = (1..31).collect::<Vec<_>>();

        sum_arrays(&a, &b, &mut c);
        assert_eq!(expected, c);
    }
}
