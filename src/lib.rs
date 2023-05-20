/*!
## Quick Start Guide

Implement this trait to provide a `Display` appropriate label for your type.

This trait can be derived using a single helper attribute:
```
# #[cfg(not(feature = "derive"))]
# use type_label::impl_label;
# use type_label::Label;
# #[cfg(feature = "derive")]
#[derive(Label)]
#[label = "Label goes here"]
pub struct Foo {}
# #[cfg(not(feature = "derive"))]
# pub struct Foo {}

# #[cfg(not(feature = "derive"))]
# impl_label!(Foo, "Label goes here");

fn main() {
    println!("{}", Foo::LABEL);
    // prints "Label goes here"
}
```

## Motivation

Generally, this trait is useful to provide a label for your types that is suitable for human users to read.

You can think of it as a const alternative to Display. The label of a type is entirely static.

An `AnimalTypes` enum might implement `Display` to display to the user any specific variant: "Bear" say,
but would implement `Label` to display to the user `AnimalTypes` _as a concept_: "type of Animal".

To put it in other words:
`Label` is to `Display`, what "class" is to "instance"

* Display prints a specific instance of a type, using its runtime data.
* Label prints the type itself, with no runtime data at all.

## Detailed Example

Let's look at a detailed example, our use case will be some boilerplate for Error Handling
encouraged/required by libraries such as [error-stack](https://docs.rs/error-stack/latest/error_stack/#crate-philosophy).

```
# #[cfg(not(feature = "derive"))]
# use type_label::impl_label;
# #[allow(dead_code)]
# use std::error::Error;
// Some types our api involves.
// there could be many more, but that's enough for this example.
pub enum ActivityType {
    Handoff,
    Invoke,
    Message,
}

pub enum InputHint {
    AcceptingInput,
    ExpectingInput,
    IgnoringInput,
}

pub enum TextFormatType {
    Markdown,
    Plain,
    Xml,
}
```

We will parse these types from strings

Let's define error types for every case!

but to quote error-stack's philosophy:

> "This crate adds some development overhead in comparison to other error handling strategies"

What's a lazy rust programmer to do...?

Define one error for parsing, generic over what is being parsed.

```
# #[cfg(not(feature = "derive"))]
# use type_label::impl_label;
# use type_label::Label;
# #[cfg(feature = "derive")]
#[derive(Debug)]
pub struct ParseError<T: Label> {
    // ...
#    _marker: std::marker::PhantomData<T>,
}
# #[cfg(not(feature = "derive"))]
# pub struct ParseError<T: Label> {
#     // ...
#    _marker: std::marker::PhantomData<T>,
# }
```

Hmm... but how to impl Display...?
```
# use std::fmt;
# pub struct ParseError<T>(T);
impl<T> fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "there was an error parsing... T") // :(
    }
}
```
We could make `T: Display`, but if we *haven't parsed a T* how can we rely on T's implementation of Display?

**Enter type_label**
```
# #[cfg(not(feature = "derive"))]
# use type_label::impl_label;
# use std::error::Error;
# use std::str::FromStr;
# use std::marker::PhantomData;
# use std::fmt::{self, Display};
# use derivative::Derivative;
use type_label::Label;

# #[cfg(feature = "derive")]
#[derive(Label)]
#[label = "activity type"]
pub enum ActivityType {
    Handoff,
    Invoke,
    Message,
}
# #[cfg(not(feature = "derive"))]
# pub enum ActivityType {
#     Handoff,
#     Invoke,
#     Message,
# }
# #[cfg(not(feature = "derive"))]
# impl_label!(ActivityType, "activity type");

// we adjust the ParseError type, to require T: Label
# #[derive(Derivative)]
# #[derivative(Debug)]
pub struct ParseError<T: Label> {
    // we need this this marker to appease the compiler because we aren't "using" T
#     #[derivative(Debug="ignore")]
    _marker: std::marker::PhantomData<T>,
}

// we can provide a new() method to avoid dealing with PhantomData at call site
impl<T: Label> ParseError<T> {
    fn new() -> Self {
        ParseError {
            _marker: PhantomData::<T>,
        }
    }
}

// we use label in our Display impl
impl<T: Label> Display for ParseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing {}", <T as Label>::LABEL)
    }
}

// let rust know our type is an Error
impl<T: Label> Error for ParseError<T> {}

// Finally, we are ready to return ParseError when something goes wrong parsing our type :)
impl FromStr for ActivityType {
    type Err = ParseError<ActivityType>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "handoff" => Ok(ActivityType::Handoff),
            "invoke" => Ok(ActivityType::Invoke),
            "message" => Ok(ActivityType::Message),
            _ => Err(ParseError::new()), // oh the sweet, sweet type inference! :)
        }
    }
}
```

*/

#[cfg(feature = "derive")]
pub use derive::Label;

/// Define a compile-time string label for your type
pub trait Label {
    /// The label your type should have. It's completely static
    const LABEL: &'static str;

    fn type_label(&self) -> &'static str {
        Self::LABEL
    }
}

/**
An alternative macro_rules macro for implementing [`Label`].

Using this and disabling default features will avoid proc_macros, which will help compile times.

# Examples

usage:
```
#[macro_use] extern crate type_label;
// alternatively: use type_label::impl_label;

pub struct Foo {}

impl_label!(Foo, "foo label");
```
*/
#[macro_export]
macro_rules! impl_label {
    ($struct:ident, $label:literal) => {
        impl ::type_label::Label for $struct {
            const LABEL: &'static str = $label;
        }
    };
}
