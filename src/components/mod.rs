//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod image;
pub use image::Image;

mod searchbar;
pub use searchbar::SearchBar;

mod image_preview;
pub use image_preview::ImagePreview;

mod tag_editor;
pub use tag_editor::TagEditor;

mod admin_panel;
pub use admin_panel::AdminPanel;
