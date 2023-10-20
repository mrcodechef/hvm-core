     

```
()---()
~~~~~~~ ERA-ERA
nothing
```

```
A1 --|\
     |a|-- ()
A2 --|/
~~~~~~~~~~~~~ CTR-ERA
A1 ------- ()
A2 ------- ()
```

```
A1 --|\     /|-- B2
     |a|---|b|   
A2 --|/     \|-- B1
~~~~~~~~~~~~~~~~~~~ CTR-CTR (if a ~~ b)
A1 -----, ,----- B2
         X
A2 -----' '----- B1
```

```
A1 --|\         /|-- B2
     |a|-------|b|   
A2 --|/         \|-- B1
~~~~~~~~~~~~~~~~~~~~~~~ CTR-CTR (if a !~ b)
      /|-------|\
A1 --|b|       |a|-- B2
      \|--, ,--|/
           X
      /|--' '--|\
A2 --|b|       |a|-- B1
      \|-------|/
```


```
() -- @REF
~~~~~~~~~~ ERA-REF
nothing
```

```
A1 --|\
     | |-- @REF
A2 --|/
~~~~~~~~~~~~~~~~ CTR-REF
A1 --|\
     | |-- {val}
A2 --|/
```

```
A1 --,
     [}-- #X
A2 --' 
~~~~~~~~~~~~~~ OP2-NUM
A2 --[#X}-- A1
```

```
A1 --[#X}-- #Y
~~~~~~~~~~~~~~ OP1-NUM
A1 -- #Z
```

```
A1 --,
     (?)-- #X
A2 --' 
~~~~~~~~~~~~~~~~~~ MAT-NUM (#X > 0)
           /|-- A2
      /|--| |
A1 --| |   \|-- #(X-1)
      \|-- ()
```

```
A1 --,
     (?)-- #X
A2 --' 
~~~~~~~~~~~~~~~~~~ MAT-NUM (#X == 0)
      /|-- ()
A1 --| |   
      \|-- A2
```

## RUN

Aqui está o fluxograma simplificado e o pseudocódigo para a implementação da estrutura `Ptr`. Essa estrutura contém várias funções que operam em valores do tipo `Ptr`. As funções lidam com várias operações, como checar os tipos, tags e valores da estrutura `Ptr`, bem como realizar cálculos e verificações.

### Implementação da Estrutura `Ptr`:

**Fluxograma**:

```plaintext
Início
|
V
Receba como entrada: "tag" (tag da Ptr), "val" (valor da Ptr)
|
V
Crie uma nova instância de Ptr com o valor ((val << 4) | (tag como valor))
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Estrutura Ptr:
    Função nova(tag, val):
        Retorna uma nova instância de Ptr com o valor ((val << 4) | (tag como valor))

    Função data():
        Retorna o valor do objeto Ptr

    Função tag():
        Retorna a tag do objeto Ptr

    Função val():
        Retorna o valor do objeto Ptr

    Função is_nil():
        Retorna verdadeiro se o valor do objeto Ptr for igual a 0; caso contrário, retorna falso

    Função is_var():
        Retorna verdadeiro se a tag do objeto Ptr estiver no intervalo [VR1, VR2]; caso contrário, retorna falso

    Função is_era():
        Retorna verdadeiro se a tag do objeto Ptr for igual a ERA; caso contrário, retorna falso

    Função is_ctr():
        Retorna verdadeiro se a tag do objeto Ptr estiver no intervalo maior ou igual a CT0

    Função is_ref():
        Retorna verdadeiro se a tag do objeto Ptr for igual a REF; caso contrário, retorna falso

    Função is_pri():
        Retorna verdadeiro se a tag do objeto Ptr estiver no intervalo maior ou igual a REF

    Função is_num():
        Retorna verdadeiro se a tag do objeto Ptr for igual a NUM; caso contrário, retorna falso

    Função is_op1():
        Retorna verdadeiro se a tag do objeto Ptr for igual a OP1; caso contrário, retorna falso

    Função is_op2():
        Retorna verdadeiro se a tag do objeto Ptr for igual a OP2; caso contrário, retorna falso

    Função is_skp():
        Retorna verdadeiro se a tag do objeto Ptr for ERA, NUM ou REF; caso contrário, retorna falso

    Função is_mat():
        Retorna verdadeiro se a tag do objeto Ptr for igual a MAT; caso contrário, retorna falso

    Função has_loc():
        Retorna verdadeiro se o objeto Ptr for uma variável (var), OP1, OP2, MAT ou CTR

    Função adjust(loc):
        Retorna uma nova instância de Ptr com a tag e valor ajustados com base na localização (loc)

    Função can_skip(a, b):
        Retorna verdadeiro se ambos a e b forem ERA ou ambos REF; caso contrário, retorna falso

Fim da Estrutura Ptr
```

A estrutura `Ptr` possui várias funções para manipular e verificar valores do tipo `Ptr`, bem como funções para criar novas instâncias de `Ptr` com valores ajustados. 

A função `alloc` na estrutura `Heap` é responsável por alocar uma posição no array de dados, retornando o índice dessa posição. Aqui está um fluxograma simplificado e pseudocódigo para a função `alloc`:

### Função `alloc` da Estrutura `Heap`:

**Fluxograma**:

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
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função alloc(size):
    Se size for igual a 0, retorne 0
    
    Se o heap não estiver cheio e next + size for menor ou igual ao tamanho do array:
        Aloque espaço no heap para size unidades de dados a partir de next
        used = used + size
        next = next + size
        Retorne next - size como o índice alocado
    Senão:
        O heap está cheio
        Inicialize uma variável space como 0
        
        Enquanto True:
            Se next for maior ou igual ao tamanho do array:
                space = 0
                next = 1
            Senão, se a porta P1 do elemento na posição next for NIL:
                space = space + 1
                Se space for igual a size:
                    used = used + size
                    Retorne next - space como o índice alocado
```

Essa função é usada para alocar espaço no array de dados na estrutura `Heap`. Ela verifica se o heap não está cheio e se há espaço contíguo disponível para alocar a quantidade especificada de dados. Se o heap estiver cheio ou não houver espaço contíguo disponível, ele realiza uma pesquisa para encontrar espaço livre no heap e, em seguida, aloca e retorna o índice apropriado. O contador "used" é aumentado para rastrear as posições alocadas.

A função `free` na estrutura `Heap` é responsável por marcar uma posição no array de dados como livre, indicando que a mesma está disponível para alocação posterior. Aqui está um fluxograma simplificado e pseudocódigo para a função `free`:

### Função `free` da Estrutura `Heap`:

**Fluxograma**:

```plaintext
Início
|
V
Receba como entrada: "index" (posição no array)
|
V
Diminua o contador "used" em 1
|
V
Defina os valores nas portas P1 e P2 do elemento na posição "index" como NULL
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função free(index):
    used = used - 1  // Diminua o contador "used" em 1
    data[index].P1 = NULL  // Defina a porta P1 do elemento na posição "index" como NULL
    data[index].P2 = NULL  // Defina a porta P2 do elemento na posição "index" como NULL
```

Essa função é usada para liberar uma posição no array de dados na estrutura `Heap` após seu uso. A diminuição do contador `used` indica que menos elementos estão em uso no heap. As portas P1 e P2 do elemento na posição "index" são definidas como NULL, indicando que não há mais referências alocadas nessa posição.


A função `get` é usada na estrutura `Heap` para recuperar o valor associado a uma determinada posição (`index`) e porta (`port`) no array de dados. Aqui está um fluxograma simplificado e pseudocódigo para a função `get`:

### Função `get` da Estrutura `Heap`:

**Fluxograma**:

```plaintext
Início
|
V
Receba como entrada: "index" (posição no array), "port" (porta para acessar)
|
V
Acesse o elemento na posição "index" no array "data"
|
V
Se "port" for igual a P1:
  |
  |-> Retorne o valor na porta P1 do elemento
|
Senão:
  |
  |-> Retorne o valor na porta P2 do elemento
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função get(index, port):
    elemento = data[index]  // Acesse o elemento na posição "index" no array "data"
    Se port for igual a P1:
        Retorne elemento.P1  // Retorne o valor na porta P1 do elemento
    Senão:
        Retorne elemento.P2  // Retorne o valor na porta P2 do elemento
```

Essa função é usada para obter o valor associado às portas P1 ou P2 de um elemento na posição "index" do array "data" na estrutura `Heap`. 

A função `set` é utilizada na estrutura `Heap` para atribuir valores a elementos em um array de dados. Ela é uma operação essencial para manipular a memória do heap.

Aqui está um fluxograma simplificado e pseudocódigo para a função `set` na estrutura `Heap`:

### Função `set` da Estrutura `Heap`:

**Fluxograma**:

```plaintext
Início
|
V
Receba como entrada: "index" (posição no array), "port" (porta para acessar), "value" (valor a ser atribuído)
|
V
Acesse o elemento na posição "index" no array "data"
|
V
Se "port" for igual a P1:
  |
  |-> Atribua o valor "value" à porta P1 do elemento
|
Senão:
  |
  |-> Atribua o valor "value" à porta P2 do elemento
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função set(index, port, value):
    elemento = data[index]  // Acesse o elemento na posição "index" no array "data"
    Se port for igual a P1:
        elemento.P1 = value  // Atribua o valor "value" à porta P1 do elemento
    Senão:
        elemento.P2 = value  // Atribua o valor "value" à porta P2 do elemento
```

Essa função é usada para definir os valores das portas P1 e P2 de um elemento na posição "index" do array "data" na estrutura `Heap`.


Aqui está um fluxograma simplificado e pseudocódigo para a função `compact` na estrutura `Heap`:

### Função `compact` da Estrutura `Heap`:

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
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função compact():
    node = Lista vazia
    índice = 0
    Enquanto o valor na posição de índice em "data" não for (NULL, NULL):
        node.adicionar(data[índice])
        índice = índice + 1
    Retorne node
```

Esta função cria uma lista chamada "node" e preenche-a com os valores contidos em "data" até encontrar um par de valores (NULL, NULL). Em seguida, retorna a lista "node" como resultado.


### Função `to_def` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Início
|
V
Crie uma nova instância da estrutura Def chamada "def"
|
V
Defina a entrada "def.rdex" como o valor de "net.rdex"
|
V
Para cada elemento (p1, p2) em "net.heap.compact()":
|   |
|   V
|   Adicione (p1, p2) à lista "def.node"
|
V
Retorne a instância "def"
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função to_def(net):
    def = Nova instância da estrutura Def
    def.rdex = net.rdex
    Para cada p1, p2 em net.heap.compact():
        def.node.Adicione((p1, p2))
    Retorne def
```

A função to_def da Estrutura Net tem a finalidade de criar uma nova instância da estrutura Def, que é um componente de dados. O processo começa pela criação dessa instância, chamada de "def". Em seguida, o valor de "def.rdex" é definido como o valor de "net.rdex". Posteriormente, a função percorre cada par de elementos (p1, p2) presente na saída da função "net.heap.compact()" e os adiciona à lista "def.node". Uma vez que todos os elementos tenham sido processados, a função retorna a instância "def", que agora contém os dados correspondentes aos elementos da estrutura Net em um formato específico para a estrutura Def. Isso possibilita a conversão e transformação de dados de um formato para outro, útil em muitos contextos de programação e processamento de informações.

### Função `from_def` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Início
|
V
Crie uma nova instância da estrutura Net chamada "net"
|
V
Para cada elemento (i, (p1, p2)) em def.node:
|   |
|   V
|   Atualize a entrada na posição i da instância "net.heap" com p1 e p2
|
V
Defina a variável "net.rdex" como def.rdex
|
V
Retorne a instância "net"
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função from_def(def):
    net = Nova instância da estrutura Net
    Para cada i, (p1, p2) em def.node:
        net.heap.Atualize(i, p1, p2)
    net.rdex = def.rdex
    Retorne net
```
A função from_def da Estrutura Net tem o propósito de criar uma nova instância da estrutura Net com base em uma instância da estrutura Def. O processo começa com a criação da nova instância de Net, chamada "net". Em seguida, a função itera sobre os elementos em def.node, que consistem em índices (i) e pares de elementos (p1, p2). Para cada um deles, a função atualiza a entrada correspondente na instância "net.heap" com os valores p1 e p2. Após a conclusão dessa etapa, a função define a variável "net.rdex" com o valor de "def.rdex". Finalmente, a instância "net" é retornada, agora contendo os dados e configurações da instância "def" no formato da estrutura "Net". Isso permite a conversão e transformação de dados de uma estrutura para outra, facilitando o uso e manipulação dessas informações em diferentes contextos.

### Função `link` da Estrutura `Net`:

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
|   |   V
|   |   Fim
|   |
|   V
|   Não podem ser pulados
|   |
|   V
|   Adicione a tupla (a, b) em rdex
|   |
|   V
|   Fim
|
V
Se a é var:
|   Sim
|   Substitua o destino de a pelo valor de b
|   |
|   V
|   Fim
|
V
Se b é var:
|   Sim
|   Substitua o destino de b pelo valor de a
|   |
|   V
|   Fim
|
V
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

A função `link` da Estrutura `Net` tem a finalidade de estabelecer conexões entre elementos, dependendo de seus tipos. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função começa verificando os tipos de `a` e `b`.
2. Se ambos `a` e `b` forem elementos pri (prioritários), ela verifica se eles podem ser pulados. Se forem puláveis, incrementa a contagem de "eras" em 1. Caso contrário, adiciona a tupla (a, b) à lista `rdex`, que armazena as conexões.
3. Se `a` for uma variável (`var`), a função substitui o destino de `a` pelo valor de `b`.
4. Se `b` for uma variável (`var`), a função substitui o destino de `b` pelo valor de `a`.

Dessa forma, a função `link` realiza a ligação ou conexão entre elementos da estrutura `Net` de acordo com as regras especificadas para cada tipo de elemento, seja pri (prioritário) ou var (variável). Isso permite a criação e manipulação de conexões entre elementos da rede, o que é útil em diversas aplicações, como sistemas de inferência e processamento de informações.

### Função `interact` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Inicio
|
V
Verifica tipo de A e B
|
V
Se A e B são ref e skp
|
V
  - Incrementa eras
|
V
Se A e B são ctr e ctr e suas tags são iguais
|
V
  - Executa anni(A, B)
|
V
Se A e B são ctr e ctr e suas tags são diferentes
|
V
  - Executa comm(A, B)
|
V
Se A ou B são era
|
V
  - Incrementa eras
|
V
Se A é ctr e B é era
|
V
  - Executa era2(A)
|
V
Se A é era e B é ctr
|
V
  - Executa era2(B)
|
V
Se A é ref e B é era
|
V
  - Incrementa eras
|
V
Se A é era e B é ref
|
V
  - Incrementa eras
|
V
Se A e B são era
|
V
  - Incrementa eras
|
V
Se A é var
|
V
  - Incrementa eras
  - Chama link(A, B)
|
V
Se B é var
|
V
  - Incrementa eras
  - Chama link(B, A)
|
V
Se A é ctr e B é num
|
V
  - Chama copy(A, B)
|
V
Se A é num e B é ctr
|
V
  - Chama copy(B, A)
|
V
Se A e B são num
|
V
  - Incrementa eras
|
V
Se A é op2 e B é num
|
V
  - Chama op2n(A, B)
|
V
Se A é num e B é op2
|
V
  - Chama op2n(B, A)
|
V
Se A é op1 e B é num
|
V
  - Chama op1n(A, B)
|
V
Se A é num e B é op1
|
V
  - Chama op1n(B, A)
|
V
Se A é mat e B é num
|
V
  - Chama mtch(A, B)
|
V
Se A é num e B é mat
|
V
  - Chama mtch(B, A)
|
V
Se A é mat e B é ctr
|
V
  - Executa comm(A, B)
|
V
Se A é ctr e B é mat
|
V
  - Executa comm(B, A)
|
V
Se A é mat e B é era
|
V
  - Executa era2(A)
|
V
Se A é era e B é mat
|
V
  - Executa era2(B)
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função interact(a, b):
    Se a for ref e b for pri e não for skp:
        Atribua a = deref(book, a, b)
    Senão, se b for ref e a for pri e não for skp:
        Atribua b = deref(book, b, a)

    Se a for ctr e b for ctr e a.tag for igual a b.tag:
        Execute anni(a, b)
    Senão, se a for ctr e b for ctr e a.tag for diferente de b.tag:
        Execute comm(a, b)
    Senão, se a for era ou b for era:
        Incremente eras em 1
    Senão, se a for ctr e b for era:
        Execute era2(a)
    Senão, se a for era e b for ctr:
        Execute era2(b)
    Senão, se a for ref e b for era:
        Incremente eras em 1
    Senão, se a for era e b for ref:
        Incremente eras em 1
    Senão, se a e b forem ambos era:
        Incremente eras em 1
    Senão, se a for var:
        Incremente eras em 1
    Senão, se b for var:
        Incremente eras em 1
    Senão, se a for ctr e b for num:
        Execute copy(a, b)
    Senão, se a for num e b for ctr:
        Execute copy(b, a)
    Senão, se a for num e b for era:
        Incremente eras em 1
    Senão, se a for era e b for num:
        Incremente eras em 1
    Senão, se a e b forem ambos num:
        Incremente eras em 1
    Senão, se a for op2 e b for num:
        Execute op2n(a, b)
    Senão, se a for num e b for op2:
        Execute op2n(b, a)
    Senão, se a for op1 e b for num:
        Execute op1n(a, b)
    Senão, se a for num e b for op1:
        Execute op1n(b, a)
    Senão, se a for mat e b for num:
        Execute mtch(a, b)
    Senão, se a for num e b for mat:
        Execute mtch(b, a)
    Senão, se a for mat e b for ctr:
        Execute comm(a, b)
    Senão, se a for ctr e b for mat:
        Execute comm(b, a)
    Senão, se a for mat e b for era:
        Execute era2(a)
    Senão, se a for era e b for mat:
        Execute era2(b)
    Senão:
        Emita um erro
```

A função `interact` da Estrutura `Net` é uma função complexa que define as interações entre diferentes tipos de elementos na estrutura. Ela é usada para realizar operações específicas com base nos tipos dos elementos `a` e `b`. Vou explicar as ações realizadas com base nos tipos dos elementos:

1. Se `a` for uma referência (`ref`) e `b` for prioritário (`pri`) e não for do tipo `skp`, ela atribui `a` a `deref(book, a, b)`.
2. Se `b` for uma referência (`ref`) e `a` for prioritário (`pri`) e não for do tipo `skp`, ela atribui `b` a `deref(book, b, a)`.
3. Se ambos `a` e `b` forem do tipo `ctr` e tiverem a mesma tag, ela executa a função `anni(a, b)`.
4. Se ambos `a` e `b` forem do tipo `ctr`, mas tiverem tags diferentes, ela executa a função `comm(a, b)`.
5. Se `a` ou `b` forem do tipo `era`, ela incrementa a variável `eras` em 1.
6. Se `a` for do tipo `ctr` e `b` for do tipo `era`, ela executa a função `era2(a)`.
7. Se `a` for do tipo `era` e `b` for do tipo `ctr`, ela executa a função `era2(b)`.
8. Se `a` for uma referência (`ref`) e `b` for do tipo `era`, ela incrementa a variável `eras` em 1.
9. Se `a` for do tipo `era` e `b` for uma referência (`ref`), ela incrementa a variável `eras` em 1.
10. Se ambos `a` e `b` forem do tipo `era`, ela incrementa a variável `eras` em 1.
11. Se `a` for do tipo `var`, ela incrementa a variável `eras` em 1 e chama a função `link(a, b)`.
12. Se `b` for do tipo `var`, ela incrementa a variável `eras` em 1 e chama a função `link(b, a)`.
13. Se `a` for do tipo `ctr` e `b` for do tipo `num`, ela chama a função `copy(a, b)`.
14. Se `a` for do tipo `num` e `b` for do tipo `ctr`, ela chama a função `copy(b, a)`.
15. Se ambos `a` e `b` forem do tipo `num`, ela incrementa a variável `eras` em 1.
16. Se `a` for do tipo `op2` e `b` for do tipo `num`, ela chama a função `op2n(a, b)`.
17. Se `a` for do tipo `num` e `b` for do tipo `op2`, ela chama a função `op2n(b, a)`.
18. Se `a` for do tipo `op1` e `b` for do tipo `num`, ela chama a função `op1n(a, b)`.
19. Se `a` for do tipo `num` e `b` for do tipo `op1`, ela chama a função `op1n(b, a)`.
20. Se `a` for do tipo `mat` e `b` for do tipo `num`, ela chama a função `mtch(a, b)`.
21. Se `a` for do tipo `num` e `b` for do tipo `mat`, ela chama a função `mtch(b, a)`.
22. Se `a` for do tipo `mat` e `b` for do tipo `ctr`, ela executa a função `comm(a, b)`.
23. Se `a` for do tipo `ctr` e `b` for do tipo `mat`, ela executa a função `comm(b, a)`.
24. Se `a` for do tipo `mat` e `b` for do tipo `era`, ela executa a função `era2(a)`.
25. Se `a` for do tipo `era` e `b` for do tipo `mat`, ela executa a função `era2(b)`.
26. Caso nenhum dos casos acima seja satisfeito, um erro é emitido.

A função `interact` é fundamental para as operações de interação entre diferentes tipos de elementos na estrutura `Net`, permitindo a realização de diversas operações de processamento de informações e lógica na rede.

### Função `conn` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Início
|
V
Incremente o valor de `anni` em 1
Link o valor de P1 com um valor derivado de P2 em `a`
Link o valor de P1 com um valor derivado de P2 em `b`
Link o valor de P2 com um valor derivado de P1 em `a`
Link o valor de P2 com um valor derivado de P1 em `b`
Libere a memória referente a `a`
Libere a memória referente a `b`
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função conn(a, b):
    Incremente o valor de `anni` em 1
    Link o valor de P1 com um valor derivado de P2 em `a`
    Link o valor de P1 com um valor derivado de P2 em `b`
    Link o valor de P2 com um valor derivado de P1 em `a`
    Link o valor de P2 com um valor derivado de P1 em `b`
    Libere a memória referente a `a`
    Libere a memória referente a `b`
```

A função `conn` da Estrutura `Net` tem o propósito de realizar a conexão entre dois elementos `a` e `b` na rede. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia incrementando o valor da variável `anni` em 1. Isso é um indicativo de uma nova conexão sendo estabelecida.
2. Em seguida, a função realiza uma série de chamadas à função `link` para criar conexões entre os elementos `P1` e `P2` de `a` com os elementos derivados de `P2` de `b`, e vice-versa. Isso estabelece conexões bidirecionais entre esses elementos.
3. Após as conexões serem estabelecidas, a função libera a memória referente aos elementos `a` e `b`.

Essa função é usada para estabelecer conexões específicas entre elementos na estrutura `Net`, o que pode ser útil em várias aplicações, como em sistemas de inferência, onde as conexões representam relações lógicas entre conceitos ou entidades. O aumento de `anni` é importante para acompanhar a evolução da rede e das conexões ao longo do tempo.

### Função `anni` da Estrutura `Net`:

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
A função `anni` da Estrutura `Net` tem o propósito de realizar uma ação de aninhamento, que envolve a criação de conexões entre elementos e o incremento do valor da variável `anni`. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia incrementando o valor da variável `anni` em 1. Esse incremento é um indicativo de que uma nova operação de aninhamento está ocorrendo.
2. Em seguida, a função realiza uma série de chamadas à função `link` para criar conexões entre os valores de `P1` com valores derivados de `a` e `b`, e também entre os valores de `P2` com valores derivados de `a` e `b`. Isso estabelece múltiplas conexões entre os elementos envolvidos no aninhamento.
3. Após as conexões serem estabelecidas, a função libera a memória referente aos elementos `a` e `b`, indicando que esses elementos não são mais necessários.

Essa função é usada para realizar operações de aninhamento e criação de conexões em uma estrutura de rede, comumente encontrada em sistemas de processamento de informações e lógica. O incremento de `anni` é importante para acompanhar e controlar as operações de aninhamento ao longo do tempo.


### Função `comm` da Estrutura `Net`:

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

A função `comm` da Estrutura `Net` tem a finalidade de realizar uma comunicação entre dois elementos `a` e `b`, onde são estabelecidas várias conexões específicas entre eles, além de realizar alocações de memória para armazenar informações relacionadas a essa comunicação. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia incrementando o valor da variável `comm` em 1. Isso é um indicativo de uma nova operação de comunicação.
2. Em seguida, a função aloca 4 slots de memória em `loc` para armazenar informações sobre a comunicação.
3. A função realiza uma série de chamadas à função `link` para criar conexões entre os valores de `P1` com valores derivados de `a` e `b`, e também entre os valores de `P2` com valores derivados de `a` e `b`. Isso estabelece conexões entre esses elementos.
4. A função também cria conexões adicionais, vinculando o valor de `P1` com um valor derivado de `b` e o valor de `P2` com um valor derivado de `a`.
5. Em seguida, a função aloca 2 slots de memória em `space` com valor zero.
6. Inicia um loop enquanto o valor de `space` for menor que 4.
7. No loop, a função verifica se o valor do índice `next` no vetor `data` é maior ou igual ao comprimento do vetor. Se isso for verdade, ela redefine o valor de `space` como 0 e define o valor de `next` como 1.
8. A função também verifica se o valor do índice `next` no vetor `data` para o porto `P1` é igual a NULL. Se for, ela incrementa o valor de `space` em 1. Caso contrário, atribui 0 ao valor de `space` e incrementa o valor de `next` em 1.
9. Após o loop, a função incrementa o valor de `used` em 4, indicando que 4 slots de memória estão em uso.
10. A função encerra e retorna.

Essa função é usada para estabelecer conexões complexas entre elementos na estrutura `Net` durante uma operação de comunicação, e também para gerenciar alocações de memória relacionadas a essa operação. Isso é importante em sistemas de processamento de informações onde a comunicação e o gerenciamento de recursos são fundamentais.

### Função `pass` da Estrutura `Net`:

**Fluxograma**:

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
A função `pass` da Estrutura `Net` tem o propósito de realizar uma ação de passagem de informações entre dois elementos `a` e `b`. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia incrementando o valor da variável `comm` em 1. Isso é um indicativo de uma nova operação de comunicação.
2. A função aloca 3 slots de memória em `loc` para armazenar informações relacionadas à passagem.
3. A função realiza uma série de chamadas à função `link` para criar conexões entre os valores de `P2` com valores derivados de `b`, entre o valor de `P1` com um valor derivado de `a`, e entre o valor de `P2` com um valor derivado de `a`.
4. Após a criação das conexões, a função retorna.

Essa função é usada para estabelecer conexões específicas entre elementos na estrutura `Net` durante uma operação de passagem de informações, que pode ser útil em diversas aplicações, como sistemas de comunicação e processamento de dados. O incremento de `comm` é importante para acompanhar e controlar as operações de comunicação ao longo do tempo.

### Função `copy` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Início
|
V
Incremente o valor de `comm` em 1
Obtenha o valor de P1 de a.val()
Obtenha o valor de P2 de a.val()
Link do valor de P1 com b
Link do valor de P2 com b
Libere o valor de a.val()
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função copy(a, b):
    Incremente o valor de `comm` em 1
    Obtenha o valor de P1 de a.val()
    Obtenha o valor de P2 de a.val()
    Link do valor de P1 com b
    Link do valor de P2 com b
    Libere o valor de a.val()
    Retorne
```

A função `copy` da Estrutura `Net` tem o propósito de realizar uma operação de cópia de informações de um elemento `a` para um elemento `b`. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia incrementando o valor da variável `comm` em 1. Isso é um indicativo de uma nova operação de cópia.
2. A função obtém os valores dos portos `P1` e `P2` do elemento `a.val()`. Isso indica que a função está adquirindo informações do elemento `a` para copiá-las para o elemento `b`.
3. A função realiza duas chamadas à função `link` para criar conexões entre os valores de `P1` e `P2` de `a.val()` com o elemento `b`. Isso estabelece as conexões necessárias para realizar a cópia das informações.
4. Após a criação das conexões, a função libera a memória referente a `a.val()`, indicando que essas informações não são mais necessárias.

Essa função é usada para copiar informações específicas de um elemento para outro na estrutura `Net`, o que pode ser útil em diversas aplicações, como sistemas de processamento de dados e lógica. O incremento de `comm` é importante para acompanhar e controlar as operações de cópia ao longo do tempo.

### Função `era2` da Estrutura `Net`:

**Fluxograma**:

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
A função `era2` da Estrutura `Net` tem o propósito de realizar uma operação de "eraser," que envolve a remoção de informações de um elemento `a` e a criação de conexões com o valor "ERAS." O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia incrementando o valor da variável `eras` em 1. Isso é um indicativo de uma nova operação de "eraser."
2. A função obtém os valores dos portos `P1` e `P2` do elemento `a.val()`. Isso indica que a função está adquirindo informações do elemento `a` que serão posteriormente apagadas.
3. A função realiza duas chamadas à função `link` para criar conexões entre os valores de `P1` e `P2` de `a.val()` com o valor "ERAS." Isso remove efetivamente as informações contidas nesses portos e substitui-os pelo valor "ERAS."
4. Após a criação das conexões, a função libera a memória referente a `a.val()`, indicando que essas informações foram apagadas.

Essa função é usada para realizar operações de apagamento de informações em uma estrutura de rede, o que pode ser útil em sistemas de processamento de dados onde a remoção de informações é necessária. O incremento de `eras` é importante para acompanhar e controlar as operações de apagamento ao longo do tempo.

### Função `era1` da Estrutura `Net`:

**Fluxograma**:

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

**Pseudocódigo**:

```plaintext
Função era1(a):
    Incremente o valor de `eras` em 1
    Obtenha o valor de P2 de a.val()
    Link do valor de P2 com o valor ERAS
    Libere o valor de a.val()
    Retorne
```

A função `era1` da Estrutura `Net` tem o propósito de realizar uma operação de "eraser" mais específica, que envolve a remoção de informações de um único porto `P2` do elemento `a` e a criação de uma conexão com o valor "ERAS" nesse porto. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia incrementando o valor da variável `eras` em 1. Isso é um indicativo de uma nova operação de "eraser."
2. A função obtém o valor do porto `P2` do elemento `a.val()`. Isso indica que a função está adquirindo informações de um único porto de `a` que serão posteriormente apagadas.
3. A função realiza uma chamada à função `link` para criar uma conexão entre o valor de `P2` de `a.val()` e o valor "ERAS." Isso remove efetivamente as informações contidas no porto `P2` e substitui-o pelo valor "ERAS."
4. Após a criação da conexão, a função libera a memória referente a `a.val()`, indicando que a informação foi apagada.

Essa função é usada para realizar operações de apagamento específico de informações em uma estrutura de rede, focando em um único porto. O incremento de `eras` é importante para acompanhar e controlar as operações de apagamento ao longo do tempo.




### Função `op2n` da Estrutura `Net`:

**Fluxograma**:

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

A função `op2n` da Estrutura `Net` tem o propósito de realizar uma operação específica que envolve a manipulação de números. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia obtendo o valor de `p1` a partir de `a.val()`, que representa o primeiro operando da operação.
2. Em seguida, a função verifica se `p1` é um número:
   - Se for um número, a função calcula `rt` como o resultado da função `prim` com parâmetros (valor de `p1`, valor de `b`). Isso significa que está realizando uma operação específica com os operandos `p1` e `b`, e o resultado é armazenado em `rt`.
   - Se `p1` não for um número, a função define o valor do porto `P1` de `a.val()` como `b`. Isso indica que `a` agora possui o valor de `b`.
3. A função então obtém o valor do porto `P2` de `a.val()`.
4. A função cria uma conexão entre o novo valor de `NUM` (representando o valor calculado `rt`) e o valor de `P2`.
5. Finalmente, a função libera a memória referente a `a.val()`.

Essa função é usada para realizar operações específicas envolvendo números na estrutura `Net`. Dependendo do tipo de operando `p1`, a função pode executar diferentes ações, como calcular o resultado da operação ou atribuir um novo valor a `a`. A operação `prim` é usada para realizar o cálculo necessário, e o resultado é armazenado em `rt`.

### Função `op1n` da Estrutura `Net`:

**Fluxograma**:

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

A função `op1n` da Estrutura `Net` tem o propósito de realizar uma operação específica que envolve a manipulação de números. O processo é descrito no fluxograma e pseudocódigo da seguinte forma:

1. A função inicia obtendo o valor de `p1` a partir de `a.val()`, que representa o primeiro operando da operação.
2. A função também obtém o valor de `p2` a partir de `b.val()`, que representa o segundo operando da operação.
3. A função extrai `v0` dos bits 0-23 de `p1`.
4. A função extrai `v1` dos bits 0-23 de `p2`.
5. A função calcula `v2` como o resultado da função `prim` com os parâmetros `v0` e `v1`. Isso significa que está realizando uma operação específica com os operandos `v0` e `v1`, e o resultado é armazenado em `v2`.
6. A função cria uma nova instância de `Ptr` com o operador `NUM` e o valor `v2`.
7. A função define o valor do porto `P2` do novo `Ptr` como a instância `p2`, estabelecendo uma conexão entre eles.
8. Finalmente, a função libera a memória referente a `a.val()`.

Essa função é usada para realizar operações específicas envolvendo números na estrutura `Net`. Ela extrai partes dos valores `p1` e `p2`, realiza uma operação específica (`prim`), cria uma nova instância de `Ptr` com o resultado e estabelece conexões necessárias. O resultado da operação é retornado como uma nova instância de `Ptr` chamada `result`.

### Função `prim` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Início
|
V
Obtenha o valor do operador a (a_opr) de a
Obtenha o valor do operador b (b_opr) de b (não usado por enquanto)
Obtenha o valor a_val de a
Obtenha o valor b_val de b
|
V
Se o operador a_opr for igual a USE:
|
|----> Crie um novo valor (result) com os bits 24-28 iguais aos bits 0-3 de b_val e os bits 0-23 de a_val
|
V
Senão, se o operador a_opr for igual a ADD:
|
|----> Crie um novo valor (result) com a soma de a_val e b_val (aplicando operação de módulo 2^24)
|
V
Senão, se o operador a_opr for igual a SUB:
|
|----> Crie um novo valor (result) com a subtração de a_val e b_val (aplicando operação de módulo 2^24)
|
V
Senão, se o operador a_opr for igual a MUL:
|
|----> Crie um novo valor (result) com a multiplicação de a_val e b_val (aplicando operação de módulo 2^24)
|
V
Senão, se o operador a_opr for igual a DIV:
|
|----> Crie um novo valor (result) com a divisão de a_val por b_val (aplicando operação de módulo 2^24)
|
V
Senão, se o operador a_opr for igual a MOD:
|
|----> Crie um novo valor (result) com o módulo da divisão de a_val por b_val (aplicando operação de módulo 2^24)
|
V
Senão, se o operador a_opr for igual a EQ:
|
|----> Crie um novo valor (result) com 1 se a_val for igual a b_val, caso contrário, 0
|
V
Senão, se o operador a_opr for igual a NE:
|
|----> Crie um novo valor (result) com 1 se a_val for diferente de b_val, caso contrário, 0
|
V
Senão, se o operador a_opr for igual a LT:
|
|----> Crie um novo valor (result) com 1 se a_val for menor que b_val, caso contrário, 0
|
V
Senão, se o operador a_opr for igual a GT:
|
|----> Crie um novo valor (result) com 1 se a_val for maior que b_val, caso contrário, 0
|
V
Senão, se o operador a_opr for igual a AND:
|
|----> Crie um novo valor (result) com a operação lógica AND entre a_val e b_val
|
V
Senão, se o operador a_opr for igual a OR:
|
|----> Crie um novo valor (result) com a operação lógica OR entre a_val e b_val
|
V
Senão, se o operador a_opr for igual a XOR:
|
|----> Crie um novo valor (result) com a operação lógica XOR entre a_val e b_val
|
V
Senão, se o operador a_opr for igual a NOT:
|
|----> Crie um novo valor (result) com a operação lógica NOT de b_val
|
V
Senão, se o operador a_opr for igual a LSH:
|
|----> Crie um novo valor (result) com o deslocamento à esquerda de a_val em b_val posições (aplicando operação de módulo 2^24)
|
V
Senão, se o operador a_opr for igual a RSH:
|
|----> Crie um novo valor (result) com o deslocamento à direita de a_val em b_val posições (aplicando operação de módulo 2^24)
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função prim(a, b):
    a_opr <- Obtenha o valor do operador a (a_opr) de a
    b_opr <- Obtenha o valor do operador b (b_opr) de b (não usado por enquanto)
    a_val <- Obtenha o valor a_val de a
    b_val <- Obtenha o valor b_val de b

    Se o operador a_opr for igual a USE:
        Crie um novo valor (result) com os bits 24-28 iguais aos bits 0-3 de b_val e os bits 0-23 de a_val
    Senão, se o operador a_opr for igual a ADD:
        Crie um novo valor (result) com a soma de a_val e b_val (aplicando operação de módulo 2^24)
    Senão, se o operador a_opr for igual a SUB:
        Crie um novo valor (result) com a subtração de a_val e b_val (aplicando operação de módulo 2^24)
    Senão, se o operador a_opr for igual a MUL:
        Crie um novo valor (result) com a multiplicação de a_val e b_val (aplicando operação de módulo 2^24)
    Senão, se o operador a_opr for igual a DIV:
        Crie um novo valor (result) com a divisão de a_val por b_val (aplicando operação de módulo 2^24)
    Senão, se

 o operador a_opr for igual a MOD:
        Crie um novo valor (result) com o módulo da divisão de a_val por b_val (aplicando operação de módulo 2^24)
    Senão, se o operador a_opr for igual a EQ:
        Crie um novo valor (result) com 1 se a_val for igual a b_val, caso contrário, 0
    Senão, se o operador a_opr for igual a NE:
        Crie um novo valor (result) com 1 se a_val for diferente de b_val, caso contrário, 0
    Senão, se o operador a_opr for igual a LT:
        Crie um novo valor (result) com 1 se a_val for menor que b_val, caso contrário, 0
    Senão, se o operador a_opr for igual a GT:
        Crie um novo valor (result) com 1 se a_val for maior que b_val, caso contrário, 0
    Senão, se o operador a_opr for igual a AND:
        Crie um novo valor (result) com a operação lógica AND entre a_val e b_val
    Senão, se o operador a_opr for igual a OR:
        Crie um novo valor (result) com a operação lógica OR entre a_val e b_val
    Senão, se o operador a_opr for igual a XOR:
        Crie um novo valor (result) com a operação lógica XOR entre a_val e b_val
    Senão, se o operador a_opr for igual a NOT:
        Crie um novo valor (result) com a operação lógica NOT de b_val
    Senão, se o operador a_opr for igual a LSH:
        Crie um novo valor (result) com o deslocamento à esquerda de a_val em b_val posições (aplicando operação de módulo 2^24)
    Senão, se o operador a_opr for igual a RSH:
        Crie um novo valor (result) com o deslocamento à direita de a_val em b_val posições (aplicando operação de módulo 2^24)
    
    Retorne result
```

A função `prim` da Estrutura `Net` desempenha o papel de realizar operações binárias e lógicas em valores numéricos. O processo é determinado pelo operador contido na estrutura e envolve a manipulação dos valores numéricos.

Aqui está a explicação da função com base no fluxograma e pseudocódigo:

1. A função começa obtendo o operador contido na estrutura `a` (denominado `a_opr`) e o operador contido na estrutura `b` (denominado `b_opr`). O operador em `b` não é usado no momento.
2. A função obtém os valores numéricos de `a` e `b`, denominados `a_val` e `b_val`, respectivamente.

Agora, dependendo do operador `a_opr`, a função executa uma ação específica:

- Se `a_opr` for igual a `USE`:
  - A função cria um novo valor chamado `result` com os bits 24-28 iguais aos bits 0-3 de `b_val` e os bits 0-23 de `a_val`.

- Se `a_opr` for igual a `ADD`:
  - A função cria um novo valor `result` que é a soma de `a_val` e `b_val`, aplicando uma operação de módulo com 2^24.

- Se `a_opr` for igual a `SUB`:
  - A função cria um novo valor `result` que é a subtração de `a_val` e `b_val`, aplicando uma operação de módulo com 2^24.

- Se `a_opr` for igual a `MUL`:
  - A função cria um novo valor `result` que é o produto de `a_val` e `b_val`, aplicando uma operação de módulo com 2^24.

- Se `a_opr` for igual a `DIV`:
  - A função cria um novo valor `result` que é a divisão de `a_val` por `b_val`, aplicando uma operação de módulo com 2^24.

- Se `a_opr` for igual a `MOD`:
  - A função cria um novo valor `result` que é o módulo da divisão de `a_val` por `b_val`, aplicando uma operação de módulo com 2^24.

- Se `a_opr` for igual a `EQ`:
  - A função cria um novo valor `result` com o valor 1 se `a_val` for igual a `b_val`, caso contrário, com o valor 0.

- Se `a_opr` for igual a `NE`:
  - A função cria um novo valor `result` com o valor 1 se `a_val` for diferente de `b_val`, caso contrário, com o valor 0.

- Se `a_opr` for igual a `LT`:
  - A função cria um novo valor `result` com o valor 1 se `a_val` for menor do que `b_val`, caso contrário, com o valor 0.

- Se `a_opr` for igual a `GT`:
  - A função cria um novo valor `result` com o valor 1 se `a_val` for maior do que `b_val`, caso contrário, com o valor 0.

- Se `a_opr` for igual a `AND`:
  - A função cria um novo valor `result` com o resultado da operação lógica `AND` entre `a_val` e `b_val`.

- Se `a_opr` for igual a `OR`:
  - A função cria um novo valor `result` com o resultado da operação lógica `OR` entre `a_val` e `b_val`.

- Se `a_opr` for igual a `XOR`:
  - A função cria um novo valor `result` com o resultado da operação lógica `XOR` entre `a_val` e `b_val`.

- Se `a_opr` for igual a `NOT`:
  - A função cria um novo valor `result` com o resultado da operação lógica `NOT` em `b_val`.

- Se `a_opr` for igual a `LSH`:
  - A função cria um novo valor `result` com o resultado do deslocamento à esquerda de `a_val` em `b_val` posições, aplicando uma operação de módulo com 2^24.

- Se `a_opr` for igual a `RSH`:
  - A função cria um novo valor `result` com o resultado do deslocamento à direita de `a_val` em `b_val` posições, aplicando uma operação de módulo com 2^24.

A função retorna o valor `result`, que é o resultado da operação determinada pelo operador `a_opr`. Essa função permite realizar várias operações matemáticas e lógicas com os valores contidos nas estruturas `a` e `b`.

### Função `mtch` da Estrutura `Net`:

**Fluxograma**:

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

A função `mtch` da Estrutura `Net` realiza operações com ponteiros com base no valor do segundo argumento `b`. Aqui está a explicação da função com base no fluxograma e pseudocódigo:

1. A função começa obtendo o primeiro argumento da ponteira `a` e armazena-o em `p1` usando a função `heap.get`.

2. Em seguida, a função obtém o segundo argumento da ponteira `a` e armazena-o em `p2` usando a função `heap.get`.

3. A função verifica se o valor do segundo argumento, acessando `b.val()`, é igual a 0.

4. Se o valor de `b.val()` for igual a 0, a função realiza as seguintes etapas:
   - Cria um novo local de memória denominado `loc` na pilha de memória.
   - Define o valor na posição `(loc+0, P2)` como `ERAS`.
   - Realiza um link entre o primeiro argumento da ponteira `p1` e o local `(loc+0)` com a tag `CT0`.
   - Realiza um link entre o segundo argumento da ponteira `p2` e o local `(loc+0)` com a tag `VR1`.
   - Libera a ponteira `a` na memória `heap`.

5. Se o valor do segundo argumento `b.val()` for diferente de 0, a função realiza as seguintes etapas:
   - Cria um novo local de memória denominado `loc` na pilha de memória.
   - Define o valor na posição `(loc+0, P1)` como `ERAS`.
   - Define o valor na posição `(loc+0, P2)` como uma nova ponteira (`PTR`) com a tag `CT0` e a posição `(loc+1)` como valor.
   - Realiza um link entre o primeiro argumento da ponteira `p1` e o local `(loc+0)` com a tag `CT0`.
   - Realiza um link entre o segundo argumento da ponteira `p2` e o local `(loc+1)` com a tag `VR2`.
   - Libera a ponteira `a` na memória `heap`.

Essa função lida com ponteiros e valores em relação ao valor do segundo argumento `b`. Dependendo do valor de `b`, diferentes operações de ligação e alocação de memória são executadas. Essa função é usada para manipular a estrutura de dados da rede e alocar memória com base nas condições definidas pelo valor de `b`.

### Função `deref` da Estrutura `Net`:

**Fluxograma**:

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
V
Fim
```

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
A função `deref` da Estrutura `Net` realiza operações de desreferência de ponteiros, expandindo-os conforme necessário. Aqui está a explicação da função com base no fluxograma e pseudocódigo:

1. A função `deref` recebe como entrada um livro (`book`), um ponteiro (`ptr`), e um ponteiro pai (`parent`).

2. A função entra em um loop enquanto `ptr` for um ponteiro do tipo REF, ou seja, um ponteiro que precisa ser desreferenciado.

3. No loop, a função verifica se o `ptr` aponta para uma rede fechada no livro `book`.

4. Se o `ptr` apontar para uma rede fechada no livro, a função realiza as seguintes etapas:
   - Carrega a rede fechada do livro.
   - Ajusta os nós da rede com um novo local (`loc`) na memória do heap.
   - Conecta os nós da rede ao local atual (`loc`) no heap.
   - Carrega os redexes da rede.
   - Ajusta os redexes com base no local (`loc`) atual.
   - Conecta os redexes ajustados ao heap.
   - Define o novo valor de `ptr` como o nó raiz da rede fechada.

5. Após todas as expansões, a função retorna o valor de `ptr`.

Essa função é usada para desreferenciar ponteiros que apontam para redes fechadas, permitindo o acesso aos nós e redexes dessas redes. É uma parte fundamental para a manipulação de estruturas de rede na estrutura `Net`.


### Função `expand` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Início
|
V
Obtém o alvo (ptr) usando a função get_target
|
V
Se o alvo (ptr) for um contador (CTR)
|
V
  Expande a cabeça do contador (CTR) usando as sub-funções expand para VR1 e VR2
|
V
Senão, se o alvo (ptr) for uma referência (REF)
|
V
  Chamada da função deref com o livro (book), o alvo (ptr) e a direção (dir)
  |
  V
  O valor retornado da função deref é definido como o novo alvo (ptr)
  |
  V
  Define o novo alvo (ptr) como o alvo da direção (dir)
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função expand(book, dir):
    Alvo (ptr) <- Obtenha o alvo (ptr) usando a função get_target

    Se o alvo (ptr) for um contador (CTR):
        Expande a cabeça do contador (CTR) chamando expand para VR1
        Expande a cabeça do contador (CTR) chamando expand para VR2
    Senão, se o alvo (ptr) for uma referência (REF):
        Novo alvo (exp) <- Chamada da função deref com o livro book, o alvo (ptr) e a direção (dir)
        Define o novo alvo (exp) como o alvo da direção (dir)
```

A função `expand` da Estrutura `Net` é responsável por expandir um ponteiro, o que envolve desreferenciar o ponteiro e realizar operações com base no tipo do ponteiro. Aqui está uma explicação da função com base no fluxograma e pseudocódigo fornecidos:

1. A função `expand` inicia obtendo o alvo (`ptr`) usando a função `get_target`.

2. Em seguida, a função verifica o tipo do alvo (`ptr`):
   - Se o alvo (`ptr`) for um contador (CTR), a função expande a cabeça do contador chamando a subfunção `expand` para `VR1` e depois para `VR2`. Isso implica expandir ambos os lados do contador.
   - Senão, se o alvo (`ptr`) for uma referência (REF):
     - A função `deref` é chamada com o livro (`book`), o alvo (`ptr`), e a direção (`dir`). A função `deref` é responsável por desreferenciar o ponteiro e expandi-lo conforme necessário.
     - O valor retornado pela função `deref` é definido como o novo alvo (`ptr`).
     - O novo alvo (`ptr`) é definido como o alvo da direção (`dir`).

3. Após as expansões, a função termina.

Essa função é fundamental para a manipulação de ponteiros e redes na estrutura `Net`, permitindo a exploração de estruturas de dados mais complexas e a realização de operações em seus elementos. Ela expande tanto contadores quanto referências, garantindo que os ponteiros sejam desreferenciados e manipulados adequadamente.

### Função `reduce` da Estrutura `Net`:

**Fluxograma**:

```plaintext
Início
|
V
Copia a lista de redexes para rdex
|
V
Enquanto rdex não estiver vazio
|
V
  Para cada redex (a, b) em rdex
  |
  V
    Chama a função interact com os redexes (a, b) e o livro (book)
  |
  V
  Limpa a lista de redexes (rdex)
  |
  V
  Copia a lista de redexes novamente
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função reduce(book):
    Copia a lista de redexes para rdex

    Enquanto rdex não estiver vazio:
        Para cada redex (a, b) em rdex:
            Chama a função interact com os redexes (a, b) e o livro book

        Limpa a lista de redexes (rdex)
        Copia a lista de redexes novamente
```

A função `reduce` da Estrutura `Net` é responsável por realizar a redução de redexes na rede. Aqui está uma explicação da função com base no fluxograma e pseudocódigo fornecidos:

1. A função `reduce` inicia copiando a lista de redexes para uma variável chamada `rdex`.

2. Em seguida, a função entra em um loop enquanto a lista `rdex` não estiver vazia.

3. Dentro do loop, a função itera sobre cada redex (par de redexes) `(a, b)` na lista `rdex`.

4. Para cada redex `(a, b)`, a função chama a subfunção `interact` com os redexes `(a, b)` e o livro (`book`). A função `interact` é responsável por processar a interação entre os redexes e aplicar as regras de redução apropriadas.

5. Após processar todos os redexes na lista `rdex`, a função limpa a lista de redexes, tornando-a vazia.

6. A função copia a lista de redexes novamente, que pode ter sido atualizada durante a interação dos redexes. Isso permite que novos redexes sejam descobertos e processados na próxima iteração.

7. O loop continua até que a lista `rdex` esteja vazia, indicando que não há mais redexes a serem reduzidos.

Essa função desempenha um papel crucial na execução de reduções na rede, permitindo que os redexes sejam identificados e manipulados de acordo com as regras específicas da estrutura `Net`. Isso é fundamental para a computação realizada pela rede.

### Função `normal` da Estrutura `Net`:

**Fluxograma**:

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
  Chama a função reduce com o livro (book)
  |
  V
  Chama a função expand com um ponteiro (ROOT) e o livro (book)
|
V
Fim
```

**Pseudocódigo**:

```plaintext
Função normal(book):
    Chama expand com o ponteiro ROOT e o livro book

    Enquanto existirem redexes:
        Chama a função reduce com o livro book
        Chama expand com o ponteiro ROOT e o livro book
```

A função `normal` da Estrutura `Net` é responsável por normalizar a rede, o que envolve a redução de redexes até que não haja mais redexes na rede. Aqui está uma explicação da função com base no fluxograma e pseudocódigo fornecidos:

1. A função `normal` começa recebendo uma referência para o `livro` como seu parâmetro. O `livro` pode ser uma parte importante da estrutura `Net` que contém informações adicionais necessárias para a normalização da rede.

2. Em seguida, a função chama a subfunção `expand` com um ponteiro `ROOT` e o `livro`. A subfunção `expand` é responsável por expandir a rede e descobrir redexes.

3. A função entra em um loop que continua enquanto houver redexes na rede. Os redexes são expressões da rede que podem ser reduzidas de acordo com as regras específicas da estrutura `Net`.

4. Dentro do loop, a função chama a subfunção `reduce` com o `livro`. A subfunção `reduce` é responsável por realizar a redução de redexes na rede.

5. Após a redução dos redexes, a função chama novamente a subfunção `expand` com o ponteiro `ROOT` e o `livro`. Isso permite que a rede seja expandida novamente, e novos redexes possam ser descobertos ou a rede possa ser reconfigurada após a redução.

6. O loop continua até que não haja mais redexes na rede. Quando todos os redexes foram reduzidos, a função conclui sua tarefa.

Essa função desempenha um papel central na normalização da rede, assegurando que todos os redexes sejam reduzidos de acordo com as regras da estrutura `Net`. A normalização é um passo importante em sistemas de redução ou computação formal, onde a expressão é simplificada até que alcance um estado irreversível.

