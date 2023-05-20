## Quick Start Guide

Implement `Label` to provide a `Display` appropriate label for your type.

`Label` can be derived using a single helper attribute:
```rust
#[derive(Label)]
#[label = "Label goes here"]
pub struct Foo {}

fn main() {
    println!("{}", Foo::LABEL);
    // prints "Label goes here"
}
```

## Motivation

Generally, this trait is useful to provide a label for your types that is suitable for human users to read.

You can think of it as a const alternative to `Display`. The label of a type is entirely static.

An `AnimalTypes` enum might implement `Display` to display to the user any specific variant: "Bear" say,
but would implement `Label` to display to the user `AnimalTypes` _as a concept_: "type of Animal", say.

To put it in other words:
`Label` is to `Display`, what "class" is to "instance".

* Display prints a specific instance of a type, using its runtime data.
* Label prints the type itself, with no runtime data at all.

## Detailed Example

Let's look at a detailed example, our use case will be some boilerplate for Error Handling
encouraged/required by libraries such as [error-stack](https://docs.rs/error-stack/latest/error_stack/#crate-philosophy).

```rust
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

```rust
use type_label::Label;

#[derive(Debug)]
pub struct ParseError<T: Label> {
    // ...
}
```

Hmm... but how to impl Display...?
```rust
impl<T> fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "there was an error parsing... T") // :(
    }
}
```
We could make `T: Display`, but if we *haven't parsed a T* how can we rely on T's implementation of Display?

**Enter type_label**
```rust
use type_label::Label;

#[derive(Label, Debug)]
#[label = "activity type"]
pub enum ActivityType {
    Handoff,
    Invoke,
    Message,
}

// we adjust the ParseError type, to require T: Label
pub struct ParseError<T: Label> {
    // we need this this marker to appease the compiler because we aren't "using" T
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

// Here it is in action
let error = "handof".parse::<ActivityType>().unwrap_err();
assert_eq!(&format!("{error}"), "Error parsing activity type");
```

## Alternatives?

I had a look and found these, none of which particularly suited my needs.

* [label](https://lib.rs/crates/label) provides a way to kind of tag functions so that you can group them and iterate over them. Pretty cool! Nothing like this crate.
* [tynm](https://lib.rs/crates/tynm) a variation of `std::any::type_name` which is automatically available but not fully customizable
* [name_of](https://lib.rs/crates/nameof) allows printing the name of things as written in source code: mainly intended for debugging purposes
* [type_description](https://lib.rs/crates/type_description) is able to automatically generate descriptions of your types (and their fields!) from their names and doc comments. It can also generate machine readable JSON.

Let me know if you are aware of others, I could easily have missed some!

`type_label` fits in as a simple solution that requires manual labelling of your types.

In fact, that's probably the best way to implement Label for third party types.
I don't currently have any plans to provide implementations of this trait for third party crates.

Feel free to raise an issue with implementation requests, **but fair warning I will most likely ignore them for longer than is comfortable for everyone involved**.

## Is this crate abandoned?

Prempting this question! It's a really simple crate, there's absolutely nothing I need to change (he says confidently...).
Please judge abandoment by unresponded to issues rather than frequency of code changes.

If there are no issues then I'm probably still the only person using it `:')`
