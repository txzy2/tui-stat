use tui_stat::system::memory::Ram;

#[test]
fn test_ram_collector_integration() {
    let mut ram = Ram::new();
    let data = ram.get_ram_info();

    // Проверяем, что мы получаем реальные данные из системы
    assert!(
        data.total_memory > 0.0,
        "System should have some total memory"
    );
    assert!(
        data.used_memory >= 0.0,
        "Used memory should be non-negative"
    );
    assert!(
        data.available_memory >= 0.0,
        "Available memory should be non-negative"
    );
    assert!(
        data.usage_memory >= 0.0 && data.usage_memory <= 100.0,
        "Usage should be a valid percentage"
    );
}

#[test]
fn test_ram_collector_multiple_calls() {
    println!("test_ram_collector_multiple_calls");
    let mut ram = Ram::new();

    // Первый вызов
    let data1 = ram.get_ram_info();
    dbg!(&data1);

    // Второй вызов - данные могут немного отличаться
    let data2 = ram.get_ram_info();
    dbg!(&data2);

    // Total memory не должна меняться между вызовами
    assert_eq!(
        data1.total_memory, data2.total_memory,
        "Total memory should remain constant"
    );

    // Used memory может меняться, но должна оставаться в разумных пределах
    assert!(data2.used_memory >= 0.0);
}

#[test]
fn test_ram_data_consistency() {
    let mut ram = Ram::new();
    let data = ram.get_ram_info();

    // Проверяем математическую согласованность
    // used + available ≈ total (с некоторой погрешностью)
    let sum = data.used_memory + data.available_memory;
    let diff_percent = ((sum - data.total_memory).abs() / data.total_memory) * 100.0;

    assert!(
        diff_percent < 1.0,
        "Used + Available should approximately equal Total (difference: {}%)",
        diff_percent
    );
}

#[test]
fn test_ram_collector_default() {
    // Проверяем, что Default trait работает корректно
    let mut ram = Ram::default();
    let data = ram.get_ram_info();

    assert!(data.total_memory > 0.0);
}
