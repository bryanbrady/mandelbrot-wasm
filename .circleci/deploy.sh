#!/bin/sh
set -x

wasm-pack build
cd www
npm install
npm run build

sha=`git rev-parse HEAD`
git config --global user.email "bryan.brady@gmail.com"
git config --global user.name "bryan brady"
git clone git@github.com:bryanbrady/asdf-sh.git
rm -rf asdf-sh/mandelbrot-wasm/*
cp dist/* asdf-sh/mandelbrot-wasm/
cd asdf-sh
if ! git diff --exit-code; then
  git add .
  git commit -m "https://github.com/bryanbrady/mandelbrot-wasm/commit/$sha"
  git push origin master
fi
