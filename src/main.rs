mod early_late_bound;
mod quiz1;
mod quiz10;
mod quiz11;
mod quiz12;
mod quiz13;
mod quiz14;
mod quiz15;
mod quiz16;
mod quiz17;
mod quiz18;
mod quiz19;
mod quiz2;
mod quiz20;
mod quiz21;
mod quiz22;
mod quiz24;
mod quiz25;
mod quiz26;
mod quiz27;
mod quiz28;
mod quiz29;
mod quiz3;
mod quiz30;
mod quiz31;
mod quiz32;
mod quiz33;
mod quiz4;
mod quiz5;
mod quiz6;
mod quiz7;
mod quiz8;
mod quiz9;
fn main() {}

// fn f<'a>() where 'a: 'a {}
// fn g<'a: 'a>() {}

// fn main() {
//     let pf = f::<'static> as fn();
//     let pg = g::<'static> as fn();
//     print!("{}", pf == pg);
//     dbg!(pf);
//     dbg!(pg);
// }

// fn f<'a>() {}
// fn main() {
//     // let _ = f::<'static>;
//     // let _ = f; // just leave off the lifetime specifier
//     // let _: for<'a> fn() = f; // With type ascription
//     ////
// // // This happens to work but no lifetimes can be specified without lint or error
// // let _ = mixed;

// // // For example, this triggers the lint as a warning; specifying 1 or 3+ lifetimes triggers an error:
// // let _ = mixed::<'static, 'static>;

// // // type ascription allows specifying the lifetimes without warning or error
// // let _: for<'a> fn(&'a Foo<'static, 'static>) -> Foo<'static, 'static> = mixed;
// // These compiled with `mixed` defined as above.  The first version fires the lint as a warning.
// // ////
// // let _ = mixed::<'static, 'static>;
// // let _: for<'a> fn(&'a Foo<'static, 'static>) -> Foo<'static, 'static> = mixed;

// // Apply the suggested hint:

// // This now fails: expected 3 lifetime arguments
// // let _ = mixed::<'static, 'static>;
// // let _ = mixed::<'static, 'static, 'static>;

// // This now fails: one type is more general than the other
// let _: for<'a> fn(&'a Foo<'static, 'static>) -> Foo<'static, 'static> = mixed;
// }
// struct Foo<'a, 'b> { a: &'a(), b: &'b () }
// // There is an implicit late bound parameter due to &Foo; 'a and 'b are early bound
// // fn mixed<'a: 'a, 'b: 'b>(_: &Foo<'a, 'b>) -> Foo<'a, 'b> { todo!() }
// fn mixed<'a: 'a, 'b: 'b, 'c: 'c>(_: &'c Foo<'a, 'b>) -> Foo<'a, 'b> { todo!() }
