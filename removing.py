from tkinter.filedialog import askdirectory # метод для вывода окна выбора папки
from tkinter import Tk
import os # этот модуль помогает нам удалять дубликаты файлов
import hashlib # чтобы использовать хеш-функцию md5, импортируем данную библиотеку
from pathlib import Path

# Мы не хотим, чтобы окно GUI
# tkinter появится на нашем экране
Tk().withdraw() #берем withdraw для того чтобы появлялось только окно для выбора папки
file_path = askdirectory(title="Select a folder") # Диалоговое окно для выбора папки.
list_of_files = os.walk(file_path) # Список всех файлов внутри нашей корневой папки.

# Чтобы обнаружить дубликат  файлы, в которых мы собираемся определить пустой словарь.
unique_files = dict()

for root, folders, files in list_of_files:
    for file in files:                       # Запуск цикла for для всех файлов
        file_path = Path(os.path.join(root, file)) # Поиск полного пути к файлу
        # Конвертация всего содержимого наш файл в хэш md5.
        Hash_file = hashlib.md5(open(file_path, 'rb').read()).hexdigest()

        # Если хэш файла уже есть, был добавлен, мы просто удалим этот файл
        if Hash_file not in unique_files:
            unique_files[Hash_file] = file_path
        else:
            os.remove(file_path)
            print(f"{file_path} был удален")
