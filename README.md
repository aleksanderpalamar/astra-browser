<p align="center">
  <img src="data/icons/src/astra.svg" width="128" alt="Ícone do Astra Browser">
</p>

# Astra Browser

Navegador web desktop minimalista escrito em Rust, com interface nativa **GTK4** e
renderização feita pelo **WebKitGTK 6.0**. Toda a aplicação ao redor da engine
(janela, toolbar, barra de endereço, navegação, atalhos e sincronização de estado)
é implementada em Rust — sem Electron, Tauri ou frontend web.

![Astra Browser exibindo a página inicial do DuckDuckGo](docs/screenshot.png)

## Funcionalidades

- Toolbar com **Voltar**, **Avançar**, **Recarregar**, **Home**, barra de endereço e
  indicador de carregamento (spinner).
- Barra de endereço inteligente:
  - `github.com` → `https://github.com`
  - endereços locais usam HTTP: `localhost:3000` → `http://localhost:3000`
    (também `127.0.0.1`, IPs de rede privada como `192.168.0.10`, IPv6 local
    e nomes `.localhost`, `.local`, `.lan`, `.internal`, `.home.arpa` e `.test`)
  - só vira domínio o que termina em um sufixo conhecido da
    [Public Suffix List](https://publicsuffix.org/) (`.com`, `.com.br`, `.dev`,
    `.рф`...): `astra.ownership` e `index.html` viram pesquisa
  - uma palavra sem ponto vira endereço HTTP quando escrita como endereço, com
    porta, caminho ou barra final: `intranet:8080`, `intranet/`, `nas/admin`;
    `intranet` sozinho continua sendo pesquisa
  - URLs com `http://` ou `https://` são mantidas intactas
  - texto comum vira pesquisa: `Rust ownership` → `https://duckduckgo.com/?q=Rust%20ownership`
- Botões Voltar/Avançar habilitados somente quando existe histórico.
- Barra de endereço e título da janela sincronizados com a página
  (`Título da página — Astra Browser`).
- Abas: nova aba (botão `+` ou `Ctrl+T`), fechar pelo `✕` da aba ou `Ctrl+W`,
  alternar com `Ctrl+Tab`/`Ctrl+Shift+Tab` e reordenar arrastando. A toolbar e o
  título da janela acompanham a aba ativa; fechar a última aba fecha a janela.
- Favoritos: a estrela na barra de endereço (ou `Ctrl+D`) adiciona/remove a
  página atual; o botão de favoritos na toolbar (ou `Ctrl+Shift+O`) lista os
  favoritos para abrir ou remover. Ficam salvos entre execuções.
- Histórico persistente: cada página visitada é registrada; o botão de
  histórico (ou `Ctrl+H`) lista as visitas mais recentes, com busca por título
  ou endereço e opção de limpar tudo.
- Downloads: anexos e arquivos que o WebKit não sabe exibir são baixados para a
  pasta de Downloads do usuário (sem sobrescrever: `foto (1).png`); o painel de
  downloads (ou `Ctrl+Shift+Y`) mostra progresso, permite cancelar e abrir o
  arquivo concluído.
- Modo privado: `Ctrl+Shift+P` (ou o menu principal) abre uma janela privada,
  identificada pelo selo "Privado" e pelo título. Ela usa uma sessão efêmera
  do WebKit (cookies, cache e armazenamento só em memória, descartados ao
  fechar a janela) e não grava histórico.
- Várias janelas: `Ctrl+N` abre uma nova janela normal; o menu principal reúne
  nova aba, nova janela, nova janela privada, ferramentas do desenvolvedor,
  configurações e sair.
- Bloqueador de anúncios nativo: usa o motor de filtros de conteúdo do próprio
  WebKit (o mesmo formato dos bloqueadores do Safari) com a EasyList, bloqueando
  requisições de redes de anúncio e escondendo elementos de anúncio em todas as
  abas e janelas, inclusive as privadas. A lista é baixada na primeira execução,
  compilada uma vez e atualizada semanalmente. No YouTube, onde os anúncios de
  vídeo vêm do mesmo servidor que o conteúdo, um *scriptlet* remove os anúncios
  do JSON do player (como o uBlock Origin faz) e regras próprias do Astra
  escondem os cards patrocinados; essas regras são compiladas uma vez e só
  recompiladas quando mudam em uma nova versão do Astra. O bloqueador é ligado/desligado em
  Configurações (a escolha é lembrada; recarregue a página para ver o efeito).
- Configurações: janela nativa aberta pelo menu principal ou `Ctrl+,`, com as
  seções "Privacidade e segurança" (interruptor "Bloquear anúncios") e
  "Desempenho" (interruptor "Baixo consumo de memória").
- Baixo consumo de memória (desligado por padrão): troca o modelo de cache do
  WebKit para `DocumentBrowser` e desliga o cache de páginas de Voltar/Avançar,
  que passam a recarregar a página. Com o modo ativo desde a abertura do Astra,
  os processos das abas fechadas são encerrados em vez de ficarem guardados
  para reutilização; ao ligar o interruptor com o Astra aberto, essa parte só
  vale depois de reiniciar.
- Microfone e câmera: quando um site pede acesso (ex.: ditado por voz), o
  Astra pergunta "*site* quer usar o seu microfone" com as opções Bloquear e
  Permitir; fechar o diálogo bloqueia. Outros pedidos de permissão continuam
  negados.
- Ferramentas do desenvolvedor: `F12` (ou `Ctrl+Shift+I`) abre e fecha o Web
  Inspector do WebKit na aba ativa — Elements, Console, Sources, Network,
  Storage e demais painéis. O menu de contexto da página também passa a
  oferecer "Inspecionar elemento".
- Pop-ups: links com `target="_blank"` e `window.open` abrem em uma nova aba
  vinculada à página de origem (`window.opener`, como os logins OAuth exigem).
  Quando o site pede um tamanho (`width`/`height`), o pop-up abre em uma janela
  própria desse tamanho, com o endereço visível e só para leitura. Quando o site
  fecha o pop-up (`window.close()`), a aba ou janela fecha e o Astra volta para
  a aba de origem. Pop-ups abertos sem interação do usuário são bloqueados.
- Falhas de carregamento exibem a página de erro do WebKit e são registradas no
  `stderr`, sem derrubar a aplicação.
- Recuperação de travamentos: se o processo web de uma aba travar, o Astra
  recarrega a página uma vez automaticamente. Se a mesma página travar de novo,
  ou se o WebKit a encerrar por excesso de memória, a aba mostra "A página
  falhou" com o botão Recarregar, sem novas tentativas automáticas (evitando um
  loop de travamentos). O encerramento continua registrado no `stderr`.
- Limite de memória por processo web: cada página pode usar até 2 GiB. Ao
  passar de cerca de 676 MiB (33%), o WebKit começa a liberar memória não
  essencial e, a partir de 1 GiB (50%), também a essencial. Nenhuma página é
  encerrada por excesso de memória.
- Na primeira página carregada, o navegador pergunta ao WebKit se consegue
  reproduzir os formatos de mídia essenciais da web (VP9, H.264, Opus, AAC) e
  avisa no `stderr` quais plugins do GStreamer instalar se faltar algum.
- Página inicial: `https://duckduckgo.com`.

## Requisitos

- Linux (desenvolvido e testado no Arch Linux, GNOME sobre Wayland)
- Rust stable (edition 2024, Rust ≥ 1.92)
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

## Instalação no sistema (atalho e ícone)

```bash
./scripts/install.sh      # compila em release e instala em ~/.local
./scripts/uninstall.sh    # remove o que foi instalado
```

O script instala o binário em `~/.local/bin/astra-browser`, o atalho
`io.github.aleksanderpalamar.AstraBrowser.desktop` e os ícones no tema
`hicolor` do usuário. Use `PREFIX=/outro/caminho` para mudar o destino. No
GNOME/Wayland o dock e o Alt+Tab encontram o ícone pelo *app ID*
(`io.github.aleksanderpalamar.AstraBrowser`), inclusive ao rodar com
`cargo run` depois da instalação.

## Identidade visual

O ícone representa o nome (*astra*, “estrelas”): um planeta de espaço profundo,
um anel orbital que lembra o globo de um navegador e uma estrela de quatro
pontas. Ele é responsivo — cada faixa de tamanho usa o desenho adequado:

| Arquivo (`data/icons/src/`) | Uso                                                        |
| --------------------------- | ---------------------------------------------------------- |
| `astra.svg`                 | 48 px ou mais (dock, grade de apps) e SVG escalável        |
| `astra-small.svg`           | 16–32 px (barra de título, barra de tarefas, bandejas): sem detalhes finos, anel e estrela mais grossos |
| `astra-symbolic.svg`        | ícone simbólico monocromático, recolorido pelo tema (painéis, bandejas, alto contraste) |

Os PNGs de `data/icons/hicolor/` são gerados a partir dos SVGs com
`./scripts/render-icons.sh` (requer `rsvg-convert`, do pacote `librsvg`).

## Testes e verificação

```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

## Versões e pacotes

Para publicar uma versão, crie e envie uma tag no formato `vMAJOR.MINOR.PATCH`,
ou `vMAJOR.MINOR.PATCH-canal[.N]` para uma pré-release:

```bash
git tag v0.2.0            # versão final
git tag v1.0.0-beta.1     # pré-release (também: -canary, -dev, -rc.2...)
git push origin v0.2.0
```

O workflow `Release Packages` usa a versão da tag em todos os pacotes, sem
precisar editar o `Cargo.toml` antes: gera os pacotes Arch (`.pkg.tar.zst`),
Debian (`.deb`) e RPM (`.rpm`) do commit da tag e os publica em uma release do
GitHub. Cada formato escreve a pré-release do jeito que o seu gerenciador de
pacotes entende como anterior à versão final:

| Tag             | Binário (Cargo) | `.deb` e `.rpm` | Arch          |
| --------------- | --------------- | --------------- | ------------- |
| `v1.0.0`        | `1.0.0`         | `1.0.0`         | `1.0.0`       |
| `v1.0.0-beta.1` | `1.0.0-beta.1`  | `1.0.0~beta.1`  | `1.0.0beta.1` |

Tags com canal viram uma release marcada como pré-release no GitHub. O canal usa
letras minúsculas e, opcionalmente, um número (`beta`, `beta.2`, `canary.3`);
entre canais vale a ordem alfabética do SemVer (`alpha` < `beta` < `canary` <
`dev` < `rc`). Tags fora desse formato (ex.: `v0.2`, `v1.0.0-rc2`, `v01.0.0`)
fazem o workflow falhar antes de gerar qualquer pacote.

## Atalhos de teclado

| Atalho                         | Ação                                         |
| ------------------------------ | -------------------------------------------- |
| `Ctrl+L`                       | Focar e selecionar a barra de endereço       |
| `Ctrl+R` / `F5`                | Recarregar                                   |
| `Alt+←`                        | Voltar                                       |
| `Alt+→`                        | Avançar                                      |
| `Alt+Home`                     | Página inicial                               |
| `Ctrl+T`                       | Nova aba                                     |
| `Ctrl+W`                       | Fechar aba                                   |
| `Ctrl+Tab` / `Ctrl+PgDn`       | Próxima aba                                  |
| `Ctrl+Shift+Tab` / `Ctrl+PgUp` | Aba anterior                                 |
| `Ctrl+D`                       | Adicionar/remover dos favoritos              |
| `Ctrl+Shift+O`                 | Abrir a lista de favoritos                   |
| `Ctrl+H`                       | Abrir o histórico                            |
| `Ctrl+Shift+Y`                 | Abrir os downloads                           |
| `Ctrl+N`                       | Nova janela                                  |
| `Ctrl+Shift+P`                 | Nova janela privada                          |
| `Ctrl+,`                       | Abrir as configurações                       |
| `F12` / `Ctrl+Shift+I`         | Abrir/fechar as ferramentas do desenvolvedor |
| `F10`                          | Abrir o menu principal                       |
| `Ctrl+Q`                       | Fechar o navegador                           |
| `Enter` (no endereço)          | Abrir URL ou pesquisar                       |

Os atalhos são registrados como *accelerators* do `GtkApplication`
(`set_accels_for_action`), a convenção do GTK4 para atalhos de aplicação.

## Arquitetura

```text
data/
├── icons/src/           SVGs do ícone (completo, pequeno e simbólico)
├── icons/hicolor/       PNGs gerados por tamanho (16 a 512 px)
└── io.github.aleksanderpalamar.AstraBrowser.desktop
scripts/
├── install.sh           instala binário, atalho e ícones em ~/.local
├── uninstall.sh         remove a instalação
├── render-icons.sh      gera os PNGs a partir dos SVGs
└── apply-tag-version.sh aplica a versão da tag de release no Cargo.toml, Cargo.lock e PKGBUILD
src/
├── main.rs              ponto de entrada
├── app.rs               ciclo de vida do GtkApplication, ações, atalhos e ícone
├── library/
│   ├── bookmarks.rs     favoritos: regras (adicionar, remover, formato do arquivo)
│   ├── bookmark_store.rs favoritos persistidos em disco
│   ├── history/         histórico: registro, busca e limite de visitas
│   ├── history_store.rs histórico persistido em disco (somente acréscimo)
│   ├── preferences.rs   preferências do usuário (bloqueador e baixo consumo de memória)
│   ├── visit.rs         visita: formato da linha e páginas registráveis
│   ├── files.rs         caminhos de dados, gravação atômica e acréscimo
│   └── tsv.rs           codificação das linhas dos arquivos de dados
├── browser/
│   ├── adblock/
│   │   ├── mod.rs       AdBlocker: carrega do cache, baixa, compila e aplica
│   │   ├── filters.rs   fonte da lista, atualização semanal e reuso das regras extras
│   │   ├── download.rs  download da lista (libsoup)
│   │   ├── sanitize.rs  correção de padrões defeituosos da lista
│   │   ├── youtube.rs   injeção do scriptlet de anúncios do player do YouTube
│   │   ├── youtube.js   scriptlet: remove adPlacements/adSlots/playerAds
│   │   └── extra_rules.json regras cosméticas próprias (cards patrocinados do YouTube)
│   ├── context.rs       WebContext dos WebViews: limite de memória e modelo de cache inicial
│   ├── crash/
│   │   ├── mod.rs       recuperação quando o processo web da aba encerra
│   │   ├── policy.rs    decisão: recarregar uma vez, página de falha ou desistir
│   │   ├── page.rs      página "A página falhou" com o botão Recarregar
│   │   └── failure.html modelo da página de falha
│   ├── downloads.rs     downloads: política de resposta e destino do arquivo
│   ├── download_name.rs nome de destino único e seguro na pasta de downloads
│   ├── inspector.rs     Web Inspector: habilitar e abrir/fechar na aba ativa
│   ├── media_formats.rs formatos de mídia essenciais e mensagem de aviso
│   ├── media_support.rs consulta ao WebKit sobre os formatos suportados
│   ├── memory_mode.rs   MemoryMode: modelo de cache e cache de páginas de cada modo
│   ├── mode.rs          BrowsingMode: sessão de rede e gravação de histórico
│   ├── navigation.rs    abrir endereço digitado e ir para a página inicial
│   ├── popup.rs         decisão entre aba e janela para cada pop-up
│   ├── session.rs       cookies da sessão padrão persistidos em SQLite
│   └── webview.rs       WebViewFactory: criação dos WebViews com configurações compartilhadas
├── ui/
│   ├── adblock.rs       ação "Bloquear anúncios" com estado persistido
│   ├── actions/
│   │   ├── spec.rs      ActionSpec: contrato comum e registro de ações e interruptores
│   │   ├── app.rs       AppAction: janelas, interruptores, configurações e sair
│   │   ├── catalog.rs   BrowserAction: navegação e abas
│   │   ├── library.rs   LibraryAction: favoritos, histórico e downloads
│   │   └── handler.rs   execução das ações de navegação sobre a aba ativa
│   ├── library/
│   │   ├── panel.rs     painel (botão + popover) reutilizável da biblioteca
│   │   └── link_row.rs  linha com título, endereço e remoção
│   ├── address_bar.rs   barra de endereço e estrela de favorito
│   ├── bookmarks.rs     favoritos na interface: estrela, painel e ações
│   ├── downloads/       painel de downloads, linha com progresso e status
│   ├── history.rs       registro das visitas e painel de histórico
│   ├── low_memory.rs    ação "Baixo consumo de memória" com estado persistido
│   ├── menu.rs          menu principal
│   ├── permissions/     pergunta de permissão para microfone e câmera
│   ├── popup_window.rs  janela de pop-up com o endereço só para leitura
│   ├── preferences.rs   janela de Configurações
│   ├── tabs/
│   │   ├── mod.rs       Tabs: abrir, fechar e selecionar abas
│   │   ├── label.rs     rótulo da aba (título + botão fechar)
│   │   ├── order.rs     navegação circular entre abas
│   │   ├── popups.rs    pop-ups vinculados à origem: aba ou janela e fechamento pelo site
│   │   ├── weak.rs      referência fraca às abas para as closures
│   │   └── watch.rs     observação de propriedades da aba ativa
│   ├── sync.rs          aba ativa → endereço, título, spinner, voltar/avançar
│   ├── title.rs         títulos da janela e das abas
│   ├── toolbar.rs       widgets da toolbar
│   └── window.rs        composição da janela conforme o modo de navegação
└── utils/
    ├── clock.rs         horário atual em segundos Unix
    └── url/
        ├── mod.rs       resolução da entrada: URL explícita, endereço ou pesquisa
        ├── host.rs      host local (HTTP), público (HTTPS) ou pesquisa
        └── tests.rs     casos da resolução de endereços
```

- **Ações GTK como abstração (DIP):** a toolbar e os atalhos de teclado não
  conhecem o WebView nem os favoritos. Botões usam `action-name` (`win.back`, `win.reload`...),
  a barra de endereço dispara `win.open-address` com o texto digitado e os
  atalhos apontam para as mesmas ações. Os enums que implementam `ActionSpec`
  (`BrowserAction`, `LibraryAction`, `AppAction`) são o contrato; cada
  funcionalidade registra os próprios handlers.
- **Modo de navegação:** `BrowsingMode` decide a sessão de rede de cada janela
  (padrão ou efêmera) e se o histórico é gravado; o resto da interface é igual.
- **Estado dos botões pelo próprio GAction:** Voltar/Avançar ficam insensíveis
  porque as ações correspondentes são desabilitadas quando não há histórico —
  o que também desativa os atalhos.
- **Sincronização reativa:** `Tabs::watch_current` observa `uri`, `title` e
  `is-loading` da aba ativa (e a troca de aba) e `ui/sync.rs` atualiza a
  interface; nada bloqueia a thread da UI.
- **Ciclo de vida das abas:** fechar uma aba remove o WebView do `GtkNotebook`,
  o que encerra a página; ao fechar a janela, todas as abas são liberadas.
  `Tabs`, `Toolbar` e os controladores implementam `glib::clone::Downgrade`,
  e as closures os capturam por referência fraca, sem ciclos de referência.
- **Regras de domínio puras e testadas:** a resolução de endereço (`utils/url/`,
  usando as crates `url` e `percent-encoding`), os títulos (`ui/title.rs`), a
  ordem das abas (`ui/tabs/order.rs`), as regras de favoritos e histórico
  (`library/`), o nome de destino dos downloads (`browser/download_name.rs`),
  a política de recuperação de travamentos (`browser/crash/policy.rs`), os
  modos de memória (`browser/memory_mode.rs`) e a apresentação dos pop-ups
  (`browser/popup.rs`) são determinísticos e têm testes unitários.

Os cookies da navegação normal são gravados em
`~/.local/share/astra-browser/cookies.sqlite`, o que mantém os logins dos sites
entre execuções; o armazenamento dos sites fica no mesmo diretório e o cache em
`~/.cache/astra-browser`. Janelas privadas não gravam nada disso. A lista de
bloqueio compilada fica em `~/.local/share/astra-browser/content-filters/` e as
preferências em `~/.config/astra-browser/preferences.ini`. Os favoritos ficam em
`~/.local/share/astra-browser/bookmarks.tsv` (uma linha por favorito:
endereço e título separados por tab) e o histórico em `history.tsv` no mesmo
diretório (data, endereço e título), limitado às 5 000 visitas mais recentes.

## Tecnologias

- [Rust](https://www.rust-lang.org/) (edition 2024)
- [GTK4](https://gtk.org/) via [`gtk4`](https://crates.io/crates/gtk4) 0.11
- [WebKitGTK 6.0](https://webkitgtk.org/) via [`webkit6`](https://crates.io/crates/webkit6) 0.6
- [`url`](https://crates.io/crates/url) 2.5 — parsing de URL (padrão WHATWG)
- [`percent-encoding`](https://crates.io/crates/percent-encoding) 2.3 — codificação da pesquisa
- [`psl`](https://crates.io/crates/psl) 2.1 — Public Suffix List para reconhecer domínios
- [`serde_json`](https://crates.io/crates/serde_json) 1.0 — correção da lista de bloqueio antes de compilar
- [EasyList](https://easylist.to/) — lista de filtros de anúncios (GPLv3 / CC BY-SA 3.0), baixada em tempo de execução

## Limitações conhecidas

- Sem extensões ou sincronização (fora do escopo desta versão).
- A EasyList no formato do WebKit (`easylist_min_content_blocker.json`) não é
  atualizada pela fonte desde maio de 2025; ela ainda cobre as principais redes
  de anúncio, mas anúncios novos podem passar. O bloqueador não tem exceções
  por site nem contador de itens bloqueados.
- O bloqueio no YouTube depende da estrutura atual do site; mudanças do YouTube
  (ou seus avisos contra bloqueadores) podem exigir ajustes no scriptlet e nas
  regras extras.
- Serviços com DRM (Prime Video, Netflix, Disney+, Spotify Web...) não
  reproduzem conteúdo. O `webkitgtk-6.0` do Arch é compilado sem Encrypted
  Media Extensions (`navigator.requestMediaKeySystemAccess` não existe, mesmo
  com a opção ligada), e o WebKitGTK não suporta o Widevine, o DRM que esses
  serviços exigem. É uma limitação da engine, a mesma do Epiphany.
- WebRTC depende do WebKitGTK do sistema: o Astra liga a opção, mas o pacote
  `webkitgtk-6.0` do Arch é compilado sem WebRTC (`RTCPeerConnection` não
  existe). Chamadas de vídeo e o modo de conversa por voz do ChatGPT não
  funcionam; o microfone para ditado e gravação funciona.
- Gravação de áudio pelos sites (`MediaRecorder`): WebM/Opus funciona, mas MP4
  — o formato padrão do WebKitGTK quando o site não escolhe um — gera arquivos
  vazios com WebKitGTK 2.52 e GStreamer 1.28 (o `encodebin` não encontra um
  perfil compatível), mesmo com o `gst-plugin-isobmff` instalado.
- A permissão de microfone/câmera não é lembrada: cada pedido do site abre a
  pergunta de novo.
- Ainda não pode ser definido como navegador padrão: o atalho não declara tipos
  MIME porque o app não abre URLs recebidas pela linha de comando.
- Cada janela privada tem a própria sessão efêmera; janelas privadas não
  compartilham cookies entre si.
- Arquivos baixados em janelas privadas ficam na pasta de Downloads.
- A lista de downloads vale só para a sessão atual (não é persistida).
- Domínios públicos sem esquema recebem `https://`; sites públicos que só
  atendem em HTTP precisam de `http://` explícito.
- Uma palavra sem ponto só vira endereço quando tem porta, caminho ou barra
  final: `intranet` sozinho vira pesquisa (use `intranet/`), e textos com barra
  como `km/h` ou `tcp/ip` abrem como endereço em vez de pesquisa.
- O WebKitGTK informa só o tamanho pedido para um pop-up: um `window.open` que
  peça exatamente o tamanho padrão da janela (1280×800) abre em aba. Nas janelas
  de pop-up, pedidos de microfone e câmera são negados.
- O botão Recarregar não vira "Parar" durante o carregamento.
- Se a URL da página mudar enquanto você digita (ex.: redirecionamento), o texto
  da barra de endereço é substituído.
