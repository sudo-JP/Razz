# Razz Grammar Rules

Parser rule for Razz, grammar notation in code representation 
- `Terminal`: code to match and consume a token
- `Nonterminal`: call to that rule function
- `|`: a `match` statement
- `{}` or `[]`: `while` or `for` loop 

```
Program ::= { FuncDecl } EOF ;

Param ::= IDENT ":" Type ;

FuncDecl ::= "fn" IDENT "(" [ Param { "," Param } ] ")" Type Block ;

Type ::= "int" 
    | "float" 
    | "bool" 
    | "string" 
    | "null" 
    | SpecificType ;

SpecificType ::= "Vec3" 
    | "Point3" 
    | "Color" 
    | "Background" 
    | "Camera" 
    | "Output" 
    | "Sphere" 
    | "Image" ;

Block ::= "{" { Stmt } "}" ;

Stmt ::= Assign 
    | While 
    | If 
    | For 
    | Return 
    | CompoundAssign 
    | HTTPRequest 
    | ExprStmt ;

ExprStmt ::= Expr ";" ;

Assign ::= IDENT [ ":" Type ] "=" Expr ";" ;

While ::= "while" Expr Block  ;

If ::= "if" Expr Block 
    { "else" "if" Expr Block }
    [ "else" Block ] ;

ForSet ::= Assign | CompoundAssign ;

For ::= "for" [ ForSet ] ";" [ Expr ] ";" [ ForSet { ","  ForSet } ] Block ;

Return ::= "return" Expr ";" ;

CompoundOp ::= "+=" 
    | "-="
    | "*="
    | "/=" ;

CompoundAssign ::= IDENT CompoundOp Expr ";" ;

HTTPMethod ::= "POST"
    | "PUT"
    | "PATCH" ;

Endpoint ::= "/sphere"
    | "/camera"
    | "/background"
    | "/image"
    | "/output" ;

HTTPRequest ::= HTTPMethod Endpoint Expr ";" ;

Expr ::= logic_or ;

logic_or ::= logic_and { "||" logic_and } ; 

logic_and ::= equality { "&&" equality } ; 

equality ::= comparison { ("==" | "!=") comparison } ;

comparison ::= term { ("<" | "<=" | ">" | ">=") term } ; 

term ::= factor { ("+" | "-") factor } ;

factor ::= unary { ("*" | "/") unary } ; 

unary ::= ("!" | "-") unary
    | field_access ;

field_access ::= function_call { "->" IDENT } ; 
    
Arg ::= IDENT ":" Expr ; 

Args ::= [ Arg { "," Arg } ] ;

function_call ::= IDENT "(" Args ")" 
    | primary ;

StructField ::= IDENT ":" Expr ;

StructFields ::= [ StructField { "," StructField } ]

StructLiteral ::= SpecificType "{" StructFields "}" ; 

GET_Request ::= "GET" Endpoint ; 

primary ::= IDENT 
    | NUMBER 
    | STRING 
    | "true"
    | "false"
    | "null"
    | StructLiteral 
    | GET_Request
    | "(" Expr ")" ;
```
