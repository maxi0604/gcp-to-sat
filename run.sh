#!/bin/bash
for i in {1..16}; do
    cargo run "$i"
    minisat "gcp$i.cnf" "out${i}.sat"
done
