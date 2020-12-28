#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

//mod bubblesort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn simple_test() {
        let mut things = vec![4, 2, 3, 1];
        sort::<_, StdSorter>(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }

    quickcheck! {
        fn proper_testing(v: Vec<u32>) -> bool {
            let mut vector = v.clone();
            sort::<_, StdSorter>(&mut vector);
            for current in 1..vector.len() {
                if vector[current - 1] > vector[current] {
                    return false;
                }
            }
            true
        }
    }
}
