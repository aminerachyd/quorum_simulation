#!/usr/bin/env bash
export EXISTING_VARS=$(printenv | awk -F= '{print $1}' | sed 's/^/\$/g' | paste -sd,);
for file in $JSFOLDER;
do
  envsubst $EXISTING_VARS < $file | sponge $file
done
nginx -g 'daemon off;'