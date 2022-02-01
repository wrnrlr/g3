#[cfg(test)]
mod tests {
  use g3::{plane,point,line};

  #[test] #[ignore] fn project_point_onto_line() {
    let l = line(1.0, 2.0, 3.0, 1.0, 2.0, 3.0);
    let a = point(1.0, 2.0, 3.0);
    let _ = l.project_point(a);
    todo!()
  }
  #[test] #[ignore] fn project_plane_onto_line() {
    let l = line(1.0, 2.0, 3.0, 1.0, 2.0, 3.0);
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let _ = l.project_plane(p);
    todo!()
  }
  #[test] #[ignore] fn project_point_onto_plane() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(1.0, 2.0, 3.0);
    let _ = p.project_point(a);
    todo!()
  }
  #[test] #[ignore] fn project_line_onto_plane() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(1.0, 2.0, 3.0, 1.0, 2.0, 3.0);
    let _ = p.project_line(l);
    todo!()
  }
  #[test] #[ignore] fn project_plane_onto_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let a = point(1.0, 2.0, 3.0);
    let _ = a.project_plane(p);
    todo!()
  }
  #[test] #[ignore] fn project_line_onto_point() {
    let p = plane(1.0, 2.0, 3.0, 4.0);
    let l = line(1.0, 2.0, 3.0, 1.0, 2.0, 3.0);
    let _ = p.project_line(l);
    todo!()
  }
}
