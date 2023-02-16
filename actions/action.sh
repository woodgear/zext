#!/usr/bin/env bash

function zext-build() {
  cargo build
}

function zext-install() {
  zext-build
  sudo install ./target/debug/libzext.so $module_path/zext.so
  if zmodload | grep zext; then zmodload -u zext; fi
  zmodload zext
}

function zext-test() {
  zext-build
  zext-install
  zext-test-time-diff
}

function zext-test-time-diff() {
  local start=$(date +"%Y-%m-%eT%T.%6N")
  local end=$(date +"%Y-%m-%eT%T.%6N")
  time-diff "$start" "$end"
}
