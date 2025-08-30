# TaskBoard

Uma aplicação moderna de quadro de tarefas construída com Slint e Rust.

## 🚀 Funcionalidades

- Interface gráfica moderna e responsiva
- Desenvolvido com Slint (UI) e Rust (backend)
- Suporte nativo para Windows
- Builds otimizados para performance

## 📦 Downloads

Vá para a [página de releases](../../releases) para baixar a versão mais recente.

### Windows
- **Instalador**: `taskboard-installer-windows-x64.zip` - Instalação completa com atalhos
- **Portátil**: `taskboard-portable-windows-x64.zip` - Execução direta sem instalação

## 🛠️ Desenvolvimento

### Pré-requisitos
- Rust (versão estável mais recente)
- CMake (para dependências do Slint)

### Build local
```bash
# Clone o repositório
git clone https://github.com/yourusername/taskboard.git
cd taskboard

# Build de desenvolvimento
cargo build

# Build de release
cargo build --release

# Executar
cargo run
```

### Estrutura do Projeto
```
taskboard/
├── src/
│   ├── main.rs          # Ponto de entrada da aplicação
│   └── ui/
│       └── App.slint    # Interface do usuário
├── .github/
│   └── workflows/
│       └── release.yml  # CI/CD para releases
├── Cargo.toml           # Configuração do projeto
└── build.rs             # Script de build
```

## 🔄 Releases Automáticos

Este projeto usa GitHub Actions para criar releases automáticos:

### Como criar uma release

1. **Via Tag Git**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **Via GitHub Web Interface**:
   - Vá para a aba "Releases"
   - Clique em "Create a new release"
   - Defina uma tag (ex: `v1.0.0`)
   - Publique a release

3. **Trigger manual**:
   - Vá para a aba "Actions"
   - Selecione o workflow "Release"
   - Clique em "Run workflow"

### O que acontece automaticamente

1. **Build para Windows**: Compila o projeto com otimizações máximas
2. **Criação de Instalador**: Gera script de instalação com atalhos
3. **Pacote Portátil**: Cria versão que roda sem instalação
4. **Upload para Release**: Faz upload automático dos arquivos
5. **Documentação**: Adiciona instruções de instalação à release

## 📋 Como usar as releases

### Instalador Windows
1. Baixe `taskboard-installer-windows-x64.zip`
2. Extraia o arquivo
3. Execute `install.bat` como administrador
4. O aplicativo será instalado em `C:\Program Files\TaskBoard`
5. Um atalho será criado na área de trabalho

### Versão Portátil
1. Baixe `taskboard-portable-windows-x64.zip`
2. Extraia em qualquer pasta
3. Execute `taskboard.exe` diretamente

## 🔧 Configurações do Workflow

O arquivo `.github/workflows/release.yml` contém:

- **Build otimizado** com `cargo build --release`
- **Cache de dependências** para builds mais rápidos
- **Instalação automática** de dependências do sistema
- **Criação de pacotes** ZIP para distribuição
- **Release automática** no GitHub

## 📄 Licença

Este projeto está licenciado sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para detalhes.
