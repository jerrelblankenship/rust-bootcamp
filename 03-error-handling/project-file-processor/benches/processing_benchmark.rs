// benches/processing_benchmark.rs
//
// Performance benchmarks for the file processor
// Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use file_processor::{FileProcessorEngine, ProcessingOptions, OutputFormat};
use std::fs;
use tempfile::TempDir;

/// Benchmark JSON processing with different file sizes
fn bench_json_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    let mut group = c.benchmark_group("json_processing");
    
    // Test different JSON file sizes
    let sizes = vec![100, 500, 1000, 5000];
    
    for size in sizes {
        let json_file = temp_dir.path().join(format!("test_{}.json", size));
        
        // Generate JSON with array of objects
        let mut json_content = String::from("[\n");
        for i in 0..size {
            if i > 0 {
                json_content.push_str(",\n");
            }
            json_content.push_str(&format!(
                r#"  {{"id": {}, "name": "User{}", "email": "user{}@example.com", "age": {}, "active": {}}}"#,
                i, i, i, 20 + (i % 50), i % 2 == 0
            ));
        }
        json_content.push_str("\n]");
        
        fs::write(&json_file, &json_content).expect("Failed to write JSON file");
        
        group.bench_with_input(
            BenchmarkId::new("records", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let result = engine.process_file(
                        black_box(json_file.to_str().unwrap()),
                        black_box(&options)
                    );
                    black_box(result).expect("Processing should succeed")
                })
            }
        );
    }
    
    group.finish();
}

/// Benchmark CSV processing with different file sizes
fn bench_csv_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    let mut group = c.benchmark_group("csv_processing");
    
    let sizes = vec![100, 500, 1000, 5000];
    
    for size in sizes {
        let csv_file = temp_dir.path().join(format!("test_{}.csv", size));
        
        // Generate CSV content
        let mut csv_content = String::from("id,name,email,age,department,salary,active\n");
        for i in 0..size {
            csv_content.push_str(&format!(
                "{},User{},user{}@company.com,{},Engineering,{},{}\n",
                i, i, i, 20 + (i % 50), 50000 + (i * 1000), i % 2 == 0
            ));
        }
        
        fs::write(&csv_file, &csv_content).expect("Failed to write CSV file");
        
        group.bench_with_input(
            BenchmarkId::new("records", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let result = engine.process_file(
                        black_box(csv_file.to_str().unwrap()),
                        black_box(&options)
                    );
                    black_box(result).expect("Processing should succeed")
                })
            }
        );
    }
    
    group.finish();
}

/// Benchmark batch processing with multiple files
fn bench_batch_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    let mut group = c.benchmark_group("batch_processing");
    
    let file_counts = vec![5, 10, 20, 50];
    
    for file_count in file_counts {
        let mut files = Vec::new();
        
        // Create multiple small JSON files
        for i in 0..file_count {
            let json_file = temp_dir.path().join(format!("batch_{}.json", i));
            let json_content = format!(
                r#"{{"batch_id": {}, "data": [{{"value": {}}}, {{"value": {}}}]}}"#,
                i, i * 2, i * 2 + 1
            );
            fs::write(&json_file, &json_content).expect("Failed to write JSON file");
            files.push(json_file.to_str().unwrap().to_string());
        }
        
        group.bench_with_input(
            BenchmarkId::new("files", file_count),
            &file_count,
            |b, _| {
                b.iter(|| {
                    let result = engine.process_batch(
                        black_box(&files),
                        black_box(&options)
                    );
                    black_box(result)
                })
            }
        );
    }
    
    group.finish();
}

/// Benchmark text processing with different content types
fn bench_text_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    let mut group = c.benchmark_group("text_processing");
    
    let line_counts = vec![100, 500, 1000, 5000];
    
    for line_count in line_counts {
        let text_file = temp_dir.path().join(format!("test_{}.log", line_count));
        
        // Generate log-like content
        let mut text_content = String::new();
        for i in 0..line_count {
            let log_level = match i % 4 {
                0 => "INFO",
                1 => "WARN", 
                2 => "ERROR",
                _ => "DEBUG",
            };
            text_content.push_str(&format!(
                "2023-12-01 10:{:02}:{:02} {} Processing item {} with value {}\n",
                (i / 60) % 60, i % 60, log_level, i, i * 42
            ));
        }
        
        fs::write(&text_file, &text_content).expect("Failed to write text file");
        
        group.bench_with_input(
            BenchmarkId::new("lines", line_count),
            &line_count,
            |b, _| {
                b.iter(|| {
                    let result = engine.process_file(
                        black_box(text_file.to_str().unwrap()),
                        black_box(&options)
                    );
                    black_box(result).expect("Processing should succeed")
                })
            }
        );
    }
    
    group.finish();
}

/// Benchmark different processing options
fn bench_processing_options(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let engine = FileProcessorEngine::new();
    
    // Create a medium-sized test file
    let csv_file = temp_dir.path().join("options_test.csv");
    let mut csv_content = String::from("id,name,value,active\n");
    for i in 0..1000 {
        csv_content.push_str(&format!("{},Item{},{},{}\n", i, i, i * 10, i % 2 == 0));
    }
    fs::write(&csv_file, &csv_content).expect("Failed to write CSV file");
    
    let mut group = c.benchmark_group("processing_options");
    
    // Test with validation enabled
    let options_with_validation = ProcessingOptions {
        validate_data: true,
        ..Default::default()
    };
    
    group.bench_function("with_validation", |b| {
        b.iter(|| {
            let result = engine.process_file(
                black_box(csv_file.to_str().unwrap()),
                black_box(&options_with_validation)
            );
            black_box(result).expect("Processing should succeed")
        })
    });
    
    // Test with validation disabled
    let options_without_validation = ProcessingOptions {
        validate_data: false,
        ..Default::default()
    };
    
    group.bench_function("without_validation", |b| {
        b.iter(|| {
            let result = engine.process_file(
                black_box(csv_file.to_str().unwrap()),
                black_box(&options_without_validation)
            );
            black_box(result).expect("Processing should succeed")
        })
    });
    
    group.finish();
}

/// Benchmark report generation
fn bench_report_generation(c: &mut Criterion) {
    use file_processor::reporting::ReportGenerator;
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    // Create test files and process them
    let mut files = Vec::new();
    for i in 0..10 {
        let json_file = temp_dir.path().join(format!("report_test_{}.json", i));
        let json_content = format!(r#"{{"id": {}, "processed": true}}"#, i);
        fs::write(&json_file, &json_content).expect("Failed to write JSON file");
        files.push(json_file.to_str().unwrap().to_string());
    }
    
    let batch_result = engine.process_batch(&files, &options);
    let report_generator = ReportGenerator::new();
    
    let mut group = c.benchmark_group("report_generation");
    
    group.bench_function("generate_report", |b| {
        b.iter(|| {
            let report = report_generator.generate_report(
                black_box(&batch_result),
                black_box(1000)
            );
            black_box(report)
        })
    });
    
    // Benchmark saving reports in different formats
    let report = report_generator.generate_report(&batch_result, 1000);
    let output_file = temp_dir.path().join("benchmark_report.json");
    
    group.bench_function("save_json_report", |b| {
        b.iter(|| {
            let result = report_generator.save_report(
                black_box(&report),
                black_box(&output_file),
                black_box(&OutputFormat::Json)
            );
            black_box(result).expect("Report saving should succeed")
        })
    });
    
    group.bench_function("save_csv_report", |b| {
        b.iter(|| {
            let result = report_generator.save_report(
                black_box(&report),
                black_box(&output_file),
                black_box(&OutputFormat::Csv)
            );
            black_box(result).expect("Report saving should succeed")
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_json_processing,
    bench_csv_processing,
    bench_batch_processing,
    bench_text_processing,
    bench_processing_options,
    bench_report_generation
);

criterion_main!(benches);
