#!/bin/bash

if $(cargo fmt --check >> check.out 2> check.out)
then
  echo "code formatted correctly"
else
  echo "code not formatted correctly"
  exit 1
fi

if $(cargo check >> check.out 2> check.out)
then
  echo "cargo check passed"
else
  echo "cargo check failing"
  exit 2
fi

if $(cargo clippy -- -Dwarnings >> check.out 2> check.out)
then
  echo "clippy happy"
else
  echo "clippy not happy"
  exit 3
fi

if $(cargo test  >> check.out 2> check.out)
then
  echo "tests passing"
else
  echo "tests not passing"
  exit 4
fi

echo "all checks passing"