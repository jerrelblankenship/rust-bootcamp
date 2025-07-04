// This benchmark is INTENTIONALLY BROKEN to teach ecosystem integration!
// Your mission: Fix the dependencies to make benchmarking work

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use crate_ecosystem::{DataExporter, utils};
use serde_json;

// This will fail because criterion features might be missing
fn json_serialization_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_serialization");
    
    // Generate test data of different sizes
    for size in [10, 100, 1000].iter() {
        let users: Vec<crate_ecosystem::User> = (0..*size)
            .map(|_| utils::generate_random_user())
            .collect();
        
        group.bench_with_input(
            BenchmarkId::new("serde_json_to_string", size),
            &users,
            |b, users| {
                b.iter(|| {
                    let _json = serde_json::to_string(black_box(users)).unwrap();
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("data_exporter_to_json", size),
            &users,
            |b, users| {
                b.iter(|| {
                    let _json = DataExporter::to_json(black_box(users)).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

// This will fail because csv crate is missing
fn csv_serialization_benchmark(c: &mut Criterion) {
    let users: Vec<crate_ecosystem::User> = (0..1000)
        .map(|_| utils::generate_random_user())
        .collect();
    
    c.bench_function("csv_serialization", |b| {
        b.iter(|| {
            let _csv = DataExporter::to_csv(black_box(&users)).unwrap();
        });
    });
}

// This will fail because serde_yaml crate is missing  
fn yaml_serialization_benchmark(c: &mut Criterion) {
    let users: Vec<crate_ecosystem::User> = (0..100)
        .map(|_| utils::generate_random_user())
        .collect();
    
    c.bench_function("yaml_serialization", |b| {
        b.iter(|| {
            let _yaml = DataExporter::to_yaml(black_box(&users)).unwrap();
        });
    });
}

// Benchmark user generation
fn user_generation_benchmark(c: &mut Criterion) {
    c.bench_function("generate_random_user", |b| {
        b.iter(|| {
            let _user = utils::generate_random_user();
        });
    });
    
    c.bench_function("generate_1000_users", |b| {
        b.iter(|| {
            let _users: Vec<crate_ecosystem::User> = (0..1000)
                .map(|_| utils::generate_random_user())
                .collect();
        });
    });
}

// This will fail because email validation requires regex crate
fn email_validation_benchmark(c: &mut Criterion) {
    let emails = vec![
        "valid@example.com",
        "also.valid@example.org", 
        "invalid.email",
        "another@invalid",
        "user@domain.co.uk",
        "bad@email@domain.com",
    ];
    
    c.bench_function("email_validation", |b| {
        b.iter(|| {
            for email in &emails {
                let _is_valid = utils::validate_email(black_box(email));
            }
        });
    });
}

// This will fail because bcrypt crate is missing
fn password_hashing_benchmark(c: &mut Criterion) {
    let password = "test_password_123";
    
    c.bench_function("password_hashing", |b| {
        b.iter(|| {
            let _hash = utils::hash_password(black_box(password)).unwrap();
        });
    });
    
    // Benchmark password verification
    let hash = utils::hash_password(password).unwrap();
    c.bench_function("password_verification", |b| {
        b.iter(|| {
            let _is_valid = utils::verify_password(black_box(password), black_box(&hash)).unwrap();
        });
    });
}

// UUID generation benchmark
fn uuid_generation_benchmark(c: &mut Criterion) {
    c.bench_function("uuid_v4_generation", |b| {
        b.iter(|| {
            let _uuid = uuid::Uuid::new_v4(); // This will fail if v4 feature is missing
        });
    });
    
    c.bench_function("1000_uuid_generation", |b| {
        b.iter(|| {
            let _uuids: Vec<uuid::Uuid> = (0..1000)
                .map(|_| uuid::Uuid::new_v4())
                .collect();
        });
    });
}

// Datetime operations benchmark
fn datetime_benchmark(c: &mut Criterion) {
    c.bench_function("utc_now", |b| {
        b.iter(|| {
            let _now = chrono::Utc::now();
        });
    });
    
    // This will fail if chrono serde feature is missing
    c.bench_function("datetime_serialization", |b| {
        let now = chrono::Utc::now();
        b.iter(|| {
            let _json = serde_json::to_string(black_box(&now)).unwrap();
        });
    });
}

// HTTP client benchmark (will fail because reqwest features are missing)
fn http_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("http_client_creation", |b| {
        b.iter(|| {
            let _client = reqwest::Client::new();
        });
    });
    
    // This is commented out because it requires network access
    // c.bench_function("http_get_request", |b| {
    //     b.to_async(&rt).iter(|| async {
    //         let client = reqwest::Client::new();
    //         let _response = client.get("https://httpbin.org/json")
    //             .send()
    //             .await
    //             .unwrap();
    //     });
    // });
}

criterion_group!(
    benches,
    json_serialization_benchmark,
    csv_serialization_benchmark,
    yaml_serialization_benchmark,
    user_generation_benchmark,
    email_validation_benchmark,
    password_hashing_benchmark,
    uuid_generation_benchmark,
    datetime_benchmark,
    http_benchmark
);
criterion_main!(benches);