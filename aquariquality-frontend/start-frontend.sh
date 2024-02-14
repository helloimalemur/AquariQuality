#!/bin/bash
cd ./aquariquality-frontend/ || exit
npm install
#bash -c "npm run build"
./node_modules/next/dist/bin/next
