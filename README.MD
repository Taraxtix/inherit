# inherits

Improved Practices for the Composition Pattern in Rust.

This crate provides two proc-macro attributes that automatically generate `Deref` and `DerefMut` implementations, making the composition pattern and newtype pattern more ergonomic.

## Install

Add `inherits` to your `Cargo.toml`:

```toml
[dependencies]
inherits = "0.1"
```

## Usage

### `#[inherits]` — Composition via Deref

Annotate a struct with `#[inherits(BaseType)]` to inject a `parent: BaseType` field and auto-derive `Deref` / `DerefMut`. All of the parent's methods and fields become directly accessible on the child.

```rust
use inherits::inherits;

struct Animal {
    name: String,
}

impl Animal {
    fn speak(&self) -> String {
        format!("{} makes a sound", self.name)
    }
}

#[inherits(Animal)]
struct Dog {
    breed: String,
}

impl Dog {
    fn bark(&self) -> String {
        format!("{} the {} barks!", self.name, self.breed)
    }
}

fn main() {
    let dog = Dog {
        parent: Animal {
            name: "Rex".into(),
        },
        breed: "German Shepherd".into(),
    };

    println!("{}", dog.speak()); // Rex makes a sound
    println!("{}", dog.bark());  // Rex the German Shepherd barks!
}
```

When parent and child have fields with the same name, disambiguate with type coercion:

```rust
let parent_name = &(&dog as &Animal).name;
let child_name  = &dog.name;
```

### `#[new_type]` — Newtype Pattern

Annotate a single-field tuple struct to auto-derive `Deref` / `DerefMut` targeting the inner type.

```rust
use inherits::new_type;

#[new_type]
struct Meters(f64);

impl Meters {
    fn to_centimeters(&self) -> f64 {
        self.0 * 100.0
    }
}

fn main() {
    let distance = Meters(2.5);
    println!("{}", distance.sqrt());          // f64::sqrt
    println!("{}", distance.to_centimeters()); // 250.0
}
```

### How It Works

Both macros expand to handwritten-style `Deref` / `DerefMut` impls at compile time with zero runtime overhead.

| Macro | Struct Kind | Injected Field | Deref Target |
|-------|-------------|:---:|---|
| `#[inherits(T)]` | named fields | `parent: T` | `T` |
| `#[new_type]` | tuple (1 field) | *(none)* | inner type |
