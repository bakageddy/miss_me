mod app;
mod types;

use app::Application;
use clap::Parser;
use futures::{stream::FuturesUnordered, StreamExt};
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
    let mut tasks = FuturesUnordered::new();
    let mut handle = tokio::fs::read_dir(path).await?;
    while let Some(dir_entry) = handle.next_entry().await? {
        tasks.push(Box::pin(append_index(dir_entry)));
        // let output = tokio::spawn(append_index(dir_entry));
    }

    while let Some(stream_output) = tasks.next().await {
        if let Some(output) = stream_output {
            indices.push(output);
        }
    }

    // let mut terminal = ratatui::init();
    // let mut app = Application::new(indices);
    //
    // let result = app.run(&mut terminal);
    //
    // ratatui::restore();
    // result
    println!("{:#?}", indices);
    Ok(())
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
