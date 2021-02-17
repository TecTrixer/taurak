#!/bin/bash
cd /home/maintainer/dev/taurak/;
if [ "$(git pull)" == "Already up to date." ]; then
  echo "Already up to date."
else
  echo "$(git pull)";
  cargo build --release;
fi;