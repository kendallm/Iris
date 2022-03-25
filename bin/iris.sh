#!/bin/bash

: "${LIGHT_1:=192.168.1.237}"
: "${LIGHT_2:=192.168.1.234}"

function toggle_lights() {
    curl -s -X PUT "http://$LIGHT_1:9123/elgato/lights" --header 'Content-Type: application/json' --data-raw '{"lights":[{"brightness":50,"temperature":165,"on":'"$1"'}],"numberOfLights":2}' > /dev/null
    curl -s -X PUT "http://$LIGHT_2:9123/elgato/lights" --header 'Content-Type: application/json' --data-raw '{"lights":[{"brightness":50,"temperature":165,"on":'"$1"'}],"numberOfLights":2}' > /dev/null
}

log stream --predicate 'subsystem contains "com.apple.UVCExtension" and composedMessage contains "Post PowerLog"' | while read line; do
    case "$line" in 
        *"On"* ) toggle_lights 1;;
        *"Off"* ) toggle_lights 0;;
    esac
done
