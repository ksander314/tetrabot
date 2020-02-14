# tetrabot
Telegram bot. Translate from en to ru. You need to get yandex dictionary api key here https://tech.yandex.com/keys/get/?service=dict

To run from russia tor must be used. For example over proxychains.

env TELEGRAM_BOT_TOKEN=xxx PROXYCHAINS_SOCKS5=9050 proxychains cargo run <yandex.dictionary.api.token>
