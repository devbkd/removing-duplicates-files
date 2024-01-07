use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn calculate_hash(file_path: &Path) -> Result<u64, io::Error> {
    let content = fs::read(file_path)?;
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    Ok(hasher.finish())
}

fn main() -> Result<(), io::Error> {
    // Запрос пользователя для выбора папки
    let file_path = {
        let mut s = String::new();
        println!("Выберите папку:");
        io::stdin().read_line(&mut s)?;
        s.trim()
    };

    let list_of_files = fs::read_dir(&file_path)?;

    // Создание HashMap для хранения уникальных файлов
    let mut unique_files: HashMap<u64, PathBuf> = HashMap::new();

    for entry in list_of_files {
        let entry = entry?;
        let file_path = entry.path();

        // Расчет хэша файла
        let hash = calculate_hash(&file_path)?;

        // Проверка наличия хэша в HashMap
        if !unique_files.contains_key(&hash) {
            unique_files.insert(hash, file_path.clone());
        } else {
            // Удаление файла, если он уже присутствует
            fs::remove_file(&file_path)?;
            println!("{} был удалён", file_path.display());
        }
    }

    Ok(())
}
