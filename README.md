# HTML Visualizer for Rust

Rustでプログラムの動作を可視化するライブラリです。  
可視化結果はHTMLファイルで出力されるため、環境を問わず見ることができます。

拙作 [HTML Visualizer for C++](https://github.com/zenito9970/HTML-Visualizer) をRustに移植したものですが、移植に伴って機能整理を行っています。

## 使い方

`Cargo.toml` に依存関係を追加します。

```toml
[dependencies]
html_visualizer = { git = "https://github.com/zenito9970/HTML-Visualizer-rs.git" }
```

### 定義されている描画命令

- `circle(x, y, r, color)`
    - 座標(`x`, `y`)を中心に半径`r`の円を描き、色`color`で塗りつぶします。
- `line(x1, y1, x2, y2, color)`
    - 座標(`x1`, `y1`)と座標(`x2`, `y2`)を結ぶ線を色`color`で描きます。
- `newpage()`
    - 以降の描画命令を次のページのものとして扱います。
- `setpage(page)`
    - 以降の描画命令を指定したページのものとして扱います。

自動で適したサイズに拡大・縮小されます。  
なお、プログラム終了前に必ず `finish()` を呼ぶ必要があります。

### 定義されている `Color` 定数

- `Color::BLACK`
- `Color::WHITE`
- `Color::GRAY`
- `Color::RED`
- `Color::GREEN`
- `Color::BLUE`

`Color::new(r, g, b)` で新しく定義することもできます。

## 出力されたHTMLファイルの使い方

生成されたHTMLファイル (`result.html`) をブラウザで開きます。
ローカルサーバ等を立てる必要はありません。

## License

MIT
