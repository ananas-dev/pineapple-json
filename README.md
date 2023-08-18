# Pineapple Json

A small json library written in rust using parser combinators.

## How to use

```rust
use pineapple_json::parser::json_value;

fn example() -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: JsonValue = json_value(data)?;

    // Print the json tree
    println!("{:?}", v);

    Ok(())
}
```

## Known issues

- Can stack overflow if you try to parse a long sequence of `[`
- No support for unicode characters
