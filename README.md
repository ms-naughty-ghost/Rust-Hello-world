# Hello, Rust
ドキュメント→[Rust](https://doc.rust-lang.org/book/title-page.html)
# 序文
必ずしも明確ではありませんでしたが、Rustプログラミング言語は基本的にエンパワーメントに関するものです。現在作成しているコードの種類に関係なく、Rustを使用すると、以前よりもさまざまなドメインで自信を持ってプログラミングできるようになります。

たとえば、メモリ管理、データ表現、および同時実行性の低レベルの詳細を処理する「システムレベル」の作業を取り上げます。伝統的に、このプログラミングの領域は難解であると見なされており、悪名高い落とし穴を回避するために必要な年数を費やした少数の選択者だけがアクセスできます。そして、それを実践する人でさえ、コードが悪用、クラッシュ、または破損にさらされないように注意して行ってください。

Rustは、古い落とし穴を取り除き、途中で役立つ洗練されたフレンドリーなツールセットを提供することで、これらの障壁を打ち破ります。下位レベルの制御に「浸る」必要があるプログラマーは、クラッシュやセキュリティホールの通常のリスクを負うことなく、また気まぐれなツールチェーンの細かい点を学ぶ必要なしに、Rustを使用してそれを行うことができます。さらに良いことに、この言語は、速度とメモリ使用量の点で効率的な信頼性の高いコードに自然に導くように設計されています。

すでに低レベルのコードで作業しているプログラマーは、Rustを使用して野心を高めることができます。たとえば、Rustに並列処理を導入することは、比較的リスクの低い操作です。コンパイラーは、古典的な間違いをキャッチします。また、クラッシュや脆弱性を誤って導入しないという自信を持って、コードのより積極的な最適化に取り組むことができます。

しかし、Rustは低レベルのシステムプログラミングに限定されていません。これは、CLIアプリ、Webサーバー、およびその他の多くの種類のコードを非常に快適に記述できるように、表現力と人間工学に基づいています。この本の後半で、両方の簡単な例を示します。Rustを使用すると、あるドメインから別のドメインに移行するスキルを構築できます。Webアプリを作成してRustを学習し、同じスキルを適用してRaspberryPiをターゲットにすることができます。

この本は、Rustがユーザーに力を与える可能性を完全に取り入れています。これは、Rustの知識だけでなく、プログラマー全般としての到達範囲と自信をレベルアップするのに役立つ、親しみやすく親しみやすいテキストです。さあ、飛び込んで、学ぶ準備をしてください。そして、Rustコミュニティへようこそ！

—ニコラス・マサキスとアーロン・テューロン

>It wasn’t always so clear, but the Rust programming language is fundamentally about empowerment: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.

>Take, for example, “systems-level” work that deals with low-level details of memory management, data representation, and concurrency. Traditionally, this realm of programming is seen as arcane, accessible only to a select few who have devoted the necessary years learning to avoid its infamous pitfalls. And even those who practice it do so with caution, lest their code be open to exploits, crashes, or corruption.

>Rust breaks down these barriers by eliminating the old pitfalls and providing a friendly, polished set of tools to help you along the way. Programmers who need to “dip down” into lower-level control can do so with Rust, without taking on the customary risk of crashes or security holes, and without having to learn the fine points of a fickle toolchain. Better yet, the language is designed to guide you naturally towards reliable code that is efficient in terms of speed and memory usage.

>Programmers who are already working with low-level code can use Rust to raise their ambitions. For example, introducing parallelism in Rust is a relatively low-risk operation: the compiler will catch the classical mistakes for you. And you can tackle more aggressive optimizations in your code with the confidence that you won’t accidentally introduce crashes or vulnerabilities.

>But Rust isn’t limited to low-level systems programming. It’s expressive and ergonomic enough to make CLI apps, web servers, and many other kinds of code quite pleasant to write — you’ll find simple examples of both later in the book. Working with Rust allows you to build skills that transfer from one domain to another; you can learn Rust by writing a web app, then apply those same skills to target your Raspberry Pi.

>This book fully embraces the potential of Rust to empower its users. It’s a friendly and approachable text intended to help you level up not just your knowledge of Rust, but also your reach and confidence as a programmer in general. So dive in, get ready to learn—and welcome to the Rust community!

>— Nicholas Matsakis and Aaron Turon

# Getting Start
## install
Rustのバージョンと関連ツールを管理するためのコマンドラインツールのインストール。
- LinuxまたはmacOSへのインストール
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
## Cargoを使用したプロジェクトの作成
Cargoがインストールされているか確認。[install](#install)で一緒にインストールされているはず。
```
cargo --version
```
hello-cargoというプロジェクトを作成する。
```
cargo new hello-cargo
```