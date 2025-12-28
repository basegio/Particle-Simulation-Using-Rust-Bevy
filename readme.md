# 🦀 Particle Simulator: Odisseia no Rust & Bevy

Status: WIP (Work In Progress) – Ou seja, se quebrar, é porque estou aprendendo. :)

## 📓 O Devlog
### Por que Rust?

Depois de passar uns bons anos no conforto do Dart (que é quase um abraço do Java), me deu aquela vontade de sofrer um pouco e entender como as coisas funcionam "debaixo do capô".

Escolhi Rust porque eu buscava algo que parecesse o C na sintaxe e na performance, mas que não me deixasse dar um tiro no pé a cada alocação de memória. Vou te falar: encarar algo compilado, sem Virtual Machine e sem Garbage Collector está sendo um desafio e tanto. É como trocar um carro automático por um manual onde se você errar a marcha, o motor explode (mas o compilador te avisa antes do impacto).


### A ideia: Um simulador de partículas

Sim, eu sei... outro simulador de partículas. Mas tem um motivo! Eu já fiz isso em várias linguagens e normalmente eu otimizo desempenho usando a GPU com Compute Shaders, incrivelmente bom para calculos de vetores.

Desta vez, quero ser purista:

- Foco na CPU: Quero ver o Rust fritar todos os núcleos do meu processador.
- Paralelismo Real: Nada de async de fachada. Quero uso intenso de multithread.
- Bevy & ECS: Escolhi o Bevy (estou usando a v0.17.3 — sim, o negócio é instável e muda todo dia) porque o foco deles em ECS (Entity Component System) é bizarramente rápido para lidar com milhares de entidades em paralelo.

Enquanto escrevia isso, já me cocei para fazer um projeto de IA independente de agentes interagindo com o ambiente, mas foco nisso aqui primeiro! Uma coisa de cada vez para não virar aquele cemitério de projetos inacabados no GitHub (minha situação atual).