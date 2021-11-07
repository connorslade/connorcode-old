#!/bin/sh

echo Build School/GameDesign
cd data/static/school/gamedesign
rm -rf . !(docs|mkdocs.yml)
python3 -m mkdocs build
mv site/* .
rm -rf site
cd ../../../..

echo Build Mc/Help
cd data/static/mc/help
rm -rf . !(docs|mkdocs.yml)
python3 -m mkdocs build
mv site/* .
rm -rf site
cd ../../../..

read -n1 -r -p "Press any key to continue..."
