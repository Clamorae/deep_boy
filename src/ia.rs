use pyo3::prelude::*;
use pyo3::types::PyTuple;


pub struct Ia{}

impl Ia {
    pub fn hellow_world() -> PyResult<()> {
        Python::with_gil(|py| {
            let fun: Py<PyAny> = PyModule::from_code(
                py,
                "import numpy as np
            from pynput.keyboard import Key, Controller

            def makeAction(state,action):
                keyboard = Controller()
                alpha = 0.3
                gamma = 0.9

                keyboard.press(action)
                keyboard.release(action)

                Q[state, action] = Q[state, action] + alpha * (reward + gamma * np.max(Q[new_state, :]) — Q[state, action])
                Return(Q)


            def eGreedy(state):
                import random

                epsilon = 0.2
                possible_action = [down,right,left,a,b]

                if random.uniform(0, 1) < epsilon:
                    random.shuffle(possible_action)
                    makeAction(state,possible_action[0])
                else:
                    'call makeAction option opti'


            state_size = [[]]
            Q = np.zeros((state_size, action_size))
            eGreedy(state)",
                "",
                "",
            )?.getattr("example")?.into();

            // call object without empty arguments
            fun.call0(py)?;

            // call object with PyTuple
            let args = PyTuple::new(py, &[arg1, arg2, arg3]);
            fun.call1(py, args)?;

            // pass arguments as rust tuple
            let args = (arg1, arg2, arg3);
            fun.call1(py, args)?;
            Ok(())
        })
    }
}
