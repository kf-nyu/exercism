/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    inputs: Vec<T>,
    computes: Vec<ComputeCell<'a, T>>,
    next_callback_id: usize,
}

type ComputeFn<'a, T> = Box<dyn Fn(&[T]) -> T + 'a>;
type CallbackFn<'a, T> = Box<dyn FnMut(T) + 'a>;

pub struct ComputeCell<'a, T> {
    value: T,
    dependencies: Vec<CellId>,
    compute_func: ComputeFn<'a, T>,
    callbacks: Vec<(CallbackId, CallbackFn<'a, T>)>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            computes: Vec::new(),
            next_callback_id: 0_usize,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.inputs.push(initial);
        println!("{:?}", self.inputs.len());
        InputCellId(self.inputs.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let mut v = Vec::new();
        for cell in dependencies {
            if let Some(cell_value) = self.value(*cell) {
                v.push(cell_value)
            } else {
                return Err(*cell);
            }
        }
        let output = ComputeCell {
            value: compute_func(&v),
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            callbacks: Vec::new(),
        };
        self.computes.push(output);
        Ok(ComputeCellId(self.computes.len() - 1))
    }

    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(input_id) => self.inputs.get(input_id.0).copied(),
            CellId::Compute(compute_id) => self.computes.get(compute_id.0).map(|cell| cell.value),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(cell) = self.inputs.get_mut(id.0) {
            *cell = new_value;
            for i in 0..self.computes.len() {
                let dep_values: Vec<T> = self.computes[i]
                    .dependencies
                    .iter()
                    .filter_map(|&dep_id| self.value(dep_id))
                    .collect();
                let updated_value = (self.computes[i].compute_func)(&dep_values);
                let old_value = self.computes[i].value;
                self.computes[i].value = updated_value;
                if old_value != updated_value {
                    for j in 0..self.computes[i].callbacks.len() {
                        (self.computes[i].callbacks[j].1)(updated_value)
                    }
                }
            }
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if let Some(cell) = self.computes.get_mut(id.0) {
            let id = CallbackId(self.next_callback_id);
            self.next_callback_id += 1;
            cell.callbacks.push((id, Box::new(callback)));
            Some(id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(found_cell) = self.computes.get_mut(cell.0) {
            if let Some(found_callback) = found_cell
                .callbacks
                .iter()
                .position(|(id, _)| *id == callback)
            {
                let _ = found_cell.callbacks.remove(found_callback);
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
