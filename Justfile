###: <https://just.systems/man/en/>

mod reuse '.config/reuse'
mod release '.config/release'

prj-root := env("PRJ_ROOT")

default:
  @just --choose

[doc: "Check the project for issues"]
check:
    biome check {{prj-root}}

[doc: "Format the project files"]
fmt:
    treefmt

push branch="main":
    for remote in origin github; do jj git push -b {{ branch }} --remote $remote; done
