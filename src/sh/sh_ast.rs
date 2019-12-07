#[derive(Debug)]
pub enum Redirect {
    Output {
        words: WordsPtr,
        append: bool
    },
    Error {
        words: WordsPtr,
        append: bool
    },
    Input {
        words: WordsPtr,
    },
    OutputAndError {
        words: WordsPtr,
        append: bool
    }
}

#[derive(Debug)]
pub enum Word {
    Word (String),
    Quotes (String),
    Expand (String),
    Execute (Box<Command>)
}

pub type WordPtr = Box<Word>;

pub type Words = Vec<WordPtr>;
pub type WordsPtr = Box<Words>;

pub type Assignments = Vec<AssignmentPtr>;
pub type AssignmentsPtr = Box<Assignments>;

pub type Parameters = Vec<WordsPtr>;
pub type ParametersPtr = Box<Parameters>;

pub type RedirectPtr = Box<Redirect>;

pub type Redirects = Vec<RedirectPtr>;
pub type RedirectsPtr = Box<Redirects>;

#[derive(Debug)]

pub struct Assignment {
    pub variable: String,
    pub values: WordsPtr
}

pub type AssignmentPtr = Box<Assignment>;

#[derive(Debug)]
pub enum Command {
    SimpleCommand {
        assignments: AssignmentsPtr,
        parameters: ParametersPtr,
        redirects: RedirectsPtr
    },
    PipeCommand (Box<Command>, Box<Command>),
    SequentialCommand (Box<Command>, Box<Command>),
    AndCommand (Box<Command>, Box<Command>),
    OrCommand (Box<Command>, Box<Command>)
}

pub type CommandPtr = Box<Command>;
