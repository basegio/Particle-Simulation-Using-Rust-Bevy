# Changelog

### 0.4.1 - 30/12/2025
> Apenas algumas melhorias de código // Desaparecimento de números mágicos // Organização e boas práticas.

#### Adicionado
- Adicionado max_particle_radius ao SimulationSettings
- Adicionado radius ao construtor da Particle
- Adicionado GridCollision e SimulationSettings como provedores de informações para spawn de particulas
#### Alterado
- Tamanho do GridCollision recebe o valor do Resource SimulationSettings
- Tamanho da célula do grid recebe o diâmetro da maior partícula definida em SimulationSettings
- Alterado nome do construtor da Particle, from_position -> new
- Particles Draw busca radius definido na Partícula


### 0.4.0 - 29/12/2025
> Cá estou eu as três da manhã do dia 29/12/2025 estudando uma estrutura de grid que seja interessante para resolver meu problema de colisões entre particulas inimiga do desempenho, estou com certa confiança em implementar uniform grid, acredito que seja uma implementação menos eficiente que uma kd-tree, mas sem dúvidas é mais fácil de implementar um multithread em um grid com tamanho constante do que recalcular uma arvore de divisões a cada fixedUpdate. De qualquer forma tudo está sujeito a mudanças no futuro, vamos ver como se sai essas abordagens.

> No meio das minhas implementações do dia seguinte, (cinco da manhã meu cérebro não estava funcionando bem), tive uma série de dificuldades por ter misturado a lógica de getIndex de uma célula a partir das coordenadas do grid com a lógica a partir das coordenadas das partículas. Enfim, era pra ser 2 sistemas independêntes pois as particulas tem posição normalizadas para o centro da simulação (0,0) e assim foi feito. Admito que com minha experiência de 2 dias com Rust tem sido bem complicado desenvolver algumas lógicas, mas deve mudar com o costume e tempo, os problemas de coordenadas eu tive que resolver a parte no dartpad.dev e converter de volta a rust depois.

> Como imaginado o ganho de desempenho foi muito alto, saindo de ~1 fps para ~>100fps (o medidor atualiza mais rápido que eu sou capaz de identificar os números, mas tem 3 números e o vsync é um limitante, 165fps no caso), com atualizações futuras teremos uma precisão maior na medida.

> Os sistemas de busca dinâmica do bevy tem sido muito úteis, junto com as buscas eficientes de ECS com querys, tem umas formas interessantes de validar e obter multiplos valores mutáveis em simultâneo sem ferir as definições de segurança de memória intrínsecas do Rust

#### Adicionado
- Uniform Grid para detecção eficiente de colisões
- Propriedade de raio para particulas
#### Removido
- Sistema de render zoom
#### Alterado
- Força de atração gravitacional minima foi reajustada para evitar problema de velocidade infinita por proximidade ao ponto de força
- Reajustado sistema de spawn
- Cálculo de colisão passou a ser baseado em grid
- Cálculo de colisão usa raio da partícula


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