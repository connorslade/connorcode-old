#!/bin/sh

echo Build School/GameDesign
cd data/web/static/school/gamedesign
python3 -m mkdocs build
mv site/* .
rm -rf site
cd ../../../..

echo Build Mc/Help
cd data/web/static/mc/help
python3 -m mkdocs build
mv site/* ../docs
rm -rf site
cd ../../../..
