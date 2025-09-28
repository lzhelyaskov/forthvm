not all words are implemented yet!

### built-in variables
* **state** *( -- 0 | 1)* Is the interpreter executing code (0) or compiling a word (1)
* **latest** *( -- addr )* latest word in dictionary
* **here** *( -- addr )* next free byte of memory in dictionary
* **dsp** *( -- addr )* top of param stack
* **rsp** *( -- addr )* top of return stack
* **base** *( -- a )* current base for printing and parsing numbers

### built-in constants
* **s0** *( -- a )* base of param stack
* **r0** *( -- a )* base of return stack
* **VERSION** *( -- a )* forth version
* **DOCOL** *( -- addr )* pointer to docol
* **F_IMMED** *( -- a )* immediate flag value
* **F_HIDDEN** *( -- a )* hidden flag value
* **F_LENMASK** *( -- a )* length mask in len byte

### arithmetic
* **\+** *( a b -- sum )* 
* **\-** *( a b -- difference )*
* **\*** *( a b -- product )*
* **\/** *( a b -- quotient )*
* **mod** *( a b -- remainder )*
* **abs** *( a -- |a| )*
* **neg** *( a -- -a )*

### boolean logic
* **true** *( -- 1 )*
* **false** *( -- 0 )*
* ...

### bit logic
* **and**
* **or**
* **not**
* **xor**
* **rshift**
* **lshift**

### param stack
* **dup** *( a -- a a )*
* **2dup** *( a b -- a b a b )*
* **?dup** *( a -- 0 | a a )* 
* **swap** *( a b -- b a )*
* **over** *( a b -- a b a )*
* **rot** *( a b c -- b c a )*
* **drop** *( a -- )*
* **nip** *( a b -- b )*
* **tuck** *( a b -- b a b )*
* **pick** *( xn..x0 n -- xn..x0 xn )* copies nth stack item on top of the stack
* **roll** *()*
* **stack?** *( -- 1 | 0 )* returns true if stack underflow has occured
* **depth** *( -- n )* 

### return stack
* **\>r** *( a -- r: a )* push value from param stack to return stack
* **r\>** *( r:a -- a )* push value from return stack to param stack
* **rdrop** *( r:a -- r: )* drop top return stack value

### comparison
* **=** *( a b -- 1 | 0 )* equal
* **0=** *( a -- 1 | 0 )* equal (is) zero
* **<\>** *( a b -- 1 | 0 )* not equal
* **\>** *( a b -- 1 | 0 )* greater then
* **<** *( a b -- 1 | 0 )* less then
* **\>=** *( a b -- 1 | 0 )* greater equal
* **<=** *( a b -- 1 | 0 )* less equal
* **min** *( a b -- a | b )* min value
* **max** *( a b -- a | b )* max value

### output
* **.** *( a -- )* print top value on stack followed by space and drop it
* **?** *( addr -- )* print value addr is pointing to ( same as : @ . )
* **.s** *( -- )* print param stack without destroying it
* **.r** *( -- )* print return stack without destroying it
* **emit** *( c -- )* print stack top as char and drop it
* **tell** *( c-addr n )* prints n chars form c-addr
* **."** xxx" *( -- )* prints xxx until " 

### input
* **key** *( -- c )* read single char from input stream and push it to stack
* **word** *( -- c-addr n )* reads word from input stream 

### memory
* **\!** *( a addr -- )* store a at addr
* **+\!** *( a addr -- )* "add store" add a to the value @ addr
* **-\!** *( a addr -- )* "sub store" sub a from the value @ addr
* **@** *( addr -- a )* load value from addr
* **c!** *( i8 addr -- )* store byte value to addr
* **c@** *( addr -- i8 )* load byte from addr
* **move** *( addr1 addr2 n -- )* copy n values from addr1 to addr2
* **cmove** *( addr1 addr2 n -- )* copy n bytes from addr1 to addr2
* **rsp@**
* **rsp!** *( a -- )* set return stack pointer to 'a'

### flow control
* **branch** *( -- )* unconditional branch 
* **0branch** *( condition -- )* branch if condition is zero
* if
* else
* then
* do
* loop
* begin
* until
* while
* repeat

### runtime
* **abort**
* **exit** ( -- )
* **quit**
* **interpret**
* **'** xxx *( -- xt )* 
* **execute** *( xt -- )*

### dictionary
* **latest** *( -- addr )* address of last added word
* **here** *( -- addr )* address of next free cell in dictionary
* **>cfa** *( addr -- addr )* takes address of a word, returns code field address
* **>dfa** *( addr -- addr )* returns data field address of a word
* **forget** xxx *( -- )* forget all words before and including xxx
* **find** *( c-addr n -- addr | 0 )* find word at addr in dictionary and return its addr. 0 if not found

### defining words
* **:** xxx
* **;**
* **const** xxx *( a -- )* creates constant named xxx with the value a
* **var** xxx *( -- )* creates variable named xxx
* **create** *( addr n -- )* creates dictionary entry named by string at addr and length n
* **lit** *( -- a )* pushes next entry in word data as a literal on stack

* **immediate** *( -- )* toggles F_IMMED flag for the last added word
* **hidden** *( addr -- )* takes word address 'a' and toggles F_HIDDEN flag of this word
* **hide** xxx *( -- )* toggles F_HIDDEN flag of xxx word


### file io
* **r/o** *( -- a )*
* **w/o** *( -- a )*
* **r/w** *( -- a )*

* **include** xxx *( -- )*
* **included** *( addr len -- )*

* **file-open** *( addr len opt -- fd f )*
* **file-create** *( addr len opt -- fd f )*
* **file-close** *( fd -- )*
* **file-read** *( addr len fd -- n f )*
* **file-write** *( addr len fd -- n f )*