## Malbolge interpreter in Rust

('&%:9]!~}|z2Vxwv-,POqponl$Hjig%eB@@>}=<M:9wv6WsU2T|nm-,jcL(I&%$#"
`CB]V?Tx<uVtT`Rpo3NlF.Jh++FdbCBA@?]!~|4XzyTT43Qsqq(Lnmkj"Fhg${z@>

### What on earth?

That right there is a Hello World in Malbolge.

What you are looking at is a language which is something worse than all languages out there. It's more esoteric than Brainfu*k and it is even harder to read and write than its friend Befunge.

Malbolge is a self-modifying programming language that is made to be deliberately difficult to program in and close to impossible to read. You could easily mistake it for a crypto-system although some researchers have made the case for it. But you probably could not have determined that this code alone prints out 'Hello!'

```
(=<`#9]76Z{z2V0/S-Qr*)M:,+*)('&%$#"!~}|{z(Kw%$t"Vq0iAm,,j<h'`%

Hello!
```
This implementation is written in pure Rust and a port from the reference C and Java implementations and is able to execute programs written in Malbolge.


This is a Malbolge implementation of the UNIX `cat` program.
````
(=BA#9"=<;:3y7x54-21q/p-,+*)"!h%B0/.
~P<
<:(8&
66#"!~}|{zyxwvu
gJ%
````

There is also a macro available to pass in a raw Malbolge binary using a raw string literal.

This is the same program that prints Hello!.

```rust
malbolge!(r#"(=<`#9]76Z{z2V0/S-Qr*)M:,+*)('&%$#"!~}|{z(Kw%$t"Vq0iAm,,j<h'`%"#);
```

See the examples for more non-trivial malbolge programs.

### Programming in Malbolge

Well I don't know why anyone would want to do this, but I guess you really want to know so here it goes.

It is known for years that Malbolge is close to impossible to program in. The first valid program in Malbolge was found via **trial and error** for 8 years with brute-force until the above 'Hello World' program was able to run.

### Why?

Because I simply can and you should too.

### Examples

See the [examples](mbi/examples/README.md) folder for running these programs through the interpreter.

### Building

Build by using this command.

`cargo build`

### Testing

`cargo test`

### License

GPLv3+