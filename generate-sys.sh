#!/bin/bash

trap 'exit 1' ERR
set -x

mkdir -p cuelang-sys
#go run ./goexport/cmd/goexport/ -get -language rust -importOnly cuelang.org/go/pkg -package cuelang-sys cuelang-sys cuelang.org/go/cue{,/{ast{,/astutil},build,parser}}
go run ./goexport/cmd/goexport/ -language rust -importOnly cuelang.org/go/pkg -package cuelang-sys cuelang-sys cuelang.org/go/cue{,/{ast{,/astutil},build,parser}}
cargo upgrade -p cuelang-sys
cargo fmt -p cuelang-sys
