// Image Processor - Performance Optimization Challenge
// 
// CURRENT STATUS: 🐌 CATASTROPHICALLY SLOW (8 images/sec, target: 100+/sec)
//
// Your mission: Fix the performance disasters in this image processing pipeline!
// This should process 100+ images per second, but multiple bottlenecks kill performance.
//
// PERFORMANCE DISASTERS TO FIX:
// 1. Allocation storm in main processing loop
// 2. Single-threaded processing (should be parallel)
// 3. Inefficient file I/O patterns
// 4. Missing SIMD optimizations
// 5. Poor memory access patterns
// 6. Unnecessary data copying

use clap::{Arg, Command};
use std::path::PathBuf;
use std::time::Instant;

mod allocator;
mod filters;
mod pipeline;
mod processor;

use processor::ImageProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Image Processor Optimization Challenge")
        .version("1.0")
        .about("Process images with terrible performance - YOUR JOB: Make it fast!")
        .arg(
            Arg::new("input")
                .help("Input directory containing images")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Output directory (default: processed/)")
                .default_value("processed"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show detailed performance metrics")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("memory-profile")
                .long("memory-profile")
                .help("Enable memory usage profiling")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_dir = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let verbose = matches.get_flag("verbose");
    let memory_profile = matches.get_flag("memory-profile");

    if !input_dir.exists() {
        eprintln!("❌ Input directory doesn't exist: {}", input_dir.display());
        std::process::exit(1);
    }

    println!("🚀 Image Processor Optimization Challenge");
    println!("📁 Input:  {}", input_dir.display());
    println!("📂 Output: {}", output_dir.display());
    println!("🎯 Target: 100+ images/second\n");

    // Create output directory
    std::fs::create_dir_all(&output_dir)?;

    // Initialize the processor (PERFORMANCE DISASTER INSIDE!)
    let mut processor = ImageProcessor::new(memory_profile);

    // Find all image files
    let image_files = find_image_files(&input_dir)?;
    println!("📸 Found {} images to process", image_files.len());

    if image_files.is_empty() {
        println!("⚠️  No image files found in {}", input_dir.display());
        return Ok(());
    }

    // Start processing and measure performance
    let start_time = Instant::now();
    let mut processed_count = 0;
    let mut total_bytes = 0u64;

    println!("\n🔄 Processing images...");
    
    // PERFORMANCE DISASTER: This loop contains multiple bottlenecks!
    for (i, image_path) in image_files.iter().enumerate() {
        let image_start = Instant::now();
        
        // Process single image (MULTIPLE PERFORMANCE ISSUES INSIDE!)
        match processor.process_image(image_path, &output_dir) {
            Ok(bytes_processed) => {
                processed_count += 1;
                total_bytes += bytes_processed;
                
                let image_time = image_start.elapsed();
                
                if verbose {
                    let throughput = if image_time.as_secs_f64() > 0.0 {
                        1.0 / image_time.as_secs_f64()
                    } else {
                        0.0
                    };
                    
                    println!(
                        "✅ [{:3}/{}] {} ({:.1} MB) - {:.1}ms ({:.1} imgs/sec)",
                        i + 1,
                        image_files.len(),
                        image_path.file_name().unwrap().to_string_lossy(),
                        bytes_processed as f64 / (1024.0 * 1024.0),
                        image_time.as_millis(),
                        throughput
                    );
                }
                
                // Memory profiling
                if memory_profile && (i + 1) % 10 == 0 {
                    let memory_mb = get_memory_usage() / (1024 * 1024);
                    println!("📊 Memory usage: {} MB", memory_mb);
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to process {}: {}", image_path.display(), e);
            }
        }
    }

    // Calculate final performance metrics
    let total_time = start_time.elapsed();
    let throughput = processed_count as f64 / total_time.as_secs_f64();
    let avg_time_per_image = total_time.as_millis() / processed_count.max(1) as u128;
    let total_mb = total_bytes as f64 / (1024.0 * 1024.0);
    let data_throughput = total_mb / total_time.as_secs_f64();

    println!("\n📊 PERFORMANCE RESULTS:");
    println!("─────────────────────────────────────");
    
    // Show current (bad) performance
    println!("🐌 CURRENT PERFORMANCE:");
    println!("   Images processed: {}", processed_count);
    println!("   Total time: {:.2}s", total_time.as_secs_f64());
    println!("   Throughput: {:.1} images/second", throughput);
    println!("   Avg time per image: {}ms", avg_time_per_image);
    println!("   Data processed: {:.1} MB", total_mb);
    println!("   Data throughput: {:.1} MB/s", data_throughput);
    
    if memory_profile {
        let final_memory = get_memory_usage() / (1024 * 1024);
        println!("   Peak memory: {} MB", final_memory);
    }
    
    println!("\n🎯 TARGET PERFORMANCE:");
    println!("   Throughput: 100+ images/second");
    println!("   Avg time per image: <10ms");
    println!("   Memory usage: <100MB");
    
    // Show how far we are from target
    println!("\n📈 IMPROVEMENT NEEDED:");
    let speed_improvement = 100.0 / throughput;
    let time_improvement = avg_time_per_image as f64 / 10.0;
    
    if throughput < 100.0 {
        println!("   Speed: {:.1}x improvement needed", speed_improvement);
    } else {
        println!("   Speed: ✅ TARGET ACHIEVED!");
    }
    
    if avg_time_per_image > 10 {
        println!("   Latency: {:.1}x improvement needed", time_improvement);
    } else {
        println!("   Latency: ✅ TARGET ACHIEVED!");
    }
    
    // Show success or failure
    if throughput >= 100.0 && avg_time_per_image <= 10 {
        println!("\n🎉 OPTIMIZATION SUCCESS!");
        println!("You've achieved professional-grade image processing performance!");
    } else {
        println!("\n🔧 OPTIMIZATION NEEDED:");
        println!("Profile the code and fix the performance bottlenecks!");
        println!("Hints available in hints/ directory");
    }

    Ok(())
}

fn find_image_files(dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut image_files = Vec::new();
    
    // PERFORMANCE ISSUE: This could be optimized for large directories
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "bmp" | "tiff" | "tif") {
                image_files.push(path);
            }
        }
    }
    
    // Sort for consistent processing order
    image_files.sort();
    Ok(image_files)
}

fn get_memory_usage() -> u64 {
    // Simple memory usage estimation (platform-specific implementation would be better)
    // This is a placeholder - real implementation would use system APIs
    use std::alloc::{GlobalAlloc, Layout, System};
    
    // For demonstration purposes, return a mock value
    // In real implementation, you'd use platform-specific APIs to get actual memory usage
    42 * 1024 * 1024 // Mock 42MB
}

/*
PERFORMANCE DISASTERS IN THIS FILE:
1. Single-threaded image processing loop → Should use parallel processing
2. Blocking file I/O operations → Could use async I/O for better throughput
3. No connection pooling or resource reuse → Each image processed independently
4. Memory usage not tracked properly → Should monitor actual allocations
5. No batching of operations → Process images one by one instead of batches

TO FIX THIS FILE:
- Use rayon::par_iter() for parallel processing
- Batch process multiple images
- Add proper memory profiling
- Use buffered I/O operations
- Consider async processing for I/O bound operations

The real performance disasters are in the processor module!
*/