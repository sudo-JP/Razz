# Razz Compiler 
Please see `docs/` for documentation on the language and `examples/` for sample language usage. 

## Pipeline 
```Source Code -> Lexer -> Parser -> AST -> Type Check -> IR -> Codegen```

Yes I handwrote all of this myself, even the docs and the `====== LEXER =====`

## Journal 
I should've written this earlier. 

Couple of refactorings were done to the compiler, each pipeline stage poses some problems. 

### Lexer: 
- Had to change from `Vec<char>` to `Vec<u8>` to save memory space. 
- Initial lexer only tracks where a token starts, not end. So I had to refactor span checking logic so that it tracks both and store it as a token span. This is one of the bigger changes I had to do. 

### AST: 
- Initially, the enum were Expr and Stmt, with no span, or ID. That is very troublesome later on, as I had to spend about 3 days (well a commit a day each) to refactor the entire parser. 
- For things like `foo->bar = 1;` or `foo->bar += 1;`, I Initially had Stmt enum as `AssignObject{ name, expr }`, which is actually worse because I should generalize these ideas. In the end, I refactored it down to `Assign { target, type, value }`, where I leave the job of validating to my semantic analyzer. 
- Each node in the enum was previous wrapped in `Spanned<T>` for better span tracking, this was solved by introducing `Expr` and `Stmt` structs.
- Walkable trait had to be refactored and added more grainular traits, so that I can perform semantic analysis on it. 

### Parser: 
- Oh this one was terrible to do. I had to convert returning a `Spanned<Stmt>` to returning new `Stmt` structs, same with expression. Then I need to assign each node an ID, this is because semantic analysis needs to construct a table to say what node to have each type. This is not like python where I can return whatever, the trait itself doesn't allow return, only traversing. So constructing a hashmap to tell the type cleaning introduces recursion-like feel. 
- I also had to refactor `AssignObject` down to just `Assign`, took some work too. 
- My parser uses LL(2) parsing, although it definitely could be reduced down to LL(1). It works right now so I'm not going to touch it for a good while. I'll come back to write Pratt Parser for expression. 
- Semantic Analysis: 
- Symbol table takes in owned String, it requires allocation each time. I tried to use lifetime specifier but it didn't work because there's more ownership stuff. I might use arena allocator or something like that later on, or find a better way to elegantly resolve this issue. 
- Type union is annoying as hell. I'm talking about how Sphere's material can be Dielectrics, Metal, or Lambertian, etc. Made a "phantom" Material enum so we can match on it with `impl satisfied()`.

### IR 
- I Initially wanted to write TAC lowering. However, I saw that SSA provides a better optimization passes for things like DCE, etc so now I have to read a paper to transform AST -> SSA. 
- This is the part that I wish to write the compiler in OCaml, or Haskell. I genuinely hates the borrow checker in this part, usually I'm fine (and happy with it being around), but this time, the borrow checker hinders me write the code accordingly to the paper. 

## AI Usage 
I asks AI to help me write tests for me, code review and fix my own bugs when tests failed. I tried to tell the AI to treat it like black box.
