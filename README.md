# monkey-interpreter

### Description
This is a basic interpreter for a mock programming language called Monkey.

I read and followed along with this book: https://interpreterbook.com/ , which uses Golang to implement an interpreter for Monkey. 
I decided to use Rust instead, as I was wanting to learn a new language and I thought this would be the perfect project to do so.

If you want to see what Monkey source code looks like, take a look in the examples folder. In it you will find some Monkey files that show off what this interpreter is capable of doing.

This interpreter has two ways to use it.
The first way is to simply 
<pre>
  cargo r 
</pre>
This starts the REPL, which is similar in functionality to Python's or NodeJS's REPL in the command line.

The second way is to pass a Monkey file to the interpreter.
<pre>
  cargo r examples/arrays.mky
</pre>
This is just like doing 
<pre>
  python main.py 
</pre>
This just runs the interpreter over the whole source code, instead of line by line in the REPL.

This interpreter does have its own error system. The parser is capable of producing errors for a program. For example the parser can find errors in syntax, such as a missing RPAREN ")" in this if expression:  
<pre>
  if (x > 10           
</pre>
Or a missing ASSIGN "=" in a let statement:
<pre>
  let foo 10;
</pre>

### TODOs
Although this interpreter already mimics all the functionality of the Go version in the book, I decided that I want to add a little more. 
Believe it or not, it is actually pretty easy to add new features like new built in functions or a new mathematical operator. 
TL;DR  The interpreter uses Pratt Parsing (a version of the Recursive Descent Parsing algorithm), which makes it super easy to add new features.

What I want to add: 

"sort" built in function which would be used like so:
<pre>
  let foo = [1, "bar", true, false, "baz", 32];
  let sorted_foo = sort(foo);
</pre>
Notice that the elements in arrays in Monkey can be of a different 'type'. It is wrong to call them 'types', they are just trait objects of the trait "Object" (src/objects/mod.rs). 
In Rust, in order to call .sort() on a 
```rust
Vec<T>
```
then T must have the following traits implemented: Ord, PartialOrd, Eq, PartialEq.
Under the hood, this what Monkey's array looks like: 
```rust
#[derive(Clone, Debug)] 
pub struct Array {
   pub elements: Vec<Box<dyn Object>>,
}
```
As you can see, the elements of an array are a vector of Object trait objects. 
I still need to think of a reasonable ordering scheme in case we want to call sort() on an Array with every differeny trait object of Object.

As for other features, I am sure an idea will pop into my head and I will attempt to add it in.


