#!/bin/bash
cd ./aquariquality-frontend/ || exit
npm install
npm run build
./node_modules/next/dist/bin/next
