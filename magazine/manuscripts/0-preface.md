# はじめに

本書では、著者がアセンブリ言語やLinuxプログラミングを学びながらBASIC言語のミニコンパイラをRustで実装している際に得た知見をまとめている。BASIC言語はもともと教育用に開発された。著者は中学時代に「プチコンmk-Ⅱ」をきっかけにプログラミングに入門した。簡単に手続き的なプログラムを記述できるという特徴がある。コンパイラ自作と言えばC言語やML系の言語のコンパイラを段階的に実装していく類の教材が有名だ。しかし一方でBASIC言語のコンパイラを自作するための教材は不足していると思い、本書を執筆することにした。途中、x86-64向けのアセンブリプログラムをいくつか手書きして挙動を確認していく。

## 対象読者

- プログラミング言語処理系を初めて作ろうとしている人
- アセンブリ言語をこれから学びたいと思っている人
- Linuxプログラミングをこれから学びたいと思っている人

## 必要な知識

- プログラミング言語Rustの基礎(公式チュートリアルを完了していることが望ましい)

## 開発環境

- x86-64アーキテクチャのCPUを搭載するPC
- Rust 1.47.0
- GNU Make

Ubuntu 20.04 LTSをインストールしたLenovo ThinkPad X230で、本文中に掲載したプログラムの動作確認を行った。

## ライセンス

本書に掲載しているソースコード及びそれをコンパイルして得られるソフトウェアに対するライセンスについてまとめる。

### 商用・商標・特許利用について

### 改変・再配布について

### 著作権表示について

### 動作保証・損害賠償責任について

### 派生ソフトウェアのライセンスについて

派生ソフトウェアのライセンスに対しては如何なる制限も課さない。
