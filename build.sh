#!/bin/sh

cd data/static/school/gamedesign
rm -rf . !(docs|mkdocs.yml)
python3 -m mkdocs build
mv site/* .
rm -rf site

cd ../../../..

read -n1 -r -p "Press any key to continue..."
