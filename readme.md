# WASMからLinderaを呼ぶだけのサンプルコード

## 目的
* WASM入門、Hellow worldだけでは物足りないので、ほんの少しだけ改変
* Rust資産を手っ取り早く活用する例として、[Lindera](https://github.com/lindera-morphology/lindera)を雑に呼び出して使ってみた例

## References
* [Mozilla:Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
* [Lindera](https://github.com/lindera-morphology/lindera)
* [Qiita:Rust初心者がRust製の日本語形態素解析器の開発を引き継いでみた](https://qiita.com/mosuka/items/0fdaaf91f5530d427dc7)

## Commands
### Rust
```
# 最初にwasm-pack入れる
cargo install wasm-pack
# ビルドチェック
cargo build
# 実行チェック
cargo run
# wasm用ビルド
wasm-pack build --target web
```
### index.html
* Pythonで手っ取り早くローカルサーバーたてて試す
```
python3 -m http.server 8000
```

# EOF