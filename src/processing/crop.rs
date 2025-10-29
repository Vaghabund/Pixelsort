use std::sync::Arc;
use crate::PixelSorterApp;

impl PixelSorterApp {
    pub fn apply_crop_and_sort(&mut self, ctx: &egui::Context) {
        if let (Some(ref original), Some(crop_rect)) = (&self.original_image, self.crop_rect) {
            self.is_processing = true;

            // The crop_rect is already in image pixel coordinates (set in render_crop_overlay)
            // No need for coordinate conversion - just clamp to image bounds
            let image_size = original.dimensions();

            // Clamp crop coordinates to image bounds
            let crop_min_x = (crop_rect.min.x.max(0.0).min(image_size.0 as f32)) as u32;
            let crop_min_y = (crop_rect.min.y.max(0.0).min(image_size.1 as f32)) as u32;
            let crop_max_x = (crop_rect.max.x.max(0.0).min(image_size.0 as f32)) as u32;
            let crop_max_y = (crop_rect.max.y.max(0.0).min(image_size.1 as f32)) as u32;

            // Ensure valid crop dimensions
            let crop_width = crop_max_x.saturating_sub(crop_min_x);
            let crop_height = crop_max_y.saturating_sub(crop_min_y);

            if crop_width > 0 && crop_height > 0 {
                // Create cropped image
                let mut cropped = image::RgbImage::new(crop_width, crop_height);

                for y in 0..crop_height {
                    for x in 0..crop_width {
                        let src_x = crop_min_x + x;
                        let src_y = crop_min_y + y;
                        if src_x < image_size.0 && src_y < image_size.1 {
                            let pixel = original.get_pixel(src_x, src_y);
                            cropped.put_pixel(x, y, *pixel);
                        }
                    }
                }

                // Apply pixel sorting to the cropped region
                let algorithm = self.current_algorithm;
                let params = self.sorting_params.clone();
                let pixel_sorter = Arc::clone(&self.pixel_sorter);

                if let Ok(sorted_cropped) = pixel_sorter.sort_pixels(&cropped, algorithm, &params) {
                    // Make the sorted cropped region the new full image
                    self.original_image = Some(sorted_cropped.clone());
                    self.processed_image = Some(sorted_cropped.clone());
                    // Use nearest filtering for cropped images so the upscaled look is crisp
                    self.create_processed_texture(ctx, sorted_cropped);

                    // Exit crop and return to Edit phase
                    self.crop_rect = None;
                    self.current_phase = crate::ui::Phase::Edit;
                }
                
                self.is_processing = false;
            } else {
                self.is_processing = false;
            }
        }
    }
}