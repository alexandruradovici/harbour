#[derive(Debug)]
pub enum Word {
    Word (String),
    Expand (String),
    Execute (Box<Command>)
}

pub type Words = Vec<Word>;
pub type Assignments = Vec<Assignment>;
pub type Parameters = Vec<Words>;

#[derive(Debug)]

pub struct Assignment {
    pub variable: String,
    pub values: Box<Words>
}

#[derive(Debug)]
pub enum OpCommand {
    AND,
    OR
}

#[derive(Debug)]
pub enum Command {
    SimpleCommand {
        words:Box<Words>,
        assignments: Box<Assignments>,
        parameters: Box<Parameters> 
    },
    PipeCommand (Box<Command>, Box<Command>),
    SequentialCommand (Box<Command>, Box<Command>),
    AndOrCommand (Box<Command>, Box<Command>, OpCommand),
}