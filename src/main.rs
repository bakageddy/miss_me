mod app;
mod types;

use app::Application;
use clap::Parser;
use miss_me::Index;
use tokio::fs::DirEntry;
use types::Args;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cmd_args = Args::parse();
    let path: String = match cmd_args.path {
        Some(path) => path,
        None => "".to_string(),
    };

    let mut indices = Vec::new();

    let mut handle = tokio::fs::read_dir(path).await?;
    while let Some(dir_entry) = handle.next_entry().await? {
        let output = tokio::spawn(append_index(dir_entry));
        if let Ok(Some(index)) = output.await {
            indices.push(index);
        }
    }

    let mut terminal = ratatui::init();
    let mut app = Application::new(indices);

    let result = app.run(&mut terminal);

    ratatui::restore();
    result
}

async fn append_index(dir_entry: DirEntry) -> Option<Index> {
    let file_type = dir_entry.file_type().await.unwrap();
    if file_type.is_dir() {
        let index = Index::extract_index(dir_entry.path()).await.map_err(|e| {
            eprintln!("{e:?}");
            ()
        }).unwrap();
        return Some(index);
    } else {
        return None;
    }

}
