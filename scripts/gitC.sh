#!/bin/bash

#-----------------------------
# SET GLOBAL VARIABLES
#-----------------------------
# Used because bash
# fn don't return values
#-----------------------------
user_input=""

#-----------------------------
# SCRIPT ENTRY FN
#-----------------------------
main () {
    get_user_input
    git_commit user_input;
}

#-----------------------------
# HELPER FNs
#-----------------------------
init () {
    echo "This script makes working with git more DRY"
    echo "Setting default commit message to ''latest commit'"
    user_input="latest commit"
}

finished () {
    echo "finished pushing the latest commit to github"
    echo "message was:"
    echo user_input
}

get_user_input () {
    read -p "please enter the commit message:  " commit
    user_input=commit
}

git_commit () {
    git add . && git commit -m $1
}

#-----------------------------
# RUN SCRIPT
#-----------------------------
main
