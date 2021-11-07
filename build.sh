#!/bin/sh

echo Build School/GameDesign
cd data/static/school/gamedesign
python3 -m mkdocs build
mv site/* .
rm -rf site
cd ../../../..

echo Build Mc/Help
cd data/static/mc/docs
python3 -m mkdocs build
mv site/* .
rm -rf site
cd ../../../..
