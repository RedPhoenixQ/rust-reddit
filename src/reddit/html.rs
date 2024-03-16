use maud::{html, Markup};

use super::types::*;

impl Post {
    pub fn render_feed(&self) -> Markup {
        html! {
            div class="rounded border bg-slate-800 p-2 text-slate-100" {
                a href=(self.url) {(self.title)}
                div class="flex gap-2" {
                    @if let Some(author) = &self.author {
                        span {(author)}
                    } @else {
                        i { "removed" }
                    }
                    a href=(self.subreddit_name_prefixed) {
                        (self.subreddit)
                    }
                }
                @if let Some(preview) = &self.preview {
                    (preview.render(&self.thumbnail))
                } @else if let Some(gallery) = &self.gallery {
                    (gallery.render())
                } @else if let Some(selftext) = &self.selftext {
                    span { (selftext) }
                } @else {
                    span class="text-center text-sm italic"
                    { "Content appears to be missing"}
                }
                }
        }
    }
}

impl SourceSet {
    fn render_img(&self) -> Markup {
        html! {
            // Todo: Add srcset for resolutions
            img class="m-auto w-full"
            width=(self.source.width)
            height=(self.source.height)
            src=(self.source.url);
        }
    }

    fn render_video(&self, poster: &Option<String>) -> Markup {
        html! {
            video class="h-full max-h-full w-full"
            width=(self.source.width)
            height=(self.source.height)
            src=(self.source.url)
            poster=[poster]
            autoplay controls playsinline "loop" muted
            {
                @for Source { url, width, height } in &self.resolutions {
                    source width=(width) height=(height) src=(url) {}
                }
            }
        }
    }
}

impl Preview {
    fn render(&self, thumbnail: &Option<String>) -> Markup {
        html! {
            @if let Some(video) = &self.reddit_video_preview {
                video class="h-full max-h-full w-full"
                width=(video.width)
                height=(video.height)
                autoplay controls playsinline "loop" muted
                src=(video.fallback_url)
                {}
            } @else if let Some(image) = self.images.first() {
                @match image.variants {
                    Some(ImageVariants { gif: Some(ref gif), .. }) => {
                        (gif.render_img())
                    }
                    Some(ImageVariants { mp4: Some(ref mp4), .. }) => {
                        (mp4.render_video(thumbnail))
                    }
                    _ => {
                        (image.default.render_img())
                    }
                }
            }
        }
    }
}

impl Gallery {
    fn render(&self) -> Markup {
        html! {
            custom-gallery class="flex w-full snap-x snap-mandatory overflow-auto" {
                @for item in &self.gallery_data.items {
                    @if let Some(data) = self.media_metadata.get(&item.media_id) {
                        @match data.content_type {
                            GalleryContentType::Image => {
                                (data.source.render_img())
                            },
                            GalleryContentType::Video => {
                                (data.source.render_video(&None))
                            }
                        }
                    } @else {
                        div class="w-full" {
                            "Missing media_id in media_metadata"
                        }
                    }
                }
            }
        }
    }
}
