#[cfg(test)]
mod tests {
    use g3::{dual, Dual};

    // trait F32 { fn f32(self)->f32; }
    // impl F32 for f32 { fn f32(self)->f32 { self } }
    // impl F32 for i64 { fn f32(self)->f32 { self as f32 } }

    // #[macro_export] macro_rules! dual {
    //   ($p:expr, $q:expr) => {
    //     { Dual::new(F32::f32($p),F32::f32($q)) };
    //   };
    // }

    #[test]
    fn macro_testing() {
        let d = dual(1.0, 2.0);
        assert_eq!(d, Dual::new(1.0, 2.0));

        //   let A = point!(0,0.8,0);
        //   let B = point!(0.8,-1,-0.8);
        //   let C = point!(-.8,-1,-0.8);
        //   let D = point!(0.8,-1,0.8);
        //   let E = point!(-0.8,-1,0.8);

        //   // Points can be joined into lines and planes
        //   let ec = E & C;
        //   let p = A & B & C;
    }
}
