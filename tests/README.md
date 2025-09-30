# Integration Tests

Эта папка содержит integration тесты для проекта tui_stat.

## Структура

- `memory_integration_test.rs` - Тесты для модуля сбора информации о памяти
- `types_test.rs` - Тесты для структур данных проекта

## Запуск тестов

### Все тесты (unit + integration)
```bash
cargo test
```

### Только unit тесты
```bash
cargo test --lib
```

### Только integration тесты
```bash
cargo test --test
```

### Конкретный integration test файл
```bash
cargo test --test memory_integration_test
cargo test --test types_test
```

### С подробным выводом
```bash
cargo test -- --nocapture
cargo test -- --show-output
```

## Покрытие тестами

**Unit тесты (15):**
- `src/system/memory.rs` - 4 теста
  - Конверсия байтов в гигабайты
  - Проверка структуры данных RAM
  - Расчет использования памяти
  - Default trait

- `src/components/mod.rs` - 6 тестов
  - Валидация constraints
  - Генерация chunks
  - Центрирование rect

- `src/app.rs` - 5 тестов
  - Форматирование текста RAM
  - Цветовая индикация
  - Граничные случаи

**Integration тесты (9):**
- `memory_integration_test.rs` - 4 теста
  - Интеграционные тесты RAM collector
  - Множественные вызовы
  - Консистентность данных

- `types_test.rs` - 5 тестов
  - Структуры данных (RamData, GeoData, WeatherInfo)
  - Clone trait

**Итого: 24 теста**

## Результаты последнего запуска

```
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured
```

## Добавление новых тестов

### Unit тесты
Добавляйте в конец соответствующего файла в модуле `#[cfg(test)] mod tests { ... }`

### Integration тесты
Создавайте новый файл `tests/имя_теста.rs`
