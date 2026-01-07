# Lotto Quick Pick

Gerador de bilhetes de loteria em Rust com estratégias otimizadas por bitwise operations.

## ⚡ Performance

- **Estratégia principal**: Operações bitwise (55-67% mais rápido que HashSet)
- **Fallback automático**: HashSet para casos especiais
- **Zero overhead**: Generic dispatch sem vtable

## 🚀 Quick Start

```bash
# Mega-Sena: 5 jogos de 6 números entre 1-60
cargo run -- -t 5 -s 1 -e 60 -p 6

# Com cálculo de probabilidade
cargo run -- -t 3 -s 1 -e 60 -p 6 -m 6
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
cargo test              # 54 unit tests + 21 doctests
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

- **u64**: Até 64 bolas (ex: Mega-Sena)
- **u128**: Até 128 bolas (ex: Lotomania)
- **Vec\<u64\>**: Ranges maiores

Seleção automática baseada no range.

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

```
C(n,k) = ∏(n-i+1)/i para i=1..k
```

- C(60,6) = 50.063.860 (Mega-Sena)
- C(100,50) calculado sem overflow usando u128

## 📄 Licença

MIT OR Apache-2.0
