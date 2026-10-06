#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    English,
    PortugueseBrazil,
    Spanish,
    French,
    Russian,
}

pub struct Texts {
    pub home: &'static str,
    pub mods: &'static str,
    pub welcome_back: &'static str,
    pub player: &'static str,
    pub adventure: &'static str,
    pub play: &'static str,
    pub game_info: &'static str,
    pub version: &'static str,
    pub forge: &'static str,
    pub java: &'static str,
    pub status: &'static str,
    pub ready: &'static str,
    pub quick_access: &'static str,
    pub open_mods: &'static str,
    pub open_resourcepacks: &'static str,
    pub open_config: &'static str,
    pub open_saves: &'static str,
    pub theme: &'static str,
    pub theme_default: &'static str,
    pub theme_midnight: &'static str,
    pub theme_violet: &'static str,
    pub language: &'static str,
    pub latest_news: &'static str,
    pub news_description: &'static str,
    pub installed_mods: &'static str,
    pub refresh: &'static str,
    pub mods_refreshed: &'static str,
    pub no_mods: &'static str,
    pub search_mods: &'static str,
    pub launching: &'static str,
    pub game_running: &'static str,
    pub already_running: &'static str,
    pub game_closed: &'static str,
    pub launch_failed: &'static str,
    pub java_missing: &'static str,
    pub folder_failed: &'static str,
    pub mods_failed: &'static str,
    pub mods_hint: &'static str,
}

impl Language {
    pub const ALL: [Language; 5] = [
        Language::English,
        Language::PortugueseBrazil,
        Language::Spanish,
        Language::French,
        Language::Russian,
    ];

    pub fn from_code(code: &str) -> Self {
        match code {
            "pt-BR" => Self::PortugueseBrazil,
            "es" => Self::Spanish,
            "fr" => Self::French,
            "ru" => Self::Russian,
            _ => Self::English,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::PortugueseBrazil => "pt-BR",
            Self::Spanish => "es",
            Self::French => "fr",
            Self::Russian => "ru",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::PortugueseBrazil => "Português (Brasil)",
            Self::Spanish => "Español",
            Self::French => "Français",
            Self::Russian => "Русский",
        }
    }

    pub fn texts(self) -> &'static Texts {
        match self {
            Self::English => &ENGLISH,
            Self::PortugueseBrazil => &PORTUGUESE,
            Self::Spanish => &SPANISH,
            Self::French => &FRENCH,
            Self::Russian => &RUSSIAN,
        }
    }
}

const ENGLISH: Texts = Texts {
    home: "Home",
    mods: "Mods",
    welcome_back: "Welcome back,",
    player: "Player",
    adventure: "Ready for your next adventure?",
    play: "Play",
    game_info: "Game Info",
    version: "Version",
    forge: "Forge",
    java: "Java",
    status: "Status",
    ready: "Ready",
    quick_access: "Quick Access",
    open_mods: "Open Mods Folder",
    open_resourcepacks: "Open Resource Packs Folder",
    open_config: "Open Config Folder",
    open_saves: "Open Saves Folder",
    theme: "Theme",
    theme_default: "Default",
    theme_midnight: "Midnight",
    theme_violet: "Violet",
    language: "Language",
    latest_news: "Latest News",
    news_description: "- Added a native Rust launcher with a redesigned dark UI for Home and Mods.
- Added an installed-mod list with search, refresh, and a shortcut to the mods folder.
- Added English, Brazilian Portuguese, Spanish, French, and Russian translations.
- Added selectable themes and portable settings saved beside the launcher.
- Added one editable latest-news card, configured in `src/news.rs`.
- Added optional Home and news artwork.
- Added a loading indicator while Minecraft starts and launch-status feedback.
- Suppressed the Java console window when starting the game on Windows.",
    installed_mods: "installed mods",
    refresh: "Refresh",
    mods_refreshed: "Mod list refreshed.",
    no_mods: "No mods found",
    search_mods: "Search installed mods...",
    launching: "Starting Minecraft...",
    game_running: "Minecraft is running.",
    already_running: "Minecraft is already running.",
    game_closed: "Minecraft closed with exit code",
    launch_failed: "Could not start Minecraft",
    java_missing: "Bundled Java 8 was not found",
    folder_failed: "Could not open folder",
    mods_failed: "Could not read installed mods",
    mods_hint: "Place .jar files in the mods folder and refresh the list.",
};

const PORTUGUESE: Texts = Texts {
    home: "Início",
    mods: "Mods",
    welcome_back: "Boas-vindas,",
    player: "Jogador",
    adventure: "Pronto para sua próxima aventura?",
    play: "Jogar",
    game_info: "Informações do jogo",
    version: "Versão",
    forge: "Forge",
    java: "Java",
    status: "Status",
    ready: "Pronto",
    quick_access: "Acesso rápido",
    open_mods: "Abrir pasta de mods",
    open_resourcepacks: "Abrir pasta de resource packs",
    open_config: "Abrir pasta de configurações",
    open_saves: "Abrir pasta de mundos",
    theme: "Tema",
    theme_default: "Padrão",
    theme_midnight: "Meia-noite",
    theme_violet: "Violeta",
    language: "Idioma",
    latest_news: "Últimas notícias",
    news_description: "- Adicionado um launcher nativo em Rust com uma interface escura redesenhada para as telas Home e Mods.
- Adicionada uma lista de mods instalados com pesquisa, atualização e atalho para a pasta de mods.
- Adicionadas traduções para Inglês, Português do Brasil, Espanhol, Francês e Russo.
- Adicionados temas selecionáveis e configurações portáteis salvas ao lado do launcher.
- Adicionado um cartão de notícias mais recentes editável, configurado em `src/news.rs`.
- Adicionadas artes opcionais para a Home e para as notícias.
- Adicionado um indicador de carregamento enquanto o Minecraft é iniciado e feedback do status de inicialização.
- Suprimida a janela do console do Java ao iniciar o jogo no Windows.",
    installed_mods: "mods instalados",
    refresh: "Atualizar",
    mods_refreshed: "Lista de mods atualizada.",
    no_mods: "Nenhum mod encontrado",
    search_mods: "Buscar mods instalados...",
    launching: "Iniciando Minecraft...",
    game_running: "Minecraft está em execução.",
    already_running: "O Minecraft já está em execução.",
    game_closed: "Minecraft fechou com código de saída",
    launch_failed: "Não foi possível iniciar o Minecraft",
    java_missing: "O Java 8 incluído não foi encontrado",
    folder_failed: "Não foi possível abrir a pasta",
    mods_failed: "Não foi possível ler os mods instalados",
    mods_hint: "Coloque arquivos .jar na pasta mods e atualize a lista.",
};

const SPANISH: Texts = Texts {
    home: "Inicio",
    mods: "Mods",
    welcome_back: "Te damos la bienvenida,",
    player: "Jugador",
    adventure: "¿Listo para tu próxima aventura?",
    play: "Jugar",
    game_info: "Información del juego",
    version: "Versión",
    forge: "Forge",
    java: "Java",
    status: "Estado",
    ready: "Listo",
    quick_access: "Acceso rápido",
    open_mods: "Abrir carpeta de mods",
    open_resourcepacks: "Abrir carpeta de paquetes de recursos",
    open_config: "Abrir carpeta de configuración",
    open_saves: "Abrir carpeta de mundos",
    theme: "Tema",
    theme_default: "Predeterminado",
    theme_midnight: "Medianoche",
    theme_violet: "Violeta",
    language: "Idioma",
    latest_news: "Últimas noticias",
    news_description: "- Añadido un launcher nativo en Rust con una interfaz oscura rediseñada para las secciones Inicio y Mods.
- Añadida una lista de mods instalados con búsqueda, actualización y un acceso directo a la carpeta de mods.
- Añadidas traducciones al inglés, portugués brasileño, español, francés y ruso.
- Añadidos temas seleccionables y ajustes portátiles guardados junto al launcher.
- Añadida una tarjeta editable de últimas noticias, configurada en `src/news.rs`.
- Añadidas ilustraciones opcionales para Inicio y las noticias.
- Añadido un indicador de carga mientras Minecraft se inicia y comentarios sobre el estado del lanzamiento.
- Suprimida la ventana de consola de Java al iniciar el juego en Windows.
",
    installed_mods: "mods instalados",
    refresh: "Actualizar",
    mods_refreshed: "Lista de mods actualizada.",
    no_mods: "No se encontraron mods",
    search_mods: "Buscar mods instalados...",
    launching: "Iniciando Minecraft...",
    game_running: "Minecraft está en ejecución.",
    already_running: "Minecraft ya está en ejecución.",
    game_closed: "Minecraft se cerró con código de salida",
    launch_failed: "No se pudo iniciar Minecraft",
    java_missing: "No se encontró el Java 8 incluido",
    folder_failed: "No se pudo abrir la carpeta",
    mods_failed: "No se pudieron leer los mods instalados",
    mods_hint: "Coloca archivos .jar en la carpeta mods y actualiza la lista.",
};

const FRENCH: Texts = Texts {
    home: "Accueil",
    mods: "Mods",
    welcome_back: "Bon retour,",
    player: "Joueur",
    adventure: "Prêt pour votre prochaine aventure ?",
    play: "Jouer",
    game_info: "Informations du jeu",
    version: "Version",
    forge: "Forge",
    java: "Java",
    status: "Statut",
    ready: "Prêt",
    quick_access: "Accès rapide",
    open_mods: "Ouvrir le dossier des mods",
    open_resourcepacks: "Ouvrir le dossier des packs de ressources",
    open_config: "Ouvrir le dossier de configuration",
    open_saves: "Ouvrir le dossier des mondes",
    theme: "Thème",
    theme_default: "Par défaut",
    theme_midnight: "Minuit",
    theme_violet: "Violet",
    language: "Langue",
    latest_news: "Dernières actualités",
    news_description: "- Ajout d’un launcher natif en Rust avec une interface sombre repensée pour les sections Accueil et Mods.
- Ajout d’une liste des mods installés avec recherche, actualisation et raccourci vers le dossier des mods.
- Ajout des traductions en anglais, portugais brésilien, espagnol, français et russe.
- Ajout de thèmes sélectionnables et de paramètres portables enregistrés à côté du launcher.
- Ajout d’une carte des dernières actualités modifiable, configurée dans `src/news.rs`.
- Ajout d’illustrations facultatives pour l’Accueil et les actualités.
- Ajout d’un indicateur de chargement pendant le démarrage de Minecraft et d’un retour sur l’état du lancement.
- Suppression de la fenêtre de console Java lors du démarrage du jeu sous Windows.",
    installed_mods: "mods installés",
    refresh: "Actualiser",
    mods_refreshed: "Liste des mods actualisée.",
    no_mods: "Aucun mod trouvé",
    search_mods: "Rechercher des mods installés...",
    launching: "Démarrage de Minecraft...",
    game_running: "Minecraft est en cours d'exécution.",
    already_running: "Minecraft est déjà en cours d'exécution.",
    game_closed: "Minecraft s'est fermé avec le code de sortie",
    launch_failed: "Impossible de démarrer Minecraft",
    java_missing: "Le Java 8 fourni est introuvable",
    folder_failed: "Impossible d'ouvrir le dossier",
    mods_failed: "Impossible de lire les mods installés",
    mods_hint: "Placez des fichiers .jar dans le dossier mods et actualisez la liste.",
};

const RUSSIAN: Texts = Texts {
    home: "Главная",
    mods: "Моды",
    welcome_back: "С возвращением,",
    player: "Игрок",
    adventure: "Готовы к новому приключению?",
    play: "Играть",
    game_info: "Информация об игре",
    version: "Версия",
    forge: "Forge",
    java: "Java",
    status: "Статус",
    ready: "Готово",
    quick_access: "Быстрый доступ",
    open_mods: "Открыть папку модов",
    open_resourcepacks: "Открыть папку ресурсов",
    open_config: "Открыть папку настроек",
    open_saves: "Открыть папку миров",
    theme: "Тема",
    theme_default: "По умолчанию",
    theme_midnight: "Полночь",
    theme_violet: "Фиолетовая",
    language: "Язык",
    latest_news: "Последние новости",
    news_description: "- Добавлен нативный лаунчер на Rust с переработанным тёмным интерфейсом для разделов «Главная» и «Моды».
- Добавлен список установленных модов с поиском, обновлением и ярлыком для открытия папки модов.
- Добавлены переводы на английский, бразильский португальский, испанский, французский и русский языки.
- Добавлены выбираемые темы оформления и портативные настройки, сохраняемые рядом с лаунчером.
- Добавлена редактируемая карточка последних новостей, настраиваемая в `src/news.rs`.
- Добавлены необязательные иллюстрации для главной страницы и новостей.
- Добавлен индикатор загрузки во время запуска Minecraft и отображение статуса запуска.
- Скрыто окно консоли Java при запуске игры в Windows.",
    installed_mods: "установленных модов",
    refresh: "Обновить",
    mods_refreshed: "Список модов обновлён.",
    no_mods: "Моды не найдены",
    search_mods: "Поиск установленных модов...",
    launching: "Запуск Minecraft...",
    game_running: "Minecraft запущен.",
    already_running: "Minecraft уже запущен.",
    game_closed: "Minecraft закрыт с кодом выхода",
    launch_failed: "Не удалось запустить Minecraft",
    java_missing: "Встроенная Java 8 не найдена",
    folder_failed: "Не удалось открыть папку",
    mods_failed: "Не удалось прочитать установленные моды",
    mods_hint: "Поместите файлы .jar в папку mods и обновите список.",
};
