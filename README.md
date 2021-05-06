# PDFToC

Add table of contents, TOC to a PDF.

* [日本語の解説](https://osanshouo.github.io/blog/2021/05/04-pdf-toc/)


## Requirement

* Rust (tested on current stable)
* PDFtk 1.45 or later 

It is tested only for Linux and WSL. Ubuntu/Debian: `sudo apt install pdftk`.


## Install

```bash
$ git clone https://github.com/osanshouo/pdftoc-rs.git
$ cd pdftoc-rs
$ cargo build --release
```

I recommend to make a symlink to `./target/release/pdftoc`.


## Usage

Create a text file that describes ToC in YAML format. 

```yaml
- Contents 1
- Introduction 1
- PDF format 3
- 
  - specifications 3
  - 
    - Adobe 4
    - ISO 5
  - advantaages 7
- Discussions 9
```

Save it as `<INPUT-PDF>.yaml` (e.g., `input.yaml` for `input.pdf` ). Then, run `pdftoc` as below.

```bash
pdftoc <INPUT-PDF>.pdf
```

You'll get `<INPUT-PDF>.toc.pdf`, which contains TOC information.

For more details, run `pdftoc --help`.


# Acknowledgement

It is heavily inspired by [SiddharthPant/booky](https://github.com/SiddharthPant/booky).
