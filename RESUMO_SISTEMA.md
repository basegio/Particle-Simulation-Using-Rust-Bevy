#### Documentação gerada por IA

# Resumo do Sistema: Simulação de Partículas Determinística

Este documento detalha o funcionamento técnico da simulação de partículas, descrevendo a arquitetura, o fluxo de execução e as regras de cada módulo no diretório `src/`.

## Arquitetura Geral
A simulação é construída utilizando a engine **Bevy** e foca em determinismo e performance. 
- **TimeStep Fixo**: A simulação roda a 64Hz constantes para garantir resultados idênticos em diferentes hardwares.
- **Integração de Verlet**: Utiliza um método de integração estável onde a velocidade é derivada da diferença entre a posição atual e a anterior.
- **Particionamento Espacial**: Uma grade uniforme (`CollisionGrid`) otimiza a detecção de colisões.
- **Substepping**: Cada frame de física é dividido em sub-passos para maior precisão e estabilidade de colisões.

---

## Detalhamento dos Módulos

### 1. Core (`src/core/`)
Responsável pelas funcionalidades básicas de infraestrutura.
- **Quando é chamado**: O plugin é carregado na inicialização. O sistema `log_fps` roda a cada quadro de renderização (`Update`).
- **Por que**: Para fornecer feedback visual de performance no título da janela.
- **Regras**: Atualiza o título da janela com o FPS suavizado.

### 2. Diagnostic (`src/diagnostic/`)
Define métricas customizadas para monitoramento.
- **Quando é chamado**: Registra diagnósticos no `Startup`.
- **Por que**: Permite medir o tempo exato gasto na atualização da grade e na resolução de colisões em microssegundos (µs).

### 3. Grid (`src/grid/`)
Gerencia a otimização espacial das partículas.
- **`CollisionGrid` (Recurso)**: Uma estrutura de dados que mapeia posições espaciais para índices de partículas.
- **`update_grid`**: Chamado no `FixedUpdate` imediatamente antes do passo de simulação.
    - **Fluxo**: Limpa a grade -> Itera por todas as partículas -> Insere o índice da partícula na célula correspondente à sua posição.
- **Regras**: O tamanho da célula é definido pelo diâmetro máximo das partículas para garantir que apenas vizinhos imediatos precisem ser checados.

### 4. Particles (`src/particles/`)
Define o comportamento e propriedades das partículas.
- **`Particle` (Componente)**: Contém `position`, `position_old` e `radius`.
- **`spawn_particles`**: Chamado uma vez no `Startup`.
    - **Fluxo**: Gera partículas em um padrão de grade circular (anel).
    - **Regra**: Usa uma semente fixa (`SmallRng::seed_from_u64(0)`) para garantir que a posição e tamanho inicial sejam determinísticos.
- **`apply_physics_logic` (Movimento)**: Chamado em cada sub-passo da simulação.
    - **Fluxo**: Calcula aceleração (Gravidade) -> Integra posição usando Verlet -> Aplica amortecimento (`damping`).
- **`solve_collisions_logic` (Colisão)**: Chamado em cada sub-passo.
    - **Fluxo**: Para cada célula da grade, verifica colisões entre partículas da mesma célula e de 4 células vizinhas específicas (evitando verificações duplicadas).
    - **Regra de Resolução**: Se duas partículas sobrepõem, elas são empurradas para fora. A `position_old` também é ajustada para simular elasticidade (`restitution`).
- **`draw_particles`**: Chamado no `Update` de renderização para desenhar círculos via Gizmos.

### 5. Simulation (`src/simulation/`)
O "cérebro" que coordena toda a lógica temporal.
- **`SimulationSettings` (Recurso)**: Centraliza constantes como gravidade, amortecimento, sub-passos e limites da simulação.
- **`step_physics_simulation`**: O sistema principal que roda no `FixedUpdate`.
    - **Fluxo**:
        1. Copia dados das entidades para um `Vec` local (otimização de cache).
        2. Executa o loop de **substepping** (padrão: 8x por frame).
        3. Dentro do loop: Aplica Movimento -> Resolve Colisões -> Aplica Fronteiras.
        4. Sincroniza os resultados de volta para os componentes `Transform` e `Particle` da Bevy.
- **`draw_constraints`**: Desenha visualmente a borda da simulação.

### 6. Boundaries (`src/boundaries/`)
Define as restrições de borda do mundo.
- **Tipos**: `None` (sem bordas), `Square` (quadrado) e `Circle` (circular).
- **Quando é chamado**: No final de cada sub-passo de simulação.
- **Regras**: Se uma partícula ultrapassa o limite, sua posição é corrigida para o limite exato e sua `position_old` é ajustada para refletir um rebatimento proporcional à `restitution_amortization`.

---

## Fluxo de Execução Resumido (por Frame de Física)

1. **Update Grid**: Limpa a grade e reposiciona os índices das partículas conforme suas novas coordenadas.
2. **Substepping Loop (8x)**:
    - **Movimento**: Aplica gravidade e inércia via Verlet.
    - **Colisão**: Usa a grade para encontrar vizinhos e resolver sobreposições.
    - **Fronteiras**: Mantém as partículas dentro da caixa ou círculo definido.
3. **Sincronização**: Atualiza os `Transforms` para que a Bevy saiba onde desenhar cada partícula no próximo frame visual.

## Regras Gerais do Sistema

1. **Determinismo Estrito**: Tudo, desde o spawn até o tempo de passo, deve ser previsível. Não se deve usar `Time.delta_seconds()` bruto dentro da física, mas sim o `fixed_delta` dividido pelos `substeps`.
2. **Estabilidade de Verlet**: A velocidade nunca é armazenada explicitamente. Isso evita que forças excessivas "lancem" partículas para fora do mapa em casos de compressão alta.
3. **Otimização de Colisão**: O uso de `Local<Vec<Particle>>` no sistema de simulação evita o overhead de acesso ao ECS em loops internos de alta frequência.
4. **Resolução de Colisão**: Utiliza um `relaxation_factor` (fator de relaxamento) para resolver colisões gradualmente, o que reduz vibrações instáveis em pilhas de partículas.
