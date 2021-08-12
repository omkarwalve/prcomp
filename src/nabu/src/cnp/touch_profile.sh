#!/bin/zsh
EXT="orel"
rm -rf profiles/*
for SITE in $(cat categories/*)
do
    touch profiles/$SITE.$EXT
done
