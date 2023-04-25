# /usr/bin/env bash

HOST=$1
PORT='1337'
USER=''
PASSWD=''
FILE='vita_rust_starter.vpk'
SOURCE_DIR='target/armv7-vita-eabihf/release/'
TARGET_DIR='ux0:'

cd $SOURCE_DIR

ftp -n $HOST $PORT <<END_SCRIPT
quote USER $USER
quote PASS $PASSWD
binary
cd $TARGET_DIR
put $FILE
quit
END_SCRIPT
exit 0