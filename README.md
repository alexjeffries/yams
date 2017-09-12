yams -- yet another mock server
===============================

```
 _   _  __ _ _ __ ___  ___ 
| | | |/ _` | '_ ` _ \/ __|
| |_| | (_| | | | | | \__ \  -- yet another mock server
 \__, |\__,_|_| |_| |_|___/
 |___/                     
```

yams is yet another mock server that mocks out (so far) HTTP requests.
It has three primary design goals:

1.  Simple, human maintainable configuration -- you should be able to use yams
    without having its manual open by your editor

2.  Reliable and performant -- be able to depend on your mock api without
    worrying about it falling over

3.  Minimal dependencies -- install and run this on your developer machine, your
    CI system, or any other runtime environment without worrying about a massive
    dependency graph

Copyright and license
---------------------

Copyright 2017 Alex Jeffries.

Released under the GPL v3.  See the `LICENSE` file.

Current status
--------------

_Very very_ alpha.  There is a ton of stuff missing, most notably usable error
handling.

Releases will follow roughly follow semver for the alpha releases:

    0.y.z
    | | |
    | | any change
    | breaking change (config or cli)
    constant

Right now this primarily exists as a learning exercise for me for Rust.

Using
-----

There is not currently a binary published anywhere.  To use yams, you'll need to
build it.  First, ensure that you have the Rust toolchain installed (`rustc` and
`cargo`).  Then clone this repo, and use cargo to build and install yams:

    $ cargo install

See `example.yml` for an example of how you could configure your HTTP
mocks.  You can run yams with `yams example.yml`.  It will bind to
`localhost:3333`.
