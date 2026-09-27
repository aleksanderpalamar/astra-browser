- Sem abas, favoritos, histórico persistente, downloads ou modo privado (fora do escopo).
- Endereços sem esquema sempre recebem https://, então servidores locais só em HTTP (ex.:
    localhost:3000) precisam de http:// explícito.
- A detecção de domínio é aproximada: intranet vira busca e rust.ownership é tratado como
    domínio.
- Pop-ups legítimos, como login OAuth, substituem a página atual.
- O botão Recarregar não vira "Parar" durante o carregamento.
- Se a URL mudar enquanto você digita (por exemplo, num redirecionamento), o texto da barra
    de endereço é substituído.
- Se o processo web do WebKit travar, o erro só é registrado no terminal e a página fica em
    branco até recarregar.
- Cookies e cache ficam em ~/.local/share/rust-browser e ~/.cache/rust-browser.