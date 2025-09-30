use tui_stat::system::memory::System;

#[test]
fn test_system_collector_integration() {
    let mut system = System::new();
    let data = system.get_info();

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

    // Проверяем CPU данные
    assert!(data.cpu.len > 0, "System should have at least one CPU core");
}

#[test]
fn test_system_collector_multiple_calls() {
    println!("test_system_collector_multiple_calls");
    let mut system = System::new();

    // Первый вызов
    let data1 = system.get_info();
    dbg!(&data1);

    // Второй вызов - данные могут немного отличаться
    let data2 = system.get_info();
    dbg!(&data2);

    // Total memory не должна меняться между вызовами
    assert_eq!(
        data1.total_memory, data2.total_memory,
        "Total memory should remain constant"
    );

    // Used memory может меняться, но должна оставаться в разумных пределах
    assert!(data2.used_memory >= 0.0);

    // CPU информация не должна меняться
    assert_eq!(
        data1.cpu.len, data2.cpu.len,
        "CPU count should remain constant"
    );
    assert_eq!(
        data1.cpu.brand, data2.cpu.brand,
        "CPU brand should remain constant"
    );
}

#[test]
fn test_system_data_consistency() {
    let mut system = System::new();
    let data = system.get_info();

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
fn test_system_collector_default() {
    // Проверяем, что Default trait работает корректно
    let mut system = System::default();
    let data = system.get_info();

    assert!(data.total_memory > 0.0);
    assert!(data.cpu.len > 0);
}

#[test]
fn test_cpu_info() {
    let mut system = System::new();
    let data = system.get_info();

    // Проверяем CPU данные
    assert!(data.cpu.len > 0, "Should have at least one CPU core");
    assert!(data.cpu.frequency > 0, "CPU frequency should be positive");
    assert!(!data.cpu.brand.is_empty(), "CPU brand should not be empty");
}
