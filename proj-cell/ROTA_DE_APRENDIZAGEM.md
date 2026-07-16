# 🛤️ Rota de Aprendizado: Recife, Código e Reflexão

Este repositório é um laboratório vivo de um estudante de Engenharia da Computação em Recife. Inspirado na pedagogia de Paulo Freire, cada projeto busca conectar a sintaxe das linguagens com a realidade vivida, seja nos corredores da universidade ou nas ruas da cidade.

---

## 🐍 1. Python: O Mediador Socrático
**Objetivo:** Criar uma ferramenta de estudo baseada no diálogo para matérias de humanas ou exatas.

* **A Analogia:** Imagine aquele **caderno de monitoria** que fica na mesa da biblioteca. Um aluno escreve uma dúvida/reflexão, e o próximo lê e complementa. O programa será esse mediador.
* **O que construir:** Um script que lê temas de um arquivo `.txt` e interage com o usuário, salvando as respostas para revisão posterior.
* **Destaques Técnicos:**
    * **Persistência:** Manipulação de arquivos (`io`).
    * **Dinamismo:** Uso de dicionários para mapear temas e perguntas.
    * **Interface:** CLI (Interface de Linha de Comando) amigável.
* **Para a Monitoria:** Explique como o software pode ser uma extensão da nossa memória e como o Python facilita a manipulação de textos.

## 🧱 2. C++: Motor de Probabilidades de RPG
**Objetivo:** Entender lógica pura, tipos de dados e cálculos de precisão aplicados a mecânicas de jogos.

* **A Analogia:** Como os **amortecedores de um ônibus** enfrentando os buracos das avenidas de Recife: precisamos de fórmulas que calculem o impacto (dano) e a resistência (armadura) com precisão matemática.
* **O que construir:** Um sistema que defina "Entidades" (Structs) e processe rodadas de ações baseadas em probabilidades matemáticas (usando funções de `<cmath>`).
* **Destaques Técnicos:**
    * **Tipagem Forte:** Uso rigoroso de `int`, `float` e `double`.
    * **Estruturas de Dados:** Uso de `struct` para organizar atributos de personagens.
    * **Performance:** Observar como o C++ lida com cálculos rápidos.
* **Para a Monitoria:** Mostre como as funções de Cálculo 1 (limites e funções lineares) se tornam "regras de mundo" dentro de um código de baixo nível.

## 🦀 3. Rust: O Fiscal de Integridade (Porteiro do Derby)
**Objetivo:** Explorar segurança de memória e interação com o Sistema Operacional (Linux/Termux).

* **A Analogia:** Ele é o **porteiro rigoroso de um prédio no Derby**. Ele não deixa ninguém entrar sem conferir a lista, garante que ninguém use a chave de outra pessoa (Ownership) e avisa se algo mudou na estrutura do prédio.
* **O que construir:** Um utilitário que monitora a pasta do seu repositório, lendo metadados dos arquivos e reportando alterações ou tamanhos anormais.
* **Destaques Técnicos:**
    * **Ownership & Borrowing:** Garantir que o programa não tente ler um arquivo que foi "movido" ou deletado.
    * **Módulo `std::fs`:** Interação direta com as syscalls do kernel.
    * **Segurança:** Prevenção de erros em tempo de compilação.
* **Para a Monitoria:** Use para explicar por que o Rust evita os famosos "erros de segmentação" que acontecem no C++.

---

## 📚 Referências de Apoio

* **Python:** [Python.org - Input/Output Tutorial](https://docs.python.org/3/tutorial/inputoutput.html)
* **C++:** [LearnCpp.com - Basic Logic & Math](https://www.learncpp.com/)
* **Rust:** [The Rust Book - Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
* **Educação:** *Pedagogia da Autonomia* (Paulo Freire) - Para lembrar que codar também é um ato político e social.

---

## 🔋 Nota sobre Energia e Saúde Mental
* **Respeite o Clock:** Se a energia estiver baixa, apenas documente uma ideia ou leia uma linha de código.
* **Progressão, não perfeição:** O GitHub aceita commits pequenos. O importante é manter o "diálogo" com o código, mesmo que seja um parágrafo por dia.
