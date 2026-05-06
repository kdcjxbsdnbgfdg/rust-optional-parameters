Adds optional parameters to rust. The syntax is kind of like python, or the C macro approach to optional parameters. <br/>
This implementation has a ton of flaws. It relies upon a hack to even work, as rusts macro hygiene makes this otherwise impossible. <br/>
I wouldnt really recommend actually using this in your programs, there are a lot of very valid reasons to dislike optional parameters. <br/>
Rust as a language is generally against things being implicit. Casts are explicit, i cant think of any other examples, etc, etc. <br/>
I made this because I wanted to make a cool project. <br/>

### Example Code
```rust
#[default_params(rhs = 10)]
fn testFunction(arg1: u32, arg2: &str) {
  return lhs + rhs;
}

testFunction!(10) // returns 10 + 10 (which is 20)
testFunction!(10, .rhs = 20) // returns 10 + 20 (which is 30)
```
