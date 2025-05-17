#![no_std]

/*

BSD 3-Clause License

Copyright (c) 2025, Isaac Budzik

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

 */

//! cxx_mangle
//!
//! Mangle names of functions for the ability to have a C++ ABI whilst allowing for overloading
//!
//! Limitations
//! 
//! - only supports types that Rust supports
//!
//! - does not support pointers
//!
//! - does not check for validity

extern crate alloc;

use core::{
    write,
    fmt::Write,
};
use alloc::string::String;
use alloc::vec::Vec;

/// Non exhaustive set of parameter types
#[derive(Clone, Copy, PartialEq, Debug)]
#[non_exhaustive]
pub enum Type {
    /// char
    Char, // c
    /// signed char
    SChar, // a
    /// double
    Double, // d
    /// float
    Float, // f
    /// __float128
    Float128, // g
    /// unsigned char
    UChar, // h
    /// int
    Int, // i
    /// unsigned int
    UInt, // j
    //Const, // K
    /// long
    Long, // l
    /// unsigned long
    ULong, // m
    /// __int128
    Int128, // n
    /// unsigned __int128
    UInt128, // o
    //Pointer, // p,
    /// short
    Short, // s
    /// unsinged short
    UShort, // t
    /// void (no parameters)
    Void, // v,
    //Volatile, // V
    /// wchar_t
    WChar, // w,
    /// long long
    LLong, // x,
    /// unsigned long logn
    ULLong, // y
    /// ... (variadic)
    Ellipsis // z
}

#[derive(Clone, PartialEq, Hash, Debug)]
#[non_exhaustive]
enum Scope {
    Unscoped(String),
    Nested(Vec<String>), //N ... E,
}

impl Scope {
    fn new(n: String) -> Self {
	if n.contains("::") {
	    let mut v = Vec::new();
	    n.split("::").collect::<Vec<&str>>().iter().for_each(|e| v.push(String::from(*e)));
	    Self::Nested(v)
	} else {
	    Self::Unscoped(n)
	}
    }

    fn mangle(&self, t: &[Type]) -> String {
	let mut s = String::from("_Z");
	match self {
	    Scope::Unscoped(st) => {
		let mut l = String::new();
		let _ = write!(l, "{}", st.len());
		s.push_str(&l);
		s.push_str(st);
	    },
	    Scope::Nested(b) => {
		s.push('N');
		for i in b {
		    let mut l = String::new();
		    let _ = write!(l, "{}", i.len());
		    s.push_str(&l);
		    s.push_str(i);
		}
		s.push('E');
	    }
	}
	for i in t {
	    s.push(match *i {
		Type::SChar => 'a',
		Type::Double => 'd',
		Type::Float => 'f',
		Type::Float128 => 'g',
		Type::UChar => 'h',
		Type::Int => 'i',
		Type::UInt => 'j',
		//Type::Const => 'K',
		Type::Long => 'l',
		Type::ULong => 'm',
		Type::Int128 => 'n',
		Type::UInt128 => 'o',
		//Type::Pointer => 'p',
		Type::Short => 's',
		Type::UShort => 't',
		Type::Void => 'v',
		//Type::Volatile => 'V',
		Type::WChar => 'w',
		Type::LLong => 'x',
		Type::ULLong => 'y',
		Type::Ellipsis => 'z',
		Type::Char => 'c'
	    });
	}
	s
    }
}

/// Function to mangle
pub struct Func {
    scope: Scope,
    params: Vec<Type>
}

impl Func {
    /// create function to mangle
    ///
    /// name - full C++ name of function
    ///
    /// params - parameters of function
    pub fn new(name: String, params: Vec<Type>) -> Self {
	Self {
	    scope: Scope::new(name),
	    params
	}
    }

    /// Mangle function according to Itanium C++ ABI
    pub fn mangle(&self) -> String {
	self.scope.mangle(self.params.as_slice())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn mangle_unscoped() {
	use super::{
	    Type,
	    Func
	};
	use alloc::string::String;
	use alloc::vec;
	let f1 = Func::new(String::from("func"), vec![Type::Void]);
	let f2 = Func::new(String::from("func"), vec![Type::Int]);
	assert_eq!(&f1.mangle(), "_Z4funcv");
	assert_eq!(&f2.mangle(), "_Z4funci");
    }

    #[test]
    fn mangle_nested() {
	use super::{
	    Type,
	    Func
	};
	use alloc::string::String;
	use alloc::vec;
	let f = Func::new(String::from("myNamespace::inner::func"), vec![Type::Int, Type::Double, Type::Ellipsis]);
	let h = Func::new(String::from("hello::world"), vec![Type::Void]);
	assert_eq!(&f.mangle(), "_ZN11myNamespace5inner4funcEidz");
	assert_eq!(&h.mangle(), "_ZN5hello5worldEv");
    }
}
