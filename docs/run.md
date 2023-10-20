## RUN

### Função `alloc` da Estrutura `Heap`

A função `alloc` na estrutura `Heap` é responsável por alocar uma posição no array de dados, retornando o índice dessa posição. Aqui está um fluxograma simplificado e pseudocódigo para a função `alloc`:

**Pseudocódigo**:

```plaintext
Função alloc(size):
    Se o tamanho for igual a zero
        Retorne 0
    Senão, se o espaço não estiver cheio e houver espaço suficiente após o próximo
        Aumente o contador de espaço usado pelo tamanho
        Atualize a próxima posição disponível
        Retorne a posição anterior da próxima disponível como um valor
    Senão
        Defina o espaço como cheio
        Enquanto houver um espaço contíguo de tamanho disponível
            Se o próximo estiver além do limite, reinicie a partir do início
            Se a porta P1 da posição próxima for nula, incremente o contador de espaço em um
            Senão, redefina o contador de espaço para zero
            Atualize a posição próxima
            Se o contador de espaço atingir o tamanho desejado
                Aumente o contador de espaço usado pelo tamanho
                Retorne a posição anterior da próxima disponível como um valor
```

Essa função é usada para alocar espaço no array de dados na estrutura `Heap`. Ela verifica se o heap não está cheio e se há espaço contíguo disponível para alocar a quantidade especificada de dados. Se o heap estiver cheio ou não houver espaço contíguo disponível, ele realiza uma pesquisa para encontrar espaço livre no heap e, em seguida, aloca e retorna o índice apropriado. O contador "used" é aumentado para rastrear as posições alocadas.

<details>
  <summary>Fluxograma</summary>

```plaintext
Início
|
V
Receba como entrada: "size" (tamanho da alocação)
|
V
Se "size" for igual a 0, retorne 0
|
V
Se o heap não estiver cheio e "next + size" for menor ou igual ao tamanho do array:
|
|--> Sim
|     |
|     V
|     Aloque espaço no heap para "size" unidades de dados a partir de "next"
|     |
|     V
|     Aumente o contador "used" em "size"
|     |
|     V
|     Atualize "next" para "next + size"
|     |
|     V
|     Retorne "next - size" como o índice alocado
|
|--> Não
|     |
|     V
|     O heap está cheio
|     |
|     V
|     Inicialize uma variável "space" como 0
|     |
|     |--> Loop
|          |
|          V
|          Se "next" for maior ou igual ao tamanho do array:
|          |
|          |--> Sim
|          |     |
|          |     V
|          |     Defina "space" como 0 e "next" como 1
|          |     |
|          |     V
|          |     Continue o loop
|          |
|          |--> Não
|          |     |
|          |     V
|          |     Se a porta P1 do elemento na posição "next" for NIL:
|          |     |
|          |     |--> Sim
|          |     |     |
|          |     |     V
|          |     |     Incremente "space" em 1
|          |     |     |
|          |     |     V
|          |     |     Se "space" for igual a "size":
|          |     |     |
|          |     |     |--> Sim
|          |     |     |     |
|          |     |     |     V
|          |     |     |     Incremente "used" em "size"
|          |     |     |     |
|          |     |     |     V
|          |     |     |     Retorne "next - space" como o índice alocado
|          |     |     |
|          |     |     |--> Não
|          |     |     |     |
|          |     |     |     V
|          |     |     |     Continue o loop
Fim
```

</details>

### Função `compact` da Estrutura `Heap`

Aqui está um fluxograma simplificado e pseudocódigo para a função `compact` na estrutura `Heap`:

**Fluxograma**:

```plaintext
Início
|
V
Crie uma lista vazia chamada "node"
Inicialize uma variável "índice" com 0
|
V
Enquanto o valor na posição de índice em "data" não for (NULL, NULL):
  |
  |-> Adicione o valor na posição de índice em "data" na lista "node"
  |-> Incremente "índice" em 1
|
V
Retorne a lista "node" como resultado da função
Fim
```

**Pseudocódigo**:

```plaintext
Função compact():
    Crie uma lista vazia chamada "nó".
    Repita enquanto o comprimento de "nó" for menor que o comprimento dos dados da heap:
        Se o primeiro componente do nó atual não for NULL ou o segundo componente não for NULL, adicione-o à lista "nó".
        Caso contrário, saia do loop.
    Retorne o "nó".
Fim da Função
```

Esta função cria uma lista chamada "node" e preenche-a com os valores contidos em "data" até encontrar um par de valores (NULL, NULL). Em seguida, retorna a lista "node" como resultado.

### Função `link` da Estrutura `Net`

A função `link` da Estrutura `Net` tem a finalidade de estabelecer conexões entre elementos, dependendo de seus tipos.

**Fluxograma**:

```plaintext
Início
|
V
Verifique os tipos de `a` e `b`
|
V
Se ambos são pri:
|   Sim
|   Verifique se `a` e `b` podem ser pulados
|   |
|   V
|   Se podem:
|   |   Sim
|   |   Incremente eras em 1
|   |   |
|   |   Fim
|   |
|   V
|   Não podem ser pulados
|   |
|   V
|   Adicione a tupla (a, b) em rdex
|   |
|   Fim
|
V
Se a é var:
|   Sim
|   Substitua o destino de a pelo valor de b
|   |
|   Fim
|
V
Se b é var:
|   Sim
|   Substitua o destino de b pelo valor de a
|   |
|   Fim
Fim
```

**Pseudocódigo**:

```plaintext
Função link(a, b):
    Se a é pri e b é pri:
        Se a e b podem ser pulados:
            Incremente eras em 1
        Senão:
            Adicione (a, b) à lista de redexes `rdex`
    Senão, se a é var:
        Substitua o destino de a pelo valor de b
    Senão, se b é var:
        Substitua o destino de b pelo valor de a
    Fim
```

Dessa forma, a função `link` realiza a ligação ou conexão entre elementos da estrutura `Net` de acordo com as regras especificadas para cada tipo de elemento, seja pri (prioritário) ou var (variável). Isso permite a criação e manipulação de conexões entre elementos da rede, o que é útil em diversas aplicações, como sistemas de inferência e processamento de informações.

### Função `interact` da Estrutura `Net`

A função `interact` da Estrutura `Net` é uma função complexa que define as interações entre diferentes tipos de elementos na estrutura. Ela é usada para realizar operações específicas com base nos tipos dos elementos `a` e `b`.

**Fluxograma**:

```plaintext
Função interact(a, b)
    Se a e b são do mesmo tipo de nó (por exemplo, ambos são do tipo CTR)
        Se a é igual a b (com base em algum critério específico)
            Chamar a função anni(a, b)
        Senão
            Chamar a função comm(a, b)
    Senão se a é um tipo de nó específico (por exemplo, CTR)
        Se a e b têm o mesmo tag
            Chamar a função anni(a, b)
        Senão
            Chamar a função comm(a, b)
    Senão se b é um tipo de nó específico (por exemplo, CTR)
        Chamar a função comm(b, a)
    Senão
        Chamar a função era2(a)
    Fim da Função
```

**Pseudocódigo**:

```plaintext
Início
 |
 V
A e B são do mesmo tipo de nó?
 |
 V
Sim
 |
 |---[A é igual a B?]---> Não
 |      |
 |      |---[Chamar a função anni(A, B)]---> Fim
 V
Não
 |
 |---[A é um tipo de nó específico (por exemplo, CTR)?]---> Não
 |      |
 |      |---[B é um tipo de nó específico (por exemplo, CTR)?]---> Não
 |      |      |
 |      |      |---[Chamar a função era2(A)]---> Fim
 |      |
 |      |---[Chamar a função comm(B, A)]---> Fim
 |
 |---[A e B têm o mesmo tag?]---> Não
 |      |
 |      |---[Chamar a função comm(A, B)]---> Fim
 V
Sim
 |
 |---[Chamar a função anni(A, B)]---> Fim
Fim
```

A função `interact` é fundamental para as operações de interação entre diferentes tipos de elementos na estrutura `Net`, permitindo a realização de diversas operações de processamento de informações e lógica na rede.

### Função `conn` da Estrutura `Net`

A função `conn` da Estrutura `Net` tem o propósito de realizar a conexão entre dois elementos `a` e `b` na rede.

**Fluxograma**:

```plaintext
Início
|
V
Incremente o valor de `anni` em 1
Obtenha o valor de P2 de `a` e P2 de `b`
|
V
Link o valor de P2 de `a` ao valor de P2 de `b`
|
V
Libere a memória referente a `a`
Libere a memória referente a `b`
|
Fim
```

**Diagrama**:

```
A2 --[#X}---[#Z}-- B2
~~~~~~~~~~~~~~~~~~~ OP1-OP1 
          ,----- B2
         X
A2 -----' 
```

**Pseudocódigo**:

```plaintext
Função conn(a, b):
    Incremente o valor de `anni` em 1
    Obtém o valor de P2 de `a` e P2 de `b`
    Link o valor de P2 de `a` ao valor de P2 de `b`
    Libera a memória referente a `a`
    Libera a memória referente a `b`
```

Essa função é usada para estabelecer conexões específicas entre elementos na estrutura `Net`, o que pode ser útil em várias aplicações, como em sistemas de inferência, onde as conexões representam relações lógicas entre conceitos ou entidades. O aumento de `anni` é importante para acompanhar a evolução da rede e das conexões ao longo do tempo.

### Função `anni` da Estrutura `Net`

A função `anni` da Estrutura `Net` tem o propósito de realizar uma ação de aninhamento, que envolve a criação de conexões entre elementos e o incremento do valor da variável `anni`.

**Fluxograma**:

```plaintext
Início
|
V
Incremente o valor de `anni` em 1
Link do valor de P1 com um valor derivado de `a`
Link do valor de P1 com um valor derivado de `b`
Link do valor de P2 com um valor derivado de `a`
Link do valor de P2 com um valor derivado de `b`
Libere a memória referente a `a`
Libere a memória referente a `b`
|
V
Fim
```

**Diagrama**:

```
A1 --|\     /|-- B2
     |a|---|b|   
A2 --|/     \|-- B1
~~~~~~~~~~~~~~~~~~~ CTR-CTR (A == B)
A1 -----, ,----- B2
         X
A2 -----' '----- B1
```

**Pseudocódigo**:

```plaintext
Função anni(a, b):
    Incremente o valor de `anni` em 1
    Link do valor de P1 com um valor derivado de `a`
    Link do valor de P1 com um valor derivado de `b`
    Link do valor de P2 com um valor derivado de `a`
    Link do valor de P2 com um valor derivado de `b`
    Libere a memória referente a `a`
    Libere a memória referente a `b`
```

Essa função é usada para realizar operações de aninhamento e criação de conexões em uma estrutura de rede, comumente encontrada em sistemas de processamento de informações e lógica. O incremento de `anni` é importante para acompanhar e controlar as operações de aninhamento ao longo do tempo.

### Função `comm` da Estrutura `Net`

A função `comm` da Estrutura `Net` tem a finalidade de realizar uma comunicação entre dois elementos `a` e `b`, onde são estabelecidas várias conexões específicas entre eles, além de realizar alocações de memória para armazenar informações relacionadas a essa comunicação.

**Fluxograma**:

```plaintext
Início
|
V
Incremente o valor de `comm` em 1
Aloque 4 slots de memória em `loc`
Link do valor de P1 com um valor derivado de `a`
Link do valor de P1 com um valor derivado de `b`
Link do valor de P2 com um valor derivado de `a`
Link do valor de P2 com um valor derivado de `b`
Link do valor de P1 com um valor derivado de `b`
Link do valor de P2 com um valor derivado de `a`
Link do valor de P1 com um valor derivado de `a`
Link do valor de P2 com um valor derivado de `b`
Aloque 2 slots de memória em `space` com valor zero
Enquanto o valor de `space` for menor que 4:
  |
  V
  Se o valor do índice `next` no vetor `data` for maior ou igual ao comprimento do vetor:
  |
  V
  Atribua 0 ao valor de `space`
  Defina o valor de `next` como 1
  |
  |
  V
  Se o valor do índice `next` no vetor `data` para o porto `P1` for igual a NULL:
  |
  V
  Incremente o valor de `space` em 1
  |
  V
  Senão, atribua 0 ao valor de `space`
  Incremente o valor de `next` em 1
  |
  |
  V
  Fim do Loop
|
V
Incrementa o valor de `used` em 4
Retorne
```

**Diagrama**:

```
A1 --|\         /|-- B2
     |a|-------|b|   
A2 --|/         \|-- B1
~~~~~~~~~~~~~~~~~~~~~~~ CTR-CTR (A != B)
      /|-------|\
A1 --|b|       |a|-- B2
      \|--, ,--|/
           X
      /|--' '--|\
A2 --|b|       |a|-- B1
      \|-------|/
```

**Pseudocódigo**:

```plaintext
Função comm(a, b):
    Incremente o valor de `comm` em 1
    Aloque 4 slots de memória em `loc`
    Link do valor de P1 com um valor derivado de `a`
    Link do valor de P1 com um valor derivado de `b`
    Link do valor de P2 com um valor derivado de `a`
    Link do valor de P2 com um valor derivado de `b`
    Link do valor de P1 com um valor derivado de `b`
    Link do valor de P2 com um valor derivado de `a`
    Link do valor de P1 com um valor derivado de `a`
    Link do valor de P2 com um valor derivado de `b`
    Aloque 2 slots de memória em `space` com valor zero
    Enquanto o valor de `space` for menor que 4:
        Se o valor do índice `next` no vetor `data` for maior ou igual ao comprimento do vetor:
            Atribua 0 ao valor de `space`
            Defina o valor de `next` como 1
        Se o valor do índice `next` no vetor `data` para o porto `P1` for igual a NULL:
            Incremente o valor de `space` em 1
        Senão, atribua 0 ao valor de `space`
            Incremente o valor de `next` em 1
    Incrementa o valor de `used` em 4
```

Essa função é usada para estabelecer conexões complexas entre elementos na estrutura `Net` durante uma operação de comunicação, e também para gerenciar alocações de memória relacionadas a essa operação. Isso é importante em sistemas de processamento de informações onde a comunicação e o gerenciamento de recursos são fundamentais.

### Função `pass` da Estrutura `Net`

A função `pass` da Estrutura `Net` tem o propósito de realizar uma ação de passagem de informações entre dois elementos `a` e `b`.

<details>
  <summary>Fluxograma</summary>
```plaintext
Início
|
V
Incremente o valor de `comm` em 1
Aloque 3 slots de memória em `loc`
Link do valor de P2 com um valor derivado de `b`
Link do valor de P1 com um valor derivado de `a`
Link do valor de P2 com um valor derivado de `a`
|
V
Fim
```
</details>

**Diagrama**:

```
WIP
A1 --|\         
     |a|-------[#Z}-- B2   
A2 --|/         
~~~~~~~~~~~~~~~~~~~~~~~ CTR-OP1 
TODO
```

**Pseudocódigo**:

```plaintext
Função pass(a, b):
    Incremente o valor de `comm` em 1
    Aloque 3 slots de memória em `loc`
    Link do valor de P2 com um valor derivado de `b`
    Link do valor de P1 com um valor derivado de `a`
    Link do valor de P2 com um valor derivado de `a`
    Retorne
```

Essa função é usada para estabelecer conexões específicas entre elementos na estrutura `Net` durante uma operação de passagem de informações, que pode ser útil em diversas aplicações, como sistemas de comunicação e processamento de dados. O incremento de `comm` é importante para acompanhar e controlar as operações de comunicação ao longo do tempo.

### Função `copy` da Estrutura `Net`

A função `copy` da Estrutura `Net` tem o propósito de realizar uma operação de cópia de informações de um elemento `a` para um elemento `b`.

<details>
  <summary>Fluxograma</summary>
```plaintext
Início
 |
 V
Obter valor de A
 |
 V
Criar uma cópia do nó B
 |
 |
 V
Definir os alvos das portas principais de A para apontar para a cópia de B
 |
 |
 V
Liberar o nó A
Fim
```
</details>

**Pseudocódigo**:

```plaintext
Função copy(a, b)
    Obtém o valor do nó A.
    Cria uma cópia do nó B.
    Define os alvos das portas principais de A para apontar para a cópia de B.
    Libera o nó A.
Fim da Função
```

Essa função é usada para copiar informações específicas de um elemento para outro na estrutura `Net`, o que pode ser útil em diversas aplicações, como sistemas de processamento de dados e lógica. O incremento de `comm` é importante para acompanhar e controlar as operações de cópia ao longo do tempo.

### Função `era2` da Estrutura `Net`

A função `era2` da Estrutura `Net` tem o propósito de realizar uma operação de "eraser," que envolve a remoção de informações de um elemento `a` e a criação de conexões com o valor "ERAS."

<details>
  <summary>Fluxograma</summary>
```plaintext
Início
|
V
Incremente o valor de `eras` em 1
Obtenha o valor de P1 de a.val()
Obtenha o valor de P2 de a.val()
Link do valor de P1 com o valor ERAS
Link do valor de P2 com o valor ERAS
Libere o valor de a.val()
|
V
Fim
```
</details>

**Diagrama**:

```
A1 --|\
     |a|-- ()
A2 --|/
~~~~~~~~~~~~~ {CTR/OP2/MAT}-ERA
A1 ------- ()
A2 ------- ()
```

**Pseudocódigo**:

```plaintext
Função era2(a):
    Incremente o valor de `eras` em 1
    Obtenha o valor de P1 de a.val()
    Obtenha o valor de P2 de a.val()
    Link do valor de P1 com o valor ERAS
    Link do valor de P2 com o valor ERAS
    Libere o valor de a.val()
    Retorne
```

Essa função é usada para realizar operações de apagamento de informações em uma estrutura de rede, o que pode ser útil em sistemas de processamento de dados onde a remoção de informações é necessária. O incremento de `eras` é importante para acompanhar e controlar as operações de apagamento ao longo do tempo.

### Função `era1` da Estrutura `Net`

A função `era1` da Estrutura `Net` tem o propósito de realizar uma operação de "eraser" mais específica, que envolve a remoção de informações de um único porto `P2` do elemento `a` e a criação de uma conexão com o valor "ERAS" nesse porto.

<details>
  <summary>Fluxograma</summary>
```plaintext
Início
|
V
Incremente o valor de `eras` em 1
Obtenha o valor de P2 de a.val()
Link do valor de P2 com o valor ERAS
Libere o valor de a.val()
|
V
Fim
```
</details>

**Diagrama**:

```
A2 --[#X}-- ()
~~~~~~~~~~~~~ OP1-ERA
A2 ------- ()
```

**Pseudocódigo**:

```plaintext
Função era1(a):
    Incremente o valor de `eras` em 1
    Obtenha o valor de P2 de a.val()
    Link do valor de P2 com o valor ERAS
    Libere o valor de a.val()
    Retorne
```

Essa função é usada para realizar operações de apagamento específico de informações em uma estrutura de rede, focando em um único porto. O incremento de `eras` é importante para acompanhar e controlar as operações de apagamento ao longo do tempo.

### Função `op2n` da Estrutura `Net`

A função `op2n` da Estrutura `Net` tem o propósito de realizar uma operação específica que envolve a manipulação de números.

<details>
  <summary>Fluxograma</summary>

```plaintext
Início
|
V
Obtenha o valor p1 a partir de a.val()
Verifique se p1 é um número
|
|---[Sim]---> Calcule `rt` como o resultado da função `prim` com parâmetros (valor de p1, valor de b)
|---[Não]---> Defina o valor P1 de a.val() como b
|
Obtenha o valor de P2 de a.val()
Link do novo valor de NUM `rt` com o valor de P2
Libere o valor de a.val()
|
V
Fim
```
</details>

**Diagrama**:

```
A1 --,
     [}-- #X
A2 --' 
~~~~~~~~~~~~~~ OP2-NUM
A2 --[#X}-- A1
```

**Pseudocódigo**:

```plaintext
Função op2n(a, b):
    p1 <- Obtenha o valor p1 a partir de a.val()
    Se p1 for um número:
        rt <- Calcule `rt` como o resultado da função `prim` com parâmetros (valor de p1, valor de b)
    Senão:
        Defina o valor P1 de a.val() como b
    Fim Se
    Obtenha o valor de P2 de a.val()
    Link do novo valor de NUM `rt` com o valor de P2
    Libere o valor de a.val()
    Retorne
```

Essa função é usada para realizar operações específicas envolvendo números na estrutura `Net`. Dependendo do tipo de operando `p1`, a função pode executar diferentes ações, como calcular o resultado da operação ou atribuir um novo valor a `a`. A operação `prim` é usada para realizar o cálculo necessário, e o resultado é armazenado em `rt`.

### Função `op1n` da Estrutura `Net`

A função `op1n` da Estrutura `Net` tem o propósito de realizar uma operação específica que envolve a manipulação de números.

<details>
  <summary>Fluxograma</summary>
```plaintext
Início
|
V
Obtenha o valor p1 a partir de a.val()
Obtenha o valor p2 a partir de b.val()
Obtenha v0 a partir dos bits 0-23 de p1
Obtenha v1 a partir dos bits 0-23 de p2
Calcule v2 como o resultado da função `prim` com parâmetros v0 e v1
Crie uma nova instância de Ptr com o operador NUM e o valor v2
Defina o valor P2 do novo Ptr como a instância p2
Libere o valor de a.val()
|
V
Fim
```
</details>

**Diagrama**:

```
A2 --[#X}-- #Y
~~~~~~~~~~~~~~ OP1-NUM
A2 -- #Z
```

**Pseudocódigo**:

```plaintext
Função op1n(a, b):
    p1 <- Obtenha o valor p1 a partir de a.val()
    p2 <- Obtenha o valor p2 a partir de b.val()
    v0 <- Obtenha v0 a partir dos bits 0-23 de p1
    v1 <- Obtenha v1 a partir dos bits 0-23 de p2
    v2 <- Calcule v2 como o resultado da função `prim` com parâmetros v0 e v1
    result <- Crie uma nova instância de Ptr com o operador NUM e o valor v2
    Defina o valor P2 do novo Ptr como a instância p2
    Libere o valor de a.val()
    Retorne result
```

Essa função é usada para realizar operações específicas envolvendo números na estrutura `Net`. Ela extrai partes dos valores `p1` e `p2`, realiza uma operação específica (`prim`), cria uma nova instância de `Ptr` com o resultado e estabelece conexões necessárias. O resultado da operação é retornado como uma nova instância de `Ptr` chamada `result`.

### Função `prim` da Estrutura `Net`

A função `prim` da Estrutura `Net` desempenha o papel de realizar operações binárias e lógicas em valores numéricos.

<details>
  <summary>Fluxograma</summary>
```plaintext
Início
 |
 V
Obter operador de A
 |
 V
Obter valor de A
 |
 V
Obter operador de B [não usado neste exemplo]
 |
 V
Obter valor de B
 |
 |
 V
Operador de A é USE?
 |
 |---[Sim]---> Define operador do resultado como operador de B
 | 
 |---[Não]---> Realizar a operação correspondente com base no operador de A
             |
             |
             V
             Retornar o resultado como um novo nó
Fim
```
</details>

**Pseudocódigo**:

```plaintext
Função prim(a, b)
    Obtém o operador do nó A (os bits superiores).
    Obtém o valor do nó A (os bits inferiores).
    Obtém o operador do nó B (os bits superiores) [não usado neste exemplo].
    Obtém o valor do nó B (os bits inferiores).
    
    Se o operador do nó A é USE
        Define o operador do resultado como o operador do nó B.
    Se não
        Realiza a operação correspondente com base no operador do nó A.
    
    Retorna o resultado como um novo nó.
Fim da Função
```

A função retorna o valor `result`, que é o resultado da operação determinada pelo operador `a_opr`. Essa função permite realizar várias operações matemáticas e lógicas com os valores contidos nas estruturas `a` e `b`.

### Função `mtch` da Estrutura `Net`

A função `mtch` da Estrutura `Net` realiza operações com ponteiros com base no valor do segundo argumento `b`.

<details>
  <summary>Fluxograma</summary>

```plaintext
Início
|
V
P1 (p1) <- Obtenha o primeiro argumento da ponteira (a) usando a função heap.get
P2 (p2) <- Obtenha o segundo argumento da ponteira (a) usando a função heap.get
|
V
Se o valor do segundo argumento (b.val()) for igual a 0:
|
|----> Crie um novo local (loc) na pilha da memória
|----> Defina o valor na posição (loc+0, P2) como ERAS
|----> Link entre o primeiro argumento da ponteira (p1) e o local (loc+0) com a tag CT0
|----> Link entre o segundo argumento da ponteira (p2) e o local (loc+0) com a tag VR1
|----> Libere a ponteira (a) na memória heap
|
V
Senão, se o valor do segundo argumento (b.val()) for diferente de 0:
|
|----> Crie um novo local (loc) na pilha da memória
|----> Defina o valor na posição (loc+0, P1) como ERAS
|----> Defina o valor na posição (loc+0, P2) como uma nova ponteira (PTR) com a tag CT0 e a posição (loc+1) como valor
|----> Link entre o primeiro argumento da ponteira (p1) e o local (loc+0) com a tag CT0
|----> Link entre o segundo argumento da ponteira (p2) e o local (loc+1) com a tag VR2
|----> Libere a ponteira (a) na memória heap
|
V
Fim
```

</details>

**Pseudocódigo**:

```plaintext
Função mtch(a, b):
    P1 (p1) <- Obtenha o primeiro argumento da ponteira (a) usando a função heap.get
    P2 (p2) <- Obtenha o segundo argumento da ponteira (a) usando a função heap.get

    Se o valor do segundo argumento (b.val()) for igual a 0:
        Crie um novo local (loc) na pilha da memória
        Defina o valor na posição (loc+0, P2) como ERAS
        Link entre o primeiro argumento da ponteira (p1) e o local (loc+0) com a tag CT0
        Link entre o segundo argumento da ponteira (p2) e o local (loc+0) com a tag VR1
        Libere a ponteira (a) na memória heap
    Senão, se o valor do segundo argumento (b.val()) for diferente de 0:
        Crie um novo local (loc) na pilha da memória
        Defina o valor na posição (loc+0, P1) como ERAS
        Defina o valor na posição (loc+0, P2) como uma nova ponteira (PTR) com a tag CT0 e a posição (loc+1) como valor
        Link entre o primeiro argumento da ponteira (p1) e o local (loc+0) com a tag CT0
        Link entre o segundo argumento da ponteira (p2) e o local (loc+1) com a tag VR2
        Libere a ponteira (a) na memória heap
```

**Diagrama**:

```
A1 --,
     (?)-- #X
A2 --' 
~~~~~~~~~~~~~~~~~~ MAT-NUM (#X > 0)
           /|-- A2
      /|--| |
A1 --| |   \|-- #(X-1)
      \|-- ()

A1 --,
     (?)-- #X
A2 --' 
~~~~~~~~~~~~~~~~~~ MAT-NUM (#X == 0)
      /|-- ()
A1 --| |   
      \|-- A2
```

Essa função lida com ponteiros e valores em relação ao valor do segundo argumento `b`. Dependendo do valor de `b`, diferentes operações de ligação e alocação de memória são executadas. Essa função é usada para manipular a estrutura de dados da rede e alocar memória com base nas condições definidas pelo valor de `b`.

### Função `deref` da Estrutura `Net`

A função `deref` da Estrutura `Net` realiza operações de desreferência de ponteiros, expandindo-os conforme necessário.

**Pseudocódigo**:

```plaintext
Função deref(book, ptr, parent):
    Enquanto ptr for um ponteiro do tipo REF:
        Se ptr apontar para uma rede fechada no livro book:
            Carrega a rede fechada do livro
            Ajusta os nós da rede com um novo local (loc)
            Conecta os nós da rede ao local atual no heap
            Carrega os redexes da rede
            Ajusta os redexes com base no local (loc) atual
            Conecta os redexes ajustados ao heap
            Define o novo valor de ptr como o nó raiz da rede
    Retorna ptr após todas as expansões
```

**Diagrama**:

```
A1 --|\
     | |-- @REF
A2 --|/
~~~~~~~~~~~~~~~~ CTR-REF
A1 --|\
     | |-- {val}
A2 --|/
```

Essa função é usada para desreferenciar ponteiros que apontam para redes fechadas, permitindo o acesso aos nós e redexes dessas redes. É uma parte fundamental para a manipulação de estruturas de rede na estrutura `Net`.

<details>
  <summary>Fluxograma</summary>
  
```plaintext
Início
|
V
Recebe a referência de um livro (book), um ponteiro (ptr) e um ponteiro pai (parent)
|
V
Enquanto ptr for um ponteiro do tipo REF
|
V
  Encontre o livro (book) do ptr atual
  |
  V
  Se o ptr atual apontar para uma rede fechada
  |
  V
    Carrega a rede fechada do livro (book)
    |
    V
    Ajusta os nós da rede fechada com um novo local (loc)
    |
    V
    Conecta os nós da rede fechada ao local atual (loc) no heap
    |
    V
    Carrega os redexes da rede fechada
    |
    V
    Ajusta os redexes com base no local (loc) atual
    |
    V
    Conecta os redexes ajustados ao heap
    |
    V
    Define o novo valor de ptr para o nó raiz da rede fechada
  |
  V
Retorna o ptr após todas as expansões
|
Fim
```

</details>

### Função `expand` da Estrutura `Net`

A função `expand` da Estrutura `Net` é responsável por expandir um ponteiro, o que envolve desreferenciar o ponteiro e realizar operações com base no tipo do ponteiro.

<details>
  <summary>Fluxograma</summary>
```plaintext
Início
|
V
Obtém ptr usando a função get_target
|
V
Se ptr for um ctr então
|    
|--> Expanda o contador para os portos auxiliares (VR1 e VR2)
|
V
Senão, se ptr for uma referência então
|    
|--> Expanda a referência e atualize o ponteiro de destino com a expansão
Fim
```
</details>

**Pseudocódigo**:

```plaintext
Função expand(net, book, dir):
    Obtenha o alvo (ptr) usando a função get_target
    Se ptr for um ctr então:
        Expanda o contador para os portos auxiliares (VR1 e VR2)
    Senão, se ptr for uma referência então:
        Expanda a referência e atualize o ponteiro de destino com a expansão
Fim da Função

```

Essa função é fundamental para a manipulação de ponteiros e redes na estrutura `Net`, permitindo a exploração de estruturas de dados mais complexas e a realização de operações em seus elementos. Ela expande tanto contadores quanto referências, garantindo que os ponteiros sejam desreferenciados e manipulados adequadamente.

### Função `reduce` da Estrutura `Net`

A função `reduce` da Estrutura `Net` é responsável por realizar a redução de redexes na rede.

<details>
  <summary>Fluxograma</summary>

```plaintext
Início
|
V
Enquanto houver redexes na rede
|
V
Para cada redex (a, b) na rede
|
V
Chame a função "interact" com os argumentos (net, book, a, b)
Fim
```
</details>

**Pseudocódigo**:

```plaintext
Função reduce(book):
    Enquanto houver redexes na rede:
        Para cada redex (a, b) na rede:
            Chame a função "interact" com os argumentos (net, book, a, b)
        Fim do loop
    Fim do loop
Fim
```

Essa função desempenha um papel crucial na execução de reduções na rede, permitindo que os redexes sejam identificados e manipulados de acordo com as regras específicas da estrutura `Net`. Isso é fundamental para a computação realizada pela rede.

### Função `normal` da Estrutura `Net`

A função `normal` da Estrutura `Net` é responsável por normalizar a rede, o que envolve a redução de redexes até que não haja mais redexes na rede.

<details>
  <summary>Fluxograma</summary>

```plaintext
Início
|
V
Recebe a referência de um livro (book)
|
V
Chama a função expand com um ponteiro (ROOT) e o livro (book)
|
V
Enquanto existirem redexes
|
V
Chama a função reduce com o livro (book) e função expand com um ponteiro (ROOT) e o livro (book)
|
Fim
```
</details>
**Pseudocódigo**:

```plaintext
Função normal(book):
    Chama expand com o ponteiro ROOT e o livro book

    Enquanto existirem redexes:
        Chama a função reduce com o livro book
        Chama expand com o ponteiro ROOT e o livro book
```

Essa função desempenha um papel central na normalização da rede, assegurando que todos os redexes sejam reduzidos de acordo com as regras da estrutura `Net`. A normalização é um passo importante em sistemas de redução ou computação formal, onde a expressão é simplificada até que alcance um estado irreversível.
