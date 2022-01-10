use crate::Point;
use baryon::{Position, geometry::Geometry};

// fn cross(a:[f32;3],b:[f32;3])->[f32;3] {
//
// }

// https://sci-hub.yncjkj.com/10.1016/j.cag.2004.04.007
// https://sci-hub.yncjkj.com/10.1109/mcg.2003.1185582


pub fn triangle(f:[Point;3])->Geometry {
  // https://gamedev.stackexchange.com/questions/60630/how-do-i-find-the-circumcenter-of-a-triangle-in-3d
  // let ac = c-a;
  // let ab = b-a;
  // let ab_ac = cross(ac,bc);

  // Calculate normals
  // https://www.cv.nrao.edu/~mmorgan2/resources/geo3.html
  // https://www.ljll.math.upmc.fr/~frey/papers/scientific%20visualisation/Zaharia%20M.D.,%20Dorst%20L.,%20Modeling%20and%20visualization%20of%203D%20polygonal%20mesh%20surfaces%20using%20geometric%20algebra.pdf
  // https://en.wikipedia.org/wiki/Comparison_of_vector_algebra_and_geometric_algebra#Determinant_expansion_of_cross_and_wedge_products
  // https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal
  // let p = f[1] & f[2] & f[3];

  let a = [f[0].x(),f[0].y(),f[0].z()];
  let b = [f[1].x(),f[1].y(),f[1].z()];
  let c = [f[2].x(),f[2].y(),f[2].z()];

  let radius:f32 = 5.0;
  Geometry{
    positions: vec!(Position(a),Position(b),Position(c)),
    normals: None, indices: None, radius
  }
}
