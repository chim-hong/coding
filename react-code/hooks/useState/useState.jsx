const MyReact = (
  () => {
    const states = [], statesSetters = [];
    let stateIndex = 0;
    function createStates(initialState, stateIndex) {
      // 判断传入的state是否存在，不存在的时候使用initialState初始化
      return states[stateIndex] = states[stateIndex] !== undefined ? states[stateIndex] : initialState;
    }

    function createStateSetters(stateIndex) {
      return function (newState) {
        if (typeof newState === 'function') {
          states[stateIndex] = newState(states[stateIndex]);
        } else {
          states[stateIndex] = newState;
        }
        render();
      };
    }

    function useState(initialState) {
      states[stateIndex] = createStates(initialState, stateIndex);
      if (!statesSetters[stateIndex]) {
        statesSetters.push(createStateSetters(stateIndex));
      }
      const state = states[stateIndex], stateSetter = statesSetters[stateIndex];
      stateIndex++;
      return [state, stateSetter];
    }

    function render() {
      stateIndex = 0;
      ReactDOM.render(<App />, document.getElementById('app'));
    }
    return {
      useState
    };
  }
)();

const { useState } = MyReact;

function App() {
  const [count, setCount] = useState(0);
  const [flag, setFlag] = useState(false);
  return (
    <>
      <div>{count}</div>
      <h2>{flag ? '开' : '关'}</h2>
      <button onClick={() => setCount(count + 1)}>add</button>
      <button onClick={() => setCount((_count) => _count - 1)}>minus</button>
      <button onClick={() => setFlag((_flag) => !_flag)}>{flag ? '开' : '关'}</button>
    </>
  );
}

ReactDOM.render(<App />, document.getElementById('app'))


