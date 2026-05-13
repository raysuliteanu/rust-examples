struct Token;
struct Ast;
struct Init;
struct Scanner(String);
struct Parser(Vec<Token>);
struct Evaluater(Vec<Ast>);
struct CompilerResult(String);

struct Compiler<S> {
    stage: S,
}

impl<Init> Compiler<Init> {
    pub fn new(source: String) -> Compiler<Scanner> {
        Compiler {
            stage: Scanner(source),
        }
    }
}

impl<Scanner> Compiler<Scanner> {
    pub fn scan(&self) -> Compiler<Parser> {
        Compiler {
            stage: Parser(Vec::new()),
        }
    }
}

impl<Parser> Compiler<Parser> {
    pub fn parse(&self) -> Compiler<Evaluater> {
        Compiler {
            stage: Evaluater(Vec::new()),
        }
    }
}

impl<Evaluater> Compiler<Evaluater> {
    pub fn evaluate(&self) -> Compiler<CompilerResult> {
        Compiler {
            stage: CompilerResult("done".to_string()),
        }
    }
}
