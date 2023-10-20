## RUN

### `alloc` Function of the `Heap` Structure

The `alloc` function in the `Heap` structure is responsible for allocating a position in the data array and returning the index of that position. Here is a simplified flowchart and pseudocode for the `alloc` function:

**Pseudocode**:

```plaintext
Function alloc(size):
    If size equals zero
        Return 0
    Else, if space is not full and there is enough space after the next
        Increase the space used counter by the size
        Update the next available position
        Return the previous position of the next available as a value
    Else
        Set space as full
        While there is contiguous space of the desired size available
            If the next is beyond the limit, restart from the beginning
            If the P1 door of the next position is null, increment the space counter by one
            Else, reset the space counter to zero
            Update the next position
            If the space counter reaches the desired size
                Increase the space used counter by the size
                Return the previous position of the next available as a value
```

This function is used to allocate space in the data array within the `Heap` structure. It checks whether the heap is not full and if there is contiguous space available to allocate the specified amount of data. If the heap is full or there is no contiguous space available, it performs a search to find free space within the heap and then allocates and returns the appropriate index. The "used" counter is increased to track the allocated positions.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Receive as input: "size" (allocation size)
|
V
If "size" equals 0, return 0
|
V
If the heap is not full and "next + size" is less than or equal to the array size:
|
|--> Yes
|     |
|     V
|     Allocate space in the heap for "size" units of data starting from "next"
|     |
|     V
|     Increase the "used" counter by "size"
|     |
|     V
|     Update "next" to "next + size"
|     |
|     V
|     Return "next - size" as the allocated index
|
|--> No
|     |
|     V
|     The heap is full
|     |
|     V
|     Initialize a variable "space" as 0
|     |
|     |--> Loop
|          |
|          V
|          If "next" is greater than or equal to the array size:
|          |
|          |--> Yes
|          |     |
|          |     V
|          |     Set "space" to 0 and "next" to 1
|          |     |
|          |     V
|          |     Continue the loop
|          |
|          |--> No
|          |     |
|          |     V
|          |     If the P1 door of the element at position "next" is NIL:
|          |     |
|          |     |--> Yes
|          |     |     |
|          |     |     V
|          |     |     Increment "space" by 1
|          |     |     |
|          |     |     V
|          |     |     If "space" equals "size":
|          |     |     |
|          |     |     |--> Yes
|          |     |     |     |
|          |     |     |     V
|          |     |     |     Increase "used" by "size"
|          |     |     |     |
|          |     |     |     V
|          |     |     |     Return "next - space" as the allocated index
|          |     |     |
|          |     |     |--> No
|          |     |     |     |
|          |     |     |     V
|          |     |     |     Continue the loop
End
```

</details>

### `compact` Function of the `Heap` Structure

Here is a simplified flowchart and pseudocode for the `compact` function in the `Heap` structure:

**Pseudocode**:

```plaintext
Function compact():
    Create an empty list called "node."
    Repeat while the length of "node" is less than the length of the heap's data:
        If the first component of the current node is not NULL or the second component is not NULL, add it to the "node" list.
        Otherwise, exit the loop.
    Return "node."
End of Function
```

This function creates a list called "node" and populates it with values from the "data" until it encounters a pair of values (NULL, NULL). It then returns the "node" list as the result.

<details>
  <summary>Flowchart</summary>
  
```plaintext
Start
|
V
Create an empty list called "node"
Initialize a variable "index" to 0
|
V
While the value at the index position in "data" is not (NULL, NULL):
  |
  |-> Add the value at the index position in "data" to the "node" list
  |-> Increment "index" by 1
|
V
Return the "node" list as the result of the function
End
```

</details>

### `link` Function of the `Net` Structure

The `link` function of the `Net` structure is intended to establish connections between elements based on their types.

**Pseudocode**:

```plaintext
Function link(a, b):
    If a is pri and b is pri:
        If a and b can be skipped:
            Increment eras by 1
        Else:
            Add (a, b) to the redex list `rdex`
    Else, if a is var:
        Replace the target of a with the value of b
    Else, if b is var:
        Replace the target of b with the value of a
    End
```

In this way, the `link` function establishes connections or links between elements of the `Net` structure according to the specified rules for each type of element, whether it's "pri" (priority) or "var" (variable). This enables the creation and manipulation of connections between network elements, which is useful in various applications such as inference systems and information processing.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Check the types of `a` and `b`
|
V
If both are pri:
|   Yes
|   Check if `a` and `b` can be skipped
|   |
|   V
|   If they can:
|   |   Yes
|   |   Increment eras by 1
|   |   |
|   |   End
|   |
|   V
|   They cannot be skipped
|   |
|   V
|   Add the tuple (a, b) to rdex
|   |
|   End
|
V
If a is var:
|   Yes
|   Replace the target of a with the value of b
|   |
|   End
|
V
If b is var:
|   Yes
|   Replace the target of b with the value of a
|   |
|   End
End
```

</details>

### `interact` Function of the `Net` Structure

The `interact` function of the `Net` structure is a complex function that defines interactions between different types of elements in the structure. It is used to perform specific operations based on the types of elements `a` and `b.

**Pseudocode**:

```plaintext
Function interact(a, b)
    If a and b are of the same node type (e.g., both are of type CTR)
        If a equals b (based on some specific criteria)
            Call the anni(a, b) function
        Else
            Call the comm(a, b) function
    Else if a is a specific node type (e.g., CTR)
        If a and b have the same tag
            Call the anni(a, b) function
        Else
            Call the comm(a, b) function
    Else if b is a specific node type (e.g., CTR)
        Call the comm(b, a) function
    Else
        Call the era2(a) function
    End of Function
```

The `interact` function is essential for interaction operations between different types of elements in the `Net` structure, allowing for various information processing and logic operations within the network.

<details>
  <summary>Flowchart</summary>
    
```plaintext
Start
 |
 V
Are A and B of the same node type?
 |
 V
Yes
 |
 |---[Is A equal to B?]---> No
 |      |
 |      |---[Call anni(A, B) function]---> End
 V
No
 |
 |---[Is A a specific node type (e.g., CTR)?]---> No
 |      |
 |      |---[Is B a specific node type (e.g., CTR)?]---> No
 |      |      |
 |      |      |---[Call era2(A) function]---> End
 |      |
 |      |---[Call comm(B, A) function]---> End
 |
 |---[Do A and B have the same tag?]---> No
 |      |
 |      |---[Call comm(A, B) function]---> End
 V
Yes
 |
 |---[Call anni(A, B) function]---> End
End
```
</details>

### `conn` Function of the `Net` Structure

The `conn` function of the `Net` structure serves the purpose of establishing a connection between two elements `a` and `b` in the network.

**Pseudocode**:

```plaintext
Function conn(a, b):
    Increment the value of `anni` by 1
    Get the value of P2 from `a` and P2 from `b`
    Link the value of P2 from `a` to the value of P2 from `b`
    Free the memory associated with `a`
    Free the memory associated with `b`
```

**Diagram**:

```
A2 --[#X}---[#Z}-- B2
~~~~~~~~~~~~~~~~~~~ OP1-OP1 
          ,----- B2
         X
A2 -----' 
```

This function is used to establish specific connections between elements in the `Net` structure, which can be useful in various applications, such as inference systems, where connections represent logical relationships between concepts or entities. The increment of `anni` is essential for tracking the network's evolution and connections over time.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Increment the value of `anni` by 1
Get the value of P2 from `a` and P2 from `b`
|
V
Link the value of P2 from `a` to the value of P2 from `b`
|
V
Free the memory associated with `a`
Free the memory associated with `b`
|
End
```

</details>

### `anni` Function of the `Net` Structure

The `anni` function of the `Net` structure is designed to perform a nesting action, which involves creating connections between elements and incrementing the value of the variable `anni`.

**Pseudocode**:

```plaintext
Function anni(a, b):
    Increment the value of `anni` by 1
    Link the value of P1 with a value derived from `a`
    Link the value of P1 with a value derived from `b`
    Link the value of P2 with a value derived from `a`
    Link the value of P2 with a value derived from `b`
    Free the memory associated with `a`
    Free the memory associated with `b`
```

**Diagram**:

```
A1 --|\     /|-- B2
     |a|---|b|   
A2 --|/     \|-- B1
~~~~~~~~~~~~~~~~~~~ CTR-CTR (A == B)
A1 -----, ,----- B2
         X
A2 -----' '----- B1
```

This function is used to perform nesting operations and create connections in a network structure commonly found in information processing and logic systems. The increment of `anni` is crucial for tracking and controlling nesting operations over time.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Increment the value of `comm` by 1
Allocate 4 memory slots in `loc`
Link the value of P1 with a value derived from `a`
Link the value of P1 with a value derived from `b`
Link the value of P2 with a value derived from `a`
Link the value of P2 with a value derived from `b`
Link the value of P1 with a value derived from `b`
Link the value of P2 with a value derived from `a`
Link the value of P1 with a value derived from `a`
Link the value of P2 with a value derived from `b`
Allocate 2 memory slots in `space` with a value of zero
While the value of `space` is less than 4:
  |
  V
  If the value at index `next` in the `data` array is greater than or equal to the array's length:
  |
  V
  Set the value of `space` to 0
  Set the value of `next` to 1
  |
  |
  V
  If the value at index `next` in the `data` array for port `P1` is equal to NULL:
  |
  V
  Increment the value of `space` by 1
  |
  V
  Else, set the value of `space` to 0
  Increment the value of `next` by 1
  |
  |
  V
  End of Loop
|
V
Increment the value of `used` by 4
Return
```

</details>

### `comm` Function of the `Net` Structure

The `comm` function of the `Net` structure is intended to facilitate communication between two elements, `a` and `b`, establishing various specific connections between them, as well as managing memory allocations related to this communication.

**Pseudocode**:

```plaintext
Function comm(a, b):
    Increment the value of `comm` by 1
    Allocate 4 memory slots in `loc`
    Link the value of P1 with a value derived from `a`
    Link the value of P1 with a value derived from `b`
    Link the value of P2 with a value derived from `a`
    Link the value of P2 with a value derived from `b`
    Link the value of P1 with a value derived from `b`
    Link the value of P2 with a value derived from `a`
    Link the value of P1 with a value derived from `a`
    Link the value of P2 with a value derived from `b`
    Allocate 2 memory slots in `space` with a value of zero
    While the value of `space` is less than 4:
        If the value at the `next` index in the `data` array is greater than or equal to the array's length:
            Set the value of `space` to 0
            Set the value of `next` to 1
        If the value at the `next` index in the `data` array for port `P1` is equal to NULL:
            Increment the value of `space` by 1
        Else, set the value of `space` to 0
            Increment the value of `next` by 1
    Increment the value of `used` by 4
```
</details>
### `pass` Function of the `Net` Structure

The `pass` function of the `Net` structure is intended to perform an information passing action between two elements, `a` and `b`.

**Pseudocode**:

```plaintext
Function pass(a, b):
    Increment the value of `comm` by 1
    Allocate 3 memory slots in `loc`
    Link the value of P2 with a value derived from `b`
    Link the value of P1 with a value derived from `a`
    Link the value of P2 with a value derived from `a`
    Return
```

**Diagram**:

```
WIP
A1 --|\         
     |a|-------[#Z}-- B2   
A2 --|/         
~~~~~~~~~~~~~~~~~~~~~~~ CTR-OP1 
TODO
```

This function is used to establish specific connections between elements in the `Net` structure during an information passing operation, which can be useful in various applications, such as communication systems and data processing. The increment of `comm` is important for tracking and controlling communication operations over time.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Increment the value of `comm` by 1
Allocate 3 memory slots in `loc`
Link the value of P2 with a value derived from `b`
Link the value of P1 with a value derived from `a`
Link the value of P2 with a value derived from `a`
|
V
End
```

</details>

### `copy` Function of the `Net` Structure

The `copy` function of the `Net` structure is intended to perform a copy operation of information from one element `a` to another element `b`.

**Pseudocode**:

```plaintext
Function copy(a, b)
    Get the value of node A.
    Create a copy of node B.
    Set the targets of the main ports of A to point to the copy of B.
    Free node A.
End of Function
```

This function is used to copy specific information from one element to another in the `Net` structure, which can be useful in various applications, such as data processing and logic systems. The increment of `comm` is important for tracking and controlling copy operations over time.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
 |
 V
Get the value of A
 |
 V
Create a copy of node B
 |
 |
 V
Set the targets of the main ports of A to point to the copy of B
 |
 |
 V
Free node A
End
```

</details>


### `era2` Function of the `Net` Structure

The `era2` function of the `Net` structure serves the purpose of performing an "eraser" operation, which involves removing information from an element `a` and creating connections with the value "ERAS."

**Pseudocode**:

```plaintext
Function era2(a):
    Increment the value of `eras` by 1
    Get the value of P1 from a.val()
    Get the value of P2 from a.val()
    Link the value of P1 with the value ERAS
    Link the value of P2 with the value ERAS
    Free the value of a.val()
    Return
```

**Diagram**:

```
A1 --|\
     |a|-- ()
A2 --|/
~~~~~~~~~~~~~ {CTR/OP2/MAT}-ERA
A1 ------- ()
A2 ------- ()
```

This function is used to perform erasure operations in a network structure, which can be useful in data processing systems where information removal is necessary. The increment of `eras` is important to track and control erasure operations over time.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Increment the value of `eras` by 1
Get the value of P1 from a.val()
Get the value of P2 from a.val()
Link the value of P1 with the value ERAS
Link the value of P2 with the value ERAS
Free the value of a.val()
|
V
End
```

</details>

### `era1` Function of the `Net` Structure

The `era1` function of the `Net` structure serves the purpose of performing a more specific "eraser" operation, involving the removal of information from a single port `P2` of element `a` and creating a connection with the value "ERAS" in that port.

**Pseudocode**:

```plaintext
Function era1(a):
    Increment the value of `eras` by 1
    Get the value of P2 from a.val()
    Link the value of P2 with the value ERAS
    Free the value of a.val()
    Return
```

**Diagram**:

```
A2 --[#X}-- ()
~~~~~~~~~~~~~ OP1-ERA
A2 ------- ()
```

This function is used to perform specific erasure operations of information in a network structure, focusing on a single port. The increment of `eras` is important to track and control erasure operations over time.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Increment the value of `eras` by 1
Get the value of P2 from a.val()
Link the value of P2 with the value ERAS
Free the value of a.val()
|
V
End
```

</details>

### `op2n` Function of the `Net` Structure

The `op2n` function of the `Net` structure serves the purpose of performing a specific operation involving the manipulation of numbers.

**Pseudocode**:

```plaintext
Function op2n(a, b):
    p1 <- Get the value of p1 from a.val()
    If p1 is a number:
        rt <- Calculate `rt` as the result of the `prim` function with parameters (value of p1, value of b)
    Else:
        Set the P1 value of a.val() as b
    End If
    Get the value of P2 from a.val()
    Link the new NUM value `rt` with the value of P2
    Free the value of a.val()
    Return
```

**Diagram**:

```
A1 --,
     [}-- #X
A2 --' 
~~~~~~~~~~~~~~ OP2-NUM
A2 --[#X}-- A1
```

This function is used to perform specific operations involving numbers in the `Net` structure. Depending on the type of operand `p1`, the function can execute different actions, such as calculating the operation's result or assigning a new value to `a`. The `prim` operation is used to perform the necessary calculation, and the result is stored in `rt`.

### `op1n` Function of the `Net` Structure

The `op1n` function of the `Net` structure serves the purpose of performing a specific operation involving the manipulation of numbers.

**Pseudocode**:

```plaintext
Function op1n(a, b):
    p1 <- Get the value of p1 from a.val()
    p2 <- Get the value of p2 from b.val()
    v0 <- Get v0 from bits 0-23 of p1
    v1 <- Get v1 from bits 0-23 of p2
    v2 <- Calculate v2 as the result of the `prim` function with parameters v0 and v1
    result <- Create a new Ptr instance with the NUM operator and value v2
    Set the P2 value of the new Ptr as the instance p2
    Free the value of a.val()
    Return result
```

**Diagram**:

```
A2 --[#X}-- #Y
~~~~~~~~~~~~~~ OP1-NUM
A2 -- #Z
```

This function is used to perform specific operations involving numbers in the `Net` structure. It extracts parts of the values `p1` and `p2`, performs a specific operation (`prim`), creates a new `Ptr` instance with the result, and establishes necessary connections. The result of the operation is returned as a new `Ptr` instance called `result`.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Get the value of p1 from a.val()
Get the value of p2 from b.val()
Get v0 from bits 0-23 of p1
Get v1 from bits 0-23 of p2
Calculate v2 as the result of the `prim` function with parameters v0 and v1
Create a new instance of Ptr with the NUM operator and value v2
Set the P2 value of the new Ptr as the instance p2
Free the value of a.val()
|
V
End
```

</details>

### `prim` Function of the `Net` Structure

The `prim` function of the `Net` structure plays the role of performing binary and logical operations on numeric values.

**Pseudocode**:

```plaintext
Function prim(a, b)
    Get the operator of node A (the upper bits).
    Get the value of node A (the lower bits).
    Get the operator of node B (the upper bits) [not used in this example].
    Get the value of node B (the lower bits).
    
    If the operator of node A is USE
        Set the operator of the result as the operator of node B.
    If not
        Perform the corresponding operation based on the operator of node A.
    
    Return the result as a new node.
End of Function
```

The function returns the `result` value, which is the result of the operation determined by the `a_opr` operator. This function allows performing various mathematical and logical operations with the values contained in the `a` and `b` structures.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
 |
 V
Get operator of A
 |
 V
Get value of A
 |
 V
Get operator of B [not used in this example]
 |
 V
Get value of B
 |
 |
 V
Operator of A is USE?
 |
 |---[Yes]---> Set operator of result as operator of B
 |
 |---[No]---> Perform the corresponding operation based on the operator of A
             |
             |
             V
             Return the result as a new node
End
```

</details>

### `mtch` Function of the `Net` Structure

The `mtch` function of the `Net` structure performs operations with pointers based on the value of the second argument `b`.

**Pseudocode**:

```plaintext
Function mtch(a, b):
    P1 (p1) <- Get the first argument of the pointer (a) using the heap.get function
    P2 (p2) <- Get the second argument of the pointer (a) using the heap.get function

    If the value of the second argument (b.val()) is equal to 0:
        Create a new location (loc) on the memory stack
        Set the value at position (loc+0, P2) to ERAS
        Link between the first argument of the pointer (p1) and the location (loc+0) with the CT0 tag
        Link between the second argument of the pointer (p2) and the location (loc+0) with the VR1 tag
        Free the pointer (a) in the heap memory
    Else, if the value of the second argument (b.val()) is not equal to 0:
        Create a new location (loc) on the memory stack
        Set the value at position (loc+0, P1) to ERAS
        Set the value at position (loc+0, P2) to a new pointer (PTR) with the CT0 tag and position (loc+1) as value
        Link between the first argument of the pointer (p1) and the location (loc+0) with the CT0 tag
        Link between the second argument of the pointer (p2) and the location (loc+1) with the VR2 tag
        Free the pointer (a) in the heap memory
```

**Diagram**:

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

This function deals with pointers and values in relation to the value of the second argument `b`. Depending on the value of `b`, different linking and memory allocation operations are executed. This function is used to manipulate the network data structure and allocate memory based on conditions defined by the value of `b`.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
P1 (p1) <- Get the first argument of the pointer (a) using the heap.get function
P2 (p2) <- Get the second argument of the pointer (a) using the heap.get function
|
V
If the value of the second argument (b.val()) is equal to 0:
|
|----> Create a new location (loc) on the memory stack
|----> Set the value at position (loc+0, P2) to ERAS
|----> Link between the first argument of the pointer (p1) and the location (loc+0) with the CT0 tag
|----> Link between the second argument of the pointer (p2) and the location (loc+0) with the VR1 tag
|----> Free the pointer (a) in the heap memory
|
V
Else, if the value of the second argument (b.val()) is not equal to 0:
|
|----> Create a new location (loc) on the memory stack
|----> Set the value at position (loc+0, P1) to ERAS
|----> Set the value at position (loc+0, P2) to a new pointer (PTR) with the CT0 tag and position (loc+1) as value
|----> Link between the first argument of the pointer (p1) and the location (loc+0) with the CT0 tag
|----> Link between the second argument of the pointer (p2) and the location (loc+1) with the VR2 tag
|----> Free the pointer (a) in the heap memory
|
V
End
```

</details>

### `deref` Function of the `Net` Structure

The `deref` function of the `Net` structure performs pointer dereference operations, expanding them as needed.

**Pseudocode**:

```plaintext
Function deref(book, ptr, parent):
    While ptr is a pointer of the REF type:
        If ptr points to a closed network in the book:
            Load the closed network from the book
            Adjust the nodes of the network with a new location (loc)
            Connect the nodes of the network to the current location in the heap
            Load the redexes of the network
            Adjust the redexes based on the current location (loc)
            Connect the adjusted redexes to the heap
            Set the new value of ptr as the root node of the network
    Return ptr after all expansions
```

**Diagram**:

```
A1 --|\
     | |-- @REF
A2 --|/
~~~~~~~~~~~~~~~~ CTR-REF
A1 --|\
     | |-- {val}
A2 --|/
```

This function is used to dereference pointers that point to closed networks, allowing access to the nodes and redexes of these networks. It is a fundamental part of network structure manipulation in the `Net` structure.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Receives a reference to a book (book), a pointer (ptr), and a parent pointer (parent)
|
V
While ptr is a pointer of the REF type:
|
V
  Find the book (book) of the current ptr
  |
  V
  If the current ptr points to a closed network
  |
  V
    Load the closed network from the book (book)
    |
    V
    Adjust the nodes of the closed network with a new location (loc)
    |
    V
    Connect the nodes of the closed network to the current location (loc) in the heap
    |
    V
    Load the redexes of the closed network
    |
    V
    Adjust the redexes based on the current location (loc)
    |
    V
    Connect the adjusted redexes to the heap
    |
    V
    Set the new value of ptr to the root node of the closed network
  |
  V
Return ptr after all expansions
|
End
```

</details>

### `expand` Function of the `Net` Structure

The `expand` function of the `Net` structure is responsible for expanding a pointer, which involves dereferencing the pointer and performing operations based on the type of the pointer.

**Pseudocode**:

```plaintext
Function expand(net, book, dir):
    Get the target (ptr) using the get_target function
    If ptr is a CTR:
        Expand the counter to auxiliary ports (VR1 and VR2)
    Else, if ptr is a REF:
        Expand the reference and update the target pointer with the expansion
End of Function
```

This function is essential for handling pointers and networks in the `Net` structure, allowing for the exploration of more complex data structures and performing operations on their elements. It expands both counters and references, ensuring that pointers are dereferenced and manipulated correctly.

This function is essential for handling pointers and networks in the `Net` structure, enabling the exploration of more complex data structures and performing operations on their elements. It expands both counters and references, ensuring that pointers are dereferenced and manipulated properly.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
Get ptr using the get_target function
|
V
If ptr is a CTR:
|
|--> Expand the counter to auxiliary ports (VR1 and VR2)
|
V
Else, if ptr is a REF:
|
|--> Expand the reference and update the target pointer with the expansion
End
```

</details>

### `reduce` Function of the `Net` Structure

The `reduce` function of the `Net` structure is responsible for performing reductions of redexes in the network.

**Pseudocode**:

```plaintext
Function reduce(book):
    While there are redexes in the network:
        For each redex (a, b) in the network:
            Call the "interact" function with arguments (net, book, a, b)
        End of loop
    End of loop
End
```

This function plays a crucial role in carrying out reductions in the network, allowing redexes to be identified and manipulated according to the specific rules of the `Net` structure. This is essential for the computation performed by the network.

<details>
  <summary>Flowchart</summary>

```plaintext
Start
|
V
While there are redexes in the network
|
V
For each redex (a, b) in the network
|
V
Call the "interact" function with arguments (net, book, a, b)
End
```

</details>

### `normal` Function of the `Net` Structure

The `normal` function of the `Net` structure is responsible for normalizing the network, which involves reducing redexes until there are no more redexes in the network.

**Pseudocode**:

```plaintext
Function normal(book):
    Call expand with the ROOT pointer and the book book

    While there are redexes:
        Call the reduce function with the book book
        Call expand with the ROOT pointer and the book book
```

This function plays a central role in normalizing the network, ensuring that all redexes are reduced according to the rules of the `Net` structure. Normalization is an important step in reduction systems or formal computation, where the expression is simplified until it reaches an irreducible state.
### Function `normal` in the `Net` Structure

The `normal` function in the `Net` Structure is responsible for normalizing the network, which involves reducing redexes until there are no more redexes left.

**Pseudocode**:

```plaintext
Function normal(book):
    Call expand with the ROOT pointer and the book
    While there are still redexes:
        Call the reduce function with the book
        Call expand with the ROOT pointer and the book
```

This function plays a central role in normalizing the network, ensuring that all redexes are reduced according to the rules of the `Net` structure. Normalization is an important step in reduction systems or formal computation, where the expression is simplified until it reaches an irreducible state.
