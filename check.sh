#!/usr/bin/env bash
directories=("music21-rs" "tuning_systems" "tuningplayground")

for dir in "${directories[@]}"; do
  pushd "./$dir"
  sh "../rustcheck.sh"
  popd
done

sh ./build.sh
pushd ./ts
sh ../tscheck.sh
popd
