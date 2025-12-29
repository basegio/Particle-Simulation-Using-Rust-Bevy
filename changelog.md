# Changelog

### 0.4.0 - a definir
> Cá estou eu as três da manhã do dia 29/12/2025 estudando uma estrutura de grid que seja interessante para resolver meu problema de colisões entre particulas inimiga do desempenho, estou com certa confiança em implementar uniform grid, acredito que seja uma implementação menos eficiente que uma kd-tree, mas sem dúvidas é mais fácil de implementar um multithread em um grid com tamanho constante do que recalcular uma arvore de divisões a cada fixedUpdate. De qualquer forma tudo está sujeito a mudanças no futuro, vamos ver como se sai essas abordagens.

### 0.3.0 - 28/12/2025
> Aumentar o tamanho da simulação e elevar o numero de particulas para ~1000 levou o fps para ~1.
Isso se dá pela lógica de colisão, levando cada particula buscar com possiveis colisões com todas as outras particulas, O(n²) como dito anteriormente.

#### Adicionado
- Suporte a gravidade para ponto fixo
- Suporte a desativar gravidade
- Medidor de fps ao nome da janela
#### Alterado
- Lógica do spawn de particulas

### 0.2.0 - 28/12/2025
> Creio que o uso de Verlet simplificado para calculo deterministico vai servir bem.

#### Adicionado
- Parâmetros position e position_old a Particle
- Função solve_collisions a Particle com O(n²) com base no raio (atualmente constante)
- Função solve_enviroment_constraints_limits a Particle
- Gravidade
#### Removido
- Parâmetro velocity
#### Alterado
- Construtor impl default de Particle foi substituído por from_position
- Lógica do spawn de particulas
- apply_physics agora usa calculo dinâmico de velocity baseado em position e position_old

### 0.1.0 - 28/12/2025
#### Versão inicial