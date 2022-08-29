#!/bin/bash

# source: https://www.baeldung.com/linux/use-command-line-arguments-in-bash-script

while getopts u:t:m: flag
do
    case "${flag}" in
        u) uname=${OPTARG};;
        t) time=${OPTARG};;
        m) msg=${OPTARG};;
    esac
done

finalCommitMsg="$msg -- by: $uname -- @$time";

echo "Committed by: $uname";
echo "Commit time: $time";
echo "Initial commit message: $msg";
echo "Final commit message: $finalCommitMsg";


git add . && git commit -m $finalCommitMsg