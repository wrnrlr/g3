
// #[allow(unused_macros)]
// #[macro_use]

// #[macro_use]
// mod inner {
//   macro_rules! define_call_fn_mut {
//     ($typ:ty,$arg:ty,$out:ty) => { impl FnMut<($arg,)> for $typ { extern "rust-call" fn call_mut(&mut self, args: ($arg,))->$out { self.call(args) } } }
//   }
//   macro_rules! define_call_fn_once {
//     ($typ:ty,$arg:ty,$out:ty) => { impl FnOnce<($arg,)> for $typ { type Output = $out; extern "rust-call" fn call_once(self, args: ($args,))->$out { self.call(args) } } }
//   }  
// }


// #[macro_export]
// macro_rules! define_call_fn {
//   ($typ:ty,$arg:ty,$out:ty) => { define_call_fn_mut!(typ,arg,out); define_call_fn_once!(typ,arg,out); } }


// // element macros


// #[macro_export] macro_rules! rotor {
//   ($ang_rad:expr,$x:expr,$y:expr,$z:expr) => {
//     { Rotor::new(F32Value.f32(&$ang_rad),F32Value.f32(&$x),F32Value.f32(&$y),F32Value.f32(&$z)) }
//   };
// }