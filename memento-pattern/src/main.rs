slint::include_modules!();

use std::cell::RefCell;
use std::rc::Rc;

// ═══════════════════════════════════════════════════════════════
//  PATRÓN MEMENTO 
// ═══════════════════════════════════════════════════════════════


// ── Memento ─────────────────────────────────────────────────────
// GetState(), SetState(), state
pub struct Memento {
    state: String,
}

impl Memento {
    /// GOF: GetState()
    pub fn get_state(&self) -> &str {
        &self.state
    }

    /// GOF: SetState()
    pub fn set_state(&mut self, s: String) {
        self.state = s;
    }
}


// ── Originator ──────────────────────────────────────────────────
//   CreateMemento() -> return new Memento(state)
//   SetMemento(m)   -> state = m -> GetState()
//   campo: state
pub struct Originator {
    state: String,   // GOF: state
}

impl Originator {
    pub fn new() -> Self {
        Self { state: String::new() }
    }

    /// Lectura del estado actual (necesario para la UI)
    pub fn get_current_state(&self) -> &str {
        &self.state
    }

    /// Escritura directa del estado (cuando el usuario escribe)
    pub fn set_state(&mut self, s: String) {
        self.state = s;
    }

    pub fn create_memento(&self) -> Memento {
        Memento { state: self.state.clone() }
    }

    pub fn set_memento(&mut self, m: &Memento) {
        self.state = m.get_state().to_string();
    }
}


// ── Caretaker ───────────────────────────────────────────────────
pub struct Caretaker {
    memento: Memento,
}

impl Caretaker {
    /// Crea un Caretaker a partir de un Memento ya existente
    pub fn new(memento: Memento) -> Self {
        Self { memento }
    }

    /// Devuelve referencia al Memento custodiado (para que el Originator lo use)
    pub fn get_memento(&self) -> &Memento {
        &self.memento
    }
}


// ═══════════════════════════════════════════════════════════════
//  HISTORIAL
//  Usa Vec<Caretaker> para undo y Vec<Caretaker> para redo
//  Cada Caretaker custodia exactamente un Memento — fiel al GOF
// ═══════════════════════════════════════════════════════════════
struct History {
    originator: Originator,
    undo_stack: Vec<Caretaker>,
    redo_stack: Vec<Caretaker>,
}

impl History {
    fn new() -> Self {
        Self {
            originator: Originator::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// El usuario escribio texto nuevo
    fn push(&mut self, new_text: String) {
        if new_text == self.originator.get_current_state() {
            return;
        }
        // originator crea un memento del estado actual
        // y un nuevo caretaker lo custodia en la pila de undo
        let memento = self.originator.create_memento();
        self.undo_stack.push(Caretaker::new(memento));
        // Nueva edición invalida el redo
        self.redo_stack.clear();
        self.originator.set_state(new_text);
    }

    /// Ctrl+Z — deshace el ultimo cambio
    fn undo(&mut self) -> Option<String> {
        let undo_caretaker = self.undo_stack.pop()?;

        // Originator crea un memento del estado actual
        // y un Caretaker lo guarda en redo
        let redo_memento = self.originator.create_memento();
        self.redo_stack.push(Caretaker::new(redo_memento));

        // Originator se restaura desde el memento del Caretaker de undo
        self.originator.set_memento(undo_caretaker.get_memento());
        Some(self.originator.get_current_state().to_string())
    }

    /// Ctrl+Y — rehace usando el patron memento
    fn redo(&mut self) -> Option<String> {
        let redo_caretaker = self.redo_stack.pop()?;

        // originator crea un Memento del estado actual
        // y un Caretaker lo guarda en undo
        let undo_memento = self.originator.create_memento();
        self.undo_stack.push(Caretaker::new(undo_memento));

        // Originator se restaura desde el Memento del Caretaker de redo
        self.originator.set_memento(redo_caretaker.get_memento());
        Some(self.originator.get_current_state().to_string())
    }

    fn can_undo(&self) -> bool { !self.undo_stack.is_empty() }
    fn can_redo(&self) -> bool { !self.redo_stack.is_empty() }
}


// ═══════════════════════════════════════════════════════════════

fn main() -> Result<(), slint::PlatformError> {
    let ui: NotepadWindow = NotepadWindow::new()?;
    let history = Rc::new(RefCell::new(History::new()));

    // El usuario escribio
    {
        let h = history.clone();
        let ui_weak = ui.as_weak();
        ui.on_text_changed(move |new_text| {
            let mut hist = h.borrow_mut();
            hist.push(new_text.to_string());
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_can_undo(hist.can_undo());
                ui.set_can_redo(hist.can_redo());
            }
        });
    }

    // Ctrl+Z / boton Deshacer
    {
        let h = history.clone();
        let ui_weak = ui.as_weak();
        ui.on_undo_requested(move || {
            let mut hist = h.borrow_mut();
            if let Some(text) = hist.undo() {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_content(slint::SharedString::from(text.as_str()));
                    ui.set_can_undo(hist.can_undo());
                    ui.set_can_redo(hist.can_redo());
                }
            }
        });
    }

    // Ctrl+Y / boton rehacer
    {
        let h = history.clone();
        let ui_weak = ui.as_weak();
        ui.on_redo_requested(move || {
            let mut hist = h.borrow_mut();
            if let Some(text) = hist.redo() {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_content(slint::SharedString::from(text.as_str()));
                    ui.set_can_undo(hist.can_undo());
                    ui.set_can_redo(hist.can_redo());
                }
            }
        });
    }

    ui.run()
}
