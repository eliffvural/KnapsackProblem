use rand::Rng;
use std::time::Instant;
use csv::Writer;
use std::error::Error;

fn knapsack(n: usize, W: usize, values: &Vec<i32>, weights: &Vec<i32>) -> (i32, Vec<i32>) {
    // DP tablosu oluştur (n+1 x W+1 boyutunda)
    let mut dp = vec![vec![0; W + 1]; n + 1];

    // DP tablosunu doldur
    for i in 1..=n {
        for w in 0..=W {
            if weights[i - 1] as usize <= w {
                dp[i][w] = std::cmp::max(dp[i - 1][w], dp[i - 1][w - weights[i - 1] as usize] + values[i - 1]);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }

    // Optimal değeri bul
    let optimal_value = dp[n][W];

    // Seçilen eşyaları bul (geri izleme)
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

fn generate_data(num_items: usize, max_weight: usize) -> (Vec<i32>, Vec<i32>) {
    let mut rng = rand::thread_rng();
    let values: Vec<i32> = (0..num_items).map(|_| rng.gen_range(1..=100)).collect();
    let weights: Vec<i32> = (0..num_items).map(|_| rng.gen_range(1..=max_weight as i32)).collect();
    (values, weights)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Ödevin örnek verisi
    let example_n = 4;
    let example_W = 11;
    let example_values = vec![8, 10, 15, 4];
    let example_weights = vec![4, 5, 8, 3];

    // Diğer boyutlar için veri setleri
    let sizes = vec![40, 300, 1000, 10000];
    let max_weight = 1000;

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

    // Diğer boyutlar için çalıştır
    for &size in sizes.iter() {
        let (values, weights) = generate_data(size, max_weight);
        let start = Instant::now();
        let (optimal_value, selected_items) = knapsack(size, max_weight, &values, &weights);
        let duration = start.elapsed().as_secs_f64();
        all_sizes.push(size);
        all_optimal_values.push(optimal_value);
        all_selected_items.push(selected_items);
        all_runtimes.push(duration);
    }

    // Tabloyu yazdır
    println!("Öğrenci Numarası, Ad Soyad");
    println!("Dosya Boyut\tOptimal Değer\tOptimal Çözüm\t\tOptimal Çözüme Dahil Edilen Itemler (TÜMÜ)");
    println!("\t\t\t(itemler arasında SADECE bir virgül bırakılmalıdır, virgül dışında herhangi bir karakter kullanılmamalıdır)\t(itemler arasında SADECE bir virgül bırakılmalıdır, virgül dışında herhangi bir karakter kullanılmamalıdır)");
    println!("ÖRNEK\t4\t19\t\t0,0,1,1\t\t3,4");
    for i in 0..all_sizes.len() {
        let size = all_sizes[i];
        let optimal_value = all_optimal_values[i];
        let selected = &all_selected_items[i];
        let runtime = all_runtimes[i];

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

        // Satırı yazdır
        if i == 0 {
            println!("SONUÇ\t{}\t{}\t\t{}\t\t{}", size, optimal_value, selected_str, selected_indices_str);
        } else {
            println!("\t{}\t{}\t\t{}\t\t{}", size, optimal_value, selected_str, selected_indices_str);
        }
    }

    // CSV dosyasına yaz
    let mut wtr = Writer::from_path("runtimes.csv")?;
    wtr.write_record(&["Dosya Boyutu", "Çalışma Süresi (saniye)"])?;
    for (size, runtime) in all_sizes.iter().zip(all_runtimes.iter()) {
        wtr.write_record(&[size.to_string(), runtime.to_string()])?;
    }
    wtr.flush()?;
    println!("\nSüreler 'runtimes.csv' dosyasına kaydedildi.");

    Ok(())
}