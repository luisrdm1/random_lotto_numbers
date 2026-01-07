# Lotto Quick Pick

Gerador de bilhetes de loteria em Rust com estratégias otimizadas por bitwise operations.

## ⚡ Performance

- **Estratégia principal**: Operações bitwise (55-67% mais rápido que HashSet)
- **Fallback automático**: HashSet para casos especiais
- **Zero overhead**: Funções genéricas sem vtable (monomorphization)
- **Seleção inteligente**: Escolhe automaticamente u64/u128/Vec\<u64\> baseado no tamanho do range

## 🚀 Quick Start

```bash
# Mega-Sena: 5 jogos de 6 números entre 1-60
cargo run -- -t 5 -s 1 -e 60 -p 6

# Com cálculo de probabilidade
cargo run -- -t 3 -s 1 -e 60 -p 6 -m 6
```

## 🎰 Exemplos de Uso

### Loterias Brasileiras

```bash
# Mega-Sena (6 números de 1 a 60)
cargo run -- -t 10 -s 1 -e 60 -p 6

# Quina (5 números de 1 a 80)
cargo run -- -t 10 -s 1 -e 80 -p 5

# Lotofácil (15 números de 1 a 25)
cargo run -- -t 5 -s 1 -e 25 -p 15

# Lotomania (50 números de 0 a 99)
cargo run -- -t 3 -s 0 -e 99 -p 50

# Dupla Sena (6 números de 1 a 50, gerar 2 jogos)
cargo run -- -t 2 -s 1 -e 50 -p 6
```

### Com Cálculo de Probabilidade

```bash
# Mega-Sena: probabilidade de acertar a sena (6 números)
cargo run -- -t 1 -s 1 -e 60 -p 6 -m 6

# Mega-Sena: probabilidade de acertar a quina (5 números)
cargo run -- -t 1 -s 1 -e 60 -p 6 -m 5

# Lotofácil: probabilidade de acertar 15 pontos
cargo run -- -t 1 -s 1 -e 25 -p 15 -m 15
```

### Ajuda

```bash
# Ver todas as opções disponíveis
cargo run -- --help
```

## 📦 Como Biblioteca

```rust
use lotto_quick_pick::{Config, generate_tickets};
use rand::rng;

let config = Config::new(10, 1, 60, 6)?;
let mut rng = rand::rng();
let tickets = generate_tickets(&mut rng, &config);
```

## 🛠️ Tecnologias

- **Rust Edition 2024** (versão 1.92)
- **rand 0.9.2** - Geração de números aleatórios
- **clap 4.5** - CLI parser
- **criterion 0.8** - Benchmarks
- **colored 3.0** - Output colorido

## 📊 Benchmarks

Execute com `cargo bench`:

```
bitwise_mega_sena    ~1.2μs  (u64 bitmap)
hashset_mega_sena    ~3.5μs  (55% mais lento)
```

## 🧪 Testes

```bash
cargo test              # 68 unit tests + 23 doctests
cargo clippy --all-targets  # Linting
```

## 🏗️ Arquitetura

```
src/
├── lib.rs              # API pública
├── main.rs             # CLI
├── ticket.rs           # Geração (bitwise + fallback)
├── ticket_bitwise.rs   # Estratégias otimizadas (u64/u128/Vec)
├── newtypes.rs         # Domain types (BallNumber, Ticket, etc)
├── probability.rs      # Cálculos combinatórios (sem overflow)
├── rng.rs              # Trait RandomNumberGenerator
└── error.rs            # Error handling
```

### Estratégias Bitwise

Seleção automática baseada no **tamanho do range** (não no valor máximo):

- **u64**: Range com até 64 valores (ex: Mega-Sena 1-60 tem 60 valores)
- **u128**: Range com 65-128 valores (ex: Lotomania 0-99 tem 100 valores)
- **Vec\<u64\>**: Range com mais de 128 valores

**Exemplo**: Range 200-255 tem apenas 56 valores → usa u64 (mais eficiente)

### Construção de Tickets

- **`Ticket::new(balls)`**: Cria ticket sem validação (apenas ordena)
- **`Ticket::try_new(balls, range, pick)`**: Cria ticket com validação completa
  - Valida tamanho correto
  - Valida que todas as bolas estão no range
  - Valida unicidade (sem duplicatas)

Use `try_new()` quando receber dados externos. Use `new()` apenas quando tem certeza que os dados são válidos.

## 🔧 RNG Customizado

```rust
impl RandomNumberGenerator for MyRng {
    fn gen_range_u8(&mut self, low: u8, high: u8) -> u8 {
        // Sua implementação (Sobol, quasi-random, etc)
    }
}
```

## 📐 Cálculo de Probabilidade

Algoritmo iterativo sem fatorial (sem BigInt):

$$C(n,k) = \prod_{i=1}^{k} \frac{n-i+1}{i}$$

Exemplos:
- $C(60,6) = 50.063.860$ (Mega-Sena)
- $C(100,50)$ calculado sem overflow usando `u128`

## 📄 Licença

MIT OR Apache-2.0
