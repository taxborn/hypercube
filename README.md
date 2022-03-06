# HyperCube
This is a [4DChess](https://esolangs.org/wiki/4DChess) interpreter in Rust.

## Motivation
I've been having more and more of an interest in compilers and interpreters, 
and to start off a little easier, I chose a language that didn't have many 
instructions and seemed easy to implement. I also wanted to find a language that 
did not have an implementation yet. 4DChess is largely based off the esoteric 
language [brainfuck](https://en.wikipedia.org/wiki/Brainfuck).

## What is 4DChess
As stated earlier, it is a derivative of brainfuck. There are two main differences.

- Instead of a 1 dimensional infinite tape, it uses a 4 dimensional 'cube' as memory.
- There is also an 8 cell limit in any of the 4 directions, giving us 4,096 cells of memory.

