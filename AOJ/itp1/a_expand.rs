//! # Bundled libraries
//!
//! - `lazy_static 1.4.0 (registry+https://github.com/rust-lang/crates.io-index)` licensed under `MIT/Apache-2.0`    as `crate::__bundled::__lazy_static_1_4_0`
//! - `proconio 0.4.3 (registry+https://github.com/rust-lang/crates.io-index)`    licensed under `MIT OR Apache-2.0` as `crate::__bundled::proconio`
//!
//! # Procedural macros
//!
//! - `proconio-derive 0.2.1 (registry+https://github.com/rust-lang/crates.io-index)` licensed under `MIT OR Apache-2.0`
//!
//! # License and Copyright Notices
//!
//! - `lazy_static 1.4.0 (registry+https://github.com/rust-lang/crates.io-index)`
//!
//!     ```text
//!     Copyright (c) 2010 The Rust Project Developers
//!
//!     Permission is hereby granted, free of charge, to any
//!     person obtaining a copy of this software and associated
//!     documentation files (the "Software"), to deal in the
//!     Software without restriction, including without
//!     limitation the rights to use, copy, modify, merge,
//!     publish, distribute, sublicense, and/or sell copies of
//!     the Software, and to permit persons to whom the Software
//!     is furnished to do so, subject to the following
//!     conditions:
//!
//!     The above copyright notice and this permission notice
//!     shall be included in all copies or substantial portions
//!     of the Software.
//!
//!     THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
//!     ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
//!     TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
//!     PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
//!     SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//!     CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//!     OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
//!     IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//!     DEALINGS IN THE SOFTWARE.
//!     ```
//!
//! - `proconio 0.4.3 (registry+https://github.com/rust-lang/crates.io-index)`
//!
//!     ```text
//!     The MIT License
//!     Copyright 2019 (C) statiolake <statiolake@gmail.com>
//!
//!     Permission is hereby granted, free of charge, to any person obtaining a copy of
//!     this software and associated documentation files (the "Software"), to deal in
//!     the Software without restriction, including without limitation the rights to
//!     use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
//!     the Software, and to permit persons to whom the Software is furnished to do so,
//!     subject to the following conditions:
//!
//!     The above copyright notice and this permission notice shall be included in all
//!     copies or substantial portions of the Software.
//!
//!     THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//!     IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
//!     FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
//!     COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
//!     IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
//!     CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//!
//!     Copyright for original `input!` macro is held by Hideyuki Tanaka, 2019.  The
//!     original macro is licensed under BSD 3-clause license.
//!
//!     Redistribution and use in source and binary forms, with or without
//!     modification, are permitted provided that the following conditions are met:
//!
//!     1. Redistributions of source code must retain the above copyright notice, this
//!        list of conditions and the following disclaimer.
//!
//!     2. Redistributions in binary form must reproduce the above copyright notice,
//!        this list of conditions and the following disclaimer in the documentation
//!        and/or other materials provided with the distribution.
//!
//!     3. Neither the name of the copyright holder nor the names of its contributors
//!        may be used to endorse or promote products derived from this software
//!        without specific prior written permission.
//!
//!     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
//!     ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
//!     WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//!     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//!     FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//!     DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//!     SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//!     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//!     OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//!     OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//!     ```

// keywords :

__prelude_for_main_crate!();

#[allow(unused_imports)]
use proconio::fastout;
use proconio::input;

/*#[fastout]
fn main() {
    println!("Hello World");
}*/
fn main() {
    let __proconio_stdout = ::std::io::stdout();
    let mut __proconio_stdout = ::std::io::BufWriter::new(__proconio_stdout.lock());
    #[allow(unused_macros)]
    macro_rules!print{($($tt:tt)*)=>{{use std::io::Write as _;::std::write!(__proconio_stdout,$($tt)*).unwrap();}};}
    #[allow(unused_macros)]
    macro_rules!println{($($tt:tt)*)=>{{use std::io::Write as _;::std::writeln!(__proconio_stdout,$($tt)*).unwrap();}};}
    let __proconio_res = {
        println!("Hello World");
    };
    <::std::io::BufWriter<::std::io::StdoutLock> as ::std::io::Write>::flush(
        &mut __proconio_stdout,
    )
    .unwrap();
    return __proconio_res;
}

// The following code was expanded by `cargo-equip`.

#[macro_export]
macro_rules! __prelude_for_main_crate(() => (pub use crate::__bundled::{__lazy_static_1_4_0::__macros::*,*};));

#[cfg_attr(any(), rustfmt::skip)]
const _: () = {
    #[macro_export]macro_rules!__macro_def___lazy_static_1_4_0___lazy_static_create{($NAME:ident,$T:ty)=>{static$NAME:$crate::__bundled::__lazy_static_1_4_0::lazy::Lazy<$T> =$crate::__bundled::__lazy_static_1_4_0::lazy::Lazy::INIT;}}
    #[macro_export]macro_rules!__macro_def___lazy_static_1_4_0___lazy_static_internal{($(#[$attr:meta])*($($vis:tt)*)static ref$N:ident:$T:ty=$e:expr;$($t:tt)*)=>{__lazy_static_internal!(@MAKE TY,$(#[$attr])*,($($vis)*),$N);__lazy_static_internal!(@TAIL,$N:$T=$e);lazy_static!($($t)*);};(@TAIL,$N:ident:$T:ty=$e:expr)=>{impl$crate::__bundled::__lazy_static_1_4_0::__Deref for$N{type Target=$T;fn deref(&self)->&$T{#[inline(always)]fn __static_ref_initialize()->$T{$e}#[inline(always)]fn __stability()->&'static$T{__lazy_static_create!(LAZY,$T);LAZY.get(__static_ref_initialize)}__stability()}}impl$crate::__bundled::__lazy_static_1_4_0::LazyStatic for$N{fn initialize(lazy:&Self){let _=&**lazy;}}};(@MAKE TY,$(#[$attr:meta])*,($($vis:tt)*),$N:ident)=>{#[allow(missing_copy_implementations)]#[allow(non_camel_case_types)]#[allow(dead_code)]$(#[$attr])*$($vis)*struct$N{__private_field:()}#[doc(hidden)]$($vis)*static$N:$N=$N{__private_field:()};};()=>()}
    #[macro_export]macro_rules!__macro_def___lazy_static_1_4_0_lazy_static{($(#[$attr:meta])*static ref$N:ident:$T:ty=$e:expr;$($t:tt)*)=>{__lazy_static_internal!($(#[$attr])*()static ref$N:$T=$e;$($t)*);};($(#[$attr:meta])*pub static ref$N:ident:$T:ty=$e:expr;$($t:tt)*)=>{__lazy_static_internal!($(#[$attr])*(pub)static ref$N:$T=$e;$($t)*);};($(#[$attr:meta])*pub($($vis:tt)+)static ref$N:ident:$T:ty=$e:expr;$($t:tt)*)=>{__lazy_static_internal!($(#[$attr])*(pub($($vis)+))static ref$N:$T=$e;$($t)*);};()=>()}
    #[macro_export]macro_rules!__macro_def___proconio_derive_0_2_1_derive_readable{($(_:tt)*)=>(::std::compile_error!("`derive_readable` from `proconio-derive 0.2.1` should have been expanded");)}
    #[macro_export]macro_rules!__macro_def___proconio_derive_0_2_1_fastout{($(_:tt)*)=>(::std::compile_error!("`fastout` from `proconio-derive 0.2.1` should have been expanded");)}
    #[macro_export]macro_rules!__macro_def_proconio_input{(@from[$source:expr]@rest)=>{};(@from[$source:expr]@rest mut$($rest:tt)*)=>{$crate::__bundled::proconio::input!{@from[$source]@mut[mut]@rest$($rest)*}};(@from[$source:expr]@rest$($rest:tt)*)=>{$crate::__bundled::proconio::input!{@from[$source]@mut[]@rest$($rest)*}};(@from[$source:expr]@mut[$($mut:tt)?]@rest$var:tt:$($rest:tt)*)=>{$crate::__bundled::proconio::input!{@from[$source]@mut[$($mut)*]@var$var@kind[]@rest$($rest)*}};(@from[$source:expr]@mut[$($mut:tt)?]@var$var:tt@kind[$($kind:tt)*]@rest)=>{let$($mut)*$var=$crate::__bundled::proconio::read_value!(@source[$source]@kind[$($kind)*]);};(@from[$source:expr]@mut[$($mut:tt)?]@var$var:tt@kind[$($kind:tt)*]@rest,$($rest:tt)*)=>{$crate::__bundled::proconio::input!(@from[$source]@mut[$($mut)*]@var$var@kind[$($kind)*]@rest);$crate::__bundled::proconio::input!(@from[$source]@rest$($rest)*);};(@from[$source:expr]@mut[$($mut:tt)?]@var$var:tt@kind[$($kind:tt)*]@rest$tt:tt$($rest:tt)*)=>{$crate::__bundled::proconio::input!(@from[$source]@mut[$($mut)*]@var$var@kind[$($kind)*$tt]@rest$($rest)*);};(from$source:expr,$($rest:tt)*)=>{#[allow(unused_variables,unused_mut)]let mut s=$source;$crate::__bundled::proconio::input!{@from[&mut s]@rest$($rest)*}};($($rest:tt)*)=>{let mut locked_stdin=$crate::__bundled::proconio::STDIN_SOURCE.lock().expect(concat!("failed to lock the stdin; please re-run this program.  ","If this issue repeatedly occur, this is a bug in `proconio`.  ","Please report this issue from ","<https://github.com/statiolake/proconio-rs/issues>."));$crate::__bundled::proconio::input!{@from[&mut*locked_stdin]@rest$($rest)*}drop(locked_stdin);};}
    #[macro_export]macro_rules!__macro_def_proconio_read_value{(@source[$source:expr]@kind[[$($kind:tt)*]])=>{$crate::__bundled::proconio::read_value!(@array@source[$source]@kind[]@rest$($kind)*)};(@array@source[$source:expr]@kind[$($kind:tt)*]@rest)=>{{let len=<usize as$crate::__bundled::proconio::__Readable>::read($source);$crate::__bundled::proconio::read_value!(@source[$source]@kind[[$($kind)*;len]])}};(@array@source[$source:expr]@kind[$($kind:tt)*]@rest;$($rest:tt)*)=>{$crate::__bundled::proconio::read_value!(@array@source[$source]@kind[$($kind)*]@len[$($rest)*])};(@array@source[$source:expr]@kind[$($kind:tt)*]@rest$tt:tt$($rest:tt)*)=>{$crate::__bundled::proconio::read_value!(@array@source[$source]@kind[$($kind)*$tt]@rest$($rest)*)};(@array@source[$source:expr]@kind[$($kind:tt)*]@len[$($len:tt)*])=>{{let len=$($len)*;(0..len).map(|_|$crate::__bundled::proconio::read_value!(@source[$source]@kind[$($kind)*])).collect::<Vec<_>>()}};(@source[$source:expr]@kind[($($kinds:tt)*)])=>{$crate::__bundled::proconio::read_value!(@tuple@source[$source]@kinds[]@current[]@rest$($kinds)*)};(@tuple@source[$source:expr]@kinds[$([$($kind:tt)*])*]@current[]@rest)=>{($($crate::__bundled::proconio::read_value!(@source[$source]@kind[$($kind)*]),)*)};(@tuple@source[$source:expr]@kinds[$($kinds:tt)*]@current[$($curr:tt)*]@rest)=>{$crate::__bundled::proconio::read_value!(@tuple@source[$source]@kinds[$($kinds)*[$($curr)*]]@current[]@rest)};(@tuple@source[$source:expr]@kinds[$($kinds:tt)*]@current[$($curr:tt)*]@rest,$($rest:tt)*)=>{$crate::__bundled::proconio::read_value!(@tuple@source[$source]@kinds[$($kinds)*[$($curr)*]]@current[]@rest$($rest)*)};(@tuple@source[$source:expr]@kinds[$($kinds:tt)*]@current[$($curr:tt)*]@rest$tt:tt$($rest:tt)*)=>{$crate::__bundled::proconio::read_value!(@tuple@source[$source]@kinds[$($kinds)*]@current[$($curr)*$tt]@rest$($rest)*)};(@source[$source:expr]@kind[])=>{compile_error!(concat!("Reached unreachable statement while parsing macro input.  ","This is a bug in `proconio`.  ","Please report this issue from ","<https://github.com/statiolake/proconio-rs/issues>."));};(@source[$source:expr]@kind[$kind:ty])=>{<$kind as$crate::__bundled::proconio::__Readable>::read($source)}}
};

#[rustfmt::skip]
#[allow(unused)]
pub mod __bundled {
    pub mod __lazy_static_1_4_0{#![no_std]mod __external_local_inner_macros{pub(super)use crate::__bundled::__lazy_static_1_4_0::__macros::*;}use self::__external_local_inner_macros::*;pub mod __macros{use super::__external_local_inner_macros::*;pub use crate::{__macro_def___lazy_static_1_4_0___lazy_static_create as __lazy_static_create,__macro_def___lazy_static_1_4_0___lazy_static_internal as __lazy_static_internal,__macro_def___lazy_static_1_4_0_lazy_static as lazy_static};}pub use self::__macros::*;#[path="inline_lazy.rs"]pub mod lazy{use super::__external_local_inner_macros::*;extern crate core;extern crate std;use self::std::prelude::v1::*;use self::std::cell::Cell;use self::std::hint::unreachable_unchecked;use self::std::sync::Once;#[allow(deprecated)]pub use self::std::sync::ONCE_INIT;pub struct Lazy<T:Sync>(Cell<Option<T> >,Once);impl<T:Sync>Lazy<T>{#[allow(deprecated)]pub const INIT:Self=Lazy(Cell::new(None),ONCE_INIT);#[inline(always)]pub fn get<F>(&'static self,f:F)->&T where F:FnOnce()->T,{self.1.call_once(| |{self.0.set(Some(f()));});unsafe{match*self.0.as_ptr(){Some(ref x)=>x,None=>{debug_assert!(false,"attempted to derefence an uninitialized lazy static. This is a bug");unreachable_unchecked()},}}}}unsafe impl<T:Sync>Sync for Lazy<T>{}}pub use core::ops::Deref as __Deref;pub trait LazyStatic{fn initialize(lazy:&Self);}pub fn initialize<T:LazyStatic>(lazy:&T){LazyStatic::initialize(lazy);}}
    pub mod proconio{#![allow(clippy::needless_doctest_main,clippy::print_literal)]mod __external_local_inner_macros{pub(super)use crate::__bundled::__lazy_static_1_4_0::__macros::*;}mod __pseudo_extern_prelude{pub(super)use crate::__bundled::{__lazy_static_1_4_0 as lazy_static,__proconio_derive_0_2_1 as proconio_derive};}use self::{__external_local_inner_macros::*,__pseudo_extern_prelude::*};pub mod __macros{use super::{__external_local_inner_macros::*,__pseudo_extern_prelude::*};pub use crate::{__macro_def_proconio_input as input,__macro_def_proconio_read_value as read_value};}pub use self::__macros::*;pub use proconio_derive::*;pub mod marker{use super::{__external_local_inner_macros::*,__pseudo_extern_prelude::*};use crate::__bundled::proconio::source::{Readable,Source};use std::io::BufRead;pub enum Chars{}impl Readable for Chars{type Output=Vec<char>;fn read<R:BufRead,S:Source<R> >(source:&mut S)->Vec<char>{source.next_token_unwrap().chars().collect()}}pub enum Bytes{}impl Readable for Bytes{type Output=Vec<u8>;fn read<R:BufRead,S:Source<R> >(source:&mut S)->Vec<u8>{source.next_token_unwrap().bytes().collect()}}pub enum Usize1{}impl Readable for Usize1{type Output=usize;fn read<R:BufRead,S:Source<R> >(source:&mut S)->usize{usize::read(source).checked_sub(1).expect("attempted to read the value 0 as a Usize1")}}pub enum Isize1{}impl Readable for Isize1{type Output=isize;fn read<R:BufRead,S:Source<R> >(source:&mut S)->isize{isize::read(source).checked_sub(1).unwrap_or_else(| |{panic!(concat!("attempted to read the value {} as a Isize1:"," the value is isize::MIN and cannot be decremented"),std::isize::MIN,)})}}}pub mod source{use super::{__external_local_inner_macros::*,__pseudo_extern_prelude::*};use std::any::type_name;use std::fmt::Debug;use std::io::BufRead;use std::str::FromStr;pub mod line{use super::super::{__external_local_inner_macros::*,__pseudo_extern_prelude::*};use super::Source;use std::io::BufRead;use std::iter::Peekable;use std::str::SplitWhitespace;pub struct LineSource<R:BufRead>{tokens:Peekable<SplitWhitespace<'static> >,current_context:Box<str>,reader:R,}impl<R:BufRead>LineSource<R>{pub fn new(reader:R)->LineSource<R>{LineSource{current_context:"".to_string().into_boxed_str(),tokens:"".split_whitespace().peekable(),reader,}}fn prepare(&mut self){while self.tokens.peek().is_none(){let mut line=String::new();let num_bytes=self.reader.read_line(&mut line).expect("failed to get linel maybe an IO error.");if num_bytes==0{return;}self.current_context=line.into_boxed_str();self.tokens=unsafe{std::mem::transmute::<_,&'static str>(&*self.current_context)}.split_whitespace().peekable();}}}impl<R:BufRead>Source<R>for LineSource<R>{fn next_token(&mut self)->Option<&str>{self.prepare();self.tokens.next()}fn is_empty(&mut self)->bool{self.prepare();self.tokens.peek().is_none()}}use std::io::BufReader;impl<'a>From<&'a str>for LineSource<BufReader<&'a[u8]> >{fn from(s:&'a str)->LineSource<BufReader<&'a[u8]> >{LineSource::new(BufReader::new(s.as_bytes()))}}}pub mod once{use super::super::{__external_local_inner_macros::*,__pseudo_extern_prelude::*};use super::Source;use std::io::BufRead;use std::iter::Peekable;use std::marker::PhantomData;use std::str::SplitWhitespace;pub struct OnceSource<R:BufRead>{tokens:Peekable<SplitWhitespace<'static> >,context:Box<str>,_read:PhantomData<R>,}impl<R:BufRead>OnceSource<R>{pub fn new(mut source:R)->OnceSource<R>{let mut context=String::new();source.read_to_string(&mut context).expect("failed to read from source; maybe an IO error.");let context=context.into_boxed_str();let mut res=OnceSource{context,tokens:"".split_whitespace().peekable(),_read:PhantomData,};use std::mem;let context:&'static str=unsafe{mem::transmute(&*res.context)};res.tokens=context.split_whitespace().peekable();res}}impl<R:BufRead>Source<R>for OnceSource<R>{fn next_token(&mut self)->Option<&str>{self.tokens.next()}fn is_empty(&mut self)->bool{self.tokens.peek().is_none()}}use std::io::BufReader;impl<'a>From<&'a str>for OnceSource<BufReader<&'a[u8]> >{fn from(s:&'a str)->OnceSource<BufReader<&'a[u8]> >{OnceSource::new(BufReader::new(s.as_bytes()))}}}pub mod auto{use super::super::{__external_local_inner_macros::*,__pseudo_extern_prelude::*};#[cfg(debug_assertions)]pub use super::line::LineSource as AutoSource;#[cfg(not(debug_assertions))]pub use super::once::OnceSource as AutoSource;}pub trait Source<R:BufRead>{fn next_token(&mut self)->Option<&str>;fn is_empty(&mut self)->bool;fn next_token_unwrap(&mut self)->&str{self.next_token().expect(concat!("failed to get the next token; ","maybe reader reached an end of input. ","ensure that arguments for `input!` macro is correctly ","specified to match the problem input."))}}impl<R:BufRead,S:Source<R> >Source<R>for&'_ mut S{fn next_token(&mut self)->Option<&str>{(*self).next_token()}fn is_empty(&mut self)->bool{(*self).is_empty()}}pub trait Readable{type Output;fn read<R:BufRead,S:Source<R> >(source:&mut S)->Self::Output;}impl<T:FromStr>Readable for T where T::Err:Debug,{type Output=T;fn read<R:BufRead,S:Source<R> >(source:&mut S)->T{let token=source.next_token_unwrap();match token.parse(){Ok(v)=>v,Err(e)=>panic!(concat!("failed to parse the input `{input}` ","to the value of type `{ty}`: {err:?}; ","ensure that the input format is collectly specified ","and that the input value must handle specified type.",),input=token,ty=type_name::<T>(),err=e,),}}}}use crate::__bundled::proconio::source::auto::AutoSource;use lazy_static::lazy_static;use std::io;use std::io::{BufReader,Stdin};use std::sync::Mutex;pub use crate::__bundled::proconio::source::Readable as __Readable;lazy_static!{#[doc(hidden)]pub static ref STDIN_SOURCE:Mutex<AutoSource<BufReader<Stdin>>>=Mutex::new(AutoSource::new(BufReader::new(io::stdin())));}pub fn is_stdin_empty()->bool{use crate::__bundled::proconio::source::Source;let mut lock=STDIN_SOURCE.lock().expect(concat!("failed to lock the stdin; please re-run this program.  ","If this issue repeatedly occur, this is a bug in `proconio`.  ","Please report this issue from ","<https://github.com/statiolake/proconio-rs/issues>."));lock.is_empty()}}
    pub mod __proconio_derive_0_2_1{pub mod __macros{pub use crate::{__macro_def___proconio_derive_0_2_1_derive_readable as derive_readable,__macro_def___proconio_derive_0_2_1_fastout as fastout};}pub use self::__macros::*;}
}
