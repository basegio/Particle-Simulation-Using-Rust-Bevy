# Changelog

### dev
#### Adicionado
- Suporte a gravidade para ponto fixo
- Suporte a desativar gravidade
#### Alterado
- Lógica do spawn de particulas

### 0.2.0 - 28/12/2025
> Creio que o uso de Verlet simplificado para calculo deterministico vai servir bem.
#### Adicionado
- Parâmetros **position** e **position_old** a **Particle**
- Função **solve_collisions** a **Particle** com O(n²) com base no raio (atualmente constante)
- Função **solve_enviroment_constraints_limits** a **Particle**
- Gravidade
#### Removido
- Parâmetro **velocity**
#### Alterado
- Construtor impl default de **Particle** foi substituído por **from_position**
- Lógica do spawn de particulas
- **apply_physics** agora usa calculo dinâmico de **velocity** baseado em **position** e **position_old**

### 0.1.0 - 28/12/2025
#### Versão inicial