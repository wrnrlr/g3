// Join Operation, Regressive Product, &
#[cfg(test)]
mod tests {
    use g3::{branch, ideal_line, line, plane, point};
    #[test]
    fn join_point_point() {
        let a = point(1.0, 2.0, 3.0);
        let b = point(1.0, 2.0, 3.0);
        let _l1 = a & b;
        todo!();
    }
    #[test]
    fn join_point_line() {
        let a = point(1.0, 2.0, 3.0);
        let l = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let _p1 = a & l;
        let _p2 = l & a;
        todo!();
    }
    #[test]
    fn join_point_branch() {
        let a = point(1.0, 2.0, 3.0);
        let l = branch(1.0, 2.0, 3.0);
        let _p1 = a & l;
        let _p2 = l & a;
        todo!();
    }
    #[test]
    fn join_point_ideal_line() {
        let a = point(1.0, 2.0, 3.0);
        let l = ideal_line(1.0, 2.0, 3.0);
        let _p1 = a & l;
        let _p2 = l & a;
        todo!();
    }
    #[test]
    fn join_plane_point() {
        let p1 = plane(1.0, 2.0, 3.0, 4.0);
        let a = point(1.0, 2.0, 3.0);
        let _d1 = p1 & a;
        let _d2 = a & p1;
        todo!();
    }
}
