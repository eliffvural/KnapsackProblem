use std::time::Instant;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;

fn knapsack(n: usize, W: usize, values: &Vec<i32>, weights: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut dp = vec![vec![0; W + 1]; n + 1];
    for i in 1..=n {
        for w in 0..=W {
            if weights[i - 1] as usize <= w {
                dp[i][w] = std::cmp::max(dp[i - 1][w], dp[i - 1][w - weights[i - 1] as usize] + values[i - 1]);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    let optimal_value = dp[n][W];
    let mut selected_items = vec![0; n];
    let mut w = W;
    for i in (1..=n).rev() {
        if dp[i][w] != dp[i - 1][w] {
            selected_items[i - 1] = 1;
            w -= weights[i - 1] as usize;
        }
    }
    (optimal_value, selected_items)
}

fn read_dataset(filename: &str) -> (usize, usize, Vec<i32>, Vec<i32>) {
    let file = File::open(filename).expect("Dosya açılamadı");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    let first_line = lines.next().expect("Dosyada veri yok").expect("Satır okunamadı");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let n: usize = parts[0].parse().expect("Eşya sayısı parse edilemedi");
    let W: usize = parts[1].parse().expect("Kapasite parse edilemedi");

    let mut values = Vec::new();
    let mut weights = Vec::new();
    for line in lines {
        let line = line.expect("Satır okunamadı");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            values.push(parts[0].parse::<i32>().expect("Değer parse edilemedi"));
            weights.push(parts[1].parse::<i32>().expect("Ağırlık parse edilemedi"));
        }
    }
    (n, W, values, weights)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Ödevin örnek verisi
    let example_n = 4;
    let example_W = 11;
    let example_values = vec![8, 10, 15, 4];
    let example_weights = vec![4, 5, 8, 3];

    // Veri seti dosyaları
    let dataset_files = vec![
        "Dataset/ks_40_0",
        "Dataset/ks_300_0",
        "Dataset/ks_1000_0",
        "Dataset/ks_10000_0",
    ];

    // Sonuçları saklamak için vektörler
    let mut all_sizes = vec![example_n];
    let mut all_optimal_values = Vec::new();
    let mut all_selected_items = Vec::new();
    let mut all_runtimes = Vec::new();

    // Örnek veriyi çalıştır
    let start = Instant::now();
    let (optimal_value, selected_items) = knapsack(example_n, example_W, &example_values, &example_weights);
    let duration = start.elapsed().as_secs_f64();
    all_optimal_values.push(optimal_value);
    all_selected_items.push(selected_items);
    all_runtimes.push(duration);

    // Veri setlerini çalıştır
    for file in dataset_files.iter() {
        let (n, W, values, weights) = read_dataset(file);
        let start = Instant::now();
        let (optimal_value, selected_items) = knapsack(n, W, &values, &weights);
        let duration = start.elapsed().as_secs_f64();
        all_sizes.push(n);
        all_optimal_values.push(optimal_value);
        all_selected_items.push(selected_items);
        all_runtimes.push(duration);
    }

    // Terminal çıktısını bir string olarak oluştur
    let mut output = String::new();
    output.push_str("222802077, Elif Vural\n");
    output.push_str("Dosya Boyut\tOptimal Değer\tÇalışma Süresi (s)\tOptimal Çözüm\t\t\tOptimal Çözüme Dahil Edilen Itemler (TÜMÜ)\n");
    output.push_str("ÖRNEK\t4\t19\t\t-\t\t0,0,1,1\t\t3,4\n");

    for i in 0..all_sizes.len() {
        let size = all_sizes[i];
        let optimal_value = all_optimal_values[i];
        let runtime = all_runtimes[i];
        let selected = &all_selected_items[i];

        // Optimal çözümü formatla
        let selected_str: String = selected.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");

        // Seçilen itemlerin numaralarını formatla
        let mut selected_indices: Vec<String> = Vec::new();
        for j in 0..selected.len() {
            if selected[j] == 1 {
                selected_indices.push((j + 1).to_string());
            }
        }
        let selected_indices_str = selected_indices.join(",");

        // Satırı oluştur
        let line = if i == 0 {
            format!("SONUÇ\t{}\t{}\t\t{:.6}\t\t{}\t\t{}\n", size, optimal_value, runtime, selected_str, selected_indices_str)
        } else {
            format!("\t{}\t{}\t\t{:.6}\t\t{}\t\t{}\n", size, optimal_value, runtime, selected_str, selected_indices_str)
        };
        output.push_str(&line);
    }

    // Terminalde yazdır
    print!("{}", output);

    // Aynı çıktıyı runtimes.csv dosyasına yaz
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("runtimes.csv")?;
    file.write_all(output.as_bytes())?;
    file.flush()?;

    println!("\nSonuçlar 'runtimes.csv' dosyasına kaydedildi.");

    Ok(())
}