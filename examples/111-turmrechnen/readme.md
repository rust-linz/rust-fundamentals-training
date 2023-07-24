# Tower Arithmetic

## Requirements

Your task is to write a command-line application in Rust for [*tower calculations*](http://www.floriangeier.at/schule/kopf/kopf.php) (use Google Translate to translate the description into your native language).

A user passes two parameters to your application via the command line:

* The starting value (e.g., *9*); the starting value must be > 1.
* The "height" (e.g., *5); the height must be > 2.

You output the result in the following format:

```txt
   9 * 2 = 18
  18 * 3 = 54
  54 * 4 = 216
 216 * 5 = 1080
1080 / 2 = 540
 540 / 3 = 180
 180 / 4 = 45
  45 / 5 = 9
```

If the user has entered incorrect parameters on the command line or parameters are missing, display an appropriate error message on the screen.

You can ignore *overflows*.

## Tips

* [Accessing command line parameters](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
* You can [left-pad output with spaces](https://doc.rust-lang.org/std/fmt/index.html#fillalignment)
* You can convert strings to integers with [`parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse).

## Levels

Depending on prior knowledge, this task may be more difficult for some and easier for others. Therefore, here are suggestions on how you can solve the example step by step. Each can work through the number of levels that suit them based on their existing programming practice.

### Level 1 - Calculation Logic

* Make assumptions for starting value and height and omit command line parameters.
* Only output the intermediate results. For example:

```txt
9
18
54
216
1080
540
180
45
9
```

### Level 2 - Command Line Parameters

* Add the ability to pass parameters via the command line.
* Remember to check the parameters with appropriate error messages.

### Level 3 - Improved Output

* Improve the output so that the result looks as requested at the beginning of this description.
* If possible, display the original value right-justified.

### Level 4 - Unit Testing

* Structure your code so that you can test it well.
* Write at least one meaningful unit test ([brief guide](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html))
