use super::context::TaskContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PriorityClass {
    /// Audio, VR, input ; ne peut pas être préempté par les autres classes
    RealTime    = 0,
    /// UI, fenêtre active, curseur, vidéo
    Interactive = 1,
    /// Compilation, rendu, calcul
    Productive  = 2,
    /// Sync, indexation, analytics
    Background  = 3,
}

/// État d'une tâche dans le scheduler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Prête à s'exécuter
    Ready,
    /// En cours d'exécution sur ce CPU
    Running,
    /// Bloquée ;attend un message IPC, un timer, etc
    Blocked,
    /// Terminée, slot réutilisable
    Dead,
}

/// Identifiant unique d'une tâche
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskId(pub u32);

/// Taille de pile par tâche : 2 pages à 8 KB
pub const TASK_STACK_SIZE: usize = 4096 * 4;

#[repr(C, align(16))]
pub struct TaskStack {
    pub bytes: [u8; TASK_STACK_SIZE],
}

impl TaskStack {
    pub const fn new() -> Self {
        Self { bytes: [0; TASK_STACK_SIZE] }
    }

    pub fn top_addr(&self) -> u64 {
        self.bytes.as_ptr() as u64 + TASK_STACK_SIZE as u64
    }
}

pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub priority: PriorityClass,
    pub context: TaskContext,
    pub ticks: u64,
    pub name: &'static str,
}

impl Task {
    pub fn new(
        id: TaskId,
        name: &'static str,
        priority: PriorityClass,
        entry: u64,
        stack_top: u64,
    ) -> Self {
        Self {
            id,
            name,
            state: TaskState::Ready,
            priority,
            context: TaskContext::new_kernel(entry, stack_top),
            ticks: 0,
        }
    }
}