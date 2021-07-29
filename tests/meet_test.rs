// Meet Operation, Exterior Product, ^

#[cfg(test)]
mod tests {
  use g3::{plane,point,line,branch,ideal_line};
  #[test] fn meet_plane_plane() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let p2 = plane(1.0, 2.0, 3.0, 4.0);
    let _l = p1 ^ p2;
    todo!();
  }
  #[test] fn meet_plane_branch() {
    let p1 = plane(1.0, 2.0, 3.0, 4.0);
    let l = branch(1.0,2.0,3.0);
    let _a1 = p1 ^ l;
    let _a2 = l ^ p1;
    todo!();
  }
  #[test] fn meet_plane_ideal_line() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = ideal_line(1.0, 2.0, 3.0);
    let _a1 = p ^ l;
    let _a2 = l ^ p;
    todo!();
  }
  #[test] fn meet_plane_line() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let _a1 = p ^ l;
    let _a2 = l ^ p;
    todo!();
  }
  #[test] fn meet_plane_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(1.0, 2.0, 3.0);
    let _d1 =  p ^ a;
    let _d2 =  a ^ p;
    todo!();
  }
  // Doesn't exist
  // #[test] fn meet_point_line() {
  //   let a = point(1.0, 2.0, 3.0);
  //   let l = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
  //   let _p1 = a ^ l;
  //   let _p2 = l ^ a;
  //   todo!();
  // }
  #[test] fn meet_line_line() {
    let l1 = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let l2 = line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    let _d = l1 ^ l2;
    todo!();
  }
}
