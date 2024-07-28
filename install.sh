#!/bin/bash

set -e

project_dir=$(pwd)
cargo build --release
mkdir -p "$HOME/.global-git-hooks/"
temp_dir=$(mktemp -d)
cd $temp_dir
git init > /dev/null

for file in "$temp_dir"/.git/hooks/*.sample; do
    hook="${file##*/}"           # Extract the filename with extension
    hook="${hook%.sample}"       # Remove the .sample extension
    rm -f "$HOME/.global-git-hooks/$hook"
    ln -s "$project_dir/target/release/global-git-hooks" "$HOME/.global-git-hooks/$hook"
done

git config --global core.hooksPath "$HOME/.global-git-hooks/"
rm -rf $temp_dir
echo "Installed!"
