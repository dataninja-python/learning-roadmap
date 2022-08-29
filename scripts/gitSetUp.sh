#!/bin/bash

git init -b main

read -p "Enter git username:  " name
git config user.name $name 

read -p "Enter git email:  " email
git config user.name $email 