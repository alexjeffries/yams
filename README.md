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


Using a Docker container
------------------------

The Docker image is not published anywhere so you will need to build it locally.
Ensure that you have Docker (>= 17.05.0) installed.  To build it, clone this
repo and run

    $ docker build -t yams .

To then run yams, execute

    $ docker run -it -v "$(pwd)/example.yml:/etc/yams.yml" -p 3333:3333 yams

Note that the multistage build will result in two generated images:

    REPOSITORY      TAG         IMAGE ID          CREATED             SIZE
    yams            latest      0a181727b2c1      15 seconds ago      59.8MB
    <none>          <none>      d2e9e49825cc      30 seconds ago      1.41GB

 The second, untagged image is the intermediate build-env and can be removed.


Using the binary
----------------

There is not currently a binary published anywhere.  To use yams, you'll need to
build it.  First, ensure that you have the Rust toolchain installed (`rustc` and
`cargo`).  Then clone this repo, and use cargo to build and install yams:

    $ cargo install

See `example.yml` for an example of how you could configure your HTTP
mocks.  You can run yams with `yams example.yml`.  It will bind to
`localhost:3333`.
