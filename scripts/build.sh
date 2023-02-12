#!/bin/sh

echo Build School/GameDesign
cd web/static/school/gamedesign
python3 -m mkdocs build
mv site/* .
rm -rf site
cd ../../../..

echo Build Mc/Help
cd web/static/mc/help
python3 -m mkdocs build
mv site/* ../docs
rm -rf site
cd ../../../..
