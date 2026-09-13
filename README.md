Why you should use Rust? It's not why you think
===============================================

> See the slides at https://fios-quest.com/not-why-you-think/

Many things are banded around as great reasons to chose Rust, and they're not wrong... they're just not particularly 
strong reasons.

It's fast... but C++ and Zig are about the same speed and C is faster still.

It's memory safe... well so are JavaScript, Go, Java, PHP, Python, if it's got a garbage collector, it's safe.

It's fast _and_ memory safe... true, but are you doing something where that really matters?

For example, Rust is 4x faster than JavaScript... but if you're making a request to a webserver, then in all likelihood,
this is going to trigger calls to databases or other web resources and the bulk of your response time won't be on the
server at all.

Even if it is, even if you return a response to the user 4x faster, is a user going to notice a difference that's 
measured in milliseconds?

The hype around Rusts speed and memory safety aren't "nothing", and there are use cases where they are defining 
features... but most of the time, they're really not that important.

---

Its Boring
----------

Rust is boring.

Or, at least, it's very unsurprising.

Take this TypeScript

```typescript
const getPokemon = async (id: number): Promise<Result<Pokemon, Error>> => {
    const response = await fetch(`https://pokeapi.co/api/v2/pokemon/${id}`);
    const pokemon = await response.json();
    return pokemon.id === id
        ? Result.ok(pokemon)
        : Result.error(new Error("Incorrect Pokémon returned"));
}
```

It loads a Pokémon for the given id from the pokeapi. It even checks that the returned Pokémon is correct and returns
a Result type which, if you're not familiar with Result types, forces the caller to deal with the fact this function
may error.

This function has 3 surprises. Actually, this function has 4 surprises if you include the repeated surprise and may have
more if there's stuff I haven't noticed.

It's valid TypeScript, and let's say we have an integration test that checks that when you give it an id of 25 then it
returns Pikachu. We have CI that runs all the tests, linting, styling... so what's wrong with it.

Show of hands, lets see if we can find at least the main 3 issues, and if it helps, let me show you the same function in
Rust, and that should hopefully give you your first hint.

```rust
async fn get_pokemon(id: NonZero<u16>) -> Result<User, GetPokemonError> {
    let pokemon: Pokemon = get(format!("https://pokeapi.co/api/v2/pokemon/{id}")).await?
        .json().await?;
    if pokemon.id == id {
        Ok(pokemon)
    } else {
        Err(GetPokemonError::IncorrectPokemonReturned)
    }
} 
```

So the first problem is JavaScript's single `number` type. We can send the function a bunch of numbers that make 
absolutely no sense, and the function will accept them and try to use them on the API even though we should know before
we call it that they're not going to work.

Not only could we give it zero, negative numbers, or numbers that are not whole numbers, -0, Infinity and NaN are all 
completely valid and complete nonsense.

The `NonZero<u16>` type means the only `404` you're going to trigger is for having a number that's too big, and you may
want to allow for that anyway for flexibility when the next game inevitably comes out.

Speaking of `404`s, any time you fetch data, that request could fail for a whole variety of reasons. The signature of
our function suggests that a user calling it should expect a Result type with an error if the function fails, but
instead we throw an exception... or the fetch does.

Most of us are probably pretty used to exceptions, and you may never have really thought about how bad a pattern they
actually are. We've failed to handle an error here, either by laziness or forgetfulness, which means we've abdicated
responsibility for fault-tolerant code and made it someone else's problem.

That's pretty shitty behavior when you think about it.

You might wonder how Rust is doing anything different here, and it's actually quite subtle. Rust does not have
exceptions. If you're function can fail, you must return a Result, so the `get` function is returning a Result after 
it's awaited, and that little question mark is checking the if we got an Error back, and if we did it automatically
maps the returned error to a `GetPokemonError` and returns that.

This only works if we have written that mapping code, Rust won't let you compile this without it, but that mapping code
can live with the rest of the `GetPokemonError` code, is reusable, and is not getting in the way of understanding this
function.

You can see the same thing for taking the `json` response and parsing into the Pokemon type. If the response isn't valid
JSON we get another thrown exception in the TypeScript but in the Rust we can turn it into a proper error and return
the result.

So that's two surprises and our pseudo duplicate, what's the last thing?

The final big surprise relates to how TypeScript generally reasons about parsed data. For the non-TypeScript devs,
TypeScript has two types for ambiguous data, `any` and `unknown`. `unknown` does not match any other type so you need
to use a type predicate or something similar to check that the thing is what you think it is at runtime. This is
actually a fairly new feature, and was only introduced in TypeScript 3... before that we had `any`. Unfortunately, `any`
doesn't mean the type _could_ be anything... it means this type _is_ anything. 

`any` types match all types, and we haven't actually checked that the thing we expected has been returned. It could be
we've typo'd the API and hit the wrong endpoint, or maybe they've updated the API, and it doesn't return what we expect
anymore... so long as the thing returned has an `id`, we'll return it as if it is a Pokémon, and then we're going to
have a really weird and hard to understand bug somewhere else in our code.

Rust won't let you compile code that's like this, we'll actually get an error when we try to parse the json if it
can't be deserialized into our `Pokemon` type.

So all of that is to say, Rust has an extremely robust error handling system that holds your hand and makes sure you can
see all the places your code might go wrong and think about how you'd like to deal with it.

This example might seem contrived but trust me, once you've started working like this it's really painful to go back.

Speed
-----

Wait, I said we don't need worry about Speed. Thats true.

The owner of the first software company I worked for once said CPUs are cheaper than Devs. A little inefficiency is
not as expensive as a developer wasting lots of time trying to fix it.

Rust isn't just fast on the machine, it's fast to work with.

The robust type system means you're much more likely to write the correct code the first time and not have to faff
around and bug hunt later. It may take you slightly longer to get something that looks "done" but only if you don't
count fixing it later.

But what about small projects we don't care so much about.

Lets assume we want to make a little JavaScript tool, it's not going out to users but its important enough we to do a
reasonable job of it.

We've already got node, so we need to:
1. Install and configure TypeScript
2. Install and configure a linter
3. Install and configure a style checker 
4. Install and configure a testing framework
5. Fiddle with all your configurations until your tests and build all play nice together.

Perhaps I'm really slow, but all that configuring and the fact I forget and need to google everything and everytime I
do this... this usually takes me an entire hour.

That's an hour before I can even start thinking about code.

With Rust I do `cargo new` and I get all of that... plus one more thing we'll come back to later... out of the box for
free, and it all works nicely together without any effort.

And that tooling, folks, oh my god, the default tooling is so good.

Tooling
-------

Obviously we don't need a separate type system but its worth talking about how much the compiler holds your hand. The
compiler will almost never tell you that you can't do something or worse give you an incredibly cryptic linker error for
some simple mistake, thank-you C++ I can never get that time back.

The compiler will usually go into detail about what specifically is wrong with your code and often times will make
suggestions on how to fix it.

Our old friend Tris [has a video](https://www.youtube.com/watch?v=CJtvnepMVAU) where he writes hello world in JavaScript
and the Rust compiler steps him through every change he needs to make to turn it into valid Rust.

I don't know if any other language is that helpful.

The formatter is really nothing to write home about, you use `cargo fmt` and it does its thing. Most IDEs will do this
for you on save. You _can_ configure the formatter to do different things, but the nice thing about it coming out of the
box with a sensible default configuration is that people rarely touch it, so almost all Rust looks the same. There's no
cognitive overhead of jumping between projects and trying to work out how this project wants you to write closures now.

I'm looking at you AirBnB.

Testing similarly is simple. You typically place tests by the code being tested. We use conditional compilation so the
tests only appear when doing a test build.

Tests are functions marked with a test attribute, and there's a handful of assertion macros.

```rust
pub fn add_one(n: u32) -> u32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
    }
}
```

Boring, but in a good way.

There are a ton of libraries that extend testing if thats what you need, but honestly, I never use them.

The linter is where things are a bit more exciting. `Clippy`, yes, named after _that_ clippy, will go to extreme levels
of helping you write perfect Rust. It'll spot things that will make unnecessary copies in memory, or, conversely, advise
you to swap single letter string slices to character types in certain situations because it removes a reference hop.

Clippy also comes with sensible defaults which are great for newbies, learning the ropes, but you'll also see try hard's
*cough* turning clippy to hard mode to make sure they've picked up every potential improvement.

```rust
let arr = [0.0, 0.1, 0.2, 0.3];
let average = arr.iter().sum::<f64>() / arr.len() as f64;
```

For example, casting the length of an array to a 64bit float is almost always fine... but what if your array has more 
than 2^52 elements in it.

Clippy pedantic will advise against it. It's a bit hardcore, but it makes you think about the possible rammifications
of the choices you're making. I'll usually leave that lint on, and then write the reason I'm ok with it into the 
exception. Now when I or someone else next looks at the code, we can quickly see and assess the risk

```rust
let arr = [0.0, 0.1, 0.2, 0.3];
#[expect(clippy::cast_precision_loss, reason = "accuracy isn't necessary")]
let average = arr.iter().sum::<f64>() / arr.len() as f64;
```

And finally there's that last tool I alluded to, the thing that _really_ speeds you up.

Documentation
-------------

Documentation is a pain in the arse... usually.

But it's a really important way to communicate to others, or simply remember for ourselves, how everything works.

In Rust, Documentation is a first class citizen.

Documentation (for the most part) is added before the thing being documented in comments with three slashes. So far, so
normal.

But we can then run `cargo doc`, and it will create a website version of the documentation for us, so that's handy...
but on its own it's not really a big productivity win.

However, the fact everyone uses the same documentation tool means there's a consistency to everyones documentation. In
fact, all libraries uploaded to crates.io, automatically have their documentation published on docs.rs. Again, this is
just a nice reduction in unnecessary cognitive load.

But the real trick happens when you write examples.

You'll notice that examples in Rust documentation usually include assertions to help you understand how things behave. 

Taking our `add_one` function from earlier, we might document it like this, but this documentation is subtly wrong

```rust
/// Adds one to a number
/// 
/// ```
/// use example::add_one;
/// 
/// assert_eq!(add_one(1), 3);
/// ```
pub fn add_one(n: u32) -> u32 {
    n + 1
}
```

But examples run when you run your tests.

This makes it really hard to make mistakes in your documentation. 

As a bonus, this counts for coverage, so you don't need to duplicate all your tests, you just write your documentation
and cover off any weird cases you want to have separately.

Conclusion
----------

Yes, Rust is fast and memory safe, but those are just nice extras.

The real strengths of Rust are:

1. There are no surprises
2. You can build really fast
3. The tooling lets you focus on the things that matter

So maybe I've convinced you to give it a try... what can you use it for.

Literally anything

This presentation is built in RevealJS... inside of Dioxus, a Rust UI framework that works on Web, Desktop and Mobile.

Rust can be used for basically anything, and for most things, I'd argue that its better than even specialist tools.

There is still a bit of a gap for libraries in some places but  
