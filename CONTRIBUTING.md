# Contributing

Cpp is needed to wrap functions since giac is writed in cpp.
Most functions should return `result` (basically a char pointer, pointing to `NULL` if there is no exceptions and to an error string if any),
because nearly all giac methods throws an exception.
Most functions can be easily implemented with macros in both rust and cpp, which add a try catch block and manage exceptions in the rust side.

Implemented functions come from <https://www-fourier.ujf-grenoble.fr/~parisse/giac/doc/en/cascmd_en/cascmd_en.html>
