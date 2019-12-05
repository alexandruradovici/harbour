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
    Expand (String),
    Execute (Box<Command>),
    Redirection (Box<Redirect>)
}

pub type WordPtr = Box<Word>;

pub type Words = Vec<WordPtr>;
pub type WordsPtr = Box<Words>;

pub type Assignments = Vec<AssignmentPtr>;
pub type AssignmentsPtr = Box<Assignments>;

#[derive(Debug)]

pub struct Assignment {
    pub variable: String,
    pub values: WordsPtr
}

pub type AssignmentPtr = Box<Assignment>;

#[derive(Debug)]
pub enum OpCommand {
    AND,
    OR
}

#[derive(Debug)]
pub enum Command {
    SimpleCommand {
        assignments: AssignmentsPtr,
        parameters: WordsPtr,
        redirects: WordsPtr
    },
    PipeCommand (Box<Command>, Box<Command>),
    SequentialCommand (Box<Command>, Box<Command>),
    AndOrCommand (Box<Command>, Box<Command>, OpCommand),
}

pub type CommandPtr = Box<Command>;

pub fn get_redirects (r:WordsPtr, w:WordsPtr) -> () {
    for word in (*w).iter () {
        // if let Word::Redirection (redirect) = word {
        //     match **redirect {
        //         Redirect::Output {words, append: _} => get_redirects (r, &*words),
        //         Redirect::Error {words, append: _} => get_redirects (r, &*words),
        //         Redirect::OutputAndError {words, append: _} => get_redirects (r, &*words),
        //         Redirect::Input {words} => get_redirects (r, &*words)
        //     };
        //     println! ("{:#?}", redirect);
        //     // r.push (r);
        // }
        println! ("{:#?}", word);
    }
}