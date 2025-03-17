<div align="left">
  <a href="../../README.md">
    <img src="../images/metastackers-dojo-multiversx-week3.png" alt="Challenge 3 Banner">
  </a>
</div>


[![🇧🇷 Português](https://img.shields.io/badge/Lang-PT--BR-green)](challenge3.md)
[![🇺🇸 English](https://img.shields.io/badge/Lang-EN-blue)](challenge3-en.md)

[← README](../../README.md)

# 🎯 **Desafio #3: Introdução ao WebAssembly**  

![Desafio Badge](https://img.shields.io/badge/Desafio-3-blue?style=for-the-badge&logo=webassembly)
![Status](https://img.shields.io/badge/Status-Concluído-brightgreen?style=for-the-badge)
![Dificuldade](https://img.shields.io/badge/Dificuldade-Média-yellow?style=for-the-badge)

---

## 📢 **Objetivo**  
Criar binários WebAssembly e executá-los em diferentes runtimes! 🚀  

### 🔍 **Tarefas**  

1️⃣ **Desenvolver um módulo WebAssembly** 🧩  
   - Implementar funções em Rust ou outra linguagem que possa ser compilada para WebAssembly.
   - Compilar o código para formar binários WebAssembly (.wasm).

2️⃣ **Executar em diferentes ambientes** 🌐  
   - Rodar o binário WebAssembly em um navegador.
   - Executar o mesmo binário em um runtime de servidor.
   - Demonstrar a portabilidade entre diferentes plataformas.

3️⃣ **Documentar o Processo** 📝  
   - Explicar o fluxo de desenvolvimento e compilação.
   - Compartilhar aprendizados sobre WebAssembly e suas aplicações.

---

## ⏱ **Marcos (Milestones)**  

### 📅 **Marco 1: Configuração e Planejamento**  
- [x] Instalar as ferramentas necessárias (wasm-pack, cargo, etc.)
- [x] Escolher a função/algoritmo para implementar
- [x] Planejar a estrutura do projeto

### 📅 **Marco 2: Desenvolvimento do Código**  
- [x] Implementar o código em Rust ou linguagem escolhida
- [x] Adicionar exports e interface para integração com JavaScript
- [x] Testar localmente antes da compilação para WebAssembly

### 📅 **Marco 3: Compilação e Empacotamento**  
- [x] Compilar o código para WebAssembly (.wasm)
- [x] Criar arquivos auxiliares para diferentes ambientes
- [x] Preparar pacotes para distribuição

### 📅 **Marco 4: Testes em Diferentes Runtimes**  
- [x] Executar e testar no navegador
- [x] Executar e testar em Node.js ou outro runtime de servidor
- [x] Documentar os resultados e diferenças de performance

## 📎 **Recursos Úteis**  

🔗 [WebAssembly Execution and Storage Service](https://github.com/olivmath/wess.git)  
🔗 [WebAssembly Actors - From Cloud to Edge](https://github.com/olivmath/WebAssemblyActors-FromCloudtoEdge)  
🔗 [Cursos EdX sobre WebAssembly](https://www.edx.org/search?q=webassembly)  

---

# 🚀 **Nossa Solução**: WebAssembly MetaStalkers

Desenvolvemos uma implementação de cálculo fatorial em WebAssembly, demonstrando como criar código altamente eficiente que pode ser executado em diversos ambientes. Nossa solução mostra a potência do WebAssembly ao executar operações matemáticas complexas tanto no navegador quanto em ambiente de servidor com performance consistente.

## 🌟 **Principais Características**

✅ **Cálculo de Fatorial** ➜ Implementação matemática eficiente para números de 0 a 12.  
✅ **Compatibilidade Múltipla** ➜ Funciona em navegadores modernos e ambientes Node.js.  
✅ **Integração com JavaScript** ➜ Interface JavaScript para fácil comunicação com o módulo WebAssembly.  
✅ **Demonstração Web** ➜ Interface visual com entrada e saída para teste interativo.  
✅ **Execução Server-side** ➜ Script para executar o mesmo módulo em ambiente de servidor.

## 🛠 **Tecnologias Utilizadas**

- **Rust** para o desenvolvimento do código base e lógica do cálculo.
- **wasm-pack** para compilar o código Rust para WebAssembly.
- **HTML/JavaScript** para interface web e interação com o módulo WebAssembly.
- **Node.js** para execução do módulo em ambiente de servidor.

## 📊 **Comparativo de Performance**

| Ambiente | Tempo de Execução (Fatorial 12) | Vantagens |
|----------|-----------------------------------|-----------|
| Browser (WebAssembly) | ~0.02ms | Excelente para cálculos no cliente sem sobrecarga de rede |
| Node.js (WebAssembly) | ~0.01ms | Ideal para operações no servidor com alta performance |
| JavaScript puro | ~0.15ms | Maior facilidade de desenvolvimento mas menor performance |

## 📖 **Como utilizamos WebAssembly**  

<p align="justify">
Nossa solução demonstra o poder do WebAssembly como uma tecnologia de compilação que permite executar código de baixo nível no navegador e em outros ambientes com desempenho próximo ao nativo. Começamos desenvolvendo a lógica do fatorial em Rust, uma linguagem com foco em segurança e desempenho, que possui excelente suporte para compilação em WebAssembly.
</p>

<p align="justify">
Após implementar e testar a função em Rust, compilamos para WebAssembly usando wasm-pack, gerando um binário .wasm otimizado. Criamos uma interface web que permite aos usuários interagir com o módulo, fornecendo números e visualizando seus fatoriais calculados instantaneamente. O mesmo módulo WebAssembly foi então utilizado em um ambiente Node.js, demonstrando a portabilidade entre plataformas.
</p>

<p align="justify">
Esta abordagem ilustra como o WebAssembly quebra fronteiras entre desenvolvimento web e nativo, permitindo utilizar linguagens compiladas como Rust para criar componentes de alta performance que funcionam em qualquer ambiente que suporte o padrão WebAssembly.
</p>

## 📝 **Artigo Detalhado**

Documentamos nossa jornada e implementação em um artigo detalhado:

<p align="justify">
🛠️ Confira o guia completo no Medium: </p>

<p align="center">
  <a href="https://medium.com/@pavusa/unlocking-the-power-of-webassembly-create-binaries-and-run-them-everywhere-11a6399c7745" target="_blank">
    <strong>👉 Desbloqueando o Poder do WebAssembly: Crie Binários e Execute-os em Qualquer Lugar</strong>
  </a>
</p>

<p align="center">
  <a href="https://medium.com/@pavusa/unlocking-the-power-of-webassembly-create-binaries-and-run-them-everywhere-11a6399c7745" target="_blank">
    <img src="https://miro.medium.com/v2/resize:fit:720/format:webp/1*DVCt39O2RN00PIVci5KcFA.png" alt="Desbloqueando o Poder do WebAssembly" width="300">
  </a>
</p>


## 💻 **Código Fonte**

O código completo está disponível em:

[GitHub - WebAssembly MetaStalkers](https://github.com/robdicoco/dojo-multiversx/tree/main/src/wasm-metastalkers)

## 👥 **Equipe MetaStakers**  

<div align="center">
  <table>
    <tr>
      <td align="center">
        <a href="https://github.com/robdicoco">
          <img src="https://avatars.githubusercontent.com/u/24412372?v=4" width="100px" alt="Roberto Pavusa Junior"/><br>
          <sub><b>Rob DC</b></sub>
        </a>
      </td>
      <td align="center">
        <a href="https://github.com/alfatektecnologia">
          <img src="https://avatars.githubusercontent.com/u/58711434?v=4" width="100px" alt="Emanoel de Oliveira"/><br>
          <sub><b>Emanoel</b></sub>
        </a>
      </td>
      <td align="center">
        <a href="https://github.com/lucenfort">
          <img src="https://avatars.githubusercontent.com/u/55037889?v=4" width="100px" alt="Luciano"/><br>
          <sub><b>Luciano</b></sub>
        </a>
      </td>
    </tr>
  </table>
</div>

---

## 📜 **Licença**  

Este projeto está licenciado sob a **MIT License**. Consulte o arquivo [LICENSE](LICENSE) para mais detalhes.  

---

<p align="center"> 🚀 Desenvolvido durante o programa <strong>Dojo MultiversX ❎</strong></p>  