#! /bin/bash
# Cd
cd data/static/assets/css/

# Compile
npx sass files.scss out/files.css
npx sass index.scss out/index.css
npx sass randomimg.scss out/randomimg.css
npx sass pi.scss out/pi.css
npx sass backgrounds.scss out/backgrounds.css
npx sass christmas.scss out/christmas.css
npx sass colornamegen.scss out/colornamegen.css

# Return
cd ../../../..
