// Этот файл - "лицо" вашей библиотеки.
// Он делает скрытый сгенерированный код доступным для внешнего мира.

pub mod my_exchange {
    // Магия та же: мы включаем файл из скрытой папки OUT_DIR
    include!(concat!(env!("OUT_DIR"), "/my_exchange.rs"));
}

pub mod geyser {
    // Магия та же: мы включаем файл из скрытой папки OUT_DIR
    include!(concat!(env!("OUT_DIR"), "/geyser.rs"));
}

// Опционально: можно сделать ре-экспорт, чтобы пользователи писали
// use proto_types::Trade; вместо use proto_types::my_exchange::Trade;
pub use my_exchange::*;
pub use geyser::*;