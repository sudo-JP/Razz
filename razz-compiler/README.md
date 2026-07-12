# Razz Compiler 
Please see `docs/` for documentation on the language and `examples/` for sample language usage. 

## Pipeline 
```Source Code -> Lexer -> Parser -> AST -> Type Check -> SSA IR -> HIR -> Codegen```

Yes I handwrote all of this myself, even the docs and the `====== LEXER =====`

## TODO: 
- Change String -> String Interner for perf and measure it 

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

### SSA IR 
- I Initially wanted to write TAC lowering. However, I saw that SSA provides a better optimization passes for things like DCE, etc so now I have to read a paper to transform AST -> SSA. 
- This is the part that I wish to write the compiler in OCaml, or Haskell. I genuinely hates the borrow checker in this part, usually I'm fine (and happy with it being around), but this time, the borrow checker hinders me write the code accordingly to the paper. 
- The greatest test of patience. 
- This by far the greatest pain to do in this project. I'm constantly coming back here to fix bugs and I hate it. I want to write a part, fix bugs, move on and forget about it. But SSA lowerer is unforgiving. 
- To reiterate, this is part really, REALLY demotivates me. It is the part holding me back, yet I have to admit, very rewarding to get it working. 

### HIR 
- The reason why HIR is needed is because my target language is high level language, like Rust, or Python. With SSA, there are bunch of gotos so it doesn't translate well to these languages. Though I think codegen to C would be easy compared to Rust from SSA, but I'm not doing that yet. 
- Then why even bother with SSA where HIR prints nicely to high level language? Aka why not do AST -> HIR? It's because I hate myself. Jokes, but SSA is great for optimization, HIR structure is too nested for optimization. 
- Unexpected but expected situation where BFS and DFS apply here. DFS is for finding the the cycle in the CFG con struct loops, and BFS is for finding meaningful convergence path of a diverged node. 
- This is where I learn compiler is the greatest test of patience, as said above. A lot of the IR stuff, I have to take 2, or even 3 steps back, just so I can jump 4 steps ahead. 
- Nuances like readjusting Phi args, where it came from, resolving the entire structure, etc. Supporting arrays for this language will be a great pain.
- Although I gotta admit, it's pretty fun to see my algorithms knowledges (specifically graph) are being applied here. If you read through the code for structurizing, it's filled with BFS/DFS. 
- Genuinely the best compiler eng ever. 
- The above is a lie, I have to pair program with Claude to figure out some stuff. Hats off to people who wrote compiler from scratch, they are the insane one.  
- But that being said, it's not like I don't think at all. I had tons of fun performing some cool tricks to structurize, such as inlining phi to if expr, DFS to find loop, BFS to find conv path etc. Truly toxic relationship between me and compiler. Though I think that goes for ever complex program if I wanna make them. Gonna plan to write an OS for this next, why? I hate myself ofc. But asides from hating myself, I really -- yes really -- enjoys the fun of solving some puzzle that randomly came up in my code. 

## Summary overall feelings throughout this journey

## AI Usage 
I asks AI to help me write tests for me, code review and fix my own bugs when tests failed. I tried to tell the AI to treat it like black box. It's not like I don't use AI at all, but I'm very frugal in the sense that I'll continue to use free services until the end of time. That being said, I use Claude to basically do pair programming, where I think, implement, and ask for ideas when problems arise. 
