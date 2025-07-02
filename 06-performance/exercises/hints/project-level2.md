# Image Processor Project Hints - Level 2 (Specific Guidance)

## 🎯 Specific optimization strategies for the Image Processor...

### 🚀 Main.rs: Add Parallel Processing

**Problem**: Sequential image processing
```rust
// BAD: One image at a time
for (i, image_path) in image_files.iter().enumerate() {
    processor.process_image(image_path, &output_dir)?;
}
```

**Solution**: Use Rayon for parallel processing
```rust
use rayon::prelude::*;

// GOOD: Process multiple images simultaneously
image_files.par_iter().enumerate().try_for_each(|(i, image_path)| {
    processor.process_image(image_path, &output_dir)
})?;
```

### 🔧 Processor.rs: Fix Multiple Performance Issues

**Issue 1: Cache grows unbounded**
```rust
// BAD: Memory leak
cache.insert(cache_key, output_buffer.clone());
```
**Fix**: Add cache size limit or TTL

**Issue 2: Lock contention**  
```rust
// BAD: Global lock for every image
let mut cache = self.cache.lock().unwrap();
```
**Fix**: Use `DashMap` or thread-local caches

**Issue 3: Allocation in hot path**
```rust
// BAD: New Vec for every image
let mut output_buffer = Vec::new();
```
**Fix**: Use buffer pool with `Vec::with_capacity()`

**Issue 4: Unnecessary clones**
```rust
// BAD: Clone entire image
let mut processed_img = img.clone();
```
**Fix**: Work with references or move semantics

### 🎨 Filters.rs: Add SIMD Optimizations

**Problem**: Scalar pixel operations
```rust
// BAD: One pixel at a time
for pixel in pixels.chunks_mut(4) {
    pixel[0] = (pixel[0] as u16 * 110 / 100).min(255) as u8;
    pixel[1] = (pixel[1] as u16 * 110 / 100).min(255) as u8;
    pixel[2] = (pixel[2] as u16 * 110 / 100).min(255) as u8;
}
```

**Solution**: Process multiple pixels with SIMD
```rust
// GOOD: Process 4+ pixels simultaneously (pseudo-code)
use std::simd::*;
for pixels_chunk in pixels.chunks_exact_mut(16) { // 4 pixels × 4 channels
    let pixels_vec = u8x16::from_slice(pixels_chunk);
    let multiplied = pixels_vec * u8x16::splat(110) / u8x16::splat(100);
    multiplied.copy_to_slice(pixels_chunk);
}
```

### 🏊 Pipeline.rs: Streaming and Batching

**Add streaming processing**:
```rust
// Instead of loading entire image into memory
let reader = BufReader::new(File::open(path)?);
let mut decoder = image::codecs::jpeg::JpegDecoder::new(reader)?;
// Process in chunks/tiles instead of whole image
```

**Add batch processing**:
```rust
// Process multiple images in single batch
pub fn process_batch(&mut self, paths: &[PathBuf]) -> Result<Vec<u64>> {
    // Parallel processing with shared resources
    paths.par_iter().map(|path| self.process_single(path)).collect()
}
```

### 💾 Allocator.rs: Buffer Pool Implementation

**Add buffer reuse**:
```rust
pub struct BufferPool {
    image_buffers: Mutex<Vec<Vec<u8>>>,
    temp_buffers: Mutex<Vec<Vec<u8>>>,
}

impl BufferPool {
    pub fn get_image_buffer(&self, capacity: usize) -> Vec<u8> {
        self.image_buffers.lock().unwrap()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(capacity))
    }
    
    pub fn return_buffer(&self, mut buffer: Vec<u8>) {
        buffer.clear();
        if buffer.capacity() > 0 {
            self.image_buffers.lock().unwrap().push(buffer);
        }
    }
}
```

## 🔧 Specific Optimization Techniques

### 1. Memory Pool Pattern
```rust
// Reuse allocations across image processing
let mut buffer_pool = BufferPool::new();
for image in images {
    let mut work_buffer = buffer_pool.get_buffer(estimated_size);
    // ... process image
    buffer_pool.return_buffer(work_buffer);
}
```

### 2. SIMD Brightness Adjustment
```rust
fn adjust_brightness_simd(pixels: &mut [u8], factor: f32) {
    let factor_int = (factor * 256.0) as u16;
    
    for chunk in pixels.chunks_exact_mut(16) {
        // Process 16 bytes (4 pixels) at once
        for byte in chunk {
            *byte = ((*byte as u16 * factor_int) >> 8).min(255) as u8;
        }
    }
}
```

### 3. Cache-Friendly Convolution
```rust
fn convolve_cache_friendly(image: &[u8], width: usize, height: usize, kernel: &[i32]) {
    // Process in tiles to maximize cache usage
    const TILE_SIZE: usize = 64;
    
    for tile_y in (0..height).step_by(TILE_SIZE) {
        for tile_x in (0..width).step_by(TILE_SIZE) {
            // Process tile with good cache locality
            process_tile(image, tile_x, tile_y, TILE_SIZE, kernel);
        }
    }
}
```

## 📊 Expected Performance Improvements

After implementing these optimizations:
- **Parallel processing**: 4-8x improvement (depends on CPU cores)
- **Buffer reuse**: 2-3x improvement (reduce allocations)
- **SIMD operations**: 4-8x improvement (vectorized math)
- **Cache optimization**: 2-4x improvement (better memory access)
- **Algorithm improvements**: 2-5x improvement (faster filters)

**Total expected**: 50-100x improvement (8 imgs/sec → 400+ imgs/sec)

Need complete implementations? Check Level 3 hints! 🎯