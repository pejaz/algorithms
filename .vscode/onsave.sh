#!/bin/bash

FILE=$1

# 目标文件路径
TARGET_FILE="leet-code-rs/src/lib.rs"

# 记录执行时间和参数
if [[ $(grep -c "$FILE" "$TARGET_FILE") -eq 0 ]]; then
    sed -i '' -E "s/(CURRENT: &str = \")[^\"]*(\";.*)/\1$FILE\2/" "$TARGET_FILE"
    if [[ $(grep -c '::dir!(pub' "$TARGET_FILE") -eq 1 ]]; then
        sed -i '' "s|::dir!(pub |::dir!(|" "$TARGET_FILE"
    else
        sed -i '' "s|::dir!(|::dir!(pub |" "$TARGET_FILE"
    fi
fi