// edition:2021

#![allow(unused_macros)]

// FOLLOW(pat) = {FatArrow, Comma, Eq, Ident(if), Ident(in)}
macro_rules! follow_pat {
    ($p:pat ()) => {};       //~ERROR `$p:pat` is followed by `(`
    ($p:pat []) => {};       //~ERROR `$p:pat` is followed by `[`
    ($p:pat {}) => {};       //~ERROR `$p:pat` is followed by `{`
    ($p:pat :) => {};        //~ERROR `$p:pat` is followed by `:`
    ($p:pat >) => {};        //~ERROR `$p:pat` is followed by `>`
    ($p:pat +) => {};        //~ERROR `$p:pat` is followed by `+`
    ($p:pat |) => {};        //~ERROR `$p:pat` is followed by `|`
    ($p:pat ident) => {};    //~ERROR `$p:pat` is followed by `ident`
    ($p:pat $q:pat) => {};   //~ERROR `$p:pat` is followed by `$q:pat`
    ($p:pat $e:expr) => {};  //~ERROR `$p:pat` is followed by `$e:expr`
    ($p:pat $t:ty) => {};    //~ERROR `$p:pat` is followed by `$t:ty`
    ($p:pat $s:stmt) => {};  //~ERROR `$p:pat` is followed by `$s:stmt`
    ($p:pat $q:path) => {};  //~ERROR `$p:pat` is followed by `$q:path`
    ($p:pat $b:block) => {}; //~ERROR `$p:pat` is followed by `$b:block`
    ($p:pat $i:ident) => {}; //~ERROR `$p:pat` is followed by `$i:ident`
    ($p:pat $t:tt) => {};    //~ERROR `$p:pat` is followed by `$t:tt`
    ($p:pat $i:item) => {};  //~ERROR `$p:pat` is followed by `$i:item`
    ($p:pat $m:meta) => {};  //~ERROR `$p:pat` is followed by `$m:meta`
}

fn main() {}
