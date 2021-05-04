use std::path::{Path, PathBuf};
use std::io::Write;
const TMPFILE: &'static str = "temporary64-9w3W6Z55flKXMI3q2B3z_xwrrx.txt";

fn main() {
    eprintln!("PDFToC {}", env!("CARGO_PKG_VERSION"));
    let args = std::env::args().collect::<Vec<_>>();

    match args.get(1).map(String::as_str) {
        Some("-h") | Some("--help") | Some("-V") | Some("--version") | None => help(),
        Some(arg) => {
            // INPUT-PDF のパスを取得
            let input: &Path = Path::new(arg);

            // TOC-YAML のパスを取得
            let toc: PathBuf = match args.get(2) {
                Some(toc) => PathBuf::from(toc),
                None => input.with_extension("yaml"),
            };

            if !input.is_file() { eprintln!("Error: {:?} does not exist.", input); }
            else if !toc.is_file() { eprintln!("Error: {:?} does not exist.", toc); }
            else {
                // 出力ファイルのパスを取得
                let output_path = input.with_extension("toc.pdf");

                // 情報を出力
                eprintln!("input:  {:?}", &input);
                eprintln!("toc:    {:?}", &toc);
                eprintln!("output: {:?}", &output_path);

                // 出力ファイルを上書きするかどうかを確認
                if output_path.exists() {
                    eprint!("Overwrite output file? (yes/no) ");
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "yes" {
                        panic!("Abort.");
                    }
                }

                // YAML ファイルを PDFtk 可読な形に変換
                let toc: String = pdftoc::yaml_to_pdftk(&toc);

                // 目次情報を一時ファイルに出力
                let tmp = input.with_file_name(TMPFILE);
                if tmp.exists() { panic!("Error: Temporary file already exists. Delete {:?}", tmp); }
                std::fs::write(&tmp,&toc)
                    .expect("Error: failed to write a temporary file.");

                // PDFtk を実行
                std::process::Command::new("pdftk")
                    .arg(input)
                    .arg("update_info_utf8")
                    .arg(&tmp)
                    .arg("output")
                    .arg(&output_path)
                    .output()
                    .expect("failed to execute process");

                // 一時ファイルを消去
                std::fs::remove_file(tmp)
                    .expect("Error: failed to delete a temporary file.");
            }
        }
    }
}

fn help() {
    eprintln!(r#"
Add table of contents (bookmarks) to a PDF.

USAGE:
  pdftoc <INPUT-PDF> <TOC-YAML>

<TOC-YAML> is optional. If <TOC-YAML> does not exist, 
then PDFToC try to read a YAML file with the same basename as the <INPUT-PDF>
(e.g., if <INPUT-PDF> is "input.pdf", then <TOC-YAML> is "input.yaml").

OUTPUT:
    $(basename <INPUT-PDF>).toc.pdf"#);
}
