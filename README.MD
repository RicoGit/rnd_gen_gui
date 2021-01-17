## Курсовая работа

### Разработка и исследование генераторов псевдослучайных числовых последовательностей

Выполнена на языке программирования Rust c использованием технологии [webview](https://github.com/webview/webview)


## Сборка проекта (на Linux)

Требует установленный cargo

    cargo build
    
На Ubuntu на сборки дополнительно потребовалось поставить подолнительные пакеты
    
    sudo apt-get install libcairo2-dev libjpeg-dev libpango1.0-dev libgif-dev build-essential
    sudo apt-get install libgtk2.0-dev libglib2.0-dev
    sudo apt-get install libatk1.0-dev libsoup2.4-dev
    sudo apt-get install libgtk-3-dev 
    sudo apt-get install libwebkit2gtk-4.0-dev

## Сборка проекта (на Windows)

Требует установленный cargo

    cargo build
  
---
    
**Полезные ссылки**

[rust webview bindings library](https://github.com/Boscop/web-view)
