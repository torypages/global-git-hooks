#!/bin/bash

rm -f "$HOME/.global-git-hooks/"
git config --global --unset core.hooksPath "$HOME/.global-git-hooks/"
