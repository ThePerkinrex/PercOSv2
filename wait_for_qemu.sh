#!/bin/bash
echo "Waiting for qemu"
until pids=$(pgrep -U $USER qemu-system-x86_64)
do   
    sleep 1
done
echo "qemu found on $pids"