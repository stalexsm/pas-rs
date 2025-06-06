## 0.3.0 (2024-01-26)

### Feat

- Добаление multiselect

### Fix

- Правки в модальном окне
- Правки цвета для выпадающего списка в multiselect
- Правки в верстке и текстах для multiselect
- Доработка multiselect по дизайну
- Правки для multiselect по верстке
- Правки в верстке multiselect

### Refactor

- Внесение изменений в toml
- Изменения по обработке get параметров в  статистики
- Доработал placeholder в multiselect

## 0.2.0 (2024-01-22)

### Feat

- Добавление метода для удаления в производство
- Добавил в аналитику пользователя. Подсчет идет по пользователю и товару
- Добавление логотипа
- Добавление скачивания отчета
- Добавление метода для получения файла отчета
- Добавление генерации  excel для отчета аналитики
- Добавил вертикальный скрол в таблицы, вернул цветную корректировку
- Добавление фильтров в аналитику
- Добавление возможности просмотра пароля при авторизации
- Добавил возможность просмотра пароля при изменении
- Добавил футер в приложение
- Добавление в env.example переменной для указания DOMAIN_API
- Добавил возможность в env добавлять DOMAIN для API
- Добавление новой роли разработчика
- Init

### Fix

- Правки в верстке
- Правки чтения env файла
- Доработка фильтров для мобильной версии
- Доработка отобраджения времени учитывая часовой пояс
- Правки футера
- Правки в обработке ролевой, переделал обработку через enum
- Правки в подготовке базы для внедрения организаций
- Правки label поля для создания пользователя
- Внес правки для таблиц: overflow-auto и table-auto. И изменение PER_PAGE 5 -> 8
- Изменение получения переменных окружения при компиляции
- Внес правки в deploy и в backend
- Внес изменения в миграцию и закрыл редактирование учетной записи рахработчика

### Refactor

- Изменил возврат из функции. Подготовка для скачивания файла на бекенде
- Подготовка к внедрению организаций
- Подготовка к внедрению организаций в проект
