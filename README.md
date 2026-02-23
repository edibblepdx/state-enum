# state-enum

Simple state enum.

## Example

```rust
use state_enum::state_enum;

#[state_enum]
enum State {
    Cats,
    Dogs,
    Fish,
}
```

## Expands to

```rust
#[derive(Copy, Clone, Default, PartialEq)]
enum State {
    #[default]
    Cats,
    Dogs,
    Fish,
}

impl State {
    const ALL: [Self; 3] = [Self::Cats, Self::Dogs, Self::Fish];

    pub fn next(self) -> Self {
        Self::ALL[(self as usize + 1) % 3]
    }

    pub fn prev(self) -> Self {
        Self::ALL[(self as usize + 2) % 3]
    }
}
```
