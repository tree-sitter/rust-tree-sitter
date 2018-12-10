#!/usr/bin/env bash

GRAMMARS_DIR=$(dirname $0)/../fixtures

fetch_grammar() {
  local grammar=$1
  local ref=$2
  local grammar_dir=${GRAMMARS_DIR}/${grammar}
  local grammar_url=https://github.com/tree-sitter/tree-sitter-${grammar}

  echo "Updating ${grammar} grammar..."

  if [ ! -d $grammar_dir ]; then
    git clone $grammar_url $grammar_dir --depth=1
  fi

  (
    cd $grammar_dir
    git fetch origin $ref --depth=1
    git reset --hard FETCH_HEAD
  )
}

#fetch_grammar embedded-template master
fetch_grammar javascript        master
fetch_grammar json              master
fetch_grammar c                 master
#fetch_grammar cpp               master
#fetch_grammar python            master
fetch_grammar go                master
#fetch_grammar ruby              master
fetch_grammar typescript        master
#fetch_grammar bash              master
#fetch_grammar html              master
fetch_grammar rust              master
fetch_grammar scala             master
fetch_grammar css               master
fetch_grammar java              master
#fetch_grammar haskell           master
#fetch_grammar php               master

