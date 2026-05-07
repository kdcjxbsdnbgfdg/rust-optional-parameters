Adds optional parameters to rust. The syntax is kind of like python, or the C macro approach to optional parameters. <br/>
This implementation has a ton of flaws. It relies upon a hack to even work, as rusts macro hygiene makes this otherwise impossible. <br/>
I wouldnt really recommend actually using this in your programs, there are a lot of very valid reasons to dislike optional parameters. <br/>
Rust as a language is generally against things being implicit. Casts are explicit, i cant think of any other examples, etc, etc. <br/>
I made this because I wanted to make a cool project. <br/>

### Example Code
```rust
#[default_params(rhs = 10)]
fn testFunction(lhs: u32, rhs: u32) {
  return lhs + rhs;
}

testFunction!(10) // returns 10 + 10 (which is 20)
testFunction!(10, .rhs = 20) // returns 10 + 20 (which is 30)
```

I found out, only after publishing this, that someone else did the exact same thing as me already. <br/>
Their solution is almost identical to mine, using a proc macro to generate a regular macro. <br/>
However, theirs has a bit of a major problem, you can only write the optional arguments in the same order as theyre declared. <br/>
If you want to bypass this, they allow you to use a shuffle attribute, however enabling this option generates 1 macro branch for each possible ordering.<br>
This means that the macro will have to generate n! macro branches for a function with n optional arguments.<br>
I hope I dont have to explain why you dont want O(n!) compile time in your projects, for example 8 factorial is 40,320.<br/>
<br/>
My version avoids this problem by initialising variables, and then overriding them with the macro input.<br/>
The downside to this approach is that for now im using a trick to bypass macro hygiene (why doesnt rust have a way to turn off macro hygiene??).
I am later going to try to change this.
<br/>
My version does not YET allow you to rename the output macro.<br/>
