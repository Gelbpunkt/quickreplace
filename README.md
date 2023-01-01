# quickreplace

Quickreplace offers optimized routines for replacing in strings in slices in Rust.

It is up to 2.5x faster on kilobytes to megabytes of input and achieves this by mutating the haystack in place and only allowing for replacing with the same length as the input.
