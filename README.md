# wasn

`wasn` is a simple, small command line calculator.


## Usage
Run `wasn a op b`, with:
- a and b being numbers
- op being an operator

The result can be used for the next calulation:

```
$ wasn 1 + 3
1 + 3 = 4       + 6.3
4 + 6.3 = 10.3
```
**Note:** Per input only 1 operation is allowed.

To exit press enter without any new input.

**Note:** `wasn` will also exit if the input cannot be parsed.


### Numbers
Numbers are everything that `rust` can parse to an f64 (see https://doc.rust-lang.org/std/primitive.f64.html#method.from_str).

Additionally numbers in the German format can be entered: e.g. `1,23` instead of `1.23`.

**Note:** Output will always use `.`.

### Operators
As with numbers, German names are also supported.

| Operation | Input |
|-----------|-------|
| Addition | \+, plus |
| Substraction | \-, minus |
| Multiplication | \*, x, times, mal |
| Division | /, :, div, durch |
| Power | ^, \*\*, pow, hoch |
| Remainder | %, mod, modulo |


