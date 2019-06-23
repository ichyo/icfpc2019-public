#!/bin/sh

while true; do
    cargo run --bin mining --release
    sleep 60
done
