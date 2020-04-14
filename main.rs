// MIT License
// 
// Copyright (c) 2019 Barry DeZonia
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub trait Zero<U> {
	fn is_zero(&self, &U) -> bool; // is zero
	fn zero(&self, &mut U);  // set to zero
}

pub trait Unity<U> {
	fn is_unity(&self, &U) -> bool; // is one
	fn unity(&self, &mut U); // set to one
}

pub trait Equality<U> {
	fn is_eq(&self, a: &U, b: &U) -> bool;
	fn is_not_eq(&self, a: &U, b: &U) -> bool;
}

pub trait Ordered<U> {
	fn is_less(&self, a: &U, b: &U) -> bool;
	fn is_less_eq(&self, a: &U, b: &U) -> bool;
	fn is_greater(&self, a: &U, b: &U) -> bool;
	fn is_greater_eq(&self, a: &U, b: &U) -> bool;
}

pub trait Constructible<U> {
	fn ctor(&self) -> U;
	fn ctor_from_ref(&self, other: &U) -> U;
	fn ctor_from_str(&self, string: &str) -> U;
}

pub trait Assignable<U> {
	fn assign(&self, a: &U, b: &mut U); // a -> b
}

pub trait Addition<U> {
	fn double(&self, a: &mut U); // a + a -> a
	fn add2b(&self, a: &U, b: &mut U); // a + b -> b
	fn add3c(&self, a: &U, b: &U, c: &mut U); // a + b -> c
	fn subtract2a(&self, a: &mut U, b: &U); // a - b -> a
	fn subtract2b(&self, a: &U, b: &mut U); // a - b -> b
	fn subtract3c(&self, a: &U, b: &U, c: &mut U); // a - b -> c
}

pub trait Multiplication<U> {
	fn square(&self, a: &mut U); // a * a -> a
	fn multiply2a(&self, a: &mut U, b: &U); // a * b -> a
	fn multiply2b(&self, a: &U, b: &mut U); // a * b -> b
	fn multiply3c(&self, a: &U, b: &U, c: &mut U); // a * b -> c
	fn power(&self, n: i32, a: &mut U); // a ^ n -> a
	fn power2b(&self, n: i32, a: &U, b: &mut U); // a ^ n -> b
}

pub trait Negatable<U> {
	fn negate(&self, a: &mut U); // -a -> a
	fn negate2b(&self, a: &U, b: &mut U); // -a -> b
}

pub trait Invertible<U> {
	fn invert(&self, a: &mut U); // 1 / a -> a
	fn invert2b(&self, a: &U, b: &mut U); // 1 / a -> b
	fn divide2a(&self, a: &mut U, b: &U); // a / b -> a
	fn divide2b(&self, a: &U, b: &mut U); // a / b -> b
	fn divide3c(&self, a: &U, b: &U, c: &mut U); // a / b -> c
}

// an example data structure 

pub trait IndexedDataSource<U> {
	fn size(&self) -> u64;
	fn set(&self, idx: u64, value: &U);
	fn get(&self, idx: u64, value: &mut U);
}

// an example algorithm: transform all values in an IndexedDataSource

pub fn transform_list<T: Constructible<U> + Multiplication<U>, U>
	(
	  alg: &T,
	  xform: &dyn Fn(&mut U), 
	  list: &IndexedDataSource<U>
	)
{
	let mut tmp: U = alg.ctor();
	for i in 0 .. (list.size()-1) {
		list.get(i, &mut tmp);
		xform(&mut tmp);
		list.set(i, &tmp);
	}
}

pub struct SInt4Algebra {
}

pub struct SInt4 {
	val: i8,
}

impl Assignable<SInt4> for SInt4Algebra {
	
	fn assign(&self, a: &SInt4, b: &mut SInt4) {
		b.val = a.val;
	}

}

fn wrap_sint4(a: &mut SInt4) {
	if a.val > 7 {
		a.val = a.val - 16;
	}
	else if a.val < -8 {
		a.val = a.val + 16;
	}
}

impl Addition<SInt4> for SInt4Algebra {

	fn double(&self, a: &mut SInt4) {
		a.val = a.val + a.val;
		wrap_sint4(a);
	}
	
	fn add2b(&self, a: &SInt4, b: &mut SInt4) {
		b.val = a.val + b.val;
		wrap_sint4(b);
	}
	
	fn add3c(&self, a: &SInt4, b: &SInt4, c: &mut SInt4) {
		c.val = a.val + b.val;
		wrap_sint4(c);
	}
	
	fn subtract2a(&self, a: &mut SInt4, b: &SInt4) {
		a.val = a.val - b.val;
		wrap_sint4(a);
	}
	
	fn subtract2b(&self, a: &SInt4, b: &mut SInt4) {
		b.val = a.val - b.val;
		wrap_sint4(b);
	}
	
	fn subtract3c(&self, a: &SInt4, b: &SInt4, c: &mut SInt4) {
		c.val = a.val - b.val;
		wrap_sint4(c);
	}
}

fn main() {
	let x = SInt4{ val: 4 };
	let mut y = SInt4{ val: 2 };
	let alg = SInt4Algebra{};
	alg.add2b(&x, &mut y);
	println!("{}", y.val);
}
