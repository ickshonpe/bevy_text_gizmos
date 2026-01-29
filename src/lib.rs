use bevy_color::Color;
use bevy_gizmos::config::GizmoConfigGroup;
use bevy_gizmos::gizmos::GizmoBuffer;
use bevy_math::prelude::{Isometry2d, Isometry3d, Vec2, Vec3};
use font::*;

mod font;

pub mod prelude {
    pub use crate::TextGizmos;
}

pub trait TextGizmos {
    /// Draw text using a stroke font with the given isometry applied.
    ///
    /// # Arguments
    ///
    /// - `isometry`: defines the translation and rotation of the text.
    /// - `text`: the text to be drawn.
    /// - `size`: the size of the text in pixels.
    /// - `color`: the color of the text.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_math::prelude::*;
    /// # use bevy_color::Color;
    /// # use bevy_text_gizmos::TextGizmos;
    /// fn system(mut gizmos: Gizmos) {
    ///     gizmos.text_3d(Isometry3d::IDENTITY, "3D text gizmo", 25., Color::WHITE);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    fn text(
        &mut self,
        isometry: impl Into<Isometry3d>,
        text: &str,
        size: f32,
        color: impl Into<Color>,
    );

    /// Draw text using a stroke font in 2d with the given isometry applied.
    ///
    /// # Arguments
    ///
    /// - `isometry`: defines the translation and rotation of the text.
    /// - `text`: the text to be drawn.
    /// - `size`: the size of the text in pixels.
    /// - `color`: the color of the text.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_math::prelude::*;
    /// # use bevy_color::Color;
    /// # use bevy_text_gizmos::TextGizmos;
    /// fn system(mut gizmos: Gizmos) {
    ///     gizmos.text_2d(Isometry2d::IDENTITY, "2D text gizmo", 25., Color::WHITE);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    fn text_2d(
        &mut self,
        isometry: impl Into<Isometry2d>,
        text: &str,
        size: f32,
        color: impl Into<Color>,
    );
}

impl<Config, Clear> TextGizmos for GizmoBuffer<Config, Clear>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
{
    fn text(
        &mut self,
        isometry: impl Into<Isometry3d>,
        text: &str,
        size: f32,
        color: impl Into<Color>,
    ) {
        let isometry = isometry.into();
        let color = color.into();
        let scale = size / SIMPLEX_CAP_HEIGHT;
        let line_height = LINE_HEIGHT * size;
        let space_advance = SIMPLEX_GLYPHS[0].0 as f32 * scale;

        let mut rx = 0.0;
        let mut ry = 0.0;

        for c in text.chars() {
            if c == '\n' {
                rx = 0.0;
                ry -= line_height;
                continue;
            }

            let code_point = c as usize;
            if !(SIMPLEX_ASCII_START..=SIMPLEX_ASCII_END).contains(&code_point) {
                rx += space_advance;
                continue;
            }

            let glyph = &SIMPLEX_GLYPHS[code_point - SIMPLEX_ASCII_START];
            let advance = glyph.0 as f32 * scale;

            for stroke_index in glyph.1.clone() {
                let stroke = SIMPLEX_STROKES[stroke_index].clone();
                if stroke.len() < 2 {
                    continue;
                }

                self.linestrip(
                    SIMPLEX_POSITIONS[stroke].iter().map(|&[x, y]| {
                        isometry
                            * Vec3::new(
                                rx + scale * x as f32,
                                ry - scale * (SIMPLEX_CAP_HEIGHT - y as f32),
                                0.0,
                            )
                    }),
                    color,
                );
            }

            rx += advance;
        }
    }

    fn text_2d(
        &mut self,
        isometry: impl Into<Isometry2d>,
        text: &str,
        size: f32,
        color: impl Into<Color>,
    ) {
        let isometry = isometry.into();
        let color = color.into();
        let scale = size / SIMPLEX_CAP_HEIGHT;
        let line_height = LINE_HEIGHT * size;
        let space_advance = SIMPLEX_GLYPHS[0].0 as f32 * scale;

        let mut rx = 0.0;
        let mut ry = 0.0;

        for c in text.chars() {
            if c == '\n' {
                rx = 0.0;
                ry -= line_height;
                continue;
            }

            let code_point = c as usize;
            if !(SIMPLEX_ASCII_START..=SIMPLEX_ASCII_END).contains(&code_point) {
                rx += space_advance;
                continue;
            }

            let glyph = &SIMPLEX_GLYPHS[code_point - SIMPLEX_ASCII_START];
            let advance = glyph.0 as f32 * scale;

            for stroke_index in glyph.1.clone() {
                let stroke = SIMPLEX_STROKES[stroke_index].clone();
                if stroke.len() < 2 {
                    continue;
                }

                self.linestrip_2d(
                    SIMPLEX_POSITIONS[stroke].iter().map(|&[x, y]| {
                        isometry
                            * Vec2::new(
                                rx + scale * x as f32,
                                ry - scale * (SIMPLEX_CAP_HEIGHT - y as f32),
                            )
                    }),
                    color,
                );
            }

            rx += advance;
        }
    }
}
