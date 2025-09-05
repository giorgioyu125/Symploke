### What is Symploke?

Symploke is a simple and logic-focused implementation of Lisp with a Generational GC.

## What is Lisp? And why i have chose it? 
##       (A general Overview)

Lisp is a multi-paradigm interpreted programming language focused on functional and pure 
programming,it does have a Object System, but this not mean is a Object-Oriented Programming 
language in the usual way we are used to think. Classes are more like types but they appear 
in a symbolic table and a virtual machine. The virtual machine is a necessary component of 
the Symploke Interpreter, because we are implementeing a complex memory model that involeves 
mainly four different components of the VM: Garbage Collector, Abstract Syntax Tree, Evaluator 
and Symbolic Table (The semantic core of our language).


## Phases of Symploke Interpreter Execution

### The Lexer, the first part:

The lexer is the set of procedures or functions that is made to transform the source code text into a list of ordered idioms 
defined by the language, and later this will be used by the syntax analyzer, semantic checker and the symbol table to execute,
check and collect informations.

In Symploke the types are optional because as the name suggest we dont look too much at the "Intrinsic Property" but at the 
relation or the form of this relations. But in "some" sense every atom is simply a collection of letters, this does not means 
that letters are the fundematal type, just that every atom is made of them.

The Lexer transforms every token by recognizing its lenght by reading it and matching it in its internal strucuture.
Like in this example:

( + 1 2 )

Lexer output:

LEFT_PAREN ( 1 1 1
PLUS_OP + 1 1 1
INT 1 1 1
...
(LeftParen, "(", 1, 0, 1), (PlusOp, "+", 1, 1, 1), (Int, "1", 1, 1, 1), (Int, "2", 1, 1, 1), (RightParen, ")", 1, 0, 1)
or
(kind lexeme span depth sexpr_id) 


The first 4 parameters of a token are just immediate and can be calculated without any problem, but. The 
last one needs some more attention. Because the number of sexpr_id is the number of open parenthesis and assign to each 
the correct is id is not immediate. Because the same "depth" of innesting can corrispond to different s-exp we need 
a fancier algorithm to express correctly this last paramater.

The algorithm is indeed *very simple* and it looks like this little rules:

Create a stack a empty stack. and manipulate this stack for every token seen left to right.

Push - a new id when you encounter a open praenthesis '('
Pop - when you find a closed parenthesis pop from the stack the upmost element.
Peek&Assign - when you find an atom peek the stack upmost id and assign it to that token.


### The AST constructor, the second part:

This part is concerned about creating the AST accordingly to the syntax rules of the language. Note that the AST 
construction itself is the syntax checking, so if the AST cannot be constructed properly the syntax of the program
is wrong.

After this, lets showcase the AST constructor algorithm: 

    Take the following exp as an example:

    (+ (+ 1 2) (+ 7 8))

    Steps: 

    1. Make the Exp above a linked list of tokens 
    2. We search for every '(' with a different sexpr_id and we copy only everything in that list that have that id, and
        if we encounter something that have not that id we skip over it, expect for (, until we found one only including 
        the starting ) with that specific id, the closing ) of that id.
    3. Now we have a collection of linked list with a sort of reference to the others via the id, so we push every linked list 
        in an hashtable choosing a key the id and as value the list itself.
    4. We resolve the reference by id placing the corrisponding expression where its called going by lower to higher id but copying 
        only the reference to the rest of the "new" list included in the one with the lower id, but keeping the reference to the rest 
        of the old list. 
    5. While we resolve the reference we make sure that every linked list have an ending in of the two direction. Forward or Outside the 
        list that contain that new list. Essentialy we balance the ().
    6. We keep resolving all the reference and we are done, because the resulting nest of Linked List is now a tree, a tree 
        rapresenting the nesting nature of our S-Expression: the AST.

    Expected output: 
    (+ ( ( )
       | |
       + +
       | |
       1 7
       | |
       2 8
       | |
       ) )

    Because every Linked List have its own set of () the expression is a "well-formed formula" of symploke.





// REVIEWED PART END

### Garbage Collector

The Garbage collector is intrisically generational so its made of mainly three parts and follow
this hypothesis:"Younger object dies more often". This very simple assumption of atomicity is 
especially true in operations that involves substitution, so its actually everywhere!

The parts of the Generational GB are these:
1. The Stack (Or the Objects Pointer Pile)
    Is a Memory region of the virtual machine that contains all the pointer to the object and
    should not be confused with the *program stack*, this data structure follows the same rules
    of the actual program stack but can be allocated on the program heap. This will be implemented
    as a simple Deque, Double ended queue, for reasons of semplicity and remove and search speed.
    When an object is not used anymore we will destroy his pointer on the stack, so that the
    forwarding pointer will not copy it.

2. The Heap (Or the Object Space)
    The Object Space is devided in two main parts: The old gen and the New Gen.
    After every "cycle" the old object that are no longer alive are not copied in the new gen, and
    therefore they are left in the Old gen where they can be deallocated all at once. This strategy
    of Step-Copy-Destroy is especially useful and should be well optimaized for our programming language.

3. Forwarding pointer
    The Forwarding pointer is the pointer outside the stack and the heap that copies all objects still
    present on the Stack, the alive objects. When this is done we say that "we stepped one gen forward",
    so that the old objects are destroyed and the new ones are stored correctly in the heap. This process
    end the cycle of what we talked about earlier as the Step-Copy-Destroy strategy cycle.  



So how all this parts connect to one single elegant algorithms? with these steps:
1. We evaluate the expression and so the AST evaluation modifies the object-space, allocating
   the objects necessery to the evaluator and destroying the used object's pointer in the Pile.
2. After one evaluation step, we create the forwarding at the top of the stack.
3. We traverse all the stack top to bottom copying each and every object in the new-section, the other new-section
   that is the one empty, of the program.
4. We destroy the old gen section and we restart the process from (1) with the new section as the current section

### AST and Evaluator

So the Evaluator is a fundamental part of the advancment algorithm of the GGC in the VM, and rapresents the solving
process of the Abstract Syntax Tree.
Firstly we need to create the first AST, that is made of two component:
The Operand and the operators. Or the Function and Arguments.

Take this snippet of Symploke code:

(+ 1 (+ 2 3))

In the first Evaluator part we calculated the AST as follows, considering that + is the plus operator that sums integers:

```ast
     +
1       (+ 2 3)
```

In these first iteration or recursion of the AST we see that we only perform one step assuming that the first symbol 
after the ( is a function and this takes *exactly* a number of arguments, note that this fact is retrievable by the 
Symbolic Table that contains the description of the + operator, sort of as a decoder.
After that the Evaluator is called again the old state and is job is to be sure that not atomic S-Expression are not
still present and that we have reduced everything into language defined functions and pure values, in the expected order.

So the evaluator outputs this other AST

```ast
    +
1       + 
    2       3
```

So the evaluator is certain that no more non-atomic expression are present because he have only created in this last step, atomic
values and functions. Why we says there are not-atomic functions? because some functions can be defined as a collection of 
other more primitive functions, this are typically the user-defined functions. After that the evaluator is called to actually solve
the AST, because as a flag there are no more non-atomic expression as node in the AST.

First Evaluation Step:
```ast
    +
1       5 
```
It replace the + in the second row and destroys the two numbers used to perform the calculation, so it nullify the pointer on the stack
of the 2 and 3 objects. After this the evaluator is called again and finally resolve the program by reducing the AST to a single node.

```ast
    6 => RESULT!
```

This result is the value of the original S-exp (+ 1 (+ 2 3)). So... we have completed the algorithm successfully!
But still we need a map to guide us to how to use correctly the objects: The Symbolic Table.

### Symbolic Table

We are not yet discussing what is a *Symbol* in Lisp and Symploke formally but we can says that a symbol is:
"The reference to an object", thats right, very simple and straightforawrd. So the symbol have a pair in memory that is the 
pointer of that object in the Pile. This pointer description is present in an actual database that we will implement as HashTable,
and the Key being the symbol and the Value the description of that object connected to the symbol and the location in the stack of 
the symbol. So we give access to creating a new object using the quote notation used down below:

```symploke
(quote (1 (+ 2 3)))
```

Other than this we can say that the value of a symbol is an object and is essentialy an array 
that lists all its property in order. Because every object needs to fill the same fields we use an array
with size known at compile time but a number of them (number of symbol)only know at runtime or when 
evaluating the programs expressions. This is a example table for our program:

| Name | TypeTag | Flags (bitset) | Arity (packed) | ValuePtr | EntryPtr | EnvID  | StackSlot |      GCInfo (packed)     |      SizeBytes       |
|:----:|:-------:|:--------------:|:--------------:|:--------:|:--------:|:------:|:---------:|:------------------------:|:--------------------:|
|  x   |   var   | (1 0 ...... 0) |      NIL       |   &heap  |   NIL    |   1    |     0     | (gen,&age-brigade,color) |    ???               |
|  5   |   int   | (1 0 ...... 0) |      NIL       |   &heap  |   NIL    |   1    |     1     | (gen,&age-brigade,color) |     4                |
|"Save"|   str   | (1 0 ...... 0) |      NIL       |   &heap  |   NIL    |   1    |     2     | (gen,&age-brigade,color) |     4                |
|(! 3) |  s-exp  | (1 0 ...... 0) |      NIL       |   &heap  |   NIL    |   2    |     3     | (gen,&age-brigade,color) |    ???               |
| car  |  prim-f | (1 0 ...... 0) |       1        |   &car   | &native  |   2    |     4     | (gen,&age-brigade,color) | (car functions size) |


TypeTag: what this is (int, float, cons, string, symbol, function, macro, special-form, vector, nil, bool…).

EvalKind: how it participates in evaluation (data | function | macro | special-form). Lets the evaluator know when 
to evaluate arguments and how.

Flags (bitset): common traits. Suggested bits: immutable, pure, primitive, variadic, pinned, external, const-binding.

Arity (packed): for callables. Pack min/max arity into one 32-bit value (low16=min, high16=max; 0xFFFF means “unbounded”). 
For data, set to 0.

ValuePtr: pointer to the payload (heap object) or an immediate/NaN-boxed value if you choose that representation.
For cons/string/vector this points to the heap block; for ints it can be immediate.

EntryPtr: code/definition pointer. For primitives: native entrypoint; for user functions: pointer to AST/bytecode
or compiled closure; otherwise null.

EnvPtr: environment/closure pointer for captured variables; null for data and primitives.

StackSlot: index/ID of the root in your pointer pile (Deque). -1 if not 
rooted here (e.g., temporary or global managed elsewhere).

GCInfo (packed): (gen,&age-brigade,color), each one of the is rapresent the generation and which "age range" each
                 object is part of. The color is only put to open up the table to more expressivity and optimization.
                 Probabily will be another table with only three colum that is devided not in symbols but in age-brigade 
                 buckets so that the GC can easily locate younger and probabily less used object.
                 Or maybe the Symbol table address can be devided in this sort of "Age Range Buckets".


SizeBytes: size of the heap payload for copy GC. 0 for immediates.

Read all the tokens separeted left to right and place on the stack the id of the first when you see the first parenthesis,
after that keep going forward until you encounter another parenthesis, from now and then if it is closed pop an element from the stack 
if its open push it on the stack with a new fresh id. Meanwhile every atom you encounter peek from the stack and give to that 
atom that value.
