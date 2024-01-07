Утилита для удаления дубликатов файлов
Этот скрипт на языке программирования Rust предназначен для удаления дубликатов файлов в указанной пользователем папке. Он использует хэш-функцию для вычисления уникального идентификатора каждого файла и затем проверяет, есть ли файл с таким идентификатором уже в папке. Если найден дубликат, скрипт удаляет его.

Как использовать
Зависимости

Убедитесь, что у вас установлен компилятор Rust.
Зависимости указаны в коде и автоматически устанавливаются при сборке.
Компиляция

Запустите cargo build --release в терминале для компиляции в режиме release.
Запуск

Запустите скрипт, предоставив путь к целевой папке, например:
```bash
./target/release/удаление_дубликатов
```
Следуйте инструкциям, введя путь к целевой папке.
Примечание
Скрипт рассчитан на использование в терминале.
Перед удалением дубликата файлы не перемещаются в корзину, так что будьте осторожны при использовании.

