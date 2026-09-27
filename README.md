# Rust Browser

Navegador web desktop minimalista escrito em Rust, com interface nativa **GTK4** e
renderização feita pelo **WebKitGTK 6.0**. Toda a aplicação ao redor da engine
(janela, toolbar, barra de endereço, navegação, atalhos e sincronização de estado)
é implementada em Rust — sem Electron, Tauri ou frontend web.

![Rust Browser exibindo a página inicial do DuckDuckGo](docs/screenshot.png)

## Funcionalidades

- Toolbar com **Voltar**, **Avançar**, **Recarregar**, **Home**, barra de endereço e
  indicador de carregamento (spinner).
- Barra de endereço inteligente:
  - `github.com` → `https://github.com`
  - URLs com `http://` ou `https://` são mantidas intactas
  - texto comum vira pesquisa: `Rust ownership` → `https://duckduckgo.com/?q=Rust%20ownership`
- Botões Voltar/Avançar habilitados somente quando existe histórico.
- Barra de endereço e título da janela sincronizados com a página
  (`Título da página — Rust Browser`).
- Abas: nova aba (botão `+` ou `Ctrl+T`), fechar pelo `✕` da aba ou `Ctrl+W`,
  alternar com `Ctrl+Tab`/`Ctrl+Shift+Tab` e reordenar arrastando. A toolbar e o
  título da janela acompanham a aba ativa; fechar a última aba fecha a janela.
- Links que pedem nova janela (`target="_blank"`) abrem em uma nova aba;
  pop-ups abertos sem interação do usuário são bloqueados.
- Falhas de carregamento exibem a página de erro do WebKit e são registradas no
  `stderr`, sem derrubar a aplicação.
- Na primeira página carregada, o navegador pergunta ao WebKit se consegue
  reproduzir os formatos de mídia essenciais da web (VP9, H.264, Opus, AAC) e
  avisa no `stderr` quais plugins do GStreamer instalar se faltar algum.
- Página inicial: `https://duckduckgo.com`.

## Requisitos

- Linux (desenvolvido e testado no Arch Linux, GNOME sobre Wayland)
- Rust stable (edition 2024, Rust ≥ 1.85)
- GTK ≥ 4.10
- WebKitGTK 6.0
- Plugins do GStreamer para áudio e vídeo (`gst-plugins-good`, `gst-plugins-bad`,
  `gst-libav`)
- `pkgconf` e toolchain C (usados pelas crates `-sys` para localizar as bibliotecas)

## Instalação das dependências no Arch Linux

```bash
sudo pacman -S --needed webkitgtk-6.0 gtk4 gst-plugins-good gst-plugins-bad gst-libav base-devel rustup
rustup default stable
```

O WebKitGTK decodifica áudio e vídeo pelo GStreamer, e o pacote `webkitgtk-6.0`
traz esses plugins apenas como dependências opcionais. Sem o `gst-plugins-bad`
(parser Opus, AAC, H.264) e o `gst-libav`, o YouTube mostra
"Não é possível tocar este vídeo no seu navegador".

## Build

```bash
cargo build            # debug
cargo build --release  # otimizado
```

## Execução

```bash
cargo run
```

## Testes e verificação

```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

## Atalhos de teclado

| Atalho                         | Ação                                   |
| ------------------------------ | -------------------------------------- |
| `Ctrl+L`                       | Focar e selecionar a barra de endereço |
| `Ctrl+R` / `F5`                | Recarregar                             |
| `Alt+←`                        | Voltar                                 |
| `Alt+→`                        | Avançar                                |
| `Alt+Home`                     | Página inicial                         |
| `Ctrl+T`                       | Nova aba                               |
| `Ctrl+W`                       | Fechar aba                             |
| `Ctrl+Tab` / `Ctrl+PgDn`       | Próxima aba                            |
| `Ctrl+Shift+Tab` / `Ctrl+PgUp` | Aba anterior                           |
| `Ctrl+Q`                       | Fechar o navegador                     |
| `Enter` (no endereço)          | Abrir URL ou pesquisar                 |

Os atalhos são registrados como *accelerators* do `GtkApplication`
(`set_accels_for_action`), a convenção do GTK4 para atalhos de aplicação.

## Arquitetura

```text
src/
├── main.rs              ponto de entrada
├── app.rs               ciclo de vida do GtkApplication, ação quit e atalhos
├── browser/
│   ├── media_formats.rs formatos de mídia essenciais e mensagem de aviso
│   ├── media_support.rs consulta ao WebKit sobre os formatos suportados
│   ├── navigation.rs    abrir endereço digitado e ir para a página inicial
│   └── webview.rs       criação do WebView e registro de falhas
├── ui/
│   ├── actions/
│   │   ├── catalog.rs   BrowserAction: nomes, atalhos e parâmetros das ações
│   │   └── handler.rs   execução das ações sobre a aba ativa
│   ├── tabs/
│   │   ├── mod.rs       Tabs: abrir, fechar, selecionar e pop-ups em nova aba
│   │   ├── label.rs     rótulo da aba (título + botão fechar)
│   │   ├── order.rs     navegação circular entre abas
│   │   └── watch.rs     observação de propriedades da aba ativa
│   ├── sync.rs          aba ativa → endereço, título, spinner, histórico
│   ├── title.rs         títulos da janela e das abas
│   ├── toolbar.rs       widgets da toolbar
│   └── window.rs        composição da janela
└── utils/
    └── url.rs           resolução da entrada: URL explícita, domínio ou pesquisa
```

- **Ações GTK como abstração (DIP):** a toolbar e os atalhos de teclado não
  conhecem o WebView. Botões usam `action-name` (`win.back`, `win.reload`...),
  a barra de endereço dispara `win.open-address` com o texto digitado e os
  atalhos apontam para as mesmas ações. O enum `BrowserAction` é o contrato
  único; `ui/actions/handler.rs` executa cada ação sobre a aba ativa.
- **Estado dos botões pelo próprio GAction:** Voltar/Avançar ficam insensíveis
  porque as ações correspondentes são desabilitadas quando não há histórico —
  o que também desativa os atalhos.
- **Sincronização reativa:** `Tabs::watch_current` observa `uri`, `title` e
  `is-loading` da aba ativa (e a troca de aba) e `ui/sync.rs` atualiza a
  interface; nada bloqueia a thread da UI.
- **Ciclo de vida das abas:** fechar uma aba remove o WebView do `GtkNotebook`,
  o que encerra a página; ao fechar a janela, todas as abas são liberadas.
- **Regras de domínio puras e testadas:** a resolução de endereço (`utils/url.rs`,
  usando as crates `url` e `percent-encoding`), os títulos (`ui/title.rs`) e a
  ordem das abas (`ui/tabs/order.rs`) são funções determinísticas com testes
  unitários.

Cookies, cache e armazenamento de sites ficam na sessão padrão do WebKit, em
`~/.local/share/rust-browser` e `~/.cache/rust-browser`.

## Tecnologias

- [Rust](https://www.rust-lang.org/) (edition 2024)
- [GTK4](https://gtk.org/) via [`gtk4`](https://crates.io/crates/gtk4) 0.11
- [WebKitGTK 6.0](https://webkitgtk.org/) via [`webkit6`](https://crates.io/crates/webkit6) 0.6
- [`url`](https://crates.io/crates/url) 2.5 — parsing de URL (padrão WHATWG)
- [`percent-encoding`](https://crates.io/crates/percent-encoding) 2.3 — codificação da pesquisa

## Limitações conhecidas

- Sem favoritos, histórico persistente, downloads, modo privado ou
  bloqueador de anúncios (fora do escopo desta versão).
- Domínios sem esquema sempre recebem `https://`; servidores locais só em HTTP
  (ex.: `localhost:3000`) precisam de `http://` explícito.
- A detecção de domínio é heurística: nomes de host sem ponto (ex.: `intranet`)
  viram pesquisa, e textos como `rust.ownership` são tratados como domínio.
- Pop-ups legítimos (ex.: login OAuth) abrem em uma nova aba sem vínculo com a
  página de origem (`window.opener`).
- O botão Recarregar não vira "Parar" durante o carregamento.
- Se a URL da página mudar enquanto você digita (ex.: redirecionamento), o texto
  da barra de endereço é substituído.
- Se o processo web do WebKit encerrar, o erro é registrado no `stderr` e a
  página fica em branco até recarregar.
