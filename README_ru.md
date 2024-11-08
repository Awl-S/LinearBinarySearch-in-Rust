
# Проект: Анализ производительности алгоритмов линейного и двоичного поиска на Rust

## Описание проекта

Данный проект реализует два алгоритма поиска — **линейный поиск** и **двоичный поиск**, и предоставляет их сравнительный анализ. Мы используем модульные тесты для проверки корректности реализации, а также замеряем время выполнения обоих алгоритмов на различных объемах данных. Проект может использоваться как учебный пример для понимания различий между алгоритмами с линейной и логарифмической сложностью, а также для изучения тестирования и замера производительности в Rust.

## Структура проекта

```plaintext
project-root/
├── src/
│   ├── main.rs                 # Главная точка входа в программу
│   ├── binary_search.rs         # Реализация алгоритма двоичного поиска
│   ├── linear_search.rs         # Реализация алгоритма линейного поиска
│   └── utils.rs                 # Вспомогательные функции (генерация данных, замер времени)
├── Cargo.toml                   # Конфигурационный файл Cargo
└── README.md                    # Документация проекта
```

## Установка

1. Склонируйте репозиторий на ваш локальный компьютер.
   ```bash
   git clone https://github.com/Awl-S/LinearBinarySearch-in-Rust.git
   cd LinearBinarySearch-in-Rust
   ```

2. Убедитесь, что Rust и Cargo установлены, затем запустите сборку.
   ```bash
   cargo build
   ```

## Запуск программы

Для запуска программы используйте команду:

```bash
cargo run
```

## Тестирование

В проекте реализованы модульные и интеграционные тесты для проверки работы алгоритмов поиска. Для запуска всех тестов выполните команду:

```bash
cargo test
```

## Использование алгоритмов

### Линейный поиск

Линейный поиск выполняется путем последовательного перебора всех элементов массива до нахождения искомого значения. Он имеет временную сложность **O(n)** и подходит для небольших и несортированных массивов.

### Двоичный поиск

Двоичный поиск требует предварительно отсортированного массива и использует стратегию деления на части, сокращая пространство поиска вдвое на каждом шаге. Временная сложность составляет **O(log n)**, что делает его эффективным для больших отсортированных массивов.

## Пример использования

Пример использования функций поиска и замера их времени в `main.rs`:

```rust
fn main() {
    let array = utils::generate_random_array(1000); // Генерация случайного массива
    let target = 50;

    // Линейный поиск
    let linear_result = linear_search::linear_search(&array, target);
    println!("Линейный поиск: результат = {:?}", linear_result);

    // Двоичный поиск (сначала сортируем массив)
    let mut sorted_array = array.clone();
    sorted_array.sort();
    let binary_result = binary_search::binary_search(&sorted_array, target);
    println!("Двоичный поиск: результат = {:?}", binary_result);
}
```

## Лицензия

Данный проект защищён авторским правом и предназначен исключительно для личного и учебного использования. Для коммерческого использования требуется предварительное письменное разрешение от автора. Если вы планируете использовать данный проект в научных публикациях, таких как конференции, доклады, статьи или научные работы, также прошу уведомить меня.

Обратите внимание, что лицензия, описанная в README, имеет приоритет над текстом, автоматически добавляемым в каждый файл исходного кода, и остаётся основной лицензией проекта. Все права защищены, а код предоставляется "как есть" без каких-либо гарантий, включая гарантии пригодности для определённой цели или коммерческой ценности.

---

Проект создан для демонстрации алгоритмов и практики работы с Rust.