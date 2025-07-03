use image_processor::{
    processor::ImageProcessor,
    pipeline::ProcessingPipeline,
    filters::{ImageFilter, FilterType},
    allocator::MemoryPool,
};

#[cfg(test)]
mod processor_tests {
    use super::*;

    #[test]
    fn test_image_processor_basic_functionality() {
        let processor = ImageProcessor::new();
        let test_image = create_test_image(100, 100);
        
        let result = processor.process_image(&test_image);
        
        // Verify the result has expected properties
        assert_eq!(result.width(), test_image.width());
        assert_eq!(result.height(), test_image.height());
        
        // Verify processing actually changed the image
        let original_sum = calculate_pixel_sum(&test_image);
        let processed_sum = calculate_pixel_sum(&result);
        assert_ne!(original_sum, processed_sum, "Processing should modify the image");
    }

    #[test]
    fn test_processor_handles_different_sizes() {
        let processor = ImageProcessor::new();
        
        let small_image = create_test_image(50, 50);
        let large_image = create_test_image(1000, 800);
        
        let small_result = processor.process_image(&small_image);
        let large_result = processor.process_image(&large_image);
        
        assert_eq!(small_result.width(), 50);
        assert_eq!(small_result.height(), 50);
        assert_eq!(large_result.width(), 1000);
        assert_eq!(large_result.height(), 800);
    }

    #[test]
    fn test_processor_deterministic_output() {
        let processor = ImageProcessor::new();
        let test_image = create_test_image(200, 150);
        
        let result1 = processor.process_image(&test_image);
        let result2 = processor.process_image(&test_image);
        
        // Results should be identical for same input
        assert_images_equal(&result1, &result2);
    }
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    #[test]
    fn test_blur_filter() {
        let filter = ImageFilter::new(FilterType::Blur);
        let test_image = create_test_image(100, 100);
        
        let result = filter.apply(&test_image);
        
        assert_eq!(result.width(), test_image.width());
        assert_eq!(result.height(), test_image.height());
        
        // Blur should generally reduce high-frequency noise
        // Verify the output is different from input
        assert!(!images_identical(&test_image, &result));
    }

    #[test]
    fn test_brightness_filter() {
        let filter = ImageFilter::new(FilterType::Brightness(1.5));
        let test_image = create_test_image(50, 50);
        
        let result = filter.apply(&test_image);
        
        // Brightness increase should generally increase pixel values
        let original_sum = calculate_pixel_sum(&test_image);
        let filtered_sum = calculate_pixel_sum(&result);
        
        assert!(filtered_sum >= original_sum, "Brightness increase should not decrease total brightness");
    }

    #[test]
    fn test_contrast_filter() {
        let filter = ImageFilter::new(FilterType::Contrast(2.0));
        let test_image = create_gradient_image(100, 100);
        
        let result = filter.apply(&test_image);
        
        assert_eq!(result.width(), test_image.width());
        assert_eq!(result.height(), test_image.height());
        
        // High contrast should increase the difference between light and dark areas
        assert!(!images_identical(&test_image, &result));
    }

    #[test]
    fn test_edge_detection_filter() {
        let filter = ImageFilter::new(FilterType::EdgeDetection);
        let test_image = create_test_image_with_edges(100, 100);
        
        let result = filter.apply(&test_image);
        
        assert_eq!(result.width(), test_image.width());
        assert_eq!(result.height(), test_image.height());
        
        // Edge detection should produce different output
        assert!(!images_identical(&test_image, &result));
    }

    #[test]
    fn test_sharpen_filter() {
        let filter = ImageFilter::new(FilterType::Sharpen);
        let test_image = create_test_image(80, 80);
        
        let result = filter.apply(&test_image);
        
        assert_eq!(result.width(), test_image.width());
        assert_eq!(result.height(), test_image.height());
        assert!(!images_identical(&test_image, &result));
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;

    #[test]
    fn test_sequential_pipeline() {
        let pipeline = ProcessingPipeline::new_sequential();
        let test_images = vec![
            create_test_image(100, 100),
            create_test_image(150, 120),
            create_test_image(200, 180),
        ];
        
        let results = pipeline.process_batch(&test_images);
        
        assert_eq!(results.len(), test_images.len());
        
        for (original, result) in test_images.iter().zip(results.iter()) {
            assert_eq!(result.width(), original.width());
            assert_eq!(result.height(), original.height());
        }
    }

    #[test]
    fn test_parallel_pipeline() {
        let pipeline = ProcessingPipeline::new_parallel(2);
        let test_images = vec![
            create_test_image(100, 100),
            create_test_image(150, 120),
            create_test_image(200, 180),
            create_test_image(120, 90),
        ];
        
        let results = pipeline.process_batch(&test_images);
        
        assert_eq!(results.len(), test_images.len());
        
        for (original, result) in test_images.iter().zip(results.iter()) {
            assert_eq!(result.width(), original.width());
            assert_eq!(result.height(), original.height());
        }
    }

    #[test]
    fn test_pipeline_consistency() {
        let sequential = ProcessingPipeline::new_sequential();
        let parallel = ProcessingPipeline::new_parallel(3);
        
        let test_images = vec![
            create_test_image(100, 100),
            create_test_image(80, 120),
        ];
        
        let seq_results = sequential.process_batch(&test_images);
        let par_results = parallel.process_batch(&test_images);
        
        assert_eq!(seq_results.len(), par_results.len());
        
        // Results should be identical regardless of processing mode
        for (seq_result, par_result) in seq_results.iter().zip(par_results.iter()) {
            assert_images_equal(seq_result, par_result);
        }
    }
}

#[cfg(test)]
mod memory_pool_tests {
    use super::*;

    #[test]
    fn test_memory_pool_basic_allocation() {
        let mut pool = MemoryPool::new(1024, 10);
        
        let buffer1 = pool.get_buffer(512);
        let buffer2 = pool.get_buffer(1024);
        
        assert_eq!(buffer1.len(), 512);
        assert_eq!(buffer2.len(), 1024);
    }

    #[test]
    fn test_memory_pool_reuse() {
        let mut pool = MemoryPool::new(1024, 5);
        
        // Allocate and release several buffers
        for _ in 0..10 {
            let buffer = pool.get_buffer(512);
            assert_eq!(buffer.len(), 512);
            // Buffer is automatically returned to pool when dropped
        }
        
        // Pool should handle reuse correctly
        let final_buffer = pool.get_buffer(1024);
        assert_eq!(final_buffer.len(), 1024);
    }

    #[test]
    fn test_memory_pool_oversized_requests() {
        let mut pool = MemoryPool::new(1024, 5);
        
        // Request larger than pool buffer size
        let large_buffer = pool.get_buffer(2048);
        assert_eq!(large_buffer.len(), 2048);
        
        // Pool should still work for normal sized requests
        let normal_buffer = pool.get_buffer(512);
        assert_eq!(normal_buffer.len(), 512);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_processing_pipeline() {
        let processor = ImageProcessor::new();
        let pipeline = ProcessingPipeline::new_parallel(2);
        
        let test_images = vec![
            create_test_image(100, 100),
            create_test_image(200, 150),
            create_test_image(150, 200),
        ];
        
        // Process through pipeline
        let pipeline_results = pipeline.process_batch(&test_images);
        
        // Process individually
        let individual_results: Vec<_> = test_images
            .iter()
            .map(|img| processor.process_image(img))
            .collect();
        
        // Results should be equivalent
        assert_eq!(pipeline_results.len(), individual_results.len());
        
        for (pipeline_result, individual_result) in pipeline_results.iter().zip(individual_results.iter()) {
            assert_images_equal(pipeline_result, individual_result);
        }
    }

    #[test]
    fn test_performance_regression_thresholds() {
        use std::time::Instant;
        
        let processor = ImageProcessor::new();
        let test_image = create_test_image(500, 500);
        
        let start = Instant::now();
        let _result = processor.process_image(&test_image);
        let duration = start.elapsed();
        
        // Performance regression test - should process 500x500 image in reasonable time
        // This threshold should be updated as optimizations are made
        assert!(duration.as_millis() < 100, 
               "Image processing took too long: {:?}ms", duration.as_millis());
    }

    #[test]
    fn test_memory_usage_bounds() {
        let pipeline = ProcessingPipeline::new_parallel(4);
        let large_images = vec![create_test_image(1000, 1000); 10];
        
        // This test ensures that processing doesn't cause excessive memory usage
        // In a real implementation, you might use a memory profiler here
        let results = pipeline.process_batch(&large_images);
        
        assert_eq!(results.len(), large_images.len());
        
        // Verify all results are valid
        for (original, result) in large_images.iter().zip(results.iter()) {
            assert_eq!(result.width(), original.width());
            assert_eq!(result.height(), original.height());
        }
    }
}

// Helper functions for testing

fn create_test_image(width: u32, height: u32) -> TestImage {
    TestImage::new(width, height)
}

fn create_gradient_image(width: u32, height: u32) -> TestImage {
    let mut image = TestImage::new(width, height);
    
    // Create a gradient pattern for more interesting test data
    for y in 0..height {
        for x in 0..width {
            let intensity = ((x + y) * 255 / (width + height)) as u8;
            image.set_pixel(x, y, Pixel { r: intensity, g: intensity, b: intensity });
        }
    }
    
    image
}

fn create_test_image_with_edges(width: u32, height: u32) -> TestImage {
    let mut image = TestImage::new(width, height);
    
    // Create a pattern with clear edges for edge detection testing
    for y in 0..height {
        for x in 0..width {
            let value = if x < width / 2 { 50 } else { 200 };
            image.set_pixel(x, y, Pixel { r: value, g: value, b: value });
        }
    }
    
    image
}

fn calculate_pixel_sum(image: &TestImage) -> u64 {
    let mut sum = 0u64;
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            sum += pixel.r as u64 + pixel.g as u64 + pixel.b as u64;
        }
    }
    sum
}

fn images_identical(image1: &TestImage, image2: &TestImage) -> bool {
    if image1.width() != image2.width() || image1.height() != image2.height() {
        return false;
    }
    
    for y in 0..image1.height() {
        for x in 0..image1.width() {
            let pixel1 = image1.get_pixel(x, y);
            let pixel2 = image2.get_pixel(x, y);
            
            if pixel1.r != pixel2.r || pixel1.g != pixel2.g || pixel1.b != pixel2.b {
                return false;
            }
        }
    }
    
    true
}

fn assert_images_equal(image1: &TestImage, image2: &TestImage) {
    assert_eq!(image1.width(), image2.width());
    assert_eq!(image1.height(), image2.height());
    
    for y in 0..image1.height() {
        for x in 0..image1.width() {
            let pixel1 = image1.get_pixel(x, y);
            let pixel2 = image2.get_pixel(x, y);
            
            assert_eq!(pixel1.r, pixel2.r, "Red channel mismatch at ({}, {})", x, y);
            assert_eq!(pixel1.g, pixel2.g, "Green channel mismatch at ({}, {})", x, y);
            assert_eq!(pixel1.b, pixel2.b, "Blue channel mismatch at ({}, {})", x, y);
        }
    }
}

// Simplified test image structure (matches benchmark structure)
#[derive(Clone)]
pub struct TestImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl TestImage {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 3) as usize; // RGB
        let mut data = vec![0u8; size];
        
        // Fill with some pattern to make processing meaningful
        for i in 0..size {
            data[i] = (i % 256) as u8;
        }
        
        Self { width, height, data }
    }
    
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let idx = ((y * self.width + x) * 3) as usize;
        Pixel {
            r: self.data[idx],
            g: self.data[idx + 1],
            b: self.data[idx + 2],
        }
    }
    
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        let idx = ((y * self.width + x) * 3) as usize;
        self.data[idx] = pixel.r;
        self.data[idx + 1] = pixel.g;
        self.data[idx + 2] = pixel.b;
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}