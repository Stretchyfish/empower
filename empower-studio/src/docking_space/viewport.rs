use empower_engine::assets::AssetId;
use serde::{Serialize, Deserialize};

pub mod empty_viewport;
pub use empty_viewport::EmptyViewport;

pub mod graph_viewport;
pub use graph_viewport::GraphViewport;

pub mod terminal_viewport;
pub use terminal_viewport::TerminalViewport;

pub mod content_browser_viewport;
pub use content_browser_viewport::ContentBrowserViewport;

pub mod image_viewer_viewport;

pub mod text_editor_viewport;
pub use text_editor_viewport::TextEditorViewport;

#[derive(Clone, Serialize, Deserialize)]
pub enum Viewport
{
    Graph { graph_viewport: GraphViewport },
    Terminal { terminal_viewport: TerminalViewport },
    ContentBrowser { content_browser_viewport: ContentBrowserViewport },
    ImageViewer { image_asset_id: AssetId },
    TextEditor { text_editor_viewport: TextEditorViewport }, 
    Empty { empty_viewport: EmptyViewport },
}
