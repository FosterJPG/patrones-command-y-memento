pub trait Command {
    fn execute(&mut self) -> bool;
    fn undo(&mut self);
}
