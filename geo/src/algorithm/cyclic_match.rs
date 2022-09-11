use crate::coords_iter::CoordsIter;
use geo_types::Coordinate;
use geo_types::{CoordFloat, LineString};

pub trait CyclicMatch<T> {
    // is p a cyclic match to `self`.
    fn is_cyclic_match(&self, rhs: Self) -> bool;
}

impl<T> CyclicMatch<T> for LineString<T>
where
    T: CoordFloat,
{
    fn is_cyclic_match(&self, rhs: Self) -> bool
    where
        T: CoordFloat,
    {
        if !self.is_closed() || !rhs.is_closed() {
            return false;
        }

        if self.coords_count() != rhs.coords_count() {
            return false;
        }

        // Remove repeated element. so cycle() will work.
        let mut b: Vec<Coordinate<T>> = rhs.into_iter().collect();
        b.pop();

        let len = b.len() as i32;

        let mut b_cycle = b.into_iter().cycle();

        let mut n_matches = 0;
        let mut n_restarts = 0;
        'outer: loop {
            let mut a_iter = self.clone().into_iter();
            '_inner: loop {
                match a_iter.next() {
                    None => {
                        return false;
                    }
                    Some(a_next) => {
                        if a_next == b_cycle.next().unwrap() {
                            n_matches += 1;
                            if n_matches == len {
                                return true;
                            }
                        } else {
                            // Restart.
                            n_matches = 0;
                            n_restarts += 1;
                            if n_restarts > len {
                                return false;
                            }
                            break 'outer true;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use geo_types::Coordinate;
    use geo_types::LineString;

    const A: Coordinate<f64> = Coordinate { x: 0., y: 0. };
    const B: Coordinate<f64> = Coordinate { x: 0., y: 10. };
    const C: Coordinate<f64> = Coordinate { x: 10., y: 10. };
    const D: Coordinate<f64> = Coordinate { x: 0., y: 10. };
    const E: Coordinate<f64> = Coordinate { x: 0.5, y: 0.5 };
    #[test]
    fn test_cyclic_match() {
        let mut ls_x = LineString(vec![A, B, C, D]);
        ls_x.close();
        let mut ls_y = LineString(vec![C, D, A, B]);
        ls_y.close();

        assert!(ls_x.is_cyclic_match(ls_x.clone()));
        assert!(ls_x.is_cyclic_match(ls_y));

        let one_less = LineString(vec![A, B, C]);
        assert!(!ls_x.is_cyclic_match(one_less));

        let one_more = LineString(vec![A, B, C, D, E]);
        assert!(!ls_x.is_cyclic_match(one_more));

        let a_cyclic = LineString(vec![A, D, C, B]);
        assert!(!ls_x.is_cyclic_match(a_cyclic));
    }
}
