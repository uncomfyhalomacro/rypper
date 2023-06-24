# Introduction to Rypper

`rypper` is an alternative package manager for openSUSE.

The intent of the project is to:
  - Be compatible with `zypper` commands except services.
  - Be performant and easily accesible to many.
  - Support parallel downloads.
  - Be a high quality package manager. 

## Why do I want `rypper`?

You will want `rypper` because it's new. It's shiny. It's Rust! 

`rypper` will have features such as parallel downloads and being written in a 
modern performant language. It also prevents you to do stupid things like running `up` instead of `dup` on a 
Tumbleweed system by default. 

`rypper`, unlike `zypper`, will avoid old stupid behavior that `zypper` does and
selects the best mirrors for you as much as possible giving you the fastest (parallel) downloads whenever 
possible.

`rypper` will also save the known best list of mirrors for you as much as possible and updates it on intervals
if possible.

## What `rypper` is NOT

`rypper` is not a full replacement of `zypper` nor ever. Oh how I wish it could be.

`rypper` will never ever replace `zypper` until there comes an interest.

And lastly, `rypper` is not a drop-in replacement of `zypper`.

## Why are you doing this project?

As a packager myself, I feel like I need to learn more of how package managers work. Therefore, even though
there exists package managers that may be suitable for this use case e.g. `dnf`, I may as well write
one and learn.
