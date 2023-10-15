#!/bin/sh
set -eu

######################################################################
# 設定
######################################################################

######################################################################
# 本体処理
######################################################################

# ビルドを実行
(
  cd ./rust/imagedecode

  cargo build --release
)

# ビルドを実行
(
  cd ./rust/alphasilhouette

  cargo build --release
)

# バイナリを配置
mv ./rust/imagedecode/target/release/imagedecode         ./bin
mv ./rust/alphasilhouette/target/release/alphasilhouette ./bin
