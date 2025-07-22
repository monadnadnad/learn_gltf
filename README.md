これは、Rustと3DCGで使われるglTFの構造を学習する目的で過去に作成した、作りかけのglTFファイルのパーサー的なコマンドライン出力プログラムです。
コマンドラインからglTFファイルを読み込み、そのシーングラフの構造をコンソールに表示するだけです。

```bash
$ cargo run -- sample.gltf

Scene 0 (<Unnamed>)
 - Node 0 (MyFirstNode)
```
