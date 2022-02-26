#!/bin/bash
if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi
work_dir=pwd"/bad_encrypter"
mv $work_dir /usr/bin