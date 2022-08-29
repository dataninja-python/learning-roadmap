#!/bin/bash

# source: https://www.baeldung.com/linux/use-command-line-arguments-in-bash-script

while getopts u:t:f: flag
do
    case "${flag}" in
        u) username=${OPTARG};;
        t) time=${OPTARG};;
        m) message=${OPTARG};;
    esac
done

finalCommitMsg="$message -- by: $username -- @$time";

echo "Committed by: $username";
echo "Commit time: $time";
echo "Initial commit message: $message";
echo "Final commit message: $finalCommitMsg";


#read -p "Enter commit message:  " ans
#git add . && git commit -m $ans