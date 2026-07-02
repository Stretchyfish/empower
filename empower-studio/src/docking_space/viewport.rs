use serde::{Serialize, Deserialize};

use once_cell::sync::Lazy;

pub mod empty_viewport;
pub use empty_viewport::EmptyViewport;

pub mod graph_viewport;
pub use graph_viewport::GraphViewport;

pub mod terminal_viewport;
pub use terminal_viewport::TerminalViewport;

pub mod content_browser_viewport;
pub use content_browser_viewport::ContentBrowserViewport;

#[derive(Clone, Serialize, Deserialize)]
pub enum Viewport
{
    Graph { graph_viewport: GraphViewport },
    Terminal { terminal_viewport: TerminalViewport },
    ContentBrowser { content_browser_viewport: ContentBrowserViewport },
    Empty { empty_viewport: EmptyViewport },
}
