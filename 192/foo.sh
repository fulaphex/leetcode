#!/usr/bin/env bash
cat words.txt | tr -s "  " " " | tr " " "\n" | grep "." | sort | uniq -c | sort -rh | tr -s '  ' ' ' | awk '{print $2 " " $1}'
