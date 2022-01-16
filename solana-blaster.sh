#!/bin/bash

while true
do
start_time=`date +%s`
solana transfer --no-wait --commitment processed 9V7nbTawDXfeBbfG7XoUQ5rd6ynvhPrncSg6c7oGuqFs 0.005
end_time=`date +%s`
echo execution time was `expr $end_time - $start_time` s.
done