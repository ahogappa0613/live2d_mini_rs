# これは何
* Live2Dのライブラリをrustから読んでみる
# 技術的な話
* Live2Dのリファレンス実装はC++
* Live2Dはレンダリングエンジンに実装を依存していないので、自由に選べる
  * 今回はレンダリングエンジンにminiquadを採用

# ビルド
* M1のみ確認
* resouces/以下にLive2Dモデルデータを配置
  * 動作確認はHiyoriのデータを使っている
* live2d_mini_sys/Core以下にLive2Dのライブラリを配置
