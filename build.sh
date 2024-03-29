#!/bin/sh
set -eu

######################################################################
# 設定
######################################################################

bdir='bin'

######################################################################
# 前準備
######################################################################

mkdir -p "$bdir"

######################################################################
# 本体処理
######################################################################

cat <<-'EOF'                                                         |
imagedecode
alphasilhouette
EOF

while read -r prog
do
  (
    # ビルド
    cd "rust/$prog"
    cargo build --release
  )

  # バイナリを配置
  mv "rust/$prog/target/release/$prog" "$bdir"
done
